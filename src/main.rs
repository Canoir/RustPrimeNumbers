use std::io;
fn main() {
    loop {
        let max: u64 = loop {
            let mut max = String::new();
            io::stdin()
                .read_line(&mut max)
                .expect("Error Retrieving Data!");
            match max.trim().parse() {
                Ok(e) => {
                    break e;
                }
                Err(_) => {
                    println!("Enter Number!");
                    continue;
                }
            };
        };
        println!("Max: {}", max);
        for i in 2..max + 1 {
            let mut is_prime: bool = true;
            for j in 2..i {
                if i % j == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                println!("{}", i);
            }
        }
    }
}
