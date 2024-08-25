use core::str;
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::ops;

fn open_file(file_name: &str) -> Result<File> {
    let file = File::open(file_name)?;
    Ok(file)
}

#[allow(clippy::naive_bytecount)]
fn get_line_count_in_buffer(buffer: &[u8]) -> usize {
    buffer.iter().filter(|&&c| c == b'\n').count()
}

fn get_word_count_in_buffer(buffer: &[u8], in_word: bool) -> (usize, bool) {
    let boundary_characters = [b'\n', b' ', b'\r', b'\t'];
    let mut word_count = 0;
    let mut on_word = in_word;
    for c in buffer {
        if boundary_characters.contains(c) {
            on_word = false;
        } else if !on_word {
            on_word = true;
            word_count += 1;
        }
    }
    (word_count, on_word)
}

fn get_multibyte_count(buffer: &[u8]) -> usize {
    String::from_utf8_lossy(buffer).len()
}

pub fn get_counts(file_name: String) -> Result<ResultOutput> {
    let file = open_file(&file_name)?;
    let mut result = ResultOutput::new(file_name);
    result.character_count = usize::try_from(file.metadata()?.len()).unwrap();
    let mut reader = BufReader::new(file);
    let mut in_word = false;
    let mut buffer = [0; 1_000];
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => {
                result.multibyte_count += get_multibyte_count(&buffer[..x]);
                result.line_count += get_line_count_in_buffer(&buffer[..x]);
                let (word_count, now_in_word) = get_word_count_in_buffer(&buffer[..x], in_word);
                result.word_count += word_count;
                in_word = now_in_word;
            }
            Err(e) => panic!("{e}"),
        }
    }
    Ok(result)
}

#[derive(Debug, Clone, Default)]
pub struct ResultOutput {
    name: String,
    line_count: usize,
    word_count: usize,
    character_count: usize,
    multibyte_count: usize,
}

impl ResultOutput {
    pub fn new(file_name: String) -> Self {
        Self {
            name: file_name,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for ResultOutput {
    //      12      36     237 Cargo.toml
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{:>8}{:>8}{:>8}{:>8} {file_name}",
            self.line_count,
            self.word_count,
            self.character_count,
            self.multibyte_count,
            file_name = self.name
        )
    }
}

impl ops::AddAssign<Self> for ResultOutput {
    fn add_assign(&mut self, rhs: Self) {
        self.character_count += rhs.character_count;
        self.line_count += rhs.line_count;
        self.word_count += rhs.word_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines() {
        let sample = " of which the \"13 chapters\" formed the first _chuan_,\r\nadding that there were two other _chuan_ besides. This has brought\r\nforth a theory, that the bulk of these 82 chapters consisted of other into practice the smaller details in which his work\r\nabounds, they have overlooked its essential purport. That is the motive\r\nwhich has led me to outline a rough explanation of the whole.\r\n\r\nOne thing to be noticed in the above is the explicit statement that the\r\n13 chapters were specially composed for King Ho Lu. This is supported\r\nby the internal evidence of I. § 15, in which it seems clear that some\r\nruler is addressed.\r\n\r\nIn the bibliographic section of the _Han Shu_, there is an entry which\r\nhas given rise to much discussion: \"The works of Sun Tzŭ of Wu in 82\r\n_p’ien_ (or chapters), with diagrams in 9 _chuan_.\" It is evident that\r\nthis cannot be merely the 13 chapters known to Ssu-ma Ch’ien, or those\r\nwe possess today. Chang Shou-chieh refers to an edition of Sun Tzŭ’s\r\n_Art of War_";
        let buffer = sample.as_bytes();
        assert_eq!(get_line_count_in_buffer(buffer), 16);
    }
    #[test]
    fn test_multibyte_count() {
        let sample = "Gutenberg™";
        let buffer = sample.as_bytes();
        assert_eq!(sample.len(), 12);
        assert_eq!(buffer.len(), 12);
        eprintln!("Testing 8 - {}", String::from_utf8_lossy(&buffer[9..]));
        assert_eq!(String::from_utf8_lossy(buffer).len(), 12);
    }
}
