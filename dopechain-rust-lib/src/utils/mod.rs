use serde::{Deserialize, Serialize};

use crate::sdk::get_user_args;

pub fn parse_json_args<T>() -> T
where 
    T: for<'de> Deserialize<'de>+Serialize
{
    let json_args = get_user_args();
    let serialized: Result<T, serde_json::Error> = serde_json::from_str::<T>(&json_args);
    let res = serialized.unwrap();
    res
}
