use clap::{Arg, ArgAction, Command};

const VERSION: &str = match option_env!("RETRY_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

fn main() {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(VERSION)
        .disable_version_flag(true)
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .action(ArgAction::Version),
        )
        .get_matches();
}
