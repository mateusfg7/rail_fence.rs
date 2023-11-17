mod cipher;
mod cli;

use clap::Parser;
use cli::CLI;

fn main() {
    let args = CLI::parse();

    let encoded = cipher::encode(args.message.clone(), args.trails);

    println!("secret : {0}\nencoded: {encoded}", args.message)
}
