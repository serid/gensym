use std::collections::HashMap;
use std::fs::read_to_string;
use std::hash::Hash;
use std::iter;
use rand::Rng;
use unicode_normalization_alignments::UnicodeNormalization;
use crate::markov::{choose, cumulative_sum, cumulative_sum_slice};
use crate::tensor::Tensor;
use crate::util::get_one;

pub fn rus() {
    // Letter _ is used as token before word start and after end
    let letters = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя_".chars().collect::<Box<[char]>>();

    let letter_ids =
        letters.iter().enumerate().map(|(i, &c)| (c, i.try_into().unwrap()))
            .collect::<HashMap<char, u8>>();

    let s = read_to_string("input/warandpeace.txt").unwrap();
    let processed = s.chars()
        .nfkc()
        .map(|(c, _i)| c)
        .map(|c| if c.is_whitespace() && c != ' ' && c != '\n' { panic!() } else { c })
        .filter(|c| c.is_ascii_whitespace() || !c.is_ascii() && c.is_alphabetic());

    //.take(500)
    let chars = processed
        .map(|c| if c == '\n' { ' ' } else { c })
        .map(|c| get_one(c.to_lowercase()))
        .collect::<String>();

    let words = chars
        .split(' ')
        .filter(|w| !w.is_empty())
        .collect::<Box<[&str]>>();
    // println!("{:?}", &words);

    //count_words(&words);

    let tokenized_words = words.iter()
        .map(|&word| word.chars().map(|c| letter_ids[&c]).collect())
        .collect::<Box<[Box<[u8]>]>>();
    // println!("{:?}", &words);

    let depth = 4;

    let biases = load_probabilities(&letters, &letter_ids, depth, &tokenized_words);

    println!("Here are 10 words that look like russian:");
    for _ in 0..10 {
        println!("{}", infer(&letters, &letter_ids, depth, &biases));
    }

    // println!("{:?}", "абвгдеёжзийклмнопрстуфхцчшщъыьэюя".chars().map(|c| u32::from(c)).collect::<Box<[u32]>>());
    // println!("{:?}", char::from_u32(1104));
}

fn count_words(words: &[&str]) {
    let counts: HashMap<&str, usize> = count_hashmap(words.iter().copied());

    let mut s = counts.iter().collect::<Vec<_>>();
    s.sort_by(|x, y| x.1.cmp(&y.1).reverse());
    println!("{:?}", s);
}

fn count_hashmap<T: Eq + Hash>(i: impl Iterator<Item=T>) -> HashMap<T, usize> {
    let mut counts: HashMap<T, usize> = HashMap::new();
    for word in i {
        *counts.entry(word).or_default() += 1;
    }
    counts
}

fn load_probabilities(letters: &[char], letter_ids: &HashMap<char, u8>, depth: usize, words: &[Box<[u8]>]) -> Tensor<f32> {
    let letters_n = letters.len();

    // Load counts from unique words
    let mut counts = Tensor::of(
        iter::repeat(letters_n).take(depth).collect(), // [34, 34, 34, 34]
        0u32,
    );

    let none_index = usize::from(letter_ids[&'_']);

    words.iter().for_each(|word| {
        // Start with an empty word, shift in letters one by one. Index into tensor and increment
        // the matching counter
        // ___п
        // __пр
        // _при
        // прив
        // риве
        // ивет
        // вет_
        // Last one determines probability of terminating a sequence after certain 3 characters
        let mut indices = iter::repeat(none_index).take(depth.try_into().unwrap())
            .collect::<Box<[usize]>>();

        // Append '_' to word before iteration
        let none_index_usize = none_index.try_into().unwrap();
        let extended_word = word.iter().chain(iter::once(&none_index_usize));
        extended_word.enumerate().for_each(|(i, &t)| {
            // Shift in a character
            (0..(indices.len() - 1)).for_each(|j| {
                indices[j] = indices[j + 1];
            });
            *indices.last_mut().unwrap() = t.into();

            // Increment
            counts[&indices] += 1;
        })
    });

    // counts.print_strides();

    // For each stride in [counts] replace every count with a probability in range [0.0, 1.0]
    let mut result = Tensor::of(counts.shape().clone(), f32::NAN);

    counts.strides_mut()
        .zip(result.strides_mut())
        .for_each(|(counts, out)| {
            assert_eq!(counts.len(), letters_n);
            assert_eq!(out.len(), letters_n);

            let sum = counts.iter().sum::<u32>();

            if sum == 0 {
                // If sum is 0, then this sequence of letters will never occur
                // counts.iter().enumerate().for_each(|(i, &_)|
                //     out[i] = f32::NAN)
            } else {
                let cumsum = cumulative_sum(counts.iter()
                    .map(|&count| (count as f32) / (sum as f32)));

                cumsum
                    .enumerate()
                    .for_each(|(i, x)|
                    out[i] = x);

                // Apply cumulative sum for further inference
                // cumulative_sum_slice(out);
            }
        });

    result.print_strides();

    result
}

fn infer(letters: &[char], letter_ids: &HashMap<char, u8>, depth: usize, biases: &Tensor<f32>) -> String {
    let a_index = usize::from(letter_ids[&'а']);
    let none_index = usize::from(letter_ids[&'_']);

    let mut rng = rand::thread_rng();

    let mut result = String::new();

    // First several letters are known letters of the word, last one is 'а' and will be filled in after
    // inference, after that the letters will be shifted left.
    let mut window: Box<[usize]> = iter::repeat(none_index).take(depth - 1)
        .chain(iter::once(a_index))
        .collect();
    loop {
        // dbg!(&window);
        let indices1 = &window;
        let mut indices2 = window.clone();
        indices2[indices2.len() - 2] += 1;

        let r = rng.gen::<f32>();
        let n = choose(biases.slice(indices1, &indices2), r);

        // Stop when '_' is shifted in
        if n == none_index { break; }

        result.push(letters[n]);

        // Shift in the new letter
        (0..(window.len() - 1)).for_each(|j| {
            window[j] = window[j + 1];
        });
        window[indices2.len() - 2] = n;
        window[indices2.len() - 1] = a_index;
    }
    result
}