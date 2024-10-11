use crate::sdk::{read_storage, write_storage};

pub trait Contract {
    fn new() -> Self;
}

pub trait Fetchable<T> {
    fn get(&mut self) -> Option<T>;
    fn set(&mut self, value: T);
}

// Representation of a variable stored in the on-chain storage, lazy loaded into memory
pub struct OnChainVar<T>
    where
    T: Clone
{
    key: String,
    fetched: bool,
    value: Option<T>,
}

impl <T> OnChainVar<T> 
where
    T: Clone
{
    pub fn new(key: &str) -> Self {
        OnChainVar { key: key.to_string(), value: None, fetched: false }
    }
}

impl <T> Fetchable<T> for OnChainVar<T>
where
    T: Clone+From<String>+Into<String>
{
    fn get(&mut self) -> Option<T> {
        if self.fetched == true {
            self.value.clone()
        } else {
            self.fetched = true;
            match read_storage(&self.key) {
                None => None,
                Some(stringified) => Some(T::from(stringified))
            }
        }
    }

    fn set(&mut self, value: T) {
        let string_value: String = value.clone().into();
        write_storage(&self.key, &string_value);
        self.value = Some(value);
    }
}


