fn main() {
    let str1 = String::from("TAUXXTAUXXTAUXXTAUXXTAUXX");
    let str2 = String::from("TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX");
    assert_eq!("TAUXX", gcd_of_strings(str1, str2));
}

// Finding a string that divides both str1 and str2 without remainder
// So "ABC" divides both "ABC" and "ABCABC" without remainder, 
// or "JK" divides both "JKJK" and "JKJKJK" without remainder
// but "AS" does not divide "ASA" and "ASASA" without remainder
fn gcd_of_strings(str1: String, str2: String) -> String {

    // If str1 + str2 is not equal to str2 + str1, then there is no gcd, so return ""
    // "JKJK" + "JKJKJK" == "JKJKJK" + "JK" <=> "JKJKJKJKJK" == "JKJKJKJKJK"
    // "ASA" + "ASASA" != "ASASA" + "ASA" <=> "ASAASASA" != "ASASAASA" 
    if format!("{str1}{str2}") != format!("{str2}{str1}") {
        return String::from("");
    }
    
    // Else calculate the gcd of lengths' of str1 and str2 and take corresponding number of characters
    // gcd of "ABC".len() and of "ABCABC".len() is 3
    // So take first three chars (excluding the last) either from str1 or str2 (does not matter),
    // I choose str1, so gcd = "ABC"
    let remainder = str1[..gcd(str1.len(), str2.len())].to_string();
    return remainder;
}

// Classic Euclidean algorithm for computing gcd
fn gcd(str1_len: usize, str2_len: usize) -> usize{
    if str2_len == 0 {
        return str1_len;
    }
    return gcd (str2_len, str1_len % str2_len);
}