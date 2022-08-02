use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct MetasyncApi {
    url: String,
}

pub async fn post(url: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

#[wasm_bindgen]
impl MetasyncApi {
    #[wasm_bindgen(constructor)]
    pub fn new(url: String) -> MetasyncApi {
        MetasyncApi { url }
    }

    pub async fn payment_destination(&self, paymail: String) -> Result<JsValue, JsValue> {
        let url = format!("{}/paymail/p2p-payment-destination/{}", self.url, paymail);
        Ok(post(url).await?.clone())
    }
}
