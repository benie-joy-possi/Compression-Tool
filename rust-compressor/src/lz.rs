use std::fmt::Debug;

#[derive(Debug)]
pub enum Token<T> {
    Literal(T),
    Match { offset: usize, length: usize },
}

pub fn compress_lz<T: Eq + Clone + Debug>(input: &[T]) -> Vec<Token<T>> {
    let window_size = 20;
    let min_match_len = 3;
    let mut output = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let mut match_offset = 0;
        let mut match_length = 0;

        let start = i.saturating_sub(window_size);
        let window = &input[start..i];

        for offset in 1..=window.len() {
            let mut length = 0;
            while length < input.len() - i
                && (window.len() - offset + length) < window.len()
                && window[window.len() - offset + length] == input[i + length]
            {
                length += 1;
            }

            if length >= min_match_len && length > match_length {
                match_offset = offset;
                match_length = length;
            }
        }

        if match_length >= min_match_len {
            output.push(Token::Match {
                offset: match_offset,
                length: match_length,
            });
            i += match_length;
        } else {
            output.push(Token::Literal(input[i].clone()));
            i += 1;
        }
    }
    println!("{:?}", output);
    output
}

pub fn decompress_lz<T: Clone>(tokens: &[Token<T>]) -> Vec<T> {
    let mut output = Vec::new();

    for token in tokens {
        match token {
            Token::Literal(val) => output.push(val.clone()),
            Token::Match { offset, length } => {
                let start = output.len().saturating_sub(*offset);
                let segment = output[start..start + *length].to_vec(); // handles overlapping copies
                output.extend_from_slice(&segment);
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lz_roundtrip() {
        let input = b"ABABABABABAB";
        let compressed = compress_lz(input);
        let decompressed = decompress_lz(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}
