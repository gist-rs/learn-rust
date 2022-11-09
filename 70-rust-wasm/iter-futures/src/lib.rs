mod utils;

use anyhow::bail;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// app
use futures::future;
use reqwest::Client;

#[wasm_bindgen]
pub async fn fetch_multiple_jsvalue() -> Result<JsValue, JsError> {
    set_panic_hook();

    let results = fetch_multiple().await;

    let mut errors = vec![];
    let value = results
        .into_iter()
        .filter_map(|e| e.map_err(|err| errors.push(err.to_string())).ok())
        .collect::<Vec<_>>();

    match errors.len() {
        0.. => Ok(serde_wasm_bindgen::to_value(&value)?),
        _ => Err(JsError::new(&errors.join(","))),
    }
}

async fn fetch_multiple() -> Vec<anyhow::Result<u32>> {
    let client = &Client::new();
    let urls = vec!["https://github.com/katopz"; 2];

    let results = future::join_all(urls.into_iter().map(|url| async move {
        let resp = client.get(url).send().await?;
        resp.bytes().await
    }))
    .await;

    results
        .into_iter()
        .map(|e| match e {
            Ok(e) => Ok(e.len() as u32),
            Err(err) => bail!(err.to_string()),
        })
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
async fn fetch_multiple_value() -> anyhow::Result<Vec<u32>> {
    let results = fetch_multiple().await;

    let results = results
        .into_iter()
        .map(|e| match e {
            Ok(e) => e,
            Err(err) => panic!("{}", err),
        })
        .collect::<Vec<u32>>();

    Ok(results)
}

#[cfg(test)]
mod test {

    use super::fetch_multiple;

    #[tokio::test]
    async fn test_fetch_multiple() {
        let results = fetch_multiple().await;
        println!("{:#?}", results);
        assert!(!results.is_empty())
    }

    use super::fetch_multiple_value;

    #[tokio::test]
    async fn test_fetch_multiple_value() {
        let results = fetch_multiple_value().await;
        println!("{:#?}", results);
        assert!(!results.unwrap().is_empty())
    }
}
