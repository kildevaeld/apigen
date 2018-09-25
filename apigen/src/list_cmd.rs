use api_analyzer::default_passes;
use api_codegen::{transform_package, write_package};
use api_codegen_rust::RustPlugin;
use api_core::{Core, GeneratorBuilder};
use clap::ArgMatches;
use error::Result;
use std::boxed::Box;

pub fn list_cmd(args: &ArgMatches) -> Result<()> {
    let mut core = Core::new().search_path("target/debug").build();

    // core.repository_mut()
    //     .add_plugin(Box::new(RustPlugin::default()));

    core.repository_mut().load()?;

    let generarors = core.repository().list();

    for gen in generarors {
        println!("{}", gen.name());
    }

    // let gen_name = args.value_of("generator").unwrap();

    // let mut builder = GeneratorBuilder::new(input, gen_name);
    // if args.is_present("output") {
    //     builder.dest(args.value_of("output").unwrap());
    // }

    // core.gen_code(builder)?;

    Ok(())
}
