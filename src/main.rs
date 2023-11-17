fn main() {
    let secret = String::from("mateus felipe");
    let trails_number = 2;

    let mut trails: Vec<Vec<char>> = vec![];
    let mut i = 0;
    loop {
        if i < trails_number {
            trails.push(vec![]);
            i += 1;
        } else {
            break;
        }
    }

    let mut curr_trail = 0;
    for letter in secret.chars() {
        trails[curr_trail].push(letter);
        curr_trail = if curr_trail < trails_number - 1 {
            curr_trail + 1
        } else {
            0
        };
    }

    let mut crypt_secret = String::new();

    trails
        .iter()
        .for_each(|trail| trail.iter().for_each(|c| crypt_secret.push(*c)));

    println!("{}", crypt_secret.to_uppercase())
}
