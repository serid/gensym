use std::fmt::Debug;
use std::ops::{Index, IndexMut};

type Indices<'a> = &'a [usize];

#[derive(Debug)]
pub struct Tensor<T> {
    shape: Box<[usize]>,
    data: Box<[T]>,
}

impl<T> Tensor<T> {
    pub(crate) fn new(shape: Box<[usize]>, data: Box<[T]>) -> Tensor<T> {
        Tensor {
            shape,
            data,
        }
    }

    fn compute_ix(&self, index: &[usize]) -> usize {
        assert_eq!(self.shape.len(), index.len());

        let significance = 1;
        for i in index.iter().rev() {

        }

        !unimplemented!();
        self.shape.iter().zip(index.iter())
            .map(|(s, i)| s * i).sum()
    }
}

impl<T> Index<&[usize]> for Tensor<T> {
    type Output = T;

    fn index(&self, index: &[usize]) -> &Self::Output {
        &self.data[self.compute_ix(index)]
    }
}

impl<T> IndexMut<&[usize]> for Tensor<T> {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        &mut self.data[self.compute_ix(index)]
    }
}

// impl<T: Debug> Debug for Tensor<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut indices: Box<[usize]> = iter::repeat(0).take(self.shape.len()).collect();
//
//         fn local<T: Debug>(this: &Tensor<T>, f: &mut Formatter, indices: &mut [usize], level: usize) -> std::fmt::Result {
//             if level < this.shape.len() {
//                 for i in 0..this.shape[level] {
//                     indices[level] = i;
//                     local(this, f, indices, level + 1);
//                 }
//             } else if level == this.shape.len() {
//                 writeln!(f, "{:?}", this[&indices])?;
//             } else {
//                 unreachable!()
//             }
//             Ok(())
//         }
//
//         local(self, f, &mut indices, 0)?;
//
//         writeln!(f, "")?;
//         Ok(())
//     }
// }