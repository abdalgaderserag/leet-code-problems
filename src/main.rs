mod problems;

use std::io;

fn main() {
    println!("enter problem index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("some thing went wrong reading input");

    let index: usize = index.trim().parse().expect("enter valid number");

    match problems::get_solver(index) {
        Some(solver) => solver(),
        None => println!("Problem with index {} not implemented.", index),
    }
}
