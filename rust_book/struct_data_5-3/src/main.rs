struct User {
    active : bool,
    username : String,
    email : String,
    
}

fn main() {
    let mut user1 = User{
        active : true,
        username : String::from("sampleusername"),
        email : String::from("sample@gmail.com")

    };

    user1.email = String::from("example@gmail.com")
    
}
//one very imp thing here is that the whole user1 instance should be mut
//only then were allowed to change the values
//we cannot nitpick some field to be mut or immut