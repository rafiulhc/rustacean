#![allow(dead_code)]
#![allow(unused_variables)]



fn main() {
    let num = 1000;

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
}

