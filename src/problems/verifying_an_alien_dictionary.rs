use std::{cmp::Ordering, collections::HashMap};

/**
In an alien language, surprisingly, they also use English lowercase letters, but possibly in a different order. The order of the alphabet is some permutation of lowercase letters.

Given a sequence of words written in the alien language, and the order of the alphabet, return true if and only if the given words are sorted lexicographically in this alien language.

Example 1:
```
Input: words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
Output: true
Explanation: As 'h' comes before 'l' in this language, then the sequence is sorted.
```
Example 2:
```
Input: words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
Output: false
Explanation: As 'd' comes after 'l' in this language, then words[0] > words[1], hence the sequence is unsorted.
```
Example 3:
```
Input: words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
Output: false
Explanation: The first three characters "app" match, and the second string is shorter (in size.) According to lexicographical rules "apple" > "app", because 'l' > '∅', where '∅' is defined as the blank character which is less than any other character (More info).
```

Constraints:
- `1 <= words.length <= 100`
- `1 <= words[i].length <= 20`
- `order.length == 26`

All characters in `words[i]` and order are English lowercase letters.
*/
pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let mut order_map = HashMap::new();

    for (index, char) in order.chars().enumerate() {
        order_map.insert(char, index);
    }

    let mut word_index = 0;
    loop {
        let a_word = words.get(word_index).unwrap();
        let b_word = words.get(word_index + 1).unwrap();
        for (index, a_char) in a_word.chars().enumerate() {
            if let Some(b_char) = b_word.chars().nth(index) {
                let a_val = order_map.get(&a_char).unwrap_or(&0);
                let b_val = order_map.get(&b_char).unwrap_or(&0);
                match a_val.cmp(b_val) {
                    Ordering::Equal => continue,
                    Ordering::Less => break,
                    Ordering::Greater => return false,
                }
            } else {
                return false;
            }
        }

        word_index += 1;
        if word_index > words.len() - 2 {
            break;
        }
    }

    return true;
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec!["hello".to_string(), "leetcode".to_string()];
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
        assert_eq!(is_alien_sorted(words, order), true);
    }

    #[test]
    fn test_2() {
        let words = vec!["word".to_string(), "world".to_string(), "row".to_string()];
        let order = "worldabcefghijkmnpqstuvxyz".to_string();
        assert_eq!(is_alien_sorted(words, order), false);
    }

    #[test]
    fn test_3() {
        let words = vec!["apple".to_string(), "app".to_string()];
        let order = "abcdefghijklmnopqrstuvwxyz".to_string();
        assert_eq!(is_alien_sorted(words, order), false);
    }
}
