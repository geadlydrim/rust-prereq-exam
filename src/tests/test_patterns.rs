// Function: pattern_1
// Description: Returns true if the last two strings in the vector start with `BLOKC`.
// Parameters:
// - input: A vector of strings.
// Returns: True if the last two strings in the vector start with `BLOKC`, false otherwise.
pub fn pattern_1(input: Vec<String>) -> bool {
    if input.len() < 2 {
        return false;
    }

    let last_two = &input[input.len() - 2..];

    last_two.iter().all(|s| s.starts_with("BLOKC"))
}

// Function: pattern_2
// Description: Returns true if the first and last string in the vector start with `BLOKC`.
// Parameters:
// - input: A vector of strings.
// Returns: True if the first and last string in the vector start with `BLOKC`, false otherwise.
pub fn pattern_2(input: Vec<String>) -> bool {
    if input.len() < 2 {
        return false;
    }
    let (first, last) = (input.first().unwrap(), input.last().unwrap());

    if first.starts_with("BLOKC") && last.starts_with("BLOKC"){
        true
    }
    else{
        false
    }
}

// Function: pattern_3
// Description: Returns true if a string contains all the letters of the word 'BLOKC'.
// Parameters:
// - input: A string.
// Returns: True if a string contains all the letters of the word 'BLOKC', false otherwise.
pub fn pattern_3(input: &str) -> bool {
    let required = "BLOKC";

    for c in required.chars(){
        if !input.contains(c){
            return false;
        }
    }

    true
}

// Function: pattern_4
// Description: Returns a string that rearranges its characters in alphabetical order.
// Parameters:
// - input: A string.
// Returns: A string that rearranges its characters in alphabetical order.
pub fn pattern_4(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    
    chars.sort();

    chars.into_iter().collect()
}

// Function: pattern_5
// Description: Returns a string where characters similar to the first character are converted.
// Parameters:
// - input: A string.
// Returns: A string with characters similar to the first character converted.
pub fn pattern_5(input: &str) -> String {
    let first_char = input.chars().next().unwrap();
    let mut result = String::new();
    
    for c in input.chars(){
        result.push(first_char);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	fn test_pattern_1() {
		let strs_1 = vec![
			"HELLO".to_string(),
			"RUST".to_string(),
			"BLOKC".to_string(),
			"BLOKCCHANG".to_string(),
		];
		assert!(pattern_1(strs_1));

        let strs_2 = vec![
			"BLOKCS".to_string(),
			"WORLD".to_string(),
			"BLOKC".to_string(),
		];
		assert!(!pattern_1(strs_2));
	}

    #[test]
    fn test_pattern_2() {
        let strs_1 = vec![
			"HELLO".to_string(),
			"RUST".to_string(),
			"BLOKC".to_string(),
			"BLOCKCHANG".to_string(),
		];
		assert!(!pattern_2(strs_1));

        let strs_2 = vec![
			"BLOKCS".to_string(),
			"WORLD".to_string(),
			"BLOKC".to_string(),
		];
		assert!(pattern_2(strs_2));
    }

    #[test]
    fn test_pattern_3() {
		assert!(pattern_3("BLOKCBUSTER"));
		assert!(pattern_3("THEBLOKC"));
        assert!(pattern_3("KCOLB"));
        assert!(pattern_3("B*L*O*C*K"));
        assert!(!pattern_3("BLOB"));
        assert!(!pattern_3("IDONTKNOW"));
    }

    #[test]
    fn test_pattern_4() {
        assert_eq!(pattern_4("BLOCK"), "BCKLO");
        assert_eq!(pattern_4("HELLO"), "EHLLO");
        assert_eq!(pattern_4("EDCBA"), "ABCDE");
    }

    #[test]
    fn test_pattern_5() {
        assert_eq!(pattern_5("BLOCK"), "BBBBB");
        assert_eq!(pattern_5("HELLO"), "HHHHH");
        assert_eq!(pattern_5("RUST"), "RRRR");
        assert_eq!(pattern_5("CONGRATS"), "CCCCCCCC");
    }
}
