use serde::Deserialize;

use crate::sdk::get_user_args;

pub fn parse_json_args<T>() -> T
where 
    T: for<'de> Deserialize<'de>
{
    let json_args = get_user_args();
    let serialized: Result<T, serde_json::Error> = serde_json::from_str(&json_args);
    serialized.unwrap()
}
