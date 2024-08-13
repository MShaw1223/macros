macro_rules! conditional_print {
    ($arg:expr, $bool:expr) => {
        if $bool == true {
            println!("{}", $arg)
        }
    };
}
fn main() {
    conditional_print!("zero", false);
    conditional_print!("one", true);
}
