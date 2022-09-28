
fn main (){
    let message = String::from("Hello, world!");
    #[warn(unused_variables)]
    let message_2 = &message;
    println!("{}", message);
}
