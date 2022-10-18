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
use async_std::task;
use reqwest::Client;

#[wasm_bindgen]
pub async fn fetch_multiple_jsvalue() -> Result<JsValue, JsError> {
    set_panic_hook();

    let results = fetch_multiple().await;

    let mut errors = vec![];
    let value = results
        .into_iter()
        .filter_map(|e| {
            e.map_err(|err| errors.push(err.to_string().to_owned()))
                .ok()
        })
        .collect::<Vec<_>>();

    match errors.len() {
        0.. => Ok(serde_wasm_bindgen::to_value(&value)?),
        _ => Err(JsError::new(&errors.join(","))),
    }
}

pub async fn fetch_multiple() -> Vec<anyhow::Result<u32>> {
    set_panic_hook();

    let urls = vec!["https://github.com/katopz"; 2];

    let handles = urls.into_iter().map(|url| {
        let client = Client::new();
        task::spawn_local(async move {
            let resp = client.get(url).send().await?;
            resp.bytes().await
        })
    });

    println!("handles:{:#?}", handles);

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    println!("results:{:#?}", results);

    results
        .into_iter()
        .map(|e| match e {
            Ok(e) => Ok(e.len() as u32),
            Err(err) => bail!("{}", err),
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {

    use async_std::task;

    #[tokio::test]
    async fn test_spawn_local() {
        // Somehow this not working
        let handle = task::spawn_local(async { 1 + 2 });
        assert_eq!(handle.await, 3);
    }

    use super::fetch_multiple;

    #[tokio::test]
    async fn test_fetch_multiple() {
        // Somehow this not working
        let results = fetch_multiple().await;
        println!("{:#?}", results);
        assert!(results.len() > 0);
    }
}
