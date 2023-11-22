use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Project1 {
    test: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Project2 {
    another: String,
}

pub fn generate_jwt(project_type: &String, content: &String, secret: &String) -> String {
    let encoding_key_result = EncodingKey::from_base64_secret(secret);

    let encoding_key = match encoding_key_result {
        Ok(result) => result,
        Err(error) => {
            println!("Problem parsing json to struct: {:?}", error);
            return String::from("");
        }
    };

    match project_type.as_str() {
        "project1" => {
            let my_claims_result = serde_json::from_str::<Project1>(content);
            let my_claims = match my_claims_result {
                Ok(result) => result,
                Err(error) => {
                    println!("Problem parsing json to struct: {:?}", error);
                    return String::from("");
                }
            };
            let result = encode(&Header::default(), &my_claims, &encoding_key);
            return match result {
                Ok(jwt) => jwt,
                Err(error) => {
                    println!("Problem parsing json to struct: {:?}", error);
                    return String::from("");
                }
            };
        }
        "project2" => {
            let my_claims_result = serde_json::from_str::<Project2>(content);
            let my_claims = match my_claims_result {
                Ok(result) => result,
                Err(error) => {
                    println!("Problem parsing json to struct: {:?}", error);
                    return String::from("");
                }
            };
            let result = encode(&Header::default(), &my_claims, &encoding_key);
            return match result {
                Ok(jwt) => jwt,
                Err(error) => {
                    println!("Problem parsing json to struct: {:?}", error);
                    return String::from("");
                }
            };
        }
        _ => panic!("Invalid project"),
    }
}
