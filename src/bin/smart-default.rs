use smart_default::SmartDefault;

#[derive(SmartDefault)]
pub struct Foo {
    #[default = 123]
    a: i32,
}

fn main() {
    let foo = Foo::default();
    println!("{}", foo.a);
}