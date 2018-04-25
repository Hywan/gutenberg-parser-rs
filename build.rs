#[cfg(feature = "nodejs")] extern crate neon_build;

#[cfg(feature = "nodejs")]
fn main() {
    neon_build::setup();
}

#[cfg(not(feature = "nodejs"))]
fn main() {
}
