use crate::cli_args::{CliArgs, Action};

mod cli_args;

fn main() {
    let cli_args: CliArgs = CliArgs::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", cli_args.config);
    println!("Using action: {}", cli_args.action);

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match cli_args.action {
        Action::Listen { target: String::from("topic_name") } => {
            println!("Listen topic")
        }
    }
}