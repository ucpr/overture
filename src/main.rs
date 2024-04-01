use clap::{Parser, Subcommand};

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
    Init,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => {
            println!("unimplemented build command");
        }
        Commands::Init => {
            println!("unimplemented init command");
        }
    }
}
