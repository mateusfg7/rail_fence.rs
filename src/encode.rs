pub fn encode(secret: String, trails_number: usize) -> String {
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

    let mut encoded_secret = String::new();

    trails.iter().for_each(|trail| {
        trail.iter().for_each(|c: &char| {
            encoded_secret.push(*c);
        });
    });

    encoded_secret.to_uppercase()
}
