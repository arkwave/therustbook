fn main() {

    // just a few examples of string slices in action. 
    // notice how first_word takes &s as argument instead of &String; this is because all
    // string literals are slices, which is why they are immutable to begin with. 
    let s = String::from("hello world");
    let word = first_word(&s[0..5]);
    let word2 = first_word(&s[..]);
    let word3 = first_word(&s);

    assert_eq!(word, word3);
    assert_eq!(word, word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
