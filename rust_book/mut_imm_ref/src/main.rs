fn main() {
    let mut s = String::from("Hello world");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    //r1 and r2 are out of scope from this line onwards
    //since the reference was last called at line println

    let r3 = &mut s; 
    // this will work now since the previous immutable
    //references scopes have ended, so the scopes don't overlap
    // hence, no error and this will compile

}

//had r1,r2,r3 been called together the code would give error
//because the immutable value can't be expected to suddenly change
//if it did then there is no point of immutable references being present