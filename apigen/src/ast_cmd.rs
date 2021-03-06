use api_analyzer::{analyze_file, default_passes};
use api_parser::build_ast;
use clap::ArgMatches;
use common::read_file;
use error::Result;
use serde_json;
use std::fs::File;
use std::io::{self, Write};

pub fn ast_cmd(args: &ArgMatches) -> Result<()> {
    let input = args.value_of("input").unwrap();
    let mut writer: Box<dyn io::Write>;

    if args.is_present("output") {
        let output_path = args.value_of("output").unwrap();
        writer = Box::new(File::create(output_path)?);
    } else {
        writer = Box::new(io::stdout());
    }

    if args.is_present("analyze") {
        let ast = analyze_file(input, &default_passes())?;
        let json = serde_json::to_string_pretty(&ast).unwrap();
        writer.write(format!("{}", json).as_str().as_bytes())?;
    } else {
        let ast = build_ast(read_file(input)?.as_str())?;
        let json = serde_json::to_string_pretty(&ast).unwrap();
        writer.write(format!("{}", json).as_str().as_bytes())?;
    }

    Ok(())
}
