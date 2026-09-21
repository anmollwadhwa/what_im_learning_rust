//by default variable in rust is immutable and cannot be reassigned but we can use "mut"

fn main(){
    let mut x = 1.1;
    x = 2.2;
    println!("{}",x);
}