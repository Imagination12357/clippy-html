//! Bounded blocking HTTPS retrieval; automatic redirects are intentionally disabled.
use std::{io::Read, time::Duration};

pub(super) const MAX_BYTES: usize = 8 * 1024 * 1024;

pub(super) fn read_bounded(reader: impl Read) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    reader
        .take((MAX_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > MAX_BYTES {
        return Err("Category document exceeds 8 MiB".into());
    }
    Ok(bytes)
}

pub(super) fn download(url: &str) -> Result<Vec<u8>, String> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .https_only(true)
        .max_redirects(0)
        .timeout_global(Some(Duration::from_secs(10)))
        .build()
        .into();
    let mut response = agent.get(url).call().map_err(|e| e.to_string())?;
    if response.status() != 200 {
        return Err(format!(
            "Category document HTTP status: {}",
            response.status()
        ));
    }
    read_bounded(response.body_mut().as_reader())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn body_limit_and_io_errors_are_explicit() {
        assert_eq!(read_bounded(&b"abc"[..]).unwrap(), b"abc");
        assert!(read_bounded(std::io::repeat(0).take(MAX_BYTES as u64 + 1)).is_err());
        struct Broken;
        impl Read for Broken {
            fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("body failed"))
            }
        }
        assert!(read_bounded(Broken).unwrap_err().contains("body failed"));
    }
}
