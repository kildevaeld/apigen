use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn read_file(input: &str) -> Result<String> {
    let mut unparsed_file = String::new();
    File::open(input)
        .expect("cannot open file")
        .read_to_string(&mut unparsed_file)
        .expect("cannot read file");
    Ok(unparsed_file)
}
