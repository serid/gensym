// trait CumulativeSum : Iterator {
//     fn cumulative_sum<I: Iterator<Item=f32>>(it: I) -> Scan<I, f32, _> {
//         it.scan(0.0, |x, y| {
//             *x += y;
//             Some(*x)
//         })
//     }
// }

pub fn cumulative_sum(it: impl Iterator<Item=f32>) -> impl Iterator<Item=f32> {
    it.scan(0.0, |x, y| {
        *x += y;
        Some(*x)
    })
}

pub fn cumulative_sum_slice(xs: &mut [f32]) {
    for i in 1..xs.len() {
        xs[i] += xs[i - 1]
    }
}

/// Choose a random natural numbers given biases for each number.
/// [biases] array should be processed by [cumulative_sum_slice] beforehand.
pub fn choose(biases: &[f32], num: f32) -> usize {
    // check for off-by-one
    // biases.binary_search(&num).unwrap()

    for (i, &x) in biases.iter().enumerate() {
        if num < x {
            return i
        }
    }
    unreachable!();
}