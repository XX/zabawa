use derive_more::{Display, Into};
use thiserror::Error;
use zabawa_validation::{InvalidLengthError, validate_length, validate_trimmed};

#[derive(Debug, Error)]
pub enum NameError {
    #[error("name has leading or trailing whitespaces")]
    Untrimmed,

    #[error("name error: {0}")]
    InvalidLength(#[from] InvalidLengthError),

    #[error("name error: {0}")]
    InvalidCharacters(#[from] InvalidCharactersError),
}

#[derive(Error, Debug, Clone, PartialEq)]
#[error("invalid characters")]
pub struct InvalidCharactersError;

#[derive(Clone, Debug, Display, Into, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(String);

impl Name {
    pub fn from_raw(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

pub trait NameBulder {
    type Error;

    fn validate(&self, input: &str) -> Result<(), Self::Error>;

    fn normalize(&self, input: &str) -> Result<String, Self::Error>;

    fn build(&self, input: impl AsRef<str> + Into<String>) -> Result<Name, Self::Error> {
        self.validate(input.as_ref())?;

        Ok(Name::from_raw(input))
    }

    fn build_with_normalize(&self, input: impl AsRef<str> + Into<String>) -> Result<Name, Self::Error> {
        if self.validate(input.as_ref()).is_ok() {
            return Ok(Name::from_raw(input));
        }

        let normalized_input = self.normalize(input.as_ref())?;
        self.build(normalized_input)
    }
}

#[derive(Debug, Default)]
pub struct DefaultNameBuilder {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub char_validation_enabled: bool,
    pub trim_validation_enabled: bool,
}

impl DefaultNameBuilder {
    pub fn new() -> Self {
        Self {
            min_length: Some(2),
            max_length: Some(512),
            char_validation_enabled: true,
            trim_validation_enabled: true,
        }
    }

    pub fn with_min_length(mut self, min: usize) -> Self {
        self.min_length = Some(min);
        self
    }

    pub fn without_min_length(mut self) -> Self {
        self.min_length = None;
        self
    }

    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    pub fn without_max_length(mut self) -> Self {
        self.max_length = None;
        self
    }

    pub fn with_char_validation(mut self, enabled: bool) -> Self {
        self.char_validation_enabled = enabled;
        self
    }

    pub fn with_trim_validation(mut self, enabled: bool) -> Self {
        self.trim_validation_enabled = enabled;
        self
    }
}

impl NameBulder for DefaultNameBuilder {
    type Error = NameError;

    fn validate(&self, input: &str) -> Result<(), Self::Error> {
        if self.trim_validation_enabled && !validate_trimmed(input) {
            return Err(NameError::Untrimmed);
        }

        if self.min_length.is_some() || self.max_length.is_some() {
            validate_length(
                input.len(),
                self.min_length.unwrap_or(0),
                self.max_length.unwrap_or(input.len()),
            )?;
        }

        if self.char_validation_enabled && !validate_name_chars(input) {
            return Err(NameError::InvalidCharacters(InvalidCharactersError));
        }

        Ok(())
    }

    fn normalize(&self, input: &str) -> Result<String, Self::Error> {
        let input = if self.trim_validation_enabled {
            input.trim()
        } else {
            input
        };

        let mut normalized = String::with_capacity(input.len());
        if self.char_validation_enabled {
            make_name(input, &mut normalized);
        } else {
            input.clone_into(&mut normalized);
        }

        Ok(normalized)
    }
}

pub fn validate_name_chars(input: &str) -> bool {
    input.chars().all(is_name_safe_char)
}

fn is_name_safe_char(ch: char) -> bool {
    ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_'
}

pub fn normalize_name(input: &str) -> String {
    let trimmed = input.trim();

    let mut normalized = String::with_capacity(trimmed.len());
    make_name(trimmed, &mut normalized);

    normalized
}

pub fn make_name(input: &str, output: &mut String) {
    for ch in input.chars() {
        if ch.is_ascii() {
            process_char(ch, output);
        } else if let Some(transliterated) = deunicode::deunicode_char(ch) {
            for trans in transliterated.chars() {
                process_char(trans, output);
            }
        } else {
            push_separator(output);
        }
    }
}

fn process_char(ch: char, output: &mut String) {
    let ch = ch.to_ascii_lowercase();
    if is_name_safe_char(ch) {
        output.push(ch);
    } else {
        push_separator(output);
    }
}

fn push_separator(output: &mut String) {
    match output.chars().last() {
        Some('-') => {},
        _ => output.push('-'),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_name_basic() {
        assert_eq!(normalize_name("WebApp"), "webapp");
        assert_eq!(normalize_name("  WebApp  "), "webapp");
    }

    #[test]
    fn test_normalize_name_non_ascii() {
        assert_eq!(normalize_name("Café"), "cafe");
        assert_eq!(normalize_name("Straße"), "strasse");
        assert_eq!(normalize_name("Москва"), "moskva");
        assert_eq!(normalize_name("北京"), "bei-jing-"); // TODO: remove last "-"
        assert_eq!(normalize_name("Æneid"), "aeneid");
        assert_eq!(normalize_name("étude"), "etude");
        assert_eq!(normalize_name("🦄☣"), "unicorn-biohazard-"); // TODO: remove last "-"
        assert_eq!(normalize_name("…"), "-");
    }

    #[test]
    fn test_normalize_name_spaces() {
        assert_eq!(normalize_name("My Project"), "my-project");
        assert_eq!(normalize_name("My  Cool  Project"), "my-cool-project");
        assert_eq!(normalize_name("  test  "), "test");
        assert_eq!(normalize_name("a   b   c"), "a-b-c");
        assert_eq!(normalize_name("hello    world"), "hello-world");
        assert_eq!(normalize_name("   multiple   spaces   "), "multiple-spaces");
    }

    #[test]
    fn test_normalize_name_forbidden_chars() {
        assert_eq!(normalize_name("hello@world"), "hello-world");
        assert_eq!(normalize_name("hello!!!world"), "hello-world");
        assert_eq!(normalize_name("hello#$%world"), "hello-world");
        assert_eq!(normalize_name("hello!!!"), "hello-");
        assert_eq!(normalize_name("!!!hello"), "-hello");
        assert_eq!(normalize_name("@@@hello"), "-hello");
        assert_eq!(normalize_name("hello@@@"), "hello-");
        assert_eq!(normalize_name("@@@hello@@@"), "-hello-");
        assert_eq!(normalize_name("   !!!   hello   !!!   "), "-hello-");
    }

    #[test]
    fn test_normalize_name_mixed_case() {
        assert_eq!(normalize_name("HeLLo-WoRLd"), "hello-world");
        assert_eq!(normalize_name("MyProject123"), "myproject123");
    }

    #[test]
    fn test_normalize_name_digits() {
        assert_eq!(normalize_name("test123"), "test123");
        assert_eq!(normalize_name("123test"), "123test");
        assert_eq!(normalize_name("123"), "123");
    }

    #[test]
    fn test_normalize_name_dashes_underscores() {
        assert_eq!(normalize_name("hello-world"), "hello-world");
        assert_eq!(normalize_name("hello_world"), "hello_world");
        assert_eq!(normalize_name("---"), "---");
        assert_eq!(normalize_name("___"), "___");
        assert_eq!(normalize_name("hello-world"), "hello-world");
        assert_eq!(normalize_name("test---test"), "test---test");
        assert_eq!(normalize_name("a----b"), "a----b");
    }

    #[test]
    fn test_normalize_name_whitespace_types() {
        assert_eq!(normalize_name("\t\n  test  \t\n"), "test");
        assert_eq!(normalize_name("hello\tworld"), "hello-world");
        assert_eq!(normalize_name("hello\nworld"), "hello-world");
    }

    #[test]
    fn test_normalize_name_complex() {
        assert_eq!(normalize_name("  My Café Project  "), "my-cafe-project");
        assert_eq!(normalize_name("hello@#$world!!!test"), "hello-world-test");
        assert_eq!(normalize_name("Привет Мир"), "privet-mir");
        assert_eq!(normalize_name("hello Мир world"), "hello-mir-world");
        assert_eq!(normalize_name("test café test"), "test-cafe-test");
        assert_eq!(normalize_name("北京 Beijing"), "bei-jing-beijing");
    }

    #[test]
    fn test_normalize_name_empty() {
        assert_eq!(normalize_name(""), "");
        assert_eq!(normalize_name("   "), "");
        assert_eq!(normalize_name("\t\n"), "");
    }

    #[test]
    fn test_normalize_name_multiple_separator_types() {
        assert_eq!(normalize_name("hello @#$ world"), "hello-world");
        assert_eq!(normalize_name("test!@#$%^&*()test"), "test-test");
        assert_eq!(normalize_name("a!b@c#d"), "a-b-c-d");
    }

    #[test]
    fn test_normalize_name_special_unicode() {
        assert_eq!(normalize_name("hello™world"), "hellotmworld");
        assert_eq!(normalize_name("test©2024"), "test-c-2024");
        assert_eq!(normalize_name("price€100"), "priceeur100");
    }

    #[test]
    fn test_normalize_name_only_separators() {
        assert_eq!(normalize_name("!!!"), "-");
        assert_eq!(normalize_name("@@@"), "-");
        assert_eq!(normalize_name("!@#$%"), "-");
    }

    #[test]
    fn test_normalize_name_mixed_separators_and_dashes() {
        assert_eq!(normalize_name("hello-@-world"), "hello--world");
        assert_eq!(normalize_name("test_!_test"), "test_-_test");
    }

    #[test]
    fn test_normalize_name_preserves_valid_structure() {
        assert_eq!(normalize_name("already-valid-slug"), "already-valid-slug");
        assert_eq!(normalize_name("test_with_underscores"), "test_with_underscores");
        assert_eq!(normalize_name("mix-of_both"), "mix-of_both");
    }

    #[test]
    fn test_validate_name_chars() {
        assert!(validate_name_chars("hello-world"));
        assert!(validate_name_chars("test123"));
        assert!(validate_name_chars("my_project"));
        assert!(validate_name_chars("abc"));
        assert!(validate_name_chars("a-b-c"));
        assert!(validate_name_chars("a"));
        assert!(validate_name_chars("1"));
        assert!(validate_name_chars("-"));
        assert!(validate_name_chars("_"));
        assert!(validate_name_chars("a1-_"));
        assert!(validate_name_chars("-hello-"));
        assert!(validate_name_chars(""));

        assert!(!validate_name_chars("Hello"));
        assert!(!validate_name_chars("hello world"));
        assert!(!validate_name_chars("café"));
        assert!(!validate_name_chars("hello@world"));
        assert!(!validate_name_chars("A"));
        assert!(!validate_name_chars(" "));
        assert!(!validate_name_chars("hello "));
        assert!(!validate_name_chars(" hello"));
    }
}
