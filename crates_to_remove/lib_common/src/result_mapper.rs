use crate::outcome::Outcome;

pub struct ResultMapper<O, E, Ex> {
    result: Result<O, E>,
    mappers: Vec<Box<dyn FnOnce(&E) -> Option<Ex>>>,
}

pub trait IntoResultMapper<O, E, Ex> {
    fn into_result_mapper(self) -> ResultMapper<O, E, Ex>;
}

impl<O, E, Ex> IntoResultMapper<O, E, Ex> for Result<O, E> {
    fn into_result_mapper(self) -> ResultMapper<O, E, Ex> {
        ResultMapper::new(self)
    }
}

impl<O, E, Ex> IntoResultMapper<O, E, Ex> for ResultMapper<O, E, Ex> {
    fn into_result_mapper(self) -> ResultMapper<O, E, Ex> {
        self
    }
}

impl<O, E, Ex> ResultMapper<O, E, Ex> {
    pub fn new(result: Result<O, E>) -> Self {
        Self {
            result,
            mappers: Vec::new(),
        }
    }
}

impl<O, E, Ex> ResultMapper<O, E, Ex>
where
    E: std::error::Error + Send + Sync + 'static,
{
    pub fn add_mapper(mut self, mapper: impl FnOnce(&E) -> Option<Ex> + 'static) -> Self {
        if self.result.is_err() {
            self.mappers.push(Box::new(mapper));
        }

        self
    }

    pub fn map(self) -> Outcome<O, Ex> {
        let err = match self.result {
            Ok(val) => return Outcome::Success(val),
            Err(err) => err,
        };

        for mapper in self.mappers {
            if let Some(ex) = mapper(&err) {
                return Outcome::Exception(ex);
            }
        }

        Outcome::Unexpected(anyhow::Error::new(err))
    }
}
