use std::{convert::Infallible, ops::FromResidual};

#[derive(Debug)]
pub enum Outcome<S, E> {
    Success(S),
    Exception(E),
    Unexpected(anyhow::Error),
}

impl<S, A> Outcome<S, A> {
    pub fn map_exception<B, F: FnOnce(A) -> B>(self, f: F) -> Outcome<S, B> {
        match self {
            Self::Exception(val) => Outcome::Exception(f(val)),
            Self::Success(val) => Outcome::Success(val),
            Self::Unexpected(val) => Outcome::Unexpected(val),
        }
    }
}

impl<A, E> Outcome<A, E> {
    pub fn map_success<B, F: FnOnce(A) -> B>(self, f: F) -> Outcome<B, E> {
        match self {
            Self::Success(val) => Outcome::Success(f(val)),
            Self::Exception(val) => Outcome::Exception(val),
            Self::Unexpected(val) => Outcome::Unexpected(val),
        }
    }
}

impl<S, E> Outcome<S, E> {
    pub fn map_unexpected<F: FnOnce(anyhow::Error) -> anyhow::Error>(self, f: F) -> Outcome<S, E> {
        match self {
            Self::Success(val) => Outcome::Success(val),
            Self::Exception(val) => Outcome::Exception(val),
            Self::Unexpected(val) => Outcome::Unexpected(f(val)),
        }
    }
}

impl<S, E> From<anyhow::Error> for Outcome<S, E> {
    fn from(value: anyhow::Error) -> Self {
        Outcome::Unexpected(value)
    }
}

impl<S, E> FromResidual<Result<Infallible, anyhow::Error>> for Outcome<S, E> {
    fn from_residual(residual: Result<Infallible, anyhow::Error>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => Self::Unexpected(e),
        }
    }
}

impl<S, E, F: From<E>> FromResidual<Outcome<Infallible, E>> for Outcome<S, F> {
    fn from_residual(residual: Outcome<Infallible, E>) -> Self {
        match residual {
            Outcome::Exception(e) => Self::Exception(From::from(e)),
            Outcome::Unexpected(u) => Self::Unexpected(u),
            Outcome::Success(_) => unreachable!(),
        }
    }
}

impl<S, E> std::ops::Try for Outcome<S, E> {
    type Output = S;
    type Residual = Outcome<Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Success(s) => std::ops::ControlFlow::Continue(s),
            Self::Unexpected(u) => std::ops::ControlFlow::Break(Outcome::Unexpected(u)),
            Self::Exception(ex) => std::ops::ControlFlow::Break(Outcome::Exception(ex)),
        }
    }
}
