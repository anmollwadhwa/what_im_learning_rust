use std::io;
fn main() {
    println!("Ssup");
loop {

    let mut a : i32= 0;
    let mut b : i32 = 1;
    println!("Which fibonacci number do you wish to know? or Press Q to exit");
    
    let mut n = String::new();
    
    io::stdin()
                .read_line(&mut n)
                .expect("Failed to readline");

    if n.trim().to_lowercase() == "q"{
        break;
    }

    let n : i32 = match n.trim().parse() {
        
        Ok(n) => n,
        Err(_e) => {
            println!("Please enter a valid number");
            continue;
        }
    };

    if n<1 {
        println!("Please enter number greater than 1");
        continue;
    } else if n>47 {
        println!("Please enter number smaller than 47");    
    } else if n==1 {
        println!("0");
    }

    for _i in 0..(n-2) {
        let next: i32 = a + b;
        a = b;
        b = next;
    };
    println!("The {n}th number in the fibonacci series is {b}");    
}
}
