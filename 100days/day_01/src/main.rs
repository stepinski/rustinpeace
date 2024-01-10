use std::io;

fn calc_age(age: u32) -> u32 {
    age * 365
}

fn main() {
    // calcAge(65) ➞ 23725
    loop {
        println!("how old are you");
        let mut age = String::new();
        io::stdin()
            .read_line(&mut age)
            .expect("this is number of years");

        let ageinput: u32 = match age.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("its {} days", calc_age(ageinput));
    }
}
