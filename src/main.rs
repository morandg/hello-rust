fn say_hello(who: &String) {
    println!("Hello {}", who);
}

fn main() {
    let mut who: String = String::from("World");
    say_hello(&who);
    who = String::from("bytesatwork");
    say_hello(&who);
}
