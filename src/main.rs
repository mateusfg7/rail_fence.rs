mod encode;
use encode::encode;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    secret: String,

    #[arg(short, long)]
    trails: usize,
}

fn main() {
    let args = Args::parse();

    let secret = args.secret;
    let trails_count = args.trails;

    let encoded = encode(secret.clone(), trails_count);

    println!("secret : {secret}\nencoded: {encoded}")
}
