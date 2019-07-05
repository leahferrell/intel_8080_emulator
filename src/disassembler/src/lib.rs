#![crate_name = "disassembler"]

pub fn hello(name: &str) -> String {
    ("Hello, ".to_owned()+name+"!")
}

mod tests { // Separates tests from code
use super::hello; // Import root `hello()` function

    #[test]
    fn test_hello() {
        assert_eq!(hello("world"), "Hello, world!");
    }
}