#![allow(unused)]

extern crate num;
extern crate itertools;

mod helper;
use helper::{string_helper, math_helper};
use animals::{Legs, Kind, Animal};
// use module_2::*;
use rand::Rng;
use std::collections::HashMap;
use ansi_term::Style;
use num::FromPrimitive;
use num::bigint::BigInt;
use num::rational::{Ratio, BigRational};
use itertools::{join, max, sorted};

fn approx_sqrt(number: u64, iterations: usize) -> BigRational {
    let start: Ratio<BigInt>
        = Ratio::from_integer(FromPrimitive::from_u64(number).unwrap());

    let mut approx = start.clone();

    for _ in 0..iterations {
        approx = (&approx + (&start / &approx)) /
            Ratio::from_integer(FromPrimitive::from_u64(2).unwrap());
    }

    approx
}

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
    println!("{}", approx_sqrt(10, 4)); // prints 4057691201/1283082416

    let a = [3, 2, 5, 8, 7];

    // Combine all iterator elements into one String,
    // seperated by *.
    println!("{:?}", join(&a, "*"));
    // Return the maximum value of the iterable.
    println!("{:?}", max(a.iter()).unwrap());
    // Collect all the iterable's elements into a
    // sorted vector in ascending order.
    println!("{:?}", sorted(a.iter()));
}
