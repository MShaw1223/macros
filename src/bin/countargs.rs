macro_rules! count_args {
    ($first:expr, $($rest:expr),*) => {
            1 + count_args!($($rest),*)
        };
        // Base case: one argument left / only one
        ($last:expr) => {
            1
        };
}
fn main() {
    print!("{}\n", count_args!(5, 6, 4, 3));
    print!("{}\n", count_args!(8, 9, 10));
    print!("{}\n", count_args!(1, 1, 1, 1, 1));
}
