use std::{convert::Infallible, fmt::Display, ops::FromResidual};

#[derive(Debug)]
pub enum Outcome<Ok, Ex> {
    Ok(Ok),
    /// Exception
    Ex(Ex),
    Error(anyhow::Error),
}

pub trait IntoOk<Ok, Ex> {
    fn into_ok(self) -> Outcome<Ok, Ex>;
}

pub trait IntoEx<Ok, Ex> {
    fn into_ex(self) -> Outcome<Ok, Ex>;
}

pub trait IntoErr<Ok, Ex> {
    fn into_ex(self) -> Outcome<Ok, Ex>;
}

impl<Ok, Ex> IntoOk<Ok, Ex> for Ok {
    fn into_ok(self) -> Outcome<Ok, Ex> {
        Outcome::Ok(self)
    }
}

impl<Ok, Ex> IntoEx<Ok, Ex> for Ex {
    fn into_ex(self) -> Outcome<Ok, Ex> {
        Outcome::Ex(self)
    }
}

impl<Ok, Ex> IntoErr<Ok, Ex> for anyhow::Error {
    fn into_ex(self) -> Outcome<Ok, Ex> {
        Outcome::Error(self)
    }
}

impl<S, A> Outcome<S, A> {
    pub fn is_ex(&self) -> bool {
        match self {
            Self::Ex(_) => true,
            _ => false,
        }
    }

    pub fn map_ex<B, F: FnOnce(A) -> B>(self, f: F) -> Outcome<S, B> {
        match self {
            Self::Ex(val) => Outcome::Ex(f(val)),
            Self::Ok(val) => Outcome::Ok(val),
            Self::Error(val) => Outcome::Error(val),
        }
    }

    pub fn flat_map_ex<B, F: FnOnce(A) -> Outcome<S, B>>(self, f: F) -> Outcome<S, B> {
        match self {
            Self::Ex(ex) => f(ex),
            Self::Ok(ok) => Outcome::Ok(ok),
            Self::Error(err) => Outcome::Error(err),
        }
    }
}

impl<A, E> Outcome<A, E> {
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok(_) => true,
            _ => false,
        }
    }

    pub fn map_ok<B, F: FnOnce(A) -> B>(self, f: F) -> Outcome<B, E> {
        match self {
            Self::Ok(val) => Outcome::Ok(f(val)),
            Self::Ex(val) => Outcome::Ex(val),
            Self::Error(val) => Outcome::Error(val),
        }
    }
}

impl<S, E> Outcome<S, E> {
    pub fn is_err(&self) -> bool {
        match self {
            Self::Error(_) => true,
            _ => false,
        }
    }

    pub fn map_err<F: FnOnce(anyhow::Error) -> anyhow::Error>(self, f: F) -> Outcome<S, E> {
        match self {
            Self::Ok(val) => Outcome::Ok(val),
            Self::Ex(val) => Outcome::Ex(val),
            Self::Error(val) => Outcome::Error(f(val)),
        }
    }
}

impl<S> Outcome<S, anyhow::Error> {
    pub fn collapse(self) -> Result<S, anyhow::Error> {
        match self {
            Self::Ok(val) => Ok(val),
            Self::Error(err) => Err(err),
            Self::Ex(err) => Err(err),
        }
    }
}

impl<S> Outcome<S, Infallible> {
    pub fn collapse(self) -> Result<S, anyhow::Error> {
        match self {
            Self::Ok(val) => Ok(val),
            Self::Error(err) => Err(err),
        }
    }
}

impl<S, E: Send + Sync + std::error::Error + 'static> Outcome<S, E> {
    pub fn collapse_with_context<C: Display + Send + Sync + 'static>(
        self,
        context: C,
    ) -> Result<S, anyhow::Error> {
        match self {
            Self::Ok(val) => Ok(val),
            Self::Error(err) => Err(err),
            Self::Ex(err) => Err(anyhow::Error::new(err).context(context)),
        }
    }
}

// impl<S, E> From<anyhow::Error> for Outcome<S, E> {
//     fn from(value: anyhow::Error) -> Self {
//         Outcome::Error(value)
//     }
// }

impl<S, E> FromResidual<Result<Infallible, anyhow::Error>> for Outcome<S, E> {
    fn from_residual(residual: Result<Infallible, anyhow::Error>) -> Self {
        match residual {
            Err(e) => Self::Error(e),
        }
    }
}

impl<S, E, F: From<E>> FromResidual<Outcome<Infallible, E>> for Outcome<S, F> {
    fn from_residual(residual: Outcome<Infallible, E>) -> Self {
        match residual {
            Outcome::Ex(e) => Self::Ex(From::from(e)),
            Outcome::Error(u) => Self::Error(u),
        }
    }
}

impl<S, E> std::ops::Try for Outcome<S, E> {
    type Output = S;
    type Residual = Outcome<Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        Self::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Ok(s) => std::ops::ControlFlow::Continue(s),
            Self::Error(u) => std::ops::ControlFlow::Break(Outcome::Error(u)),
            Self::Ex(ex) => std::ops::ControlFlow::Break(Outcome::Ex(ex)),
        }
    }
}
