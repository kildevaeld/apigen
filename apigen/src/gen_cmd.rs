use api_analyzer::default_passes;
use api_codegen::{transform_package, write_package};
use api_codegen_rust::{RustCodeGenerator, RustPass};
use clap::ArgMatches;
use error::Result;
use std::boxed::Box;

pub fn gen_cmd(args: &ArgMatches) -> Result<()> {
    let input = args.value_of("input").unwrap();

    let rust = RustCodeGenerator::new();

    let mut passes = default_passes();
    passes.push(Box::new(RustPass::new()));

    let package = transform_package(input, &rust, &passes).unwrap();
    write_package(&package, "test-ouput")?;
    println!("packages {:?}", package.len());

    // let ast = analyze_file(input, &passes)?;

    // let rust = RustCodeGenerator::new();

    // let o = rust.transform(&ast).unwrap();

    // println!("{}", o[0].as_str());

    Ok(())
}
