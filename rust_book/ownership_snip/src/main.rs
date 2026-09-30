fn main() {
    

    {
        let s = "hello"   /s has a value here
                          / operation on s will take place here
    } 
                         / s goes out of scope


}

fn scope_assignment() {
    let mut j = String::from("hello");   /::from here requests memory it needs
    s = String::from("crazy");          / space taken by hello is droped automatically by rust
                                        /only crazy is in the memory
}