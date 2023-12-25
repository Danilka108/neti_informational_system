use std::{fmt::Display, num::ParseIntError, str::FromStr};

const PASSPORT_NUMBER_LEN: usize = 6;
const PASSPORT_SERIES_LEN: usize = 4;

#[derive(Debug, Clone, Copy)]
pub struct PassportNumber {
    value: FixedLenU32<PASSPORT_NUMBER_LEN>,
}

#[derive(Debug, Clone, Copy)]
pub struct PassportSeries {
    value: FixedLenU32<PASSPORT_SERIES_LEN>,
}

#[derive(Debug, thiserror::Error)]
#[error("invalid value of passport number, {0}")]
pub struct InvalidPassportNumberError(#[from] FixedLenU32Error<PASSPORT_NUMBER_LEN>);

#[derive(Debug, thiserror::Error)]
#[error("invalid value of passport series, {0}")]
pub struct InvalidPassportSeriesError(#[from] FixedLenU32Error<PASSPORT_SERIES_LEN>);

impl FromStr for PassportNumber {
    type Err = InvalidPassportNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: FixedLenU32::from_str(s)?,
        })
    }
}

impl Display for PassportNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for PassportSeries {
    type Err = InvalidPassportSeriesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: FixedLenU32::from_str(s)?,
        })
    }
}

impl Display for PassportSeries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, Copy)]
struct FixedLenU32<const LEN: usize> {
    value: u32,
}

#[derive(Debug, thiserror::Error)]
#[error("value should consist of 4 digits {LEN}")]
enum FixedLenU32Error<const LEN: usize> {
    InvalidLen,
    ParseError(#[from] ParseIntError),
}

impl<const LEN: usize> FromStr for FixedLenU32<LEN> {
    type Err = FixedLenU32Error<LEN>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != LEN {
            return Err(FixedLenU32Error::InvalidLen);
        }

        Ok(Self {
            value: u32::from_str_radix(s, 10)?,
        })
    }
}

impl<const LEN: usize> Display for FixedLenU32<LEN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.value
                .to_string()
                .chars()
                .rev()
                .chain("0".chars().cycle())
                .take(6)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<String>()
        )
    }
}
