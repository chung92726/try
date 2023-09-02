
use std::io::{Error, ErrorKind};

async fn my_async_call(url: &str) -> Result<serde_json::Value, Error>{
    

    let response = reqwest::get(url).await.map_err(|_| Error::new(ErrorKind::Other, "Could not retrieve response" ))?;


    let json_response = response.json::<serde_json::Value>().await.map_err(|_| Error::new(ErrorKind::Other, "Could not parse response" ))?;

    Ok(json_response)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calls_async_fn(){
        let api_url: &str = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let my_res:Result<serde_json::Value, std::io::Error> = my_async_call(api_url).await;
        let res = if let Ok(i) = my_res {
            println!("{:?}", i);
        }else{
            panic!("There was an problem")
        };
    }

}