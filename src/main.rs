

struct Number {
    number_one: u64,
    number_two: u64,
}

impl Number {
    fn impl_function(&self) {
        println!(" Total is {} ", self.number_one + self.number_two);
    }
}

fn main (){
    let new_number = Number{
        number_one: 10,
        number_two: 20,
    };
    new_number.impl_function();
}
