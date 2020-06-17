extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let arg_matches = App::new("kcli")
        .version("1.0")
        .about("Kafka cli helper")
        .arg(Arg::with_name("b")
            .short("b")
            .long("broker ip, localhost by default")
            .value_name("b")
            .help("Sets kafka broker ip with port")
            .default_value("localhost:9092"))
        .subcommand(SubCommand::with_name("operation")
            .about("operation testing features")
            .arg(Arg::with_name("consume")
                .short("c")
                .help("print debug information verbosely")))
        .get_matches();

//    CLI logic
    println!("argument matches: {:?}", arg_matches);
    if let Some(broker) = arg_matches.value_of("ip") {
        println!("Some kafka broker ip: {}", broker);
    }
}