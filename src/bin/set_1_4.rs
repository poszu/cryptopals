fn main() {
    let input = include_str!("set_1_4.txt");
    let res = input
        .lines()
        .filter_map(|l| {
            cryptopals::xor::crack_single_byte_xor_cipher(hex::decode(l).unwrap()).unwrap_or(None)
        })
        .max_by_key(|(score, _, _)| *score);
    assert_eq!(
        res,
        Some((235630, 53, "Now that the party is jumping\n".to_string()))
    );
}
