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

fn main() {
    let target: &str = "METHINKS IT IS LIKE A WEASEL";
    let target_size: usize = target.len();

    for _ in 0..10 {
        let solution = create_random_string(target_size);
        print!("{:?}\n", solution);
        let fit = get_fitness(&solution, target).unwrap();
        print!("Fitness={:?}\n", fit);
    }
}
