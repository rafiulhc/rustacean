//! cargo documentation example

///
fn main() {
 let result = add(100, 2320);
    println!("The result is {}", result);
}

fn add(number_one: u64, number_two: u64) -> u64 {
 number_one + number_two
}
