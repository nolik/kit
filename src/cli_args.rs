extern crate clap;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0")]
pub struct CliArgs {
    #[clap(short = "c", long = "config", default_value = "default.conf")]
    pub config: String,

    #[structopt(subcommand)]
    pub action: Action,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub enum Action {
    #[structopt(about = "Subscribe to the topic")]
    Listen {
        #[structopt(help = "Target topic", required = true)]
        target: String,
    },
    #[structopt(about = "Produce message to the topic")]
    Produce {
        #[structopt(help = "Target topic", required = true)]
        target: String,
    },
}