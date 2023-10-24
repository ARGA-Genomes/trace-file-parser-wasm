mod utils;

use std::io::Cursor;
use cfg_if::cfg_if;
use gloo::net::http::Request;
use serde::{Serialize, Deserialize};

use abif::Abif;
use tsify::Tsify;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;


cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}


#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Error {
    r#type: String,
    message: String,
}

impl From<gloo::net::Error> for Error {
    fn from(value: gloo::net::Error) -> Self {
        Self {
            r#type: "Request".to_string(),
            message: value.to_string(),
        }
    }
}

impl From<abif::Error> for Error {
    fn from(value: abif::Error) -> Self {
        Self {
            r#type: "AbifParser".to_string(),
            message: format!("{value:?}"),
        }
    }
}

impl From<Error> for JsValue {
    fn from(value: Error) -> Self {
        serde_wasm_bindgen::to_value(&value).unwrap()
    }
}

impl From<Trace> for JsValue {
    fn from(value: Trace) -> Self {
        serde_wasm_bindgen::to_value(&value).unwrap()
    }
}


#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Trace {
    pub metadata: abif::Metadata,
    pub data: abif::TraceData,
}


#[wasm_bindgen]
pub async fn parse_remote(url: &str) -> Result<Trace, Error> {
    let resp = Request::get(url)
        .send()
        .await?;

    let binary = resp.binary().await?;
    let cursor = Cursor::new(binary);
    let abif = Abif::read(cursor)?;

    let trace = Trace {
        metadata: abif.metadata(),
        data: abif.trace_data(),
    };

    Ok(trace)
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}
