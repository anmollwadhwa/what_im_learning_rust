
//area method based on dimension
//of rectangle struct, which finds out the area with given dimensions
#[derive(Debug)]
struct rectangle {
    width : u32,
    height : u32
}

impl rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = rectangle{
        width : 30,
        height : 20
    };

    println!("The area of rect1 is {}", rect1.area());
}
