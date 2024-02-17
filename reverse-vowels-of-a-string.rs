fn main() {
    let s: String = String::from("hello");
    let expected: String = String::from("holle");
    assert_eq!(expected, reverse_vowels(s));
}

fn reverse_vowels(s: String) -> String {
    /// Checks if a character is a vowel.
    /// 
    /// Considering the small set of vowels, using `matches!` is efficient and simplifies
    /// the logic without needing a HashSet for lookup, which would be overkill given the
    /// limited number of vowel characters.
    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
    }

    // Convert the input string to a vector of characters to allow swapping.
    let mut chars: Vec<char> = s.chars().collect();

    // Two pointers approach: One starts from the beginning (left) and the other from the
    // end (right). This method eliminates the need for a separate data structure to store
    // vowels and is more space and time-efficient.
    // The idea is to advance each pointer towards the center, swapping vowels when both
    // pointers identify vowels, thus reversing the order of vowels in-place.
    let mut left = 0;
    let mut right = chars.len().saturating_sub(1);

    while left < right {
        if !is_vowel(chars[left]) {
            left += 1;
            continue;
        }
        if !is_vowel(chars[right]) {
            right = right.saturating_sub(1);
            continue;
        }

        chars.swap(left, right);
        left += 1;
        right = right.saturating_sub(1);
    }

    // Convert the characters back into a string.
    chars.into_iter().collect()
}
