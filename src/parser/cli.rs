use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: CommandType,

    #[clap(short, long, value_name = "Args")]
    pub args: Option<u8>
}

#[derive(Subcommand)]
pub enum CommandType {
    ///Path to file - defaults to current directory
    New {
        #[clap(default_value = "./new.png")]
        path: PathBuf,
        #[clap(long, default_value_t = 100, value_name = "Width")]
        width: u32,
        #[clap(long, default_value_t = 100, value_name = "Height")]
        height: u32,
    }
}