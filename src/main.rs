extern crate core;

use crate::rus::rus;
use crate::tensor::Tensor;

mod tensor;
mod util;
mod rus;
mod markov;

fn main() {
    println!("Hello, world!");

    let t = Tensor::new(Box::new([2, 3]), Box::new([1, 2, 3, 4, 5, 6]));

    t.print_strides();

    println!("{}", t[&[0, 2]]);
    println!("{}", t[&[1, 1]]);
    println!("{}", t[&[1, 2]]);

    rus();
}
