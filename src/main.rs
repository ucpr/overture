use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // init command is used to initialize a new project
    #[arg(short, long)]
    init: String,

    // build command is used to build the project
    #[arg(short, long)]
    build: String,

    // serve command is used to serve the project on local server
    #[arg(short, long)]
    serve: String,
}

fn main() {
    let args = Args::parse();

    if args.serve != "" {
        println!("Serving on port {}", args.serve);
    }
    if args.build != "" {
        println!("Building for {}", args.build);
    }
    if args.init != "" {
        println!("Initializing a new project in {}", args.init);
    }
}
