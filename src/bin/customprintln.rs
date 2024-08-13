macro_rules! my_println {
    ($msg:expr, $arg:expr ) => {
        println!("{}{}", $msg, $arg);
    };
    ($msg:expr) => {
        println!("{}", $msg);
    };
}
fn main() {
    let name = String::from("FooBar");
    my_println!("Hello: ", name);
}
