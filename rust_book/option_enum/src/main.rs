fn main() {
    let x : i8 = 5;
    let y : Option<i8> = Some(5);
    let sum = x + y;
}
//this wont work because it x is an number but 
// y is kind of box that might hold a i8
// and y will either have value Some(5) in this case or None
//the error can be solved if there is a fallback
//incase the value is none.