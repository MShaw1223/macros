macro_rules! debug_output {
    ($arg:expr) => {
        print!("Expression: {}\n", stringify!($arg));
        print!("Value: {:?}\n", $arg);
    };
}
fn main() {
    let mut v = Vec::from([1, 1, 2, 3, 5, 8, 13]);
    print!("{v:?}\n");
    debug_output!(v.pop());
    print!("{v:?}\n");
    debug_output!(v.push(4));
    print!("{v:?}\n");
}
