#![doc = include_str!("../README.md")]

pub use from_str_sequential_derive::FromStrSequential;


/// sibling trait of `FromStr`. Used on enums with unit and un-named variants, and try to convert the string to each variant sequentially (from top to bottom variant). For unit variant,
/// the string must be the variant name (case-insentive). For un-named variants, the string must match the `FromStr` implementation of the un-named type.
/// ## Example
/// ```
/// use from_str_sequential::FromStrSequential;
/// #[derive(Debug, FromStrSequential, PartialEq, Eq)]
/// enum Foo {
///     Bar,
///     Baz(usize),
/// }
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
