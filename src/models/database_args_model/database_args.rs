

pub trait DatabaseArgs {
    fn from_str(s: &str) -> Self
    where
        Self: Sized;
}