use std::time::Instant;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let limit: u64 = 100000000; // Increased limit for larger benchmark
    let now = Instant::now();
    let mut count = 0;

    for i in 2..=limit {
        if is_prime(i) {
            count += 1;
        }
    }

    let elapsed = now.elapsed();

    println!("Found {} primes up to {} in {} seconds.", count, limit, elapsed.as_secs_f64());
}

