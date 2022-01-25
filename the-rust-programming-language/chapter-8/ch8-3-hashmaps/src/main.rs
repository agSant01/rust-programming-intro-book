use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // hashmap from vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_score = vec![String::from("Blue"), String::from("Yellow")];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_score.into_iter()).collect();
}
