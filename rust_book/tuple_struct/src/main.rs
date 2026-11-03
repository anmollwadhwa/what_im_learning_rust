struct Colour ( i32, i32, i32);
struct coordinates ( i32, i32, i32);

fn main() {
    let black = Colour(0,0,0);
    let origin = coordinates(0,0,0);
}

//here black and origin are structs but
// of different instances, which means a 
// function that takes coordinates as a parameter
//won't take colour as a argument
