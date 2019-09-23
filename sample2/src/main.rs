struct Person {
    name: String,
}

impl Person {
    fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("eat: {}", self.name);
    }
}

fn main() {
    let persons = vec![
        Person::new("aaa"),
        Person::new("bbb"),
        Person::new("ccc"),
    ];
    for p in &persons {
        p.eat();
    }
}
