use clap::{StructOpt, IntoApp};
use clap::ErrorKind;
use lightup::parser::cli::{self, CommandType};
fn main() {
    let params = cli::Cli::parse();
    match params.command {
        CommandType::New {mut path, width, height} => match path.is_file() || path.is_dir() {
            true => {
                let mut app = cli::Cli::into_app();
                app.error(ErrorKind::InvalidValue, "Entry is a known file or directory").exit();
            },
            _ => {
                lightup::form_img(&mut path, &width, &height, params.args);
            }
        }       
    }
}
