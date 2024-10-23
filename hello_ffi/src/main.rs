extern "C" {
    fn say_hello();
}

fn main() {
    println!("hello from Rust");
    unsafe { say_hello() };
}
