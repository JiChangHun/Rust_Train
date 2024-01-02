mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }

    pub mod inside2 {
        use crate::outermost;
        pub fn inner_function2() {}

        pub fn inner_function3() {
            outermost::middle_secret_function();
        }
    }
}

fn try_me() {                                   // outermost is current module
    outermost::middle_function();               // not current module, but public, run
    outermost::middle_secret_function();        // not current module, private, error
    outermost::inside::inner_function();        // inside is private, error
    outermost::inside::secret_function();       // inside is private, error

    outermost::inside2::inner_function2();
    outermost::inside2::inner_function3();
}