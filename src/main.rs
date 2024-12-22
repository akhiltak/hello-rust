#![crate_name = "hello_rust"]

/// represent a human
pub struct Person {
    /// name of the person
    name: String,
}

/// impl a Person
/// learn from: <https://doc.rust-lang.org/rust-by-example/meta/doc.html>
impl Person {
    /// Create a person with given name

    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Namaste, jai shree [name](Person::name)!" to the `Person` it is called on.
    pub fn hello(&self) {
        println!("Namaste, jai shree {}!", self.name)
    }
}

fn main() {
    println!("Namaste, world!");
    println!("I'm a Rustacean!");

    let ram = Person::new("Ram");

    ram.hello();

    // comment()
}
