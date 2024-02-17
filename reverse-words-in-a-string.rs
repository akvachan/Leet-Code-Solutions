fn main() {
	let s: String = String::from("the sky is blue");
	let result_gold: String = String::from("blue is sky the");
	assert_eq!(result_gold, reverse_words(s));
}

fn reverse_words(s: String) -> String {
        
        // Maybe this solution is not idiomatic for Rust, but I like it.
        // It is clear and self-explanatory.
        // Idiomatic solution with .rev() and .join(" ") tokenized vector
        // does not bring any significant memory/performance boost.

        // Split String by whitespaces -> Tokenize it
        let mut tokens: Vec<&str> = s.split_whitespace().collect();

        // Create an empty String to push words in reveresed order to
        let mut result: String = String::new();

        // Insert tokens and whitespaces until there are no more tokens
        loop {
            result.push_str(tokens.pop().unwrap() as &str);
            if tokens.len() == 0 {
                break;
            }
            result.push_str(" ");
        }
        
        return result;
}
