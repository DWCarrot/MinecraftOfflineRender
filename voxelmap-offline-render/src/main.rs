mod loader;
mod framework;

use clap::{Arg, App};

fn main() {
    
    let matches = App::new("Voxelmap Cache Offline Render")
        .version("0.1.0")
        .author("RDCarrot <yyt226univ2017@yahoo.com>")
        .about("render voxelmap cache data to png")
        .arg(
            Arg::with_name("assets")
                .short("a")
                .long("assets")
                .takes_value(true)
                .help("assets archive; for example: .minecraft/versions/1.15.1/1.15.1.jar")
        )
        .arg(
            Arg::with_name("input-folder")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("cache data folder")
        )
        .arg(
            Arg::with_name("output-folder")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("output image folder")
        ).get_matches();

    let mut options = framework::AppOptions::default();

    if let Some(input) = matches.value_of("input-folder") {
        options.cache_folder = input.to_string();
    }

    if let Some(output) = matches.value_of("output-folder") {
        options.output_folder = output.to_string();
    }

    if let Some(assets) = matches.value_of("assets") {
        options.assets.push(assets.to_string());
    }

    framework::app(options).unwrap();

}

