use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct LimitedSlice<T, const LIMIT: usize>([T]);

impl<T, const LIMIT: usize> LimitedSlice<T, LIMIT> {
    fn from_boxed_slice(value: Box<[T]>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(value) as *mut Self) }
    }

    pub fn into_boxed_slice(self: Box<Self>) -> Box<[T]> {
        unsafe { Box::from_raw(Box::into_raw(self) as *mut [T]) }
    }

    pub fn into_vec(self: Box<Self>) -> Vec<T> {
        self.into_boxed_slice().into_vec()
    }
}

impl<T: Clone, const LIMIT: usize> Clone for Box<LimitedSlice<T, LIMIT>> {
    fn clone(&self) -> Self {
        let cloned =
            unsafe { &*(self as *const Box<LimitedSlice<T, LIMIT>> as *const Box<[T]>) }.clone();
        LimitedSlice::from_boxed_slice(cloned)
    }
}

impl<T, const LIMIT: usize> AsRef<[T]> for LimitedSlice<T, LIMIT> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

pub struct ExceededLimitError {
    actual_len: usize,
}

impl<T, const LIMIT: usize> TryFrom<Vec<T>> for Box<LimitedSlice<T, LIMIT>> {
    type Error = ExceededLimitError;

    fn try_from(value: Vec<T>) -> Result<Self, ExceededLimitError> {
        value.into_boxed_slice().try_into()
    }
}

impl<T, const LIMIT: usize> TryFrom<Box<[T]>> for Box<LimitedSlice<T, LIMIT>> {
    type Error = ExceededLimitError;

    fn try_from(value: Box<[T]>) -> Result<Self, ExceededLimitError> {
        if value.len() > LIMIT {
            Err(ExceededLimitError {
                actual_len: value.len(),
            })
        } else {
            Ok(LimitedSlice::from_boxed_slice(value))
        }
    }
}

impl<'de, T: Deserialize<'de>, const LIMIT: usize> serde::de::Deserialize<'de>
    for Box<LimitedSlice<T, LIMIT>>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Vec<T> = Vec::deserialize(deserializer)?;
        let value: Box<LimitedSlice<T, LIMIT>> =
            TryFrom::try_from(value).map_err(|e: ExceededLimitError| {
                serde::de::Error::invalid_length(
                    e.actual_len,
                    &format!("expected vector of {} items", LIMIT).as_str(),
                )
            })?;

        Ok(value)
    }
}

impl<T: Serialize, const LIMIT: usize> serde::ser::Serialize for Box<LimitedSlice<T, LIMIT>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}
