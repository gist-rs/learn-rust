mod utils;

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
pub async fn fetch_multiple() -> Result<JsValue, JsError> {
    set_panic_hook();

    let results = _fetch_multiple().await;

    let mut errors = vec![];
    let value = bodies
        .into_iter()
        .filter_map(|e| {
            e.map_err(|err| errors.push(err.to_string().to_owned()))
                .ok()
        })
        .map(|e| e.len() as u32)
        .collect::<Vec<_>>();

    match errors.len() {
        0 => Ok(serde_wasm_bindgen::to_value(&value)?),
        _ => Err(JsError::new(&errors.join(","))),
    }
}

async fn _fetch_multiple() -> Vec<anyhow::Result<Value>> {
    let client = &Client::new();
    let urls = vec!["https://github.com/katopz"; 2];

    let bodies = future::join_all(urls.into_iter().map(|url| {
        async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        }
    }))
    .await;

    let mut errors = vec![];
    let value = bodies
        .into_iter()
        .filter_map(|e| {
            e.map_err(|err| errors.push(err.to_string().to_owned()))
                .ok()
        })
        .map(|e| e.len() as u32)
        .collect::<Vec<_>>();

    match errors.len() {
        0 => Ok(serde_wasm_bindgen::to_value(&value)?),
        _ => Err(JsError::new(&errors.join(","))),
    }
}

#[cfg(test)]
mod test {

    use super::fetch_multiple;

    #[tokio::test]
    async fn test_fetch_multiple() {
        let result = fetch_multiple().await;
        println!("{:#?}", result.ok());
        // assert!(result > 0)
    }
}
