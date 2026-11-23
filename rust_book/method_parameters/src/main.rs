#[derive(Debug)]

struct rectangle {
    width : u32,
    length : u32
}

impl rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn can_hold(&self, other : &rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}
fn main() {
    let rect1 = rectangle {
        width: 30,
        length: 50,
    };
    let rect2 = rectangle {
        width: 10,
        length: 40,
    };
    let rect3 = rectangle {
        width: 60,
        length: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}