use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // sum_squares([]);
    let map = greet_map(1, "watanabe");
    println!("print: {:?}", map);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    let x = 5;
    let y = x;
}

fn greet_map(id: i32, name: &str) -> HashMap<i32, String> {
    let mut map: HashMap<i32, String> = HashMap::new();
    let message = format!("Hello, {name}");
    map.insert(id, message);
    map
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    let a = String::from("here");
    a
}
