use serde::{Deserializer, Serializer};

#[derive(Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct LimitedStr<const LIMIT: usize>(str);

impl<const LIMIT: usize> LimitedStr<LIMIT> {
    fn from_boxed_str(value: Box<str>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(value) as *mut Self) }
    }

    pub fn into_boxed_str(self: Box<Self>) -> Box<str> {
        unsafe { Box::from_raw(Box::into_raw(self) as *mut str) }
    }

    pub fn into_string(self: Box<Self>) -> String {
        String::from(self.into_boxed_str())
    }
}

impl<const LIMIT: usize> Clone for Box<LimitedStr<LIMIT>> {
    fn clone(&self) -> Self {
        let cloned =
            unsafe { &*(self as *const Box<LimitedStr<LIMIT>> as *const Box<str>) }.clone();
        LimitedStr::from_boxed_str(cloned)
    }
}

impl<const LIMIT: usize> AsRef<str> for LimitedStr<LIMIT> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct ExceededLimitError {
    actual_len: usize,
}

impl<const LIMIT: usize> TryFrom<String> for Box<LimitedStr<LIMIT>> {
    type Error = ExceededLimitError;

    fn try_from(value: String) -> Result<Self, ExceededLimitError> {
        value.into_boxed_str().try_into()
    }
}

impl<const LIMIT: usize> TryFrom<Box<str>> for Box<LimitedStr<LIMIT>> {
    type Error = ExceededLimitError;

    fn try_from(value: Box<str>) -> Result<Self, ExceededLimitError> {
        if value.len() > LIMIT {
            Err(ExceededLimitError {
                actual_len: value.len(),
            })
        } else {
            Ok(LimitedStr::from_boxed_str(value))
        }
    }
}

impl<'de, const LIMIT: usize> serde::de::Deserialize<'de> for Box<LimitedStr<LIMIT>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Box<str> = Box::deserialize(deserializer)?;
        let value: Box<LimitedStr<LIMIT>> =
            TryFrom::try_from(value).map_err(|e: ExceededLimitError| {
                serde::de::Error::invalid_length(
                    e.actual_len,
                    &format!("expected string {} characters long", LIMIT).as_str(),
                )
            })?;

        Ok(value)
    }
}

impl<const LIMIT: usize> serde::ser::Serialize for Box<LimitedStr<LIMIT>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}
