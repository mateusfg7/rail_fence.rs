pub fn encode(secret: String, trails_number: usize) -> String {
    let mut trails: Vec<Vec<char>> = vec![];

    (0..trails_number).for_each(|i| {
        if i < trails_number {
            trails.push(vec![])
        }
    });

    let mut curr_trail = 0;
    secret.chars().for_each(|ch| {
        trails[curr_trail].push(ch);
        curr_trail = if curr_trail < trails_number - 1 {
            curr_trail + 1
        } else {
            0
        };
    });

    let mut encoded_secret = String::new();

    trails.iter().for_each(|trail| {
        trail.iter().for_each(|c: &char| {
            encoded_secret.push(*c);
        });
    });

    encoded_secret.to_uppercase()
}
