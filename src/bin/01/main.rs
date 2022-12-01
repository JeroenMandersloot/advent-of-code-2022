use std::fs;

fn get_calories() -> Vec<i32> {
    let file_path = "inputs/01/a.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut cache = Vec::new();
    for elf in contents.split("\n\n") {
        let calories: i32 = elf.split("\n").into_iter().map(|i| i.parse::<i32>().unwrap()).sum();
        cache.push(calories);
    }

    cache
}

#[allow(dead_code)]
fn a() {
    let result = get_calories().into_iter().max().unwrap();
    println!("{}", result);
}


#[allow(dead_code)]
fn b() {
    let mut calories = get_calories();
    calories.sort();
    let result: i32 = calories[calories.len() - 3..].into_iter().sum();
    println!("{:?}", result);
}

fn main() {
    a();
    b();
}