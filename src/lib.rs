pub use from_str_sequencial_derive::*;

pub trait FromStrSequential: Sized {
    type Err;

    fn from_str_sequential(s: &str) -> Result<Self, Self::Err>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(FromStrSequential)]
    enum Pancakes {
        Cookie,
        Banana,
    }

    #[derive(FromStrSequential)]
    enum Foo {
        Bar,
        Baz(usize),
    }
}
