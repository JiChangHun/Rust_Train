// new, use cargo build, not run(no main.rs)
pub mod client;             // start with mod, use pub
pub mod network;            // add directory for clarity


// basic
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        client::connect();
    }
}

