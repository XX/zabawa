use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq)]
#[error("invalid length: expected {min}-{max} characters, got {actual}")]
pub struct InvalidLengthError {
    pub min: usize,
    pub max: usize,
    pub actual: usize,
}

pub fn validate_length(len: usize, min: usize, max: usize) -> Result<(), InvalidLengthError> {
    if len < min || len > max {
        Err(InvalidLengthError { min, max, actual: len })
    } else {
        Ok(())
    }
}

pub fn validate_trimmed(input: &str) -> bool {
    input.len() == input.trim().len()
}

pub fn validate_is_ascii_lowercase(input: &str) -> bool {
    input.as_bytes().iter().all(u8::is_ascii_lowercase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_length() {
        assert!(validate_length(4, 3, 40).is_ok());
        assert!(validate_length(3, 3, 40).is_ok());
        assert!(validate_length(40, 3, 40).is_ok());

        assert_eq!(
            validate_length(2, 3, 40),
            Err(InvalidLengthError {
                min: 3,
                max: 40,
                actual: 2
            })
        );
        assert_eq!(
            validate_length(0, 3, 40),
            Err(InvalidLengthError {
                min: 3,
                max: 40,
                actual: 0
            })
        );
        assert_eq!(
            validate_length(41, 3, 40),
            Err(InvalidLengthError {
                min: 3,
                max: 40,
                actual: 41
            })
        );
    }
}
