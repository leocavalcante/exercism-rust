fn sieve_of_eratosthenes(n: u32) -> Vec<u32> {
    (2..(n as f64).sqrt() as u32).fold((2..n).collect(), |list, i| {
        list.into_iter().filter(|&n| n % i != 0 || n == i).collect()
    })
}

pub fn nth(n: u32) -> u32 {
    let sieve = sieve_of_eratosthenes(200_000);
    sieve[n as usize]
}
