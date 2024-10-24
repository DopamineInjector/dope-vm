use reqwest::{blocking::Client, header::CONTENT_TYPE, StatusCode};
use serde::{Deserialize, Serialize};
use base64::prelude::*;

pub fn insert(db_url: &str, key: String, value: String, namespace: String) {
    let url = format!("{db_url}/api/insert");
    let value = BASE64_STANDARD.encode(value);
    let request = PostValueRequestDto {
        key,
        value,
        namespace
    };
    let body = serde_json::to_string(&request).unwrap();
    let client = Client::new();
    match client
        .post(&url)
        .body(body)
        .header(CONTENT_TYPE, "application/json")
        .send() 
    {
        Ok(_) => {
        },
        Err(e) => {
            eprintln!("Error while inserting value to db, {:?}", e);
        }
    }
}

pub fn get(db_url: &str, key: String, namespace: String) -> Option<String> {
    let url = format!("{db_url}/api/get");
    let request = GetValueRequestDto {
        key,
        namespace
    };
    let body = serde_json::to_string(&request).unwrap();
    let client = Client::new();
    match client
        .put(&url)
        .body(body)
        .header(CONTENT_TYPE, "application/json")
        .send() 
    {
        Ok(res) => {
            if res.status() != StatusCode::OK {
                return None
            }
            let stringified = res.text().unwrap();
            let response = serde_json::from_str::<GetValueResponseDto>(&stringified).unwrap();
            let content = String::from_utf8(BASE64_STANDARD.decode(response.value).unwrap()).unwrap();
            return Some(content);
        },
        Err(e) => {
            eprintln!("Error while getting value from db, {:?}", e);
            return None
        }
    }
}

pub fn initialize_storage(db_url: &str, namespace: String) {
    let url = format!("{db_url}/api/namespace");
    let request = PostNamespaceRequestDto {
        namespace
    };
    let body = serde_json::to_string(&request).unwrap();
    let client = Client::new();
    match client
        .post(&url)
        .body(body)
        .header(CONTENT_TYPE, "application/json")
        .send() 
    {
        Ok(_) => {
        },
        Err(e) => {
            eprintln!("Error while getting value from db, {:?}", e);
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetValueRequestDto {
    pub key: String,
    pub namespace: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetValueResponseDto {
    pub value: String,
    pub checksum: String
}


#[derive(Serialize, Deserialize)]
pub struct GetChecksumResponseDto {
    pub checksum: String
}

#[derive(Serialize, Deserialize)]
pub struct PostValueRequestDto {
    pub key: String,
    pub value: String,
    pub namespace: String
}

#[derive(Serialize, Deserialize)]
pub struct PostNamespaceRequestDto {
    pub namespace: String
}

