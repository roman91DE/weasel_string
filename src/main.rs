use rand::Rng;

struct Config {
    tokens: Vec<char>,
    target: String,
    mut_prob: f64,
    pop_size: usize,
}

fn create_random_string(n: usize, tokens: &[char]) -> Result<String, String> {
    if tokens.is_empty() {
        return Err(String::from("Empty Set of Tokens"));
    }

    let mut rng = rand::thread_rng();
    let random = (0..n)
        .map(|_| tokens[rng.gen_range(0..tokens.len())])
        .collect();
    return Ok(random);
}

fn init_population(
    popsize: usize,
    stringlen: usize,
    tokens: &[char],
) -> Result<Vec<String>, String> {
    if tokens.is_empty() {
        return Err(String::from("Empty Set of Tokens"));
    }

    let mut population: Vec<String> = Vec::with_capacity(popsize);

    for _ in 0..popsize {
        match create_random_string(stringlen, tokens) {
            Ok(random_string) => population.push(random_string),
            Err(e) => return Err(e),
        }
    }

    Ok(population)
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

fn mutate_string(input: &str, tokens: &[char], mutation_rate: f64) -> Result<String, String> {
    if tokens.is_empty() {
        return Err(String::from("Empty Set of Tokens"));
    }

    let mut rng = rand::thread_rng();

    let mutated: String = input
        .chars()
        .map(|c| {
            if rng.gen::<f64>() < mutation_rate {
                // Mutate the character by picking a random token
                tokens[rng.gen_range(0..tokens.len())]
            } else {
                // Keep the original character
                c
            }
        })
        .collect();

    Ok(mutated)
}

fn ea_simple(conf: Config) {
    let target_size: usize = conf.target.len();
    let mut population = init_population(conf.pop_size, target_size, &conf.tokens)
        .expect("Error during Population-Init");
    population.sort_by_cached_key(|v| get_fitness(v, &conf.target));
    let mut best_fitness =
        get_fitness(&population[0], &conf.target).expect("Error during Fitness Computation");

    while best_fitness < target_size as u32 {
        let elite_pop = &population[..population.len() / 2];
        let mut_pop: Vec<String> = elite_pop
            .iter()
            .filter_map(|s| mutate_string(&s, &conf.tokens, conf.mut_prob).ok())
            .collect();

        population =  elite_pop.iter().chain(mut_pop.iter()).cloned().collect();
        population.sort_by_cached_key(|v| get_fitness(v, &conf.target));
        best_fitness = get_fitness(&population[0], &conf.target).expect("...");
        println!("{:?}", population[0])

    }
}

fn main() {
    let conf = Config {
        tokens: "ABCDEFGHIJKLMNOPQRSTUVWXYZ ".chars().collect(),
        target: String::from("METHINKS IT IS LIKE A WEASEL"),
        mut_prob: 0.05,
        pop_size: 100,
    };

    ea_simple(conf);
}
