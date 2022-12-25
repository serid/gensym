extern crate core;

use crate::rus::rus;
use crate::tensor::Tensor;

mod tensor;
mod util;
mod rus;

fn main() {
    println!("Hello, world!");
    println!("{:?}", Tensor::new(Box::new([3]), Box::new([1, 2, 3])));

    rus();
}
