use std::fmt::Debug;

pub fn compress<T: Eq + Copy + Debug>(input: &[T]) -> Vec<(u8, T)> {
    let mut encoded = Vec::new();
    if input.is_empty() {
        return encoded;
    }
    let mut count: u8 = 1;
    let mut prev = input[0];
    for &item in &input[1..] {
        if item == prev && count < u8::MAX {
            count += 1;
        } else {
            encoded.push((count, prev));
            prev = item;
            count = 1;
        }
    }
    encoded.push((count, prev));
    println!("{:?}", encoded);
    encoded
}

pub fn decompress<T: Copy>(input: &[(u8, T)]) -> Vec<T> {
    let mut result = Vec::new();
    for &(count, value) in input {
        for _ in 0..count {
            result.push(value);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_roundtrip() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}
