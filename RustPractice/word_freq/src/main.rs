
fn most_frequent_word(text: &str) -> (String, usize) {
    
    // Convert the text to lowercase
    let text = text.to_lowercase();
    
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut max_word = String::new();
    let mut max_count = 0;

    for (i, &word) in words.iter().enumerate() {
        let mut count = 1;
        for &other_word in &words[i + 1..] {
            if word == other_word {
                count += 1;
            }
        }
        if count > max_count {
            max_count = count;
            max_word = word.to_string();
        }
    }

    (max_word, max_count)
}

fn main() {
    let text = "Peter Piper picked a peck of pickled peppers.
A peck of pickled peppers Peter Piper picked.
If Peter Piper picked a peck of pickled peppers,
Where’s the peck of pickled peppers Peter Piper picked?";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
