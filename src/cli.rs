pub mod cli {
    use std::env;

    pub fn get_args() {
        let pattern = env::args().nth(1).expect("no arg found on pos 1");
        println!("{}", pattern);
    }

    pub struct Cli {
        filename: std::
    }
