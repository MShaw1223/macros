macro_rules! basic_maths {
    ($first:expr, $operator:tt, $second:expr) => {
        {
            $first
            $operator
            $second
        }
    };
}
fn main() {
    print!("{}\n", basic_maths!(3,*,3));
    print!("{}\n", basic_maths!(9,+,10));
    print!("{}\n", basic_maths!(2,-,10));
    print!("{}\n", basic_maths!(8,/,2));
}
