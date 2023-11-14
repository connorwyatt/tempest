use std::fs::read_to_string;

use clap::Parser;

use crate::data_model::DataModel;

mod data_model;
mod renderer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    input_file_path: String,

    #[arg()]
    output_file_path: String,
}

fn main() {
    let args = Args::parse();

    let data_model = match read_to_string(args.input_file_path) {
        Ok(file) => DataModel::from(file),
        Err(e) => panic!("Could not parse file: {}", e),
    };

    renderer::render(&data_model, args.output_file_path).expect("should be able to render file");
}
