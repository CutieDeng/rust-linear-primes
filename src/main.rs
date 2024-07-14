use std::time::Instant;

pub mod v1; 
pub mod v2; 
pub mod v3; 
pub mod v4; 

fn main() {
    let num = 1000000000; 
    // let start = Instant::now(); 
    // let rst1 = v1::find_primes(num); 
    // let start1 = Instant::now(); 
    // let rst2 = v2::find_primes(num); 
    let start2 = Instant::now(); 
    let rst3 = v3::find_primes(num); 
    let start3 = Instant::now(); 
    let rst4 = v4::find_primes(num); 
    let start4 = Instant::now(); 
    // assert_eq!(rst2, rst3); 
    assert_eq!(rst3, rst4); 
    // println!("rst1 cost time: {}s", start1.duration_since(start).as_secs_f32()); 
    // println!("rst2 cost time: {}s", start2.duration_since(start1).as_secs_f32()); 
    println!("rst3 cost time: {}s", start3.duration_since(start2).as_secs_f32()); 
    println!("rst4 cost time: {}s", start4.duration_since(start3).as_secs_f32()); 
    println!("Find primes numbers {}", rst4.len()); 
}
