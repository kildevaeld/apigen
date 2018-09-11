use api_analyzer::{analyze_file, default_passes};
use api_codegen::transform_package;
use api_codegen_rust::{RustCodeGenerator, RustPass};
use api_parser::build_ast;
use clap::ArgMatches;
use common::read_file;
use error::Result;
use serde_json;
use std::fs::File;
use std::io::{self, Write};

pub fn gen_cmd(args: &ArgMatches) -> Result<()> {
    let input = args.value_of("input").unwrap();

    let rust = RustCodeGenerator::new();
    let package = transform_package(input, &rust).unwrap();
    println!("packages {:?}", package.len());
    // let mut passes = default_passes();
    // passes.push(Box::new(RustPass::new()));

    // let ast = analyze_file(input, &passes)?;

    // let rust = RustCodeGenerator::new();

    // let o = rust.transform(&ast).unwrap();

    // println!("{}", o[0].as_str());

    Ok(())
}
