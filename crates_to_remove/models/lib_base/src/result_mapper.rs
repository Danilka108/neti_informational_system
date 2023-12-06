use lib_common::result_mapper::ResultMapper;

pub trait SqlxResultMapperExt<O, Ex, F: From<Ex>> {
    fn on_constraint(self, constraint: &'static str, ex: Ex) -> ResultMapper<O, sqlx::Error, F>;

    fn on_row_not_found(self, ex: Ex) -> ResultMapper<O, sqlx::Error, F>;
}

impl<O, Ex: 'static, F: From<Ex>> SqlxResultMapperExt<O, Ex, F>
    for ResultMapper<O, sqlx::Error, F>
{
    fn on_constraint(self, constraint: &'static str, ex: Ex) -> ResultMapper<O, sqlx::Error, F> {
        self.add_mapper(move |err| match err {
            sqlx::Error::Database(db_err) if db_err.constraint().is_some() => {
                if constraint == db_err.constraint().unwrap() {
                    Some(ex.into())
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    fn on_row_not_found(self, ex: Ex) -> ResultMapper<O, sqlx::Error, F> {
        self.add_mapper(move |err| match err {
            sqlx::Error::RowNotFound => Some(ex.into()),
            _ => None,
        })
    }
}
