mod a {
    pub mod b {
        pub mod boi;
    }
    pub use b::boi::mary;
}

use a::b::boi::mary;

fn main() {
    a::b::boi::mary();
    a::mary();
    mary();
}
