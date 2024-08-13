macro_rules! array_of_zeros {
    ($arg:expr) => {
        [0; $arg]
    };
}
fn main() {
    print!("{:?}\n", array_of_zeros!(1));
    print!("{:?}\n", array_of_zeros!(3));
    print!("{:?}\n", array_of_zeros!(5));
}
