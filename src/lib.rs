use rand::Rng;

pub struct Config {
    tokens: Vec<char>,
    target: String,
    mut_prob: f64,
    pop_size: usize,
}

impl Config {
    pub fn new(tokens: &str, target: &str, mut_prob: f64, pop_size: usize) -> Self {
        Self {
            tokens: tokens.chars().collect(),
            target: target.to_string(),
            mut_prob,
            pop_size,
        }
    }
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

pub fn ea_simple(conf: Config) {
    let target_size = conf.target.len();
    let mut generation_counter: usize = 0;
    let mut population = init_population(conf.pop_size, target_size, &conf.tokens)
        .expect("Error during Population Initialization");
    population.sort_by_key(|v| get_fitness(v, &conf.target).expect("Error during Population Sort"));

    let mut best_fitness =
        get_fitness(&population[0], &conf.target).expect("Error during Fitness Computation");

    while best_fitness < target_size as u32 {
        generation_counter += 1;
        let elite_pop: Vec<&String> = population.iter().take(population.len() / 2).collect();
        let mut_pop: Vec<String> = elite_pop
            .iter()
            .filter_map(|s| mutate_string(s, &conf.tokens, conf.mut_prob).ok())
            .collect();

        population = elite_pop
            .into_iter()
            .cloned()
            .chain(mut_pop.into_iter())
            .collect();
        population.sort_by_cached_key(|v| std::cmp::Reverse(get_fitness(v, &conf.target).unwrap()));

        best_fitness = get_fitness(&population[0], &conf.target).unwrap();

        println!("Generation: {}", generation_counter);
        println!("Best Solution: {}", population[0]);
        println!("Best Fitness: {}", best_fitness);
        println!("---");
    }
}
