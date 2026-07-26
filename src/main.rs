mod adb;
mod agent_yaml;
mod catalogue;
mod claim_wrapper;
mod cmd;
mod config;
mod debug;
mod install_check;
mod registry;
mod semantic;
mod subprocess;
mod ui_parser;

use clap::Parser;
use cmd::Cli;

const MANIFEST_JSON: &str = r#"{"schema":"actuators-manifest/v1","name":"ddb","version":"0.1.0","category":"device-control","deps":[{"name":"adb","kind":"system","min":null,"rationale":"android device bridge"}],"provides":["ddb-device-v1"],"consumes":[],"configs":[{"path":"~/.config/ddb","kind":"config","required":false}]}"#;

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--manifest") {
        println!("{MANIFEST_JSON}");
        return;
    }
    install_check::check_binary_mtime();
    let cli = Cli::parse();
    if let Err(e) = cmd::run(cli) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
