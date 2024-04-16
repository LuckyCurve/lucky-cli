use anyhow::Error;
use local_ip_address::local_ip;
use serde::Deserialize;

pub async fn get_local_ip() -> Result<String, Error> {
    Ok(local_ip()?.to_string())
}

pub async fn get_public_ip() -> Result<String, Error> {
    let response = reqwest::get("https://api.ipify.org/?format=json")
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str::<IpResponse>(response.as_str())?.ip)
}

#[derive(Deserialize)]
struct IpResponse {
    ip: String,
}

#[cfg(test)]
mod tests {
    use crate::network::get_public_ip;

    #[tokio::test]
    async fn test_get_public_ip() {
        let string = get_public_ip().await.unwrap();
        println!("{}", string)
    }

    #[tokio::test]
    async fn test_get_local_ip() {}
}
