#![allow(unused)]

mod helper;
use helper::{string_helper, math_helper};
use animals::{Legs, Kind, Animal};
// use module_2::*;
use rand::Rng;
use std::collections::HashMap;
use ansi_term::Style;

pub mod animals {
    #[derive(Debug)]
    pub enum Legs {
        Two,
        Four,
        Six,
        Eight
    }

    #[derive(Debug)]
    pub enum Kind {
        Mamal,
        Bird,
        Reptil,
        Fish,
        Insect,
        Spider
    }

    #[derive(Debug)]
    pub struct Animal {
        pub name: String,
        pub kind: Kind,
        pub legs: Legs
    }

    impl Animal {
        pub fn create(name: String, kind: Kind, legs: Legs) -> Animal {
            Animal {name, kind, legs}
        }

        pub fn get_animal(&self) {
            println!("{:?}", self);
        }

        pub fn get_kind(&self) {
            match self.kind {
                Kind::Mamal =>
                    if self.name == "cat" {
                        println!("{}", "cat")
                    }
                    else {
                        println!("{}", "no cat")
                    },
                _ => println!("{}", "no mamal"),
            }
        }
    }
}

#[derive(Debug)]
enum Coin {
    Kopf,
    Zahl
}

fn generate() -> Coin {
    match rand::thread_rng().gen_range(0..2) {
        0 => Coin::Kopf,
        _ => Coin::Zahl
    }
}

fn store(map: &mut HashMap<String, u32>, res: Coin) {
    let entry = map.entry(format!("{:?}", res)).or_insert(0);
    *entry += 1;
}

fn main() {
    // let a = 42;
    // println!("a = {}", a);

    // let a = 66.6_f64;
    // println!("a = {}", a);

    // let res = 39 * (72 / 12 - 5);
    // println!("res = {}", res);

    // let new_res = 10 * (res / 18);
    // println!("new_res = {}", new_res);

    // println!("{} bis {}", u128::min_value(), u128::max_value());
    // println!("{} bis {}", i128::min_value(), i128::max_value());

    // let vector = [5, 23450, 99];
    // println!("vector = {:?}", vector);
    // println!("vector = {:#?}", vector);

    // let [x, y, z] = vector;
    // println!("{} {} {}", x, y, z);

    // let chars:[char;4] = ['a', 'b', 'c', 'd'];
    // println!("{}", chars[0]);

    // let b = (5, true, "das ist ein Test", 'b');
    // println!("{} {} {} {}", b.0, b.1, b.2, b.3);
    // let (eins, zwei, drei, vier) = b;
    // println!("{} {} {} {}", eins, zwei, drei, vier);
    // let string = b.2;
    // println!("{}", string);

    println!("{}", math_helper::max(3, 7));
    
    let s1 = String::from("Hallo");
    let s2 = String::from("Welt");

    // println!("{}", &s1 + &s2);
    // println!("{}", s1);
    let conc = format!("{} {}!", s1, &s2);
    println!("{}", Style::new().underline().paint(conc));

    println!("Hallo <-> {}", string_helper::reverse("Hallo"));

    let a = Animal::create("cat".to_string(), Kind::Mamal, Legs::Four);
    a.get_animal();
    a.get_kind();

    let b = Animal {name: "bee".to_string(), kind: Kind::Insect, legs: Legs::Six};
    b.get_animal();
    b.get_kind();

    let mut results: HashMap<String, u32> = HashMap::new();

	for _ in 1..=50 {
		store(&mut results, generate());
	}

	println!("Anzahl Kopf: {}", results.get("Kopf").unwrap());
	println!("Anzahl Zahl: {}", results.get("Zahl").unwrap());
/*
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
// ==================================================================
    fn ten_times<F>(f: F) where F: Fn(i32) {
        for index in 0..10 {
            f(index);
        }
    }

    ten_times(|j| println!("hello, {}", j));
    // With type annotations
    ten_times(|j: i32| -> () { println!("hello, {}", j) });

    let word = "konnichiwa".to_owned();
    ten_times(move |j| println!("{}, {}", word, j));
*/
}
