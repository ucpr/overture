use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tokio;

use overture::builder;
use overture::project;
use overture::server;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "help for build")]
    Build,

    #[command(about = "help for serve")]
    Serve {
        #[arg(short, long)]
        port: u16,
    },

    #[command(about = "help for init")]
    Init {
        #[arg(short, long)]
        root: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build => {
            let builder = builder::Builder::new().await;

            match builder.build() {
                Ok(_) => println!("Build successful"),
                Err(_) => println!("Error building project"),
            }
        }

        Commands::Serve { port } => {
            let builder = builder::Builder::new().await;

            match builder.build() {
                Ok(_) => {
                    let server = server::Server::new("127.0.0.1".to_string(), port);
                    server.serve().await
                }
                Err(_) => println!("Error building project"),
            }
        }

        Commands::Init { root } => {
            let root_path_buf = PathBuf::from(root);
            let prj = project::Project::new(root_path_buf);

            match prj.create() {
                Ok(_) => println!("Project created successfully"),
                Err(e) => println!("Error creating project: {}", e),
            }
        }
    }
}
