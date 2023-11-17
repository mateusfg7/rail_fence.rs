mod cli;
mod encode;

use clap::Parser;
use cli::CLI;
use encode::encode;

fn main() {
    let args = CLI::parse();

    let encoded = encode(args.message.clone(), args.trails);

    println!("secret : {0}\nencoded: {encoded}", args.message)
}
