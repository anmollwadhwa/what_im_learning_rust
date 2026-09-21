fn main() {
    let mut years : [i32 ; 3] = [1999,2000,2005];

    for year in years.iter(){
        println!("Next Year : {}", year + 1 );
    }
}
//arrays can be iterated over because the all elements are( must ) of the same type
//meanwhile tuples and structs cannot