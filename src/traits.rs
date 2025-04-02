pub trait Valid {
    fn valid(&self) -> bool;
}

pub trait Normalize: Valid {
    fn normalize(&self) -> Option<String>;
}