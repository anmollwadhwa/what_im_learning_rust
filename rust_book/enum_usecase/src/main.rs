fn main() {
    enum Ip_address {
        V4(u8,u8,u8,u8),
        V6(String),
    }
    let home = Ip_address::V4(127,0,0,1);
    let lab = Ip_address::V6(String::from("::1"));
}

//since ip addr has v4 or v6 version
//enum can choose only one of variant i.e
// v4 or v6, and not both