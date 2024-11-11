// The reason it was failing is that HashSet.difference() assumes the same
// lifetime for both sets it is invoked with.

use std::collections::HashSet;

struct Difference<'first, 'second> {
    first_only: Vec<&'first str>,
    second_only: Vec<&'second str>,
}

fn hash_sub<'fst, 'snd>(lhs: &HashSet<&'fst str>, rhs: &HashSet<&'snd str>) -> HashSet<&'fst str> {
    return lhs.iter().filter(|w| !rhs.contains(*w)).cloned().collect();
}

fn find_difference<'fst, 'snd>(
    sentence1: &'fst str,
    sentence2: &'snd str,
) -> Difference<'fst, 'snd> {
    let sentence_1_words: HashSet<&str> = sentence1.split(" ").collect();
    let sentence_2_words: HashSet<&str> = sentence2.split(" ").collect();

    // let words_1_minus_2: HashSet<&'fst str> = &sentence_1_words - &sentence_2_words;
    // let words_2_minus_1: HashSet<&'snd str> = &sentence_2_words - &sentence_1_words;
    let words_1_minus_2 = hash_sub(&sentence_1_words, &sentence_2_words);
    let words_2_minus_1 = hash_sub(&sentence_2_words, &sentence_1_words);

    Difference {
        first_only: words_1_minus_2.into_iter().collect(),
        second_only: words_2_minus_1.into_iter().collect(),
    }
}

fn main() {
    let first_sentence = String::from("I love the surf and the sand.");
    let second_sentence = String::from("I hate the surf and the sand.");

    let first_only = {
        let third_sentence = String::from("I hate the snow and the sand.");
        let diff = find_difference(&first_sentence, &third_sentence);
        diff.first_only
    };

    assert_eq!(
        first_only.into_iter().collect::<HashSet<&str>>(),
        ["love", "surf"].iter().cloned().collect()
    );

    let second_only = {
        let third_sentence = String::from("I hate the snow and the sand.");
        let diff = find_difference(&third_sentence, &second_sentence);
        diff.second_only
    };

    assert_eq!(second_only, vec!["surf"]);
}
