// Import (via `use`) the fmt module to make it available.
use std::fmt;

#[derive(Debug)]
struct Structure(i32);

/*
impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " DEBUG SELF.0 =>  => {}", self.0)
    }
}
*/
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SELF.0 =>  => {}", self.0)
    }
}

fn main() {
println!("Print ouput: {:?}", Structure(4));
println!("Print ouput: {}", Structure(4));
}
