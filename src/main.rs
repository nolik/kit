extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let arg_matches = App::new("kafka-cli-helper")
        .version("1.0")
        .about("Kafka cli helper")
        .arg(Arg::with_name("ip")
            .short("ip")
            .long("kafka ip, localhost by default")
            .value_name("ip")
            .help("Sets kafka ip with port")
            .default_value("localhost"))
        .subcommand(SubCommand::with_name("operation")
            .about("operation testing features")
            .arg(Arg::with_name("debug")
                .short("c")
                .help("print debug information verbosely")))
        .get_matches();

//    CLI logic
    println!("argument matches: {:?}", arg_matches);
    if let Some(ip) = arg_matches.value_of("ip") {
        println!("Some kafka ip: {}", ip);
    }
}