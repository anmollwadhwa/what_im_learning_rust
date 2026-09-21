fn main(){
    let answer = multiply_both(2.1,3.1);
    println!("2.1x3.1={}", answer);
}

fn multiply_both(x: f64, y: f64) -> f64 {
    return x * y;
}


//must define type annotations while defining a function in RUST.
