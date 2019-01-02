extern crate clap;
extern crate rand;

use clap::{App, Arg};
use world_map_gen::{draw, gen};
use std::fmt;

enum Error {
    GenFail(world_map_gen::error::Error),
    CliParseFail { name: String, msg: String },
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::GenFail(e) => write!(f, "{}", e),
            Error::CliParseFail { name, msg } => {
                write!(f, "Cannot parse CLI option '{}': {}", name, msg)
            }
        }
    }
}

impl From<world_map_gen::error::Error> for Error {
    fn from(err: world_map_gen::error::Error) -> Error {
        Error::GenFail(err)
    }
}

fn parse_opt<T>(name: &str, opt: Option<&str>) -> Result<Option<T>, Error>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: fmt::Display,
{
    match opt {
        Some(s) => match s.parse::<T>() {
            Ok(v) => Ok(Some(v)),
            Err(e) => Err(Error::CliParseFail {
                name: name.to_string(),
                msg: format!("{}", e),
            }),
        },
        None => Ok(None),
    }
}

fn main() -> Result<(), Error> {
    let matches = App::new("game-map-gen")
        .version("0.1")
        .author("rhysd <https://rhysd.github.io>")
        .about("Random game map generator")
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .value_name("INTEGER")
                .help("Seed for random number generator"),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("INTEGER")
                .help("Board width in number of cells"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("INTEGER")
                .help("Board height in number of cells"),
        )
        .arg(
            Arg::with_name("scale")
                .long("scale")
                .value_name("STRING")
                .possible_values(&["small", "middle", "large"])
                .help("Map scale"),
        )
        .arg(
            Arg::with_name("altitude")
                .short("a")
                .long("altitude")
                .help("Show altitude instead of squre as cell mainly for debug"),
        )
        .get_matches();

    let seed = parse_opt("seed", matches.value_of("seed"))?;
    let width = parse_opt("width", matches.value_of("width"))?;
    let height = parse_opt("height", matches.value_of("height"))?;
    let scale = matches.value_of("scale").map(|s| match s {
        "small" => gen::Scale::Small,
        "middle" => gen::Scale::Middle,
        "large" => gen::Scale::Large,
        _ => unreachable!(),
    });

    let board = if let Some(seed) = seed {
        gen::RandomBoardGen::from_seed(seed).gen(scale, width, height)?
    } else {
        gen::RandomBoardGen::new().gen(scale, width, height)?
    };

    draw::draw_term(&board, matches.is_present("altitude"))?;

    Ok(())
}
