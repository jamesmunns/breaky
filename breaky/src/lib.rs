pub use berp::*;

// Uncomment me!
// use darp::Foo;

pub fn example() {
    let _ = Foo;
}

pub mod berp {
    pub struct Foo;

    impl Foo {
        pub fn hah(&self) {}
        pub fn hoh(&self) {}
    }
}

pub mod darp {
    pub struct Foo;
}
