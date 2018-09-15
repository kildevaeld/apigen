extern crate api_parser;
extern crate api_analyzer;
extern crate api_codegen;
extern crate api_codegen_rust;
extern crate serde_json;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;
extern crate env_logger;

// use apigen_core::tokenize;
// use std::fs::File;
// use std::io::Read;

mod ast_cmd;
use ast_cmd::ast_cmd;
mod gen_cmd;
use gen_cmd::gen_cmd;
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
            (@arg analyze: -g <plugin> "analyze")
            (@arg output: -o [directory] "output")
        )
    ).get_matches();


    match matches.subcommand() {
        ("ast", Some(ast_matches)) => ast_cmd(&ast_matches),
        ("gen", Some(gen_matches)) => gen_cmd(&gen_matches),
        _ => Ok(())
    }

    // let mut unparsed_file = String::new();
    // File::open("examples/arangodb/arangodb.api")
    //     .expect("cannot open file")
    //     .read_to_string(&mut unparsed_file)
    //     .expect("cannot read file");

    // let pairs = tokenize(&unparsed_file).unwrap();

    // let ast = apigen_core::build_ast(&unparsed_file).unwrap();
    // let serialized = serde_json::to_string_pretty(&ast).unwrap();
    // println!("{}", serialized);
    // return;
    // for pair in pairs {
    //     let span = pair.clone().into_span();
    //     // A pair is a combination of the rule which matched and a span of input
    //     println!("Rule:    {:?}", pair.as_rule());
    //     println!("Span:    {:?}", span);
    //     //println!("Text:    {}", span.as_str());

    //     // A pair can be converted to an iterator of the tokens which make it up:
    //     for inner_pair in pair.into_inner() {
    //         let inner_span = inner_pair.clone().into_span();
    //         println!("  Rule:    {:?}", inner_pair.as_rule());
    //         println!("  Span:    {:?}", inner_span);
    //         //println!("  Text:    {}", inner_span.as_str());
    //         for iinner_pair in inner_pair.into_inner() {
    //             let iinner_span = iinner_pair.clone().into_span();
    //             println!("    Rule:    {:?}", iinner_pair.as_rule());
    //             println!("    Span:    {:?}", iinner_span);
    //             println!("    Text:    {}", iinner_span.as_str());

    //             for iiinner_pair in iinner_pair.into_inner() {
    //                 let iiinner_span = iiinner_pair.clone().into_span();
    //                 println!("      Rule:    {:?}", iiinner_pair.as_rule());
    //                 println!("      Span:    {:?}", iiinner_span);
    //                 println!("      Text:    {}", iiinner_span.as_str());

    //                 for iiiinner_pair in iiinner_pair.into_inner() {
    //                     let iiiinner_span = iiiinner_pair.clone().into_span();
    //                     println!("        Rule:    {:?}", iiiinner_pair.as_rule());
    //                     println!("        Span:    {:?}", iiiinner_span);
    //                     println!("        Text:    {}", iiiinner_span.as_str());
    //                 }
    //             }
    //         }
    //     }
    // }
}
