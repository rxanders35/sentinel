use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    // mode of a sidecar
    #[arg(long, value_enum)]
    mode: Option<Mode>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    Sender,
    Receiver,
}

fn main() {
    let args = Args::parse();
    match args.mode {
        Some(Mode::Sender) => println!("I am a Sender"),
        Some(Mode::Receiver) => println!("I am a Receiver"),
        None => println!("You need to provide a mode"),
    }
}
