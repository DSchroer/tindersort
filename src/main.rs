use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use clap::Parser;
use rand_pcg::{Lcg64Xsh32, Pcg32};
use rand::{Rng, SeedableRng};
use std::fs;
use std::io;
use std::ops::Add;
use std::process::exit;

#[derive(Parser)]
#[clap(author, version)]
#[clap(author = "Dominick Schroer <dominick@schroer.ca>")]
#[clap(about = "Sort like a millennial", long_about = None)]
struct Cli {
    /// File to sort
    file: String,
    /// RNG seed to use
    #[clap(short, long)]
    seed: Option<String>,
    /// Compare any item in the list instead of adjacent items
    #[clap(short, long)]
    fast: bool,
    /// Only print the options
    #[clap(short, long)]
    print: bool,
}

fn main() {
    let cli = Cli::parse();

    let input_file = fs::read_to_string(&cli.file)
        .expect("Unable to read file");

    let mut lines: Vec<&str> = input_file.split('\n')
        .filter(|s| !s.is_empty())
        .collect();

    let rng = create_rng(&cli.seed);

    let (a, b) = if cli.fast {
        compare_any(rng, lines.len())
    } else {
        compare_neighbours(rng, lines.len())
    };

    println!("{}\n{}", lines[a], lines[b]);

    if !cli.print {
        swap_from_input(a, b, &mut lines, &cli.file);
    }
}

fn swap_from_input(a: usize, b: usize, lines: &mut Vec<&str>, file: &str) {
    let mut choice = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut choice)
        .expect("Unable to read stdin");

    let chosen_index = if choice.trim() == lines[a] {
        a
    } else if choice.trim() == lines[b] {
        b
    } else {
        exit(1);
    };

    if chosen_index == b && b > a {
        lines.swap(a, b);
    } else if chosen_index == a && a > b {
        lines.swap(a, b);
    }

    fs::write(&file, lines.join("\n").add("\n").as_str())
        .expect("Failed to write results");
}

fn compare_neighbours(mut rng: Lcg64Xsh32, line_count: usize) -> (usize, usize) {
    let to_compare = rng.gen_range(1..line_count as u32) as usize;
    let reverse = rng.gen_range(0..2) != 0;

    if reverse {
        (to_compare, to_compare - 1)
    } else {
        (to_compare - 1, to_compare)
    }
}

fn compare_any(mut rng: Lcg64Xsh32, line_count: usize) -> (usize, usize) {
    loop {
        let first = rng.gen_range(0..line_count as u32) as usize;
        let second = rng.gen_range(0..line_count as u32) as usize;
        if first != second {
            return (first, second);
        }
    }
}

fn create_rng(seed: &Option<String>) -> Lcg64Xsh32 {
    match seed {
        Some(seed) => {
            let mut hasher = DefaultHasher::new();
            seed.hash(&mut hasher);
            Pcg32::seed_from_u64(hasher.finish())
        }
        None => {
            Pcg32::from_entropy()
        }
    }
}