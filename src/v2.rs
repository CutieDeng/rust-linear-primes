pub fn find_primes(n: u64) -> Vec<u64> {
    let mut i : u64 = 2; 
    let mut rst = Vec::new(); 
    loop {
        if i >= n { break } 
        if judge_prime(i) { rst.push(i) }; 
        i += 1; 
    }
    rst 
}

fn judge_prime(n: u64) -> bool {
    let mut i : u64 = 2; 
    loop {
        if i >= n { break } 
        if i * i > n { break } 
        if n % i == 0 { return false }
        i += 1; 
    }
    true 
}