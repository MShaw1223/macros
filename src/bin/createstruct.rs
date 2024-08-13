macro_rules! create_struct {
    ($name:ident, $($fields:ident :$type:ty),*) => {
        struct $name {
            $(
                $fields : $type,
            )*
        }
    };
}
fn main() {
    create_struct!(Person, age: i32, name: String);
    let p = Person {
        age: 20,
        name: String::from("Jeff"),
    };
    print!("Name: {} Age: {}\n", p.name, p.age);
}
