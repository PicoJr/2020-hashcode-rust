use clap::{App, Arg};

const VERSION: &str = "0.1.0";

pub fn get_app() -> App<'static, 'static> {
    App::new("bswp-cli")
        .version(VERSION)
        .author("PicoJr")
        .about("Solve hashcode")
        .arg(
            Arg::with_name("input")
                .takes_value(true)
                .multiple(true)
                .required(true)
                .min_values(1)
                .help("input file"),
        )
}
