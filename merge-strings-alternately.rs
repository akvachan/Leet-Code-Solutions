fn main() {
    let word1 = "abc".to_string();
    let word2 = "pqroooo".to_string();
    let result = merge_alternately(word1, word2);
    assert_eq!("apbqcroooo", result);
}


/*
Since LeetCode problems assumes only Latin Chars with fixed byte-size.
For simplicity reasons I will iterate over chars and not bytes.
Even though code could have been much more performant were word1
and word2 just bytes, or char vectors for that matter.
*/
fn merge_alternately(word1: String, word2: String) -> String {

    let mut result = String::from("");

    // Iterating over both Strings simultaneously and pushing chars
    for (char1, char2) in word1.chars().zip(word2.chars()) {
        result.push(char1);
        result.push(char2);
    }
    
    // Handling overhead
    let w1_len = word1.len();
    let w2_len = word2.len();
    result.push_str({
        if w1_len > w2_len {
            &word1[w2_len..]
        } else if w1_len < w2_len {
            &word2[w1_len..]
        } else {
            ""
        }
    });

    return result;
}
