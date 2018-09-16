use api_analyzer;
use api_codegen;
use api_extensions;
use api_parser;
//use plugin_manager;
use std::io;

error_chain!{
    foreign_links {
        Io(io::Error) #[doc = "Error during IO"];
        Analyzer(api_analyzer::AnalyzerError) #[doc = "Error during analyzation"];
        Parser(api_parser::ParserError);
    }

    links {
        Codegen(api_codegen::Error, api_codegen::ErrorKind);
        Plugin(api_extensions::plugins::Error, api_extensions::plugins::ErrorKind);

    }

    errors {
        Pass(name: String) {
            description("pass not found")
            display("unable to resolve path {}", name)
        }
    }
}
