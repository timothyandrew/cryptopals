use clap::{AppSettings, Clap};
use cryptopals::set1;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long)]
    set: i32,
    #[clap(short, long)]
    challenge: i32,
}

fn main() {
    let opts: Opts = Opts::parse();

    match (opts.set, opts.challenge) {
        (1, 1) => set1::c1(),
        (_, _) => panic!("Invalid set/challenge")
    }
}
