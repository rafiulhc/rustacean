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
fn main() {
    while_loop();
    for_loop();
}

