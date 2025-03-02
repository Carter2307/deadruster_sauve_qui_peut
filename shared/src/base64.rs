const BASE64_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/";
 
pub fn encode(input: &[u8]) -> String {
    let mut output = String::new();
    let mut buffer = 0u32;
    let mut bits = 0;
 
    for &byte in input {
        buffer = (buffer << 8) | (byte as u32);
        bits += 8;
        while bits >= 6 {
            bits -= 6;
            let index = (buffer >> bits) & 0b111111;
            output.push(BASE64_ALPHABET[index as usize] as char);
        }
    }
 
    if bits > 0 {
        buffer <<= 6 - bits;
        let index = buffer & 0b111111;
        output.push(BASE64_ALPHABET[index as usize] as char);
    }
 
    // while output.len() % 4 != 0 {
    //     output.push('=');
    // }
 
    output
}
 
pub fn decode(input: &str) -> Result<Vec<u8>, &'static str> {
    // Si la longueur de l'entrée est 4n+1, elle est immédiatement rejetée comme invalide
    if input.len() % 4 == 1 {
        return Err("Invalid Base64 length");
    }
 
    let mut output = Vec::new();
    let mut buffer = 0u32;
    let mut bits = 0;
 
    for c in input.chars() {
        // if c == '=' {
        //     break;
        // }
 
        let value = match BASE64_ALPHABET.iter().position(|&x| x as char == c) {
            Some(v) => v as u32,
            None => return Err("Invalid character in input"),
        };
 
        buffer = (buffer << 6) | value;
        bits += 6;
 
        if bits >= 8 {
            bits -= 8;
            output.push(((buffer >> bits) & 0xFF) as u8);
        }
    }
 
    Ok(output)
}
 
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_encode() {
        assert_eq!(encode(b"Hello"), "sgvSBg8");
    }
 
    #[test]
    fn test_decode() {
        assert_eq!(decode("sgvSBg8").unwrap(), b"Hello");
    }
 
    #[test]
    fn test_decode_invalid() {
        assert!(decode("SGVsbG@").is_err());
    }
 
    #[test]
    fn test_all_case() {
        assert_eq!(encode(&[0]), "aa");
        assert_eq!(encode(&[25]), "gq");
        assert_eq!(encode(&[26]), "gG");
        assert_eq!(encode(&[51]), "mW");
        assert_eq!(encode(&[52]), "na");
        assert_eq!(encode(&[61]), "pq");
        assert_eq!(encode(&[62]), "pG");
        assert_eq!(encode(&[63]), "pW");
        assert_eq!(encode(b"Hello, World!"), "sgvSBg8SifDVCMXKiq");
        assert_eq!(encode(&(0..=255).collect::<Vec<u8>>()), "aaecaWqfbGCicqOlda0odXareHmufryxgbKAgXWDhH8GisiJjcuMjYGPkISSls4VmdeYmZq1nJC4otO7pd0+p0bbqKneruzhseLks0XntK9quvjtvfvwv1HzwLTCxv5FygfIy2rLzMDOAwPRBg1UB3bXCNn0Dxz3EhL6E3X9FN+aGykdHiwgH4IjIOUmJy6pKjgsK5svLPEyMzQBNj2EN6cHOQoKPAANQkMQQ6YTRQ+WSBkZTlw2T7I5URU8VB6/WmhcW8tfXSFiYCRlZm3oZ9dr0Tpu1DBx2nNA29ZD3T/G4ElJ5oxM5+JP6UVS7E7V8phY8/t19VF4+FR7/p3+/W");
    }
}