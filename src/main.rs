#![allow(dead_code)]
#![allow(unused_variables)]


fn while_loop(){
      let mut x = 2;
      while x < 1000 {
            x *= 2;
            if x == 64 { continue;}
            println!(" x = {}", x);
      }
}

fn for_loop(){
      for y in 2..14 {
      if y == 3 { continue;}
      if y == 9 { break;}
            println!(" y = {}", y);
      }
}

fn country_code(){
      let my_country = 1000;
      match my_country {
            4 => println!("I am from the UK"),
            1 => println!("I am from the USA"),
            2 => println!("I am from Germany"),
            5..=1000 => println!("I am from somewhere else"),
            _ => println!("invalid country code"),
      }
}
fn main() {
      country_code();
}

