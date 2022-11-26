pub use from_str_sequencial_derive::*;

pub trait FromStrSequential: Sized {
    type Err;

    fn from_str_sequential(s: &str) -> Result<Self, Self::Err>;
}

#[derive(FromStrSequential)]
enum Pancakes {
    Cookie,
    Banana,
}


#[cfg(test)]
mod tests {
    use super::*;
}
