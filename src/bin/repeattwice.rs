macro_rules! repeat_twice {
    ($arg:expr) => {
        $arg;
        $arg;
    };
}

fn main() {
    let mut v = Vec::from([2, 45, 8, 4, 8, 1, 2, 4, 79]);
    print!("{v:?}\n");
    repeat_twice!(v.pop());
    print!("{v:?}\n");
}
