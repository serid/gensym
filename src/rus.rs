use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::Hash;
use std::iter;
use unicode_normalization_alignments::UnicodeNormalization;
use crate::tensor::Tensor;
use crate::util::{get_one, naturals};

pub fn rus() {
    // Letter _ is used as token before word start
    let letter_ids = HashMap::<char, usize>::from_iter(
        "абвгдеёжзийклмнопрстуфхцчшщъыьэюя_".chars().zip(naturals())
    );
    let letters = HashSet::<char>::from_iter(letter_ids.keys().copied());

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

    let counts: HashMap<&str, usize> = count_hashmap(words.iter().copied());

    let mut s = counts.iter().collect::<Vec<_>>();
    s.sort_by(|x, y| x.1.cmp(&y.1).reverse());
    println!("{:?}", s);

    let depth = 4;
    // Load probablities from unique words
    let tensor = Tensor::new(
        iter::repeat(letters.len()).take(depth).collect(), // [34, 34, 34, 34]
        iter::repeat(0).take(34_usize.checked_pow(depth.try_into().unwrap()).unwrap()).collect()
    );

    // println!("{:?}", "абвгдеёжзийклмнопрстуфхцчшщъыьэюя".chars().map(|c| u32::from(c)).collect::<Box<[u32]>>());
    // println!("{:?}", char::from_u32(1104));
}

fn count_hashmap<T: Eq + Hash>(i: impl Iterator<Item=T>) -> HashMap<T, usize> {
    let mut counts: HashMap<T, usize> = HashMap::new();
    for word in i {
        *counts.entry(word).or_default() += 1;
    }
    counts
}