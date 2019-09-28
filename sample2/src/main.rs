use std::thread;
use std::time::Duration;

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

        thread::sleep(Duration::from_millis(1000));
    }

    fn hoge(&mut self) {
        self.name = "hoge".to_string();
        self.fuga();
        self.name = "hoge".to_string();
    }

    fn fuga(&mut self) {
        self.name = "fuga".to_string();
    }
}

fn main() {
    let mut persons = vec![
        Person::new("aaa"),
        Person::new("bbb"),
        Person::new("ccc"),
    ];
    // for p in &persons {
    //     p.eat();
    // }
    persons[0].hoge();
    let mut hoge = &mut persons[0];
    {
        let mut fuga = &mut hoge;
//        let mut fuga = &mut persons[0];
        {
            let foo = &mut fuga;
//            let mut foo = &mut persons[0];
            foo.name = "foo".to_string();
        }
        fuga.name = "fuga".to_string();
    }
    hoge.name = "hoge".to_string();
//    hoge.name = "hoge".to_string();
//    hoge.name = "12345".to_string();
//    println!("{}", hoge.name);
//    println!("{}", persons[0].name);
//    hoge = &mut persons[0];
//    println!("{}", hoge.name);
    persons[0].eat();

    let handles: Vec<_> = persons.into_iter().map(|p| {
        thread::spawn(move || {
            p.eat();
        })
    }).collect();

    for h in handles {
        h.join().unwrap();

        println!("end");
    }
}
