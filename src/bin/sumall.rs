macro_rules! sum_all {
    ($arg:expr, $($args:expr),*) => {
        $arg + sum_all!($($args),+)
    };
    ($arg:expr)=>{
        $arg
    }
}

fn main() {
    print!("{}\n", sum_all!(1, 1, 2, 3, 5, 8, 13));
}
