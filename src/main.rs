
#[derive(Debug)]
enum Per {
    name(String),
    types,
    value,
}
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    job: String,
}

fn show_something(persion: &User) {
    println!("{}",persion.name);
}

fn main() {
    let demo = Per::name(String::from("var"));
    let joe = User {
    name: String::from("joe"),
    age: 22,
    job: String::from("programmer")
    };
    show_something(&joe)
}
