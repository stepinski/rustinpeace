
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    sock_pairs("ABABC");
}



fn sock_pairs(clothes : &str) -> i32{
    // let mut socks: HashMap<char, i32> = HashMap::new();    

    let socks = clothes.chars().into_iter()
    .fold(HashMap::<char, i32>::new(), |mut m, x| {
        *m.entry(x).or_default() += 1;
        m
    }).values().into_iter().map(|x| x/2).sum();

    socks
   
}

#[test]
fn test_socks1(){
	assert_eq!(sock_pairs("AA"),1)
}

#[test]
fn test_socks2(){
	assert_eq!(sock_pairs("ABABC"),2)
}


#[test]
fn test_socks4(){
	assert_eq!(sock_pairs("CABBACCC"),4)
}
