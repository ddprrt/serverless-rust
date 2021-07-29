use lambda_runtime::{Context, handler_fn};
use serde_json::{Value, json};
use itertools::Itertools;
use num::{FromPrimitive, Integer};
use std::{collections::{HashMap, HashSet}};


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
async fn main() -> Result<(), lambda_runtime::Error>{
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: Value, _: Context) -> Result<Value, lambda_runtime::Error> {
    let (min, max) = match (event["min"].as_u64(), event["max"].as_u64()) {
        (Some(min), Some(max)) => (min, max),
        _ => (0, 0)
    };
    match palindrome_products(min, max) {
        Some((pal_1, pal_2)) => Ok(json!({ "palindromes": { "one": pal_1.value, "two": pal_2.value } })),
        None => Ok(json!({ "message": "none found" }))
    }   
}
