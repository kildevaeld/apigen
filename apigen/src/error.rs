use api_analyzer;
use api_codegen;
use api_parser;
use plugin_manager;
use std::io;

error_chain!{
    foreign_links {
        Io(io::Error) #[doc = "Error during IO"];
        Analyzer(api_analyzer::AnalyzerError) #[doc = "Error during analyzation"];
        Parser(api_parser::ParserError);
    }

    links {
        Codegen(api_codegen::error::Error, api_codegen::error::ErrorKind);
        Plugins(plugin_manager::error::Error, plugin_manager::error::ErrorKind);
    }

    errors {
        Resolve(path: String) {
            description("could not resolve path")
            display("unable to resolve path {}", path)
        }
    }
}
