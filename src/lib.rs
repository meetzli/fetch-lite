use reqwest::{Client, Error, redirect::Policy};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub async fn fetch(url: &str) -> Result<Response, Error> {
    let client = Client::builder()
        .redirect(Policy::none())
        .build()?;
    let res = client.get(url).send().await?;
    let status = res.status().as_u16();
    let headers = res
        .headers()
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_str().unwrap().to_string()))
        .collect();
    let body = res.text().await?;

    Ok(Response { status, headers, body })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_success() {
        let url = "https://api.thecatapi.com/v1/images/search";
        let res = fetch(url).await.unwrap();
        println!("{:?}", res);
        assert_eq!(res.status, 200);
        assert!(res.body.contains("url"));
    }
}
