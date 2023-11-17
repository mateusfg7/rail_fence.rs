mod encode;
use encode::encode;

fn main() {
    let secret = String::from("Mateus Felipe");
    let trails_number = 2;

    let encoded = encode(secret.clone(), trails_number);

    println!(
        "
    secret : {secret}
    encoded: {encoded}
    "
    )
}
