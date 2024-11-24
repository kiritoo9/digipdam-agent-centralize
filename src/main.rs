use clap::{Arg, Command};

// high-level declaration of sub modules
mod configs;
mod modules;

// call specific functions each sub modules
use modules::device_parser::parser;

#[tokio::main]
async fn main() {
    // get arguments from cli
    let args = Command::new("DigiPDAM Core Engines")
        .version("1.0")
        .author("KST Developer Team")
        .about("This application will be the centralize of all core engines in DigiPDAM")
        .arg(
            Arg::new("app")
                .short('a')
                .long("app")
                .value_name("application")
                .help("Choose application you want to run")
                .value_parser(["device-parser", "consolidation", "distribution"]),
        )
        .arg (
            Arg::new("period")
                .short('p')
                .long("period")
                .value_name("YYYY-MM")
                .help("[Optional] to run specific period of agent you want to run")
                .default_value(None)
        )
        .get_matches();

    // get period value first as an optional value
    // then send value as into each applications
    let mut period: String = String::new();
    if let Some(p) = args.get_one::<String>("period") {
        period = p.to_string();
    }

    // check application user want to run
    if let Some(app) = args.get_one::<String>("app") {
        // run application with specific user choosed
        if app == "device-parser" {
            parser::parser(period).await;
        } else {
            println!("Hey mate, look I didnt create this app yet, will do my best ASAP!");
        }
    } else {
        println!("Seems like you choose nothing");
    }
}