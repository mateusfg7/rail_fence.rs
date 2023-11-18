enum State {
    Forwards,
    Backwards,
}

pub fn encode(secret: String, trails_number: usize) -> String {
    let mut trails: Vec<Vec<char>> = vec![];

    (0..trails_number).for_each(|i| {
        if i < trails_number {
            trails.push(vec![])
        }
    });

    let filtered_secret = secret.replace(" ", "");

    let mut curr_trail = 0;
    let mut state = State::Forwards;
    filtered_secret.chars().for_each(|ch| {
        trails[curr_trail].push(ch);

        curr_trail = match state {
            State::Forwards => {
                if curr_trail < trails_number - 1 {
                    curr_trail + 1
                } else {
                    state = State::Backwards;
                    curr_trail - 1
                }
            }
            State::Backwards => {
                if curr_trail > 0 {
                    curr_trail - 1
                } else {
                    state = State::Forwards;
                    curr_trail + 1
                }
            }
        }
    });

    let mut encoded_secret = String::new();

    trails.iter().for_each(|trail| {
        trail.iter().for_each(|c: &char| {
            encoded_secret.push(*c);
        });
    });

    encoded_secret.to_uppercase()
}
