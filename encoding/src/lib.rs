#![deny(clippy::all, clippy::pedantic, rust_2018_idioms, clippy::unwrap_used)]
pub use regex::Error as RegexError;
use regex::Regex;

#[must_use]
pub fn checksum_string(data: &[u8]) -> String {
    let hash = ring::digest::digest(&ring::digest::SHA256, data);
    data_encoding::HEXLOWER.encode(hash.as_ref())
}

const FILENAME_SAFE_CHAR: &str = "_";

#[cfg(target_os = "windows")]
fn windows_extra_safe<S: AsRef<str>>(input: S) -> String {
    let windows_reserved_filenames = Regex::new("^(con|prn|aux|nul|com\\d|lpt\\d)$").unwrap();
    if windows_reserved_filenames.is_match(input.as_ref()) {
        format!("{}{}", input.as_ref(), FILENAME_SAFE_CHAR)
    } else {
        input.as_ref().to_string()
    }
}

/// Get a string safe for filename.
///
/// # Errors
///
/// Fails if regex fails to compile (should never happen).
pub fn safe_filename<S: AsRef<str>>(input: S) -> Result<String, regex::Error> {
    let reserved_chars = Regex::new("[<>:\"/\\\\|?*\u{0000}-\u{001F}\u{007F}\u{0080}-\u{009F}]+")?;
    let outer_edge_periods = Regex::new("^\\.+|\\.+$")?;

    let result = reserved_chars.replace_all(input.as_ref(), FILENAME_SAFE_CHAR);
    let result = outer_edge_periods.replace_all(result.as_ref(), FILENAME_SAFE_CHAR);

    #[cfg(target_os = "windows")]
    let result = windows_extra_safe(result);

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! filename {
        ($input:expr, $output:expr) => {
            let result = crate::safe_filename($input).unwrap();
            assert_eq!(result, $output);
        };
    }

    #[test]
    fn sha256_expected_value() {
        let input = "hello world!".as_bytes();
        let result = checksum_string(input);
        assert_eq!(
            result,
            "7509e5bda0c762d2bac7f90d758b5b2263fa01ccbc542ab5e3df163be08e6ca9",
        );
    }

    #[test]
    fn safe_filename() {
        filename!(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            "https_www.youtube.com_watch_v=dQw4w9WgXcQ"
        );
        filename!("http://www.google.com", "http_www.google.com");
        filename!(".", "_");
        filename!("..", "_");
        filename!("./", "__");
        filename!("../", "__");
        filename!("foo/bar", "foo_bar");
        filename!("foo//bar", "foo_bar");
        filename!("foo\\\\\\bar", "foo_bar");
        filename!(r"foo\\bar", "foo_bar");
        filename!("foo\\bar", "foo_bar");
        filename!(r"foo\\\\\\bar", "foo_bar");
        filename!("//foo//bar//", "_foo_bar_");
        filename!("////foo////bar////", "_foo_bar_");
        filename!("\"foo<>bar*", "_foo_bar_");
        filename!("../../foo/bar", "__.._foo_bar");
        filename!(":nul|", "_nul_");
        filename!("foo\u{0000}bar", "foo_bar");
        filename!("foo.bar.", "foo.bar_");
        filename!("foo.bar...", "foo.bar_");
        filename!("file:///file.tar.gz", "file_file.tar.gz");
        filename!("foo/bar/nul", "foo_bar_nul");
        filename!("foo.bar..", "foo.bar_");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn safe_filename_windows_reserved() {
        filename!("con", "con_");
        filename!("prn", "prn_");
        filename!("aux", "aux_");
        filename!("nul", "nul_");
        filename!("com1", "com1_");
    }
}
