use std::fs::{File, OpenOptions};
use std::io::Write;
use std::{env, fs};

use std::collections::HashMap;
use crate::JHPResponse;

pub fn slurp(m: String) -> String {
    let mut content = String::new();
    if let Ok(path) = env::current_dir() {
        let base_path = format!("{}", path.display());
        if let Ok(data) = fs::read_to_string(format!("{}/{}", base_path, m)) {
            content = data;
        } else {
            println!("ERR| unbale to read content: '{}'", m);
        }
    } else {
        println!("ERR| unbale to get current dir");
    };
    content
}

pub fn spew(f: String, c: String) -> bool {
    let mut is_ok = false;
    if let Ok(path) = env::current_dir() {
        let base_path = format!("{}", path.display());
        if let Ok(mut file) = File::create(format!("{}/{}", base_path, f)) {
            if let Ok(_) = file.write_all(&c.into_bytes()) {
                is_ok = true;
            } else {
                println!("ERR| unable to write file '{}'", f);
            }
        } else {
            println!("ERR| unable to create file '{}'", f);
        }
    } else {
        println!("ERR| unbale to get current dir");
    }
    is_ok
}

pub fn pour(f: String, c: String) -> bool {
    let mut is_ok = false;
    if let Ok(path) = env::current_dir() {
        let base_path = format!("{}", path.display());
        if let Ok(mut file) = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(format!("{}/{}", base_path, f))
        {
            if let Ok(_) = file.write_all(&c.into_bytes()) {
                is_ok = true;
            } else {
                println!("ERR| unable to write file '{}'", f);
            }
        } else {
            println!("ERR| unable to create file '{}'", f);
        }
    } else {
        println!("ERR| unbale to get current dir");
    }
    is_ok
}

pub fn fetch(u: String, h: String) -> String {
    if let Ok(headers) = serde_json::from_str::<HashMap<String, String>>(&h) {
        let parsed_headers: Vec<(String, String)> = headers.into_iter().collect();

        if let Ok(res) = minreq::get(&u)
                            .with_headers(parsed_headers)
                            .send() {

            let result = JHPResponse {
                status: res.status_code,
                body: res.as_str().unwrap_or("").to_string(),
                headers: res.headers.into_iter().collect(),
            };

            if let Ok(json) = serde_json::to_string(&result) {
                return json;
            } else {
                println!("ERR| fetch, unbale to serialize fetch response");        
            }
        } else {
            println!("ERR| fetch unbale to send url '{}'", u);    
        }
    } else {
        println!("ERR| fetch, unbale to parse headers");
    }
    "".to_string()
}

pub fn post(u: String, h: String, b: String) -> String {
    if let Ok(headers) = serde_json::from_str::<HashMap<String, String>>(&h) {
        let parsed_headers: Vec<(String, String)> = headers.into_iter().collect();

        if let Ok(res) = minreq::post(&u)
                            .with_headers(parsed_headers)
                            .with_body(b)
                            .send() {

            let result = JHPResponse {
                status: res.status_code,
                body: res.as_str().unwrap_or("").to_string(),
                headers: res.headers.into_iter().collect(),
            };

            if let Ok(json) = serde_json::to_string(&result) {
                return json;
            } else {
                println!("ERR| post, unbale to serialize fetch response");        
            }
        } else {
            println!("ERR| post unbale to send url '{}'", u);    
        }
    } else {
        println!("ERR| post, unbale to parse headers");
    }
    "".to_string()
}