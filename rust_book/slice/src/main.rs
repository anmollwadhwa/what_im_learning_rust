fn main() {

let mut s = String::from("hello world");

let word = first_word(&s); //Suppose first_word is a func that returns
                          // the first word of a entered str

s.clear();

println!("the first word is {word}");

}

// now, this won't compile because rust is goated due to it's memory safety.
// if this was C, there would be a bug or i'd say the word would "dangle"
// since word has a immut ref to it, using s.clear will give error
//because we know in rust we can either many &T or mut &T, never together
// here s.clear is playing the role of mut ref
