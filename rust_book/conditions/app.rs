fn main(){
    let cat = 25
    let message = if cats > 1 {
        "Many cats"
    } else if cats > 1000 {
        "too many cats"
    } else {
        "need more cats"
    };

    println!("{}", message);
}
