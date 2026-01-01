use std::collections::BTreeMap;
use std::env;

fn main() {
    let mut x: u128 = 5;
    let mut y: u128 = 5;

    let limit: Vec<String> = env::args().collect();
    let limit: u128 = limit[1].trim().parse().expect("Please enter a number");

    let mut tree = BTreeMap::from([(1, 4), (2, 1), (3, 10), (4, 2)]);
    let next = |a: &u128| if a % 2 == 0 { a / 2 } else { a * 3 + 1 };

    while x <= limit {
        while !tree.contains_key(&y) {
            tree.insert(y, next(&y));
            y = match y % 2 {
                0 => y / 2,
                _ => y * 3 + 1,
            }
        }
        x += 1;
        y = x;
    }

    println!("digraph G {{");

    for (key, value) in tree {
        print!("{key} -> {value};");
    }

    println!("\n}}");
}
