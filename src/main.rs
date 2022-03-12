use clap::{arg, command, Command};
mod engine;
use engine::Engine;

fn main() {
    let mut engine = Engine::new();

    let cli = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("curse")
                .about("cursing images")
                .arg(arg!([FILE_PATH])),
        )
        .get_matches();
    
    match cli.subcommand() {
        Some(("curse", curse)) => {
            let img = image::open(curse.value_of("FILE_PATH").unwrap()).unwrap();
            let output = engine.curse(img);
            output.save("test.png").unwrap();
        }

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
    // println!("Hello, world!");
}
