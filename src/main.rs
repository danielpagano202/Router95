use std::path::{PathBuf};
use clap::{Parser, Subcommand};

mod webserver;
mod build;
mod utils;
mod structs;
mod create;
mod services;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    // 1. This directs clap to look for a subcommand next
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the server
    Run {
        /// The path to the folder you want to execute
        folder: PathBuf,
    },
    Build{
        // The path to the folder you want to build
        folder: PathBuf,
    },
    Create{

    },
}

fn main() {

    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { folder} => {
            webserver::start_server(PathBuf::from(folder));
        }
        Commands::Build { folder } => {
            println!("Building code in folder: {:?}", folder);
            build::build_code(PathBuf::from(folder));
        },
        Commands::Create { } => {
            println!("Creating new project...");
            _ = create::run();
        }
    }

    // Need to create a build step that will open the provided directory and then compile each of the files
    // And then the run will be able to run the compiled files in the provided directory
}
