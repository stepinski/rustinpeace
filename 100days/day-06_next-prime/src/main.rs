use primes::{Sieve, PrimeSet};

fn main() {
    println!("Hello, world!");
    next_prime(10);
}
    
fn next_prime(numb:u64)-> u64 {
    let mut pset = Sieve::new();
    let (_ix, n) = pset.find(numb);
    return n;
}




#[test]
fn test_jprime1(){
	assert_eq!(next_prime(12),13)
}

#[test]
fn test_jprime2(){
	assert_eq!(next_prime(24),29)
}

#[test]
fn test_jprime3(){
	assert_eq!(next_prime(11),11)
}



// NextPrime(12) ➞ 13

// NextPrime(24) ➞ 29

// NextPrime(11) ➞ 11
