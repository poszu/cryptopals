fn break_repeating_xor_cipher(data: impl AsRef<[u8]>) -> String {
    let data = data.as_ref();

    let mut best = (0, usize::MAX);
    for keysize in 2..40 {
        let distance = cryptopals::utils::calculate_average_distance(keysize, data);
        if distance < best.1 {
            best = (keysize, distance);
        }
    }
    let keysize = best.0;
    let transposed = cryptopals::utils::transpose(data, keysize);

    let mut key = Vec::<u8>::new();
    for block in transposed {
        let (_, k, _) = cryptopals::xor::crack_single_byte_xor_cipher(block)
            .unwrap()
            .unwrap();
        key.push(k);
    }
    String::from_utf8(key).unwrap()
}

fn main() {
    let data = base64::decode(include_str!("set_1_6.txt").replace('\n', "")).unwrap();
    let key = break_repeating_xor_cipher(&data);
    println!("The key: '{}'", key);
    println!(
        "Decoded message: '{}'",
        String::from_utf8_lossy(&cryptopals::xor::repeating_key_xor(data, key))
    );
}
