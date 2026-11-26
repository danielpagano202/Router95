use std::fs;

use axum::{Json, body::Body, http::{Response, StatusCode, header}, response::{Html, IntoResponse}};
use tera::Tera;
use tokio_util::io::ReaderStream;

use crate::structs::{routeresponse::{ResponseType, RouteResponse}, templatedata::TemplateData};

pub struct ResponseService;

impl ResponseService {

    pub fn interpret_response(response: &str) -> RouteResponse {
        serde_json::from_str::<RouteResponse>(response).unwrap_or_else(|_| {
            println!("Failed to parse response: {}", response);
            // If parsing fails, return a default RouterResponse
            RouteResponse {
                response_code: 500,
                data: "Failed to parse response".to_string(),
                response_type: ResponseType::TEXT,
            }
        })
    }

    pub fn convert_router_response(response: &RouteResponse) -> axum::response::Response{

        let response_data = response.data.clone();

        if response.response_code > 399 {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Error from process: {}", response.data)).into_response();
        } else if response.response_code > 299 {
            let status = StatusCode::from_u16(response.response_code).unwrap_or(StatusCode::OK);
            return (status, response_data).into_response();
        }

        match response.response_type {
            ResponseType::FILE => return ResponseService::handle_file_response(response),
            ResponseType::HTMLFILE => return ResponseService::handle_html_file_response(response),
            ResponseType::JSON => return (StatusCode::OK, Json(response_data)).into_response(),
            ResponseType::TEXT => return (StatusCode::OK, response_data).into_response(),
            ResponseType::TEMPLATE => return ResponseService::handle_template_response(response),
        }
    }

    fn handle_file_response(response: &RouteResponse) -> axum::response::Response {
        let contents = fs::read_to_string(&response.data).unwrap_or_else(|_| "".to_string());
        if contents.is_empty() {
            (StatusCode::NOT_FOUND, format!("File not found: {}", response.data)).into_response()
        } else {
            (StatusCode::OK, contents).into_response()
        }
    }

    fn handle_html_file_response(response: &RouteResponse) -> axum::response::Response {
        let contents = fs::read_to_string(&response.data).unwrap_or_else(|_| "".to_string());
        if contents.is_empty() {
            (StatusCode::NOT_FOUND, format!("File not found: {}", response.data)).into_response()
        } else {
            (StatusCode::OK, Html(contents)).into_response()
        }
    }

    fn handle_template_response(response: &RouteResponse) -> axum::response::Response {
        let template_data: TemplateData = serde_json::from_str(&response.data).unwrap_or_else(|_| {
            println!("Failed to parse template data: {}", response.data);
            // If parsing fails, return a default TemplateData
            TemplateData {
                template_path: "".to_string(),
                arguments: std::collections::HashMap::new(),
            }
        });

        let contents = fs::read_to_string(&template_data.template_path).unwrap_or_else(|_| "".to_string());
        if contents.is_empty() {
            (StatusCode::NOT_FOUND, format!("Template file not found: {}", &template_data.template_path)).into_response()
        } else {
            let mut tera = Tera::default();
            let title = "base_template";
            tera.add_raw_template(title, &contents).unwrap();
            let mut context = tera::Context::new();
            for (key, value) in &template_data.arguments {
                context.insert(key.clone(), &value);
            }

            let rendered = tera.render(title, &context).unwrap_or_else(|_| "".to_string());
            if rendered.is_empty() {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to render template".to_string()).into_response();
            }
            (StatusCode::OK, Html(rendered)).into_response()
        }
    }

    pub async fn handle_files(possible_public_file_path: &std::path::PathBuf) -> axum::response::Response {
         // Pass as a reference so we can use the path variable again for mime_guess below
        let file = match tokio::fs::File::open(&possible_public_file_path).await {
            Ok(file) => file,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("File not found"))
                    .unwrap()
                    .into_response();
            }
        };

        let stream = ReaderStream::new(file);
        let body = Body::from_stream(stream);
        
        // Dynamically extract the proper file format extension string
        let mime_type = mime_guess::from_path(&possible_public_file_path)
            .first_raw()
            .unwrap_or("application/octet-stream");

        // CRUCIAL: Added "return" and replaced "application/pdf" with your dynamic mime_type variable
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime_type)
            .body(body)
            .unwrap()
            .into_response();
    }


}
