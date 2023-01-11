use std::fmt::Debug;
use std::iter;
use std::ops::{Index, IndexMut};
use crate::util::ensure_equal;

type Indices<'a> = &'a [usize];

#[derive(Debug)]
pub struct Tensor<T> {
    shape: Box<[usize]>,
    data: Box<[T]>,
}

impl<T> Tensor<T> {
    pub fn new(shape: Box<[usize]>, data: Box<[T]>) -> Tensor<T> {
        assert_eq!(shape.iter().product::<usize>(), data.len());
        Tensor {
            shape,
            data,
        }
    }

    pub fn slice(&self, from: Indices, to: Indices) -> &[T] {
        let from = self.compute_ix(from);
        let to = self.compute_ix_unchecked(to);
        &self.data[from..to]
    }

    pub fn slice_mut(&mut self, from: Indices, to: Indices) -> &mut [T] {
        let from = self.compute_ix(from);
        let to = self.compute_ix_unchecked(to);
        &mut self.data[from..to]
    }

    // A stride is a slice tensor.slice(&[a, b, ..., 0], (&[a, b, ..., l]) where l is length of
    // the final dimension
    pub fn strides(&self) -> impl Iterator<Item=&[T]> {
        self.data.chunks(*self.shape.last().unwrap())
    }

    pub fn strides_mut(&mut self) -> impl Iterator<Item=&mut [T]> {
        self.data.chunks_mut(*self.shape.last().unwrap())
    }

    pub fn shape(&self) -> &Box<[usize]> { &self.shape }

    pub fn data_mut(&mut self) -> &mut [T] { &mut self.data }

    fn assert_nonempty(&self) {
        assert!(self.shape.len() > 0);
    }

    //#[inline(always)]
    fn compute_ix(&self, indices: &[usize]) -> usize {
        self.compute_ix_internal::<true>(indices)
    }

    // Some operations require computing indices one-past the tensor array and we allow it
    fn compute_ix_unchecked(&self, indices: &[usize]) -> usize {
        self.compute_ix_internal::<false>(indices)
    }

    fn compute_ix_internal<const CHECKED: bool>(&self, indices: &[usize]) -> usize {
        self.assert_nonempty();
        let len = ensure_equal(self.shape.len(), indices.len());

        let mut ix = 0;
        let mut significance = 1;
        for i in (0..len).rev() {
            // dbg!(i, key, significance);
            let index = indices[i];
            let length = self.shape[i];
            if CHECKED { assert!(index < length); }
            ix += index * significance;
            significance *= length;
        }

        ix
        // self.shape.iter().zip(key.iter())
        //     .map(|(s, i)| s * i).sum()
    }
}

impl<T: Clone> Tensor<T> {
    pub fn of(shape: Box<[usize]>, elt: T) -> Tensor<T> {
        let data = iter::repeat(elt).take(shape.iter().product::<usize>()).collect();
        Tensor {
            shape,
            data,
        }
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

impl<T: Debug> Tensor<T> {
    pub fn print_strides(&self) {
        self.strides().for_each(|x| println!("{x:?}"))
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