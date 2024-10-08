fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut word_counts: Vec<(String, usize)> = Vec::new(); // vector to store word counts
    
    for &word in &words {
        let mut found = false;
        for (w, count) in &mut word_counts {
            if w == word {
                *count += 1;
                found = true;
                break;
            }
        }
        if !found {
            word_counts.push((word.to_string(), 1));
        }
    }

    let mut max_word = String::new();
    let mut max_count = 0;
    
    for (word, count) in word_counts {
        if count > max_count {
            max_count = count;
            max_word = word;
        }
    }

    (max_word, max_count) // return the word with the highest count and its frequency
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}

