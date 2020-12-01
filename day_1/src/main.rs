use util::input::get_default_file_input;

#[macro_use]
extern crate itertools;

fn main() {
    let expenses: Vec<i64> = get_default_file_input()
        .trim()
        .split('\n')
        .map(|expense| expense.parse::<i64>().unwrap())
        .collect();

    first_puzzle(&expenses);
    second_puzzle(&expenses);
}

fn first_puzzle(expenses: &Vec<i64>) {
    let product = iproduct!(expenses, expenses)
        .filter(|it| it.0 != it.1)
        .find(|it| it.0 + it.1 == 2020)
        .map(|it| it.0 * it.1)
        .unwrap();

    println!("Puzzle 1: {}", product);
}

fn second_puzzle(expenses: &Vec<i64>) {
    let product = iproduct!(expenses, expenses, expenses)
        .filter(|it| it.0 != it.1 && it.0 != it.2 && it.1 != it.2)
        .find(|it| it.0 + it.1 + it.2 == 2020)
        .map(|it| it.0 * it.1 * it.2)
        .unwrap();

    println!("Puzzle 2: {}", product);
}
