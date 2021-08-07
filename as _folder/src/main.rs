mod package {
    pub mod main;
    pub use main as c;
}

fn main() {
    package::c::useful_meme()
}
