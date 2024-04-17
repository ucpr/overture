use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tokio;

use overture::builder;
use overture::project;
use overture::server;
// use overture::rss;

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
    /*
        let channel = rss::example_feed().await.unwrap();
        println!("Title: {}", channel.title);
        for item in channel.items.iter() {
            println!("Item: {}", item.title().unwrap());
            println!("Link: {}", item.link().unwrap());
            println!("Date: {}", item.pub_date().unwrap());
        }
    */

    let cli = Cli::parse();
    match cli.command {
        Commands::Build => {
            let builder = builder::Builder::new();

            match builder.build().await {
                Ok(_) => println!("Build successful"),
                Err(_) => println!("Error building project"),
            }
        }

        Commands::Serve { port } => {
            let builder = builder::Builder::new();

            match builder.build().await {
                Ok(_) => {
                    let server = server::Server::new("0.0.0.0".to_string(), port);
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
