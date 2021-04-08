extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde_json::json;
use serde::Deserialize;
use std::collections::HashMap;
use reqwest::blocking::Response;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct ApiError {
    code: i32,
    message: String,
}

#[derive(Deserialize, Debug)]
enum ApiResponse<T> {
    #[serde(rename = "success")]
    Success(T),
    #[serde(rename = "error")]
    Error(ApiError),
}

#[derive(Deserialize, Debug)]
struct ApiCredentials {
    token: String
}

#[derive(Debug, Clone)]
pub struct Client {
    instance_url: String,
    api_key: Option<String>,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(instance_url: String) -> Self {
        Client {
            instance_url,
            api_key: None,
            client: reqwest::blocking::Client::builder()
                .cookie_store(true)
                .build()
                .expect("Can't build http client"),
        }
    }
    pub fn url(&self, path: &str) -> String {
        format!("{}/{}", self.instance_url, path)
    }

    pub fn login_with_credential(&mut self, username: String, password: String) -> Result<Self, reqwest::Error> {
        let mut api_client = self.clone();
        match api_client.client
            .post(&self.url("login"))
            .header("Accept", "application/json")
            .json(&json!({
                "login": username,
                "password": password
            }))
            .send() {
            Ok(response) => {
                let result = response.json::<ApiResponse<ApiCredentials>>()?;
                match result {
                    ApiResponse::Success(data) => {
                        api_client.api_key = Some(data.token);
                    }
                    ApiResponse::Error(_) => {}
                }
                Ok(api_client)
            }
            Err(err) => {
                println!("Error : {:?}", err);
                Err(err)
            }
        }
    }
    pub fn login_yunohost(&mut self, username: String, password: String) -> Result<Self, reqwest::Error> {
        let mut api_client = self.clone();
        let mut body: HashMap<String, String> = HashMap::new();
        body.insert("user".to_string(), username);
        body.insert("password".to_string(), password);
        let response = api_client.client
            .post(&"https://apps.coop1d.com/yunohost/sso/".to_string())
            .header(
                "User-Agent",
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:87.0) Gecko/20100101 Firefox/87.0",
            )
            .header(
                "Referer",
                "https://apps.coop1d.com/yunohost/sso/",
            )
            .header(
                "Origin",
                "https://apps.coop1d.com",
            )
            .form(&body)
            .send()?;
        Ok(api_client)
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;
    use reqwest::Error;

    #[test]
    fn create_client() {
        match Client::new("https://apps.coop1d.com/BOS/api/index.php".to_string())
            .login_yunohost("alissonebos".to_string(), "alissonebos".to_string()) {
            Ok(mut client) => {
                println!("Yunohost {:?}", client);
                match client.login_with_credential("alissonebos".to_string(), "alissonebos".to_string()) {
                    Ok(client) => { println!("Dolibarr {:?}", client) }
                    Err(err) => { println!("{:?}", err) }
                }
            }
            Err(err) => println!("Err {:?}", err)
        }
    }
}