pub fn find_primes(n: u64) -> Vec<u64> {
    let mut i : u64 = 2; 
    let mut mini_prime = vec![0u64; n as usize]; 
    let mut rst = Vec::new(); 
    loop {
        if i >= n { break } 
        if mini_prime[i as usize] == 0 { 
            mini_prime[i as usize] = i; 
            rst.push(i); 
        } 
        for j in &rst {
            if *j > mini_prime[i as usize] as u64 { break } 
            let m = *j * i; 
            if m >= n { break } 
            mini_prime[m as usize] = *j; 
        }
        i += 1; 
    }
    rst 
}
