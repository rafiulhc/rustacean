pub mod learning_rust{

    pub struct Number {
        pub number_one: u64,
        pub number_two: u64,
    }

    pub trait SimpleTrait {
        fn add(&self);
    }

    impl SimpleTrait for Number {

        fn add(&self) {
            println!(" Total is {} ", self.number_one + self.number_two);
        }
    }

}