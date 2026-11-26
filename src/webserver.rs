use std::{fs, println};
use axum::body::{to_bytes};
use axum::response::Html;
use std::sync::{LazyLock};
use std::sync::Mutex;
use std::collections::HashMap;
use tokio::sync::Mutex as TokioMutex;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::{any},
    Router,
    response::{IntoResponse},
};

use std::path::{PathBuf};

use crate::services::pathservice::PathService;
use crate::services::processservice::ProcessService;
use crate::services::responseservice::ResponseService;
use crate::structs::routesettings::{RouteSettings};
use crate::structs::routerequest::{RouteRequest};
use crate::structs::asyncprocess::AsyncProcess;
use crate::utils::json::{match_json, walk_json};
use crate::utils::regex::replace_path;
static ASYNC_GLOBAL_MAP: LazyLock<TokioMutex<HashMap<String, AsyncProcess>>> = LazyLock::new(|| {
    TokioMutex::new(HashMap::new())
});

static ROOT_PATH: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::new()));

fn check_json_validity_against_route_settings(body: &str, route_settings: &RouteSettings) -> bool {
    let mut body_types: HashMap<Vec<String>, serde_json::Value> = HashMap::new();
    walk_json(&route_settings.payload.as_ref().unwrap_or(&serde_json::Value::Null), vec![], &mut body_types);

    let body_json: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
    let mut actual_body_types: HashMap<Vec<String>, serde_json::Value> = HashMap::new();
    walk_json(&body_json, vec![], &mut actual_body_types);

    let mut is_valid = true;
    for b_type in body_types.keys() {
        match body_types.get(b_type) {
            Some(expected_value) => {
                match actual_body_types.get(b_type) {
                    Some(actual_value) => {
                        is_valid = match_json(&expected_value.to_string(), actual_value);
                        if !is_valid {
                            break;
                        }
                    },
                    None => {
                        is_valid = false;
                    }
                }
            },
            None => {
                is_valid = false;
            }
        }
        if !actual_body_types.contains_key(b_type) {
            is_valid = false;
        }
    }
    return is_valid;
}

async fn handle_call(req: Request<Body>) -> impl IntoResponse {
    let path = req.uri().path().to_string();
    let root_path = ROOT_PATH.lock().unwrap().clone();

    let public_directory = &root_path.join("__public__");
    let possible_public_file_path = public_directory.join(&path.trim_start_matches("/"));

    if possible_public_file_path.exists() && possible_public_file_path.is_file() {
        return ResponseService::handle_files(&possible_public_file_path).await;
    }

    let method = req.method().to_string();
    let cookies = match req.headers().get("cookie"){
        Some(cookies) => {
            cookies.to_str().unwrap_or("").to_string()
        },
        None => "".to_string(),
    };
    let bytes = to_bytes(req.into_body(), 10 * 1024 * 1024).await.unwrap();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    println!("Received request for path {}", path);
    let mut processes = ASYNC_GLOBAL_MAP.lock().await;
    
    let route_settings = ProcessService::get_route_settings(&path, &root_path);

    let process_folder = PathService::get_folder_path(&path, &ROOT_PATH.lock().unwrap()).unwrap_or(PathBuf::from(&path));

    let root_folder = if route_settings.root_folder.is_empty(){
        process_folder.clone()
    }else {
        PathBuf::from(replace_path(&route_settings.root_folder, &HashMap::from(
            [("<<output_folder>>", process_folder.to_str().unwrap())]
        )))
    } ;

    println!("Root folder: {:?}", root_folder);
    let process_path = match PathService::get_process_path(&root_folder, &route_settings){
        Some(path) => path,
        None => {
            let error_page = fs::read_to_string(root_path.join("error.html")).unwrap_or("<h1>404 Not Found</h1>".to_string());
            return (StatusCode::NOT_FOUND, Html(error_page)).into_response();
        }
    };
  
    println!("Process path: {:?}", process_path);
    let current_process = ProcessService::spawn_route_process(&path, &process_path, &root_path, &root_folder, &mut processes).await.expect("No process was made!");
    
    
    let is_valid = check_json_validity_against_route_settings(&body, &route_settings);

    let route_request = RouteRequest {
        path: path.clone(),
        body: body.clone(),
        cookies: cookies.clone(),
        method: method.clone(),
        is_valid: is_valid,
        params: PathService::get_supplied_values_from_path(&PathBuf::from(&path), &process_folder),
    };
    let _ = current_process.write(&(serde_json::to_string(&route_request).unwrap().to_string() + "\n")).await;


    let result = current_process.read().await.unwrap_or("".to_string());


    if result.is_empty() {
        let error_page = fs::read_to_string(root_path.join("error.html")).unwrap_or("<h1>404 Not Found</h1>".to_string());
        return (StatusCode::INTERNAL_SERVER_ERROR, Html(error_page)).into_response();
    }
    println!("Result from process: {}", result);
    let response = ResponseService::interpret_response(&result);
    
    return ResponseService::convert_router_response(&response)

}

pub fn start_server(root_path: PathBuf) {

    let source_path = root_path.join("output");

    *ROOT_PATH.lock().unwrap() = source_path;
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move {
        
        let app = Router::new()
            .route("/", any(
                handle_call
            ))
            // The asterisk wildcard remains the same
            .route("/{*path}", any(
                handle_call
            ));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
        println!("Server running on http://127.0.0.1:3000");
        axum::serve(listener, app).await.unwrap();
    });
}