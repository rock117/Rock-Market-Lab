use std::io::prelude::*;

use flate2::{Compression, write::GzEncoder};
use flate2::read::GzDecoder;

pub fn compress(data: &[u8]) -> anyhow::Result<Vec<u8>>{
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}

pub fn de_compress(data: &[u8])  -> anyhow::Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        compress(b"hello world").unwrap();
    }
}