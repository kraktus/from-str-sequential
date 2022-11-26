pub use from_str_sequencial_derive::FromStrSequential;

/// ## Example
///
/// ```
/// // use from_str_sequencial_derive::FromStrSequential;
/// use from_str_sequential::FromStrSequential;
///
/// #[derive(FromStrSequential, Debug, PartialEq, Eq)]
/// enum Foo {
///     Bar,
///     Baz(usize),
/// }
///
/// assert_eq!(Foo::Bar, Foo::from_str_sequential("bar").unwrap());
/// assert_eq!(Foo::Bar, Foo::from_str_sequential("BaR").unwrap());
/// assert_eq!(Foo::Baz(100), Foo::from_str_sequential("100").unwrap());
/// ```
pub trait FromStrSequential: Sized {
    type Err;

    fn from_str_sequential(s: &str) -> Result<Self, Self::Err>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(FromStrSequential, Debug)]
    enum Pancakes {
        Cookie,
        Banana,
    }

    #[derive(FromStrSequential, Debug, PartialEq, Eq)]
    enum Foo {
        Bar,
        Baz(usize),
    }

    #[test]
    fn test_from_str_sequential() {
        assert_eq!(Foo::Bar, Foo::from_str_sequential("bar").unwrap());
        assert_eq!(Foo::Bar, Foo::from_str_sequential("BaR").unwrap());
        assert_eq!(Foo::Baz(100), Foo::from_str_sequential("100").unwrap());
    }
}
