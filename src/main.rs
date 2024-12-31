use rand::Rng;

fn create_random_string(n: usize) -> String {
    // Define the allowed characters (uppercase letters and space in this case)
    let characters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ ".chars().collect();

    // Create a random string of length `n`
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| characters[rng.gen_range(0..characters.len())])
        .collect()
}

fn get_fitness(solution: &str, target: &str) -> Result<u32, String> {
    if solution.len() != target.len() {
        return Err(String::from("Strings are incompatible"));
    }

    let mut fitscore: u32 = 0u32;

    for (idx, tc) in target.chars().enumerate() {
        if tc == solution.chars().nth(idx).unwrap() {
            fitscore += 1
        }
    }

    Ok(fitscore)
}

fn mutate_string(input: &str, mutation_rate: f64) -> Option<String> {
    if mutation_rate < 0.0 || mutation_rate > 1.0 {
        return None;
    }

    // Define the allowed characters (uppercase letters and space in this case)
    let characters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ ".chars().collect();

    // Create a mutable random generator
    let mut rng = rand::thread_rng();

    // Iterate through each character in the input and decide whether to mutate
    let output = input
        .chars()
        .map(|c| {
            if rng.gen::<f64>() < mutation_rate {
                // Mutate the character
                characters[rng.gen_range(0..characters.len())]
            } else {
                // Keep the original character
                c
            }
        })
        .collect();

    Some(output)
}

fn main() {
    let target: &str = "METHINKS IT IS LIKE A WEASEL";
    let target_size: usize = target.len();

    // let pop_size = 100;

    let solution = create_random_string(target_size);

    for _ in 0..100 {
        let solution = mutate_string(&solution, 0.05).expect("Mutation failed");
        print!("{:?}\n", solution);
        let fit = get_fitness(&solution, target).unwrap();
        print!("Fitness={:?}\n", fit);
    }
}
