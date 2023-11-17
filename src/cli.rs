use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="Rail Fence Cipher", author, about, version, long_about = None)]
pub struct CLI {
    #[arg(short, long, help = "The secret that will be encoded.")]
    pub message: String,

    #[arg(
        short,
        long,
        default_value_t = 2,
        help = "Number of trails to use for encode secret."
    )]
    pub trails: usize,
}
