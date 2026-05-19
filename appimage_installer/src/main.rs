use std::{env, error::Error, process};
use appimage_installer;

const DEFAULT_INSTALL_LOCATION: &str = "/usr/bin";
const DEFAULT_DESKTOP_LOCATION: &str = "~/.local/share/applications";

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    appimage_installer::install_app(
        &config.appimage_path,
        &config.install_location,
    )?;
    
    Ok(())
}

struct Config {
    appimage_path: String,
    install_location: String,
    desktop_location: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let appimage_path = args[1].clone();
        let install_location = env::var("INSTALL_LOCATION").unwrap_or_else(|e| String::from(DEFAULT_INSTALL_LOCATION));
        let desktop_location = env::var("DESKTOP_LOCATION").unwrap_or_else(|e| String::from(DEFAULT_DESKTOP_LOCATION));

        Ok(Config { appimage_path, install_location, desktop_location })
    }
}