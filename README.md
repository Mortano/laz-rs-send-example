# laz-rs-send-example

Small example illustrating a usage of the [`las-rs` crate](https://crates.io/crates/las) together with [`Rocket`](https://rocket.rs/). Requires Rust nightly to build, and also fails to build (which is the point of this example). 

The main problem is that the `las::Reader` type is not `Send` and therefore can't be used in the shared state of Rocket. 
