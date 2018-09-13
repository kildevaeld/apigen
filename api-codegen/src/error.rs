use api_analyzer;
use std::io;

error_chain!{
    foreign_links {
        Io(io::Error) #[doc = "Error during IO"];
        Analyzer(api_analyzer::AnalyzerError) #[doc = "Error during analyzation"];
    }

    errors {
        Resolve(path: String) {
            description("could not resolve path")
            display("unable to resolve path {}", path)
        }
    }
}
