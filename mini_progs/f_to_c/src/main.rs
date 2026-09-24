use std::io;

fn main() {
    let mut user_temp = String::new();
    let mut user_unit =String::new();
    println!("Welcome to temperature converter");

 loop {    
    println!("Enter the temp ( without unit ) you wish to convert");

    io::stdin()
            .read_line(&mut user_temp)
            .expect("Failed to readline");

    println!("Now enter the temperature unit");

    io::stdin()
            .read_line(&mut user_unit)
            .expect("Failed to readline");


    
    let user_temp : f64 = match user_temp.trim().parse(){
        Ok(n) => n,
        Err(_e) => {
                println!("Please enter a valid number");
                continue;

        }

    };
    let user_unit = user_unit.trim().to_lowercase();

    if user_unit == "f" {

        let converted = f_to_c(user_temp);
        println!("The converted temperature is {converted}C")

    } else if user_unit == "c" {

        let converted = c_to_f(user_temp);
        println!("The converted temperature is {converted}F")

    } else {

        println!("Please enter C or F unit");
    }

  }

}
fn f_to_c (user_temp : f64) ->  f64 {
        let converted = (user_temp - 32.0) * (5.0/9.0);
        converted

}

fn c_to_f (user_temp : f64) -> f64 {
        let converted = (user_temp * 1.8 ) + 32.0;
        converted
}