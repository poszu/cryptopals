use std::iter::repeat;

pub fn calc_score(s: &str) -> usize {
    s.chars().fold(0, |acc, c| {
        let char_score = match c {
            'a' | 'A' => 8200,
            'b' | 'B' => 1500,
            'c' | 'C' => 2700,
            'd' | 'D' => 4700,
            'e' | 'E' => 13000,
            'f' | 'F' => 2200,
            'g' | 'G' => 2000,
            'h' | 'H' => 6200,
            'i' | 'I' => 6900,
            'j' | 'J' => 160,
            'k' | 'K' => 810,
            'l' | 'L' => 4000,
            'm' | 'M' => 2700,
            'n' | 'N' => 6700,
            'o' | 'O' => 7800,
            'p' | 'P' => 1900,
            'q' | 'Q' => 110,
            'r' | 'R' => 5900,
            's' | 'S' => 6200,
            't' | 'T' => 9600,
            'u' | 'U' => 270,
            'v' | 'V' => 970,
            'w' | 'W' => 2400,
            'x' | 'X' => 150,
            'y' | 'Y' => 2000,
            'z' | 'Z' => 78,
            ' ' => 19000,
            _ => 0,
        };
        acc + char_score
    })
}

/// Convert hex to base64
/// Set 1, challenge 1
fn hex_to_base64(hex: &str) -> Result<String, hex::FromHexError> {
    let bytes = hex::decode(hex)?;
    Ok(base64::encode(bytes))
}

pub fn hamming_distance(x: impl AsRef<[u8]>, y: impl AsRef<[u8]>) -> u32 {
    x.as_ref()
        .iter()
        .zip(y.as_ref().iter())
        .map(|(x, y)| (x ^ y).count_ones())
        .sum()
}

pub fn transpose(data: &[u8], width: usize) -> Vec<Vec<u8>> {
    let mut transposed = Vec::<Vec<u8>>::from_iter(repeat(vec![]).take(width));
    for w in data.chunks(width) {
        for (i, b) in w.iter().enumerate() {
            transposed[i].push(*b);
        }
    }
    transposed
}

#[test]
fn test_transpose() {
    assert_eq!(
        transpose(b"012345678", 3),
        vec![
            vec![b'0', b'3', b'6'],
            vec![b'1', b'4', b'7'],
            vec![b'2', b'5', b'8']
        ]
    );
}

pub fn calculate_average_distance(keysize: usize, data: &[u8]) -> usize {
    let x = data.chunks(keysize).collect::<Vec<&[u8]>>();
    let dist_sum = x
        .windows(2)
        .fold(0, |acc, w| acc + hamming_distance(w[0], w[1]));

    dist_sum as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn hex_to_base64() {
        assert_eq!(
        super::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
        Ok("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string())
        );
    }

    #[test]
    fn hamming_distance() {
        assert_eq!(
            super::hamming_distance(b"this is a test", b"this is a test"),
            0
        );
        assert_eq!(
            super::hamming_distance(b"this is a test", b"wokka wokka!!!"),
            37
        );
    }
}
