use clap::{arg, command};

mod common;

const DEFAULT_CONFIG_PATH: &'static str = "./src/pub/config.json";

fn main() -> anyhow::Result<()> {
    let args = command!()
        .arg(
            arg!(--config <PATH>)
                .required(false)
                .default_value(DEFAULT_CONFIG_PATH),
        )
        .get_matches();
    // .arg(arg!(--blind).required_unless_present("config"))
    // .arg(arg!(--search <TERM>).required_unless_present_any(["config", "blind"]))
    // .arg(arg!(--targets <FILEPATH>).required_unless_present_any(["config", "db"]))
    // .arg(arg!(--db).required_unless_present_any(["targets", "config"]))
    // .arg(arg!(--single).required_unless_present("config"))
    // .get_matches();

    Ok(())
}

