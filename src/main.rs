use std::path::PathBuf;

use clap::{Parser, Subcommand};

use overture::project;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "help for build")]
    Build,
    #[command(about = "help for init")]
    Init {
        #[arg(short, long)]
        root: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => {
            println!("unimplemented build command");
        }

        Commands::Init { root } => {
            let root_path_buf = PathBuf::from(root);
            let prj = project::Project::new(root_path_buf);
            // prj.create()
            match prj.create() {
                Ok(_) => println!("Project created successfully"),
                Err(e) => println!("Error creating project: {}", e),
            }
        }
    }
}
