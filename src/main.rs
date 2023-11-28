use clap::Command;
mod build_lib;
mod server_lib;
mod init_lib;

fn main() {
    let matches = Command::new("rust-md-blog-tool")
        .version("1.0")
        .author("Your Name")
        .about("A simple static site generator")
        .subcommand(Command::new("init")
            .about("Initializes the blog with default configuration"))
        .subcommand(Command::new("build")
            .about("Builds the static site from markdown files"))
        .subcommand(Command::new("run")
            .about("Runs a local server to view the blog"))
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => init_lib::init_command(),
        Some(("build", _)) => build_lib::build_command(),
        Some(("run", _)) => server_lib::run_command(),
        _ => println!("Invalid command or no command provided"),
    }
}

