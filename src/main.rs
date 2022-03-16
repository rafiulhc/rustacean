#![allow(dead_code)]
#![allow(unused_variables)]



fn main() {
    let temp = 18;
    let num = 1 | 3;
    println!("{}", num);
   if num < 1 {
    println!("Zero");
   } else if num <= 100 {
         println!("Small number");
    } else if num <= 1000 {
         println!("Medium number");
    } else if num <= 10000 {
         println!("Big number");
   } else {
         println!("Huge");
   }
   let day = if temp > 20 {println!("It a hot day") } else {println!("It's a cold day")};
}

