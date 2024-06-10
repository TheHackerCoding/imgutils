use std::path::Path;

use clap::{arg, command, Command};
mod engine;
use engine::Engine;

fn main() {
    let mut engine = Engine::new();

    let cli = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .about("eh")
        .subcommand(
            Command::new("curse")
                .about("cursing images")
                .arg(arg!([FILE_PATH]))
                .arg(arg!(-l --limit <LIMIT> "limit the rng").required(false))
                .arg(
                    arg!(-d --darkness "only dark color?")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
                        .conflicts_with("lightness"),
                )
                .arg(
                    arg!(-L --lightness "only light?")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
                        .conflicts_with("darkness"),
                ),
        )
        .subcommand(
            Command::new("random").about("idk random imgs").arg(
                arg!(-w --width <WIDTH>)
                    .default_value("500")
                    .value_parser(clap::value_parser!(i32)),
            ),
        )
        .get_matches();

    match cli.subcommand() {
        Some(("random", random)) => {
            let output = engine.random(
                random.get_one::<i32>("width").copied(),
                random.get_one::<i32>("height").copied(),
            );
            output.save("random.png").unwrap()
        }
        Some(("curse", curse)) => {
            if curse.contains_id("FILE_PATH") {
                let file = Path::new(curse.get_one::<String>("FILE_PATH").unwrap());
                let img = image::open(file).unwrap();
                let output = engine.curse(
                    img,
                    curse.get_one::<String>("limit").map(|x| x.as_str()),
                    curse.get_flag("darkness"),
                    curse.get_flag("lightness"),
                );
                output
                    .save(format!(
                        "{}_cursed.png",
                        file.file_stem().unwrap().to_str().unwrap()
                    ))
                    .unwrap();
            } else {
                println!("No file path given")
            }
        }

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
    // println!("Hello, world!");
}
