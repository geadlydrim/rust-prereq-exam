// Function: parse_u32
// Description: Parses a string into an unsigned integer 32 and compares it with the provided unsigned integer 32.
// Parameters:
// - a: The string to parse.
// - b: The expected unsigned integer 32.
// Returns: true if the parsed value matches the expected value, false otherwise.
pub fn parse_u32(a: &str, b: &u32) -> bool {
    if let Ok(parsed) = a.parse::<u32>() {
        if parsed == *b {
            return true;
        }
    }
    false
}

// Function: parse_u128
// Description: Converts an unsigned integer 32 to an unsigned integer 128 and compares it with the provided unsigned integer 128.
// Parameters:
// - a: The unsigned integer 32 to convert.
// - b: The expected unsigned integer 128.
// Returns: true if the converted value matches the expected value, false otherwise.
pub fn parse_u128(a: u32, b: u128) -> bool {
    let convert = a as u128;
    if convert == b {
        return true;
    }
    false
}

// Function: parse_i32
// Description: Parses a string into a signed integer 32 and compares it with the provided signed integer 32.
// Parameters:
// - a: The unsigned integer 32 to parse.
// - b: The expected signed integer 32.
// Returns: true if the parsed value matches the expected value, false otherwise.
pub fn parse_i32(a: u32, b: i32) -> bool {
    let parsed_string = a.to_string();
    
    if let Ok(parsed) = parsed_string.parse::<i32>(){
        parsed == b
    }
    else{
        false
    }
}

// Function: parse_f32
// Description: Converts an unsigned integer 32 to a float 32 and compares it with the provided float 32.
// Parameters:
// - a: The unsigned integer 32 to convert.
// - b: The expected float 32.
// Returns: true if the converted value matches the expected value, false otherwise.
pub fn parse_f32(a: u32, b: f32) -> bool {
    let parsed = a as f32;
    if parsed == b {
        return true;
    }
    false
}

// Function: parse_string
// Description: Converts an unsigned integer 32 to a string and compares it with the provided string.
// Parameters:
// - a: The unsigned integer 32 to convert.
// - b: The expected string.
// Returns: true if the converted value matches the expected value, false otherwise.
pub fn parse_string(a: &u32, b: &str) -> bool {
    let parsed = a.to_string();
    if parsed == b {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_parse() {
        assert!(parse_u32("2", &2));
        assert!(parse_u32("2024", &2024));
    }

    #[test]
    fn test_u128_parse() {
        let (a, c): (u32, u32) = (12, 60);
        let (b, d): (u128, u128) = (12, 60);
        assert!(parse_u128(a, b));
        assert!(parse_u128(c, d));
    }

    #[test]
    fn test_i32_parse() {
        let (a, c): (u32, u32) = (1, 55);
        let (b, d): (i32, i32) = (1, 55);
        assert!(parse_i32(a, b));
        assert!(parse_i32(c, d));
    }

    #[test]
    fn test_f32_parse() {
        assert!(parse_f32(32, 32.0));
        assert!(parse_f32(14, 14.0));
    }

    #[test]
    fn test_string_parse() {
        assert!(parse_string(&5, "5"));
        assert!(parse_string(&10, "10"));
    }
}