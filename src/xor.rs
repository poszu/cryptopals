/// XOR two equal-length strings
/// Set 1, challenge 2
fn xor_fixed(hex1: &str, hex2: &str) -> Result<String, hex::FromHexError> {
    assert_eq!(hex1.len(), hex2.len());

    let bytes1 = hex::decode(hex1)?;
    let bytes2 = hex::decode(hex2)?;

    let out: Vec<u8> = bytes1
        .into_iter()
        .zip(bytes2.into_iter())
        .map(|(x, y)| x ^ y)
        .collect();

    Ok(hex::encode(out))
}

/// Set 1, challenge 3
pub fn crack_single_byte_xor_cipher(
    in_bytes: impl AsRef<[u8]>,
) -> Result<Option<(usize, u8, String)>, hex::FromHexError> {
    let mut best_score = 0;
    let mut res = None;
    for b in u8::MIN..u8::MAX {
        let maybe_cracked: Vec<u8> = in_bytes.as_ref().iter().map(|in_b| in_b ^ b).collect();
        if let Ok(msg) = String::from_utf8(maybe_cracked) {
            let score = crate::utils::calc_score(&msg);
            if score > best_score {
                res = Some((score, b, msg));
                best_score = score;
            }
        }
    }
    Ok(res)
}

/// Set 1, challenge 5
pub fn repeating_key_xor(input: impl AsRef<[u8]>, key: impl AsRef<[u8]>) -> Vec<u8> {
    input
        .as_ref()
        .iter()
        .zip(key.as_ref().iter().cycle())
        .map(|(b, k)| b ^ k)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {

    #[test]
    fn xor_fixed() {
        assert_eq!(
            super::xor_fixed(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            Ok("746865206b696420646f6e277420706c6179".to_string())
        );
    }

    #[test]
    fn repeating_key_xor() {
        assert_eq!(
            hex::encode(super::repeating_key_xor(
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
                "ICE"
            )),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
                .to_string()
        );
    }

    #[test]
    fn crack_single_byte_xor_cipher() {
        let res = super::crack_single_byte_xor_cipher(
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            res,
            Some((251490, 88, "Cooking MC's like a pound of bacon".to_string()))
        );
    }
}
