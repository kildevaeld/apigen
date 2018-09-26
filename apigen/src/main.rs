extern crate api_parser;
extern crate api_analyzer;
extern crate api_codegen;
extern crate api_codegen_rust;
extern crate api_core;
extern crate serde_json;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

// #[macro_use]
extern crate log;
extern crate env_logger;


mod ast_cmd;
use ast_cmd::ast_cmd;
mod gen_cmd;
use gen_cmd::gen_cmd;
mod list_cmd;
use list_cmd::list_cmd;
mod common;
mod error;

fn main() -> error::Result<()> {
     env_logger::init();

    let matches = clap_app!(apigen =>
        (version: "1.0")
        (author: "Rasmus Kildevæld")
        (about: "API generator")
        (@subcommand ast => 
            (@arg input: * "Input file")
            (@arg analyze: -a "analyze")
            (@arg output: -o [file] "output")
        )
        (@subcommand gen => 
            (@arg input: * "Input file")
            (@arg generator: -g <plugin> "generator")
            (@arg output: -o [directory] "output")
        )
        (@subcommand list =>
            
        )
    ).get_matches();


    match matches.subcommand() {
        ("ast", Some(ast_matches)) => ast_cmd(&ast_matches),
        ("gen", Some(gen_matches)) => gen_cmd(&gen_matches),
        ("list", Some(list_matches)) => list_cmd(&list_matches),
        _ => Ok(())
    }

}
