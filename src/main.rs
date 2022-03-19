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

fn main() {
    while_loop();
}

