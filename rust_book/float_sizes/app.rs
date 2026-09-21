fn main(){

    let x: f64 = 10.0 /3.0;       // f64 holds 8 bytes of storage hence gives more digits after decimal point
    let y: f32 = 10.0/3.0;        // f32 holds 4 bytes of storage hence gives less digits.....
    println!("{},{}", x, y);
}

// more memory used -> precision but might be slow