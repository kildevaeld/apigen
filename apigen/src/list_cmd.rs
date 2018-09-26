use api_core::Core;
use clap::ArgMatches;
use error::Result;

pub fn list_cmd(_args: &ArgMatches) -> Result<()> {
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
