mod mod1 {
    pub mod thing11;
}
mod mod2 {
    pub mod thing21;
    pub use thing21::a25;
}
mod mod3;

fn main() {
    mod3::lol();
    mod2::a25();
}
