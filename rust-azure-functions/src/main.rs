use std::{collections::{HashMap, HashSet}};
use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

use itertools::Itertools;
use num::{FromPrimitive, Integer};


#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    factors: HashSet<(u64, u64)>,
    value: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        let mut set: HashSet<(u64, u64)> = HashSet::new();
        set.insert((a, b));
        Self {
            value: a * b,
            factors: set,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.insert((a, b));
    }
}

fn is_palindrome(a: u64, b: u64) -> bool {
    let prod = a * b;
    prod == reverse(prod)
}

/// Reverses an unsigned integer number (e.g. 123 -> 321)
fn reverse<T: Copy + FromPrimitive + Integer>(a: T) -> T {
    let radix = FromPrimitive::from_usize(10).unwrap();
    let mut n = a;
    let mut reversed = FromPrimitive::from_usize(0).unwrap();

    while !n.is_zero() {
        reversed = reversed * radix + n % radix;
        n = n / radix;
    }

    reversed
}

fn check_palindrom(map: &mut HashMap<u64, Palindrome>, a: u64, b: u64) {
    if is_palindrome(a, b) {
        let prod = a * b;
        map.entry(prod)
            .or_insert_with(|| Palindrome::new(a, b))
            .insert(a, b);
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut map: HashMap<u64, Palindrome> = HashMap::new();

    for (a, b) in (min..=max).tuple_combinations() {
        check_palindrom(&mut map, a, b);
    }
    for c in min..=max {
        check_palindrom(&mut map, c, c);
    }

    let prod_min_max = map.keys().minmax().into_option();

    if let Some((i, j)) = prod_min_max {
        let (i, j) = (i.to_owned(), j.to_owned());
        Some((map.remove(&i).unwrap(), map.remove(&j).unwrap()))
    } else {
        None
    }
}


#[tokio::main]
async fn main() {
    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("httpexample"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match (p.get("min"), p.get("max")) {
            (Some(min), Some(max)) => {
                let min: u64 = min.parse().unwrap();
                let max: u64 = max.parse().unwrap();
                let val = match palindrome_products(min, max) {
                    Some((pal_one, pal_two)) => 
                        format!("min {} max {}", pal_one.value, pal_two.value),
                    None => String::from("Error")
                };
                Response::builder().body(val)
            },
            _ => Response::builder().body(String::from("Error"))
        });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number"),
        Err(_) => 3000,
    };

    println!("Starting at {}", port);

    warp::serve(example1)
        .run((Ipv4Addr::UNSPECIFIED, port))
        .await
}
