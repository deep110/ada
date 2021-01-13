
// forbid unsafe code
#![forbid(unsafe_code)]

mod point;
mod shape;
mod canvas;

pub use canvas::Canvas;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
