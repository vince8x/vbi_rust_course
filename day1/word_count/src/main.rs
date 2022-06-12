use regex::Regex;
use std::io;

fn main() {
    const TEXT_TO_SEARCH: &str =
        "https://ars.els-cdn.com/content/image/1-s2.0-S0960982203005347-mmc6.txt";

    let mut input_word = String::new();
    println!("Enter the search word:");
    io::stdin().read_line(&mut input_word).unwrap();

    let result = search_by_matches(TEXT_TO_SEARCH, &input_word);

    println!("Result is: {}", result);
}

fn search_by_matches(text: &str, substr: &str) -> usize {
    return text.matches(substr.trim()).count();
}

fn search_by_regex(text: &str, regex: &str) -> u32 {
    let re: Regex = Regex::new(regex).unwrap();

    let mut result: u32 = 0;
    for cap in re.captures_iter(text) {
        result += 1;
    }
    return result;
}

#[test]
fn test_search_by_regex() {
    let text: &str = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    let regex_str = r"This\sis";
    assert_eq!(search_by_regex(text, regex_str), 5);
}

#[test]
fn test_search_by_matches() {
    let text: &str = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    assert_eq!(search_by_matches(text, "This is"), 5);
}
