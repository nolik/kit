extern crate clap;

use clap::{Arg, App, SubCommand};
use std::env::args;

fn main() {
    let matches = App::new("kafka-cli-helper")
        .version("1.0")
        .about("Kafka cli tools helper")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .subcommand(SubCommand::with_name("operation")
            .about("operation testing features")
            .arg(Arg::with_name("debug")
                .short("c")
                .help("print debug information verbosely")))
        .get_matches();

//    CLI logic
    match args {
        _ => {
            println!("here should be implemented li logic")
        }
    }
}