use std::time::Instant;

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut primes = vec![true; limit + 1];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=limit {
        if primes[i] {
            let mut multiple = i * i;
            while multiple <= limit {
                primes[multiple] = false;
                multiple += i;
            }
        }
    }
    primes
        .iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(i, _)| i)
        .collect()
}

fn main() {
    let limit = 10_000_000; // 10^7
    let start_time = Instant::now();
    let primes = sieve_of_eratosthenes(limit);
    let elapsed_time = start_time.elapsed();
    println!(
        "Found {} primes up to {} in {:?}.",
        primes.len(),
        limit,
        elapsed_time
    );
}
