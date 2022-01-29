//! The enum [`Either`] is a generic sum type of two types,
//! with the variants `Left` or `Right` corresponding to each
//! type respectively.

pub use Either::{Left, Right};

// Note: commonly derived traits (such as `Clone`, `Copy`, etc)
// are manually provided as blanket implementations rather than
// derived via macros. This is to allow for types that don't
// implement these traits.
// TODO: CONFIRM WHETHER THIS ^  IS NECESSARY.
pub enum Either<L, R> {
    /// The first case holds a value of type `L`.
    /// This may be treated as the default case.
    Left(L),
    /// The second case holds a value of type `R`.
    /// This may be treated as the alternative case.
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Returns `true` if `self` is `Left` variant.
    ///
    /// # Example
    ///
    /// ```
    /// use eithr::*;
    ///
    /// let one_or_more = (Left('a'), Right(vec!['a', 'b', 'c']));
    /// assert_eq!(one_or_more.0.is_left(), true);
    /// assert_eq!(one_or_more.1.is_left(), false);
    /// ```
    pub fn is_left(&self) -> bool {
        matches!(self, Left(_))
    }

    pub fn is_right(&self) -> bool {
        !self.is_left()
    }

    /// Given two closures, both of which have the same return type, applies
    /// whichever one is compatible with the inner type, returning the
    /// resolved value instead of an `Either` type.
    pub fn resolve<F, G, X>(self, left: F, right: G) -> X
    where
        F: FnOnce(L) -> X,
        G: FnOnce(R) -> X,
    {
        match self {
            Left(l) => left(l),
            Right(r) => right(r),
        }
    }

    /// Consumes the inner value of type `L` and returns it as n `Option<L>`.
    /// If `self` contained a value of type `R`, `None` is returned.
    pub fn take_left(self) -> Option<L> {
        if let Left(l) = self {
            Some(l)
        } else {
            None
        }
    }

    /// Consumes inner value of type `R` and returns it as an `Option<R>`.
    /// If `self` contained a value of type `L`, `None` is returned.
    pub fn take_right(self) -> Option<R> {
        if let Right(r) = self {
            Some(r)
        } else {
            None
        }
    }

    /// Applies a function to the wrapped `Left` value, leaving the `Right`
    /// value untouched.
    pub fn map_left<F, X>(self, f: F) -> Either<X, R>
    where
        F: FnOnce(L) -> X,
    {
        match self {
            Left(l) => Left(f(l)),
            Right(r) => Right(r),
        }
    }

    /// Applies a function to the wrapped `Right` value, leaving the `Left`
    /// value untouched.
    pub fn map_right<F, Y>(self, f: F) -> Either<L, Y>
    where
        F: FnOnce(R) -> Y,
    {
        match self {
            Left(l) => Left(l),
            Right(r) => Right(f(r)),
        }
    }

    /// Returns an `Either` variant containing a reference to the inner value.
    pub fn as_ref(&self) -> Either<&L, &R> {
        match *self {
            Left(ref l) => Left(l),
            Right(ref r) => Right(r),
        }
    }

    /// Returns an `Either` variant containing a mutable reference to the inner
    /// value.
    pub fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        match *self {
            Left(ref mut l) => Left(l),
            Right(ref mut r) => Right(r),
        }
    }

    /// Consumes the inner value and returns it in the alternate `Either`
    /// variant.
    pub fn transpose(self) -> Either<R, L> {
        match self {
            Left(lr) => Right(lr),
            Right(rl) => Left(rl),
        }
    }

    /// Makes a new copy of the inner value, returning it in the same `Either`
    /// variant.
    pub fn copied(&self) -> Either<L, R>
    where
        L: Copy,
        R: Copy,
    {
        match self {
            Left(l) => Left(*l),
            Right(r) => Right(*r),
        }
    }

    /// Clones the inner value and returns it in the same `Either` variant.
    pub fn cloned(&self) -> Either<L, R>
    where
        L: Clone,
        R: Clone,
    {
        match self {
            Left(l) => Left(l.clone()),
            Right(r) => Right(r.clone()),
        }
    }

    /// Converts the inner `IntoIterator` value into an `Iterator`, returning
    /// it wrapped in the corresponding `Either` variant.
    pub fn into_iter(self) -> Either<L::IntoIter, R::IntoIter>
    where
        L: IntoIterator,
        R: IntoIterator<Item = L::Item>,
    {
        match self {
            Left(l) => Left(l.into_iter()),
            Right(r) => Right(r.into_iter()),
        }
    }
}

impl<L, R> Eq for Either<L, R>
where
    L: Eq,
    R: Eq,
{
}

impl<L, R> std::cmp::PartialEq for Either<L, R>
where
    L: std::cmp::PartialEq,
    R: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Left(lx), Left(ly)) => lx == ly,
            (Right(rx), Right(ry)) => rx == ry,
            _ => false,
        }
    }
}

impl<L, R> std::hash::Hash for Either<L, R>
where
    L: std::hash::Hash,
    R: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Left(l) => l.hash(state),
            Right(r) => r.hash(state),
        }
    }
}

impl<L, R> std::fmt::Debug for Either<L, R>
where
    L: std::fmt::Debug,
    R: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Left(l) => f.debug_tuple("Left").field(l).finish(),
            Right(r) => f.debug_tuple("Right").field(r).finish(),
        }
    }
}

impl<L, R> Clone for Either<L, R>
where
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Left(l) => Left(l.clone()),
            Right(r) => Right(r.clone()),
        }
    }
}

impl<L, R> Copy for Either<L, R>
where
    L: Copy,
    R: Copy,
{
}

impl<L> From<Either<L, ()>> for Option<L> {
    fn from(either: Either<L, ()>) -> Self {
        either.resolve(|l| Some(l), |_| None)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
