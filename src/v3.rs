pub fn find_primes(n: u64) -> Vec<u64> {
    let mut i : u64 = 2; 
    let mut not_primes = vec![false; n as usize]; 
    let mut rst = Vec::new(); 
    loop {
        if i >= n { break } 
        if !not_primes[i as usize] { 
            rst.push(i); 
            let mut j = 2; 
            loop {
                let m = i * j; 
                if m >= n { break } 
                not_primes[m as usize] = true; 
                j += 1; 
            }
        } 
        i += 1; 
    }
    rst 
}
