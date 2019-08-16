mod utils;

use cfg_if::cfg_if;
use qrcode_generator::QrCodeEcc;
use url::Url;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

const DEFAULT_SIZE: usize = 512;
const DEFAULT_ECC: QrCodeEcc = QrCodeEcc::Low;

#[wasm_bindgen]
pub fn handle_request(url: String) -> String {
    let url = Url::parse(&url).unwrap();

    let text = match url.query() {
        Some(text) => text.to_owned(),
        None => url.clone().into_string(),
    };

    let result = qrcode_generator::to_svg_to_string(
        text,
        match url.path_segments().unwrap().nth(1) {
            Some("l") => QrCodeEcc::Low,
            Some("m") => QrCodeEcc::Medium,
            Some("q") => QrCodeEcc::Quartile,
            Some("h") => QrCodeEcc::High,
            Some(_) => DEFAULT_ECC,
            None => DEFAULT_ECC,
        },
        match url.path_segments().unwrap().nth(0) {
            Some(size_str) => match size_str.parse() {
                Ok(size) => size,
                Err(_) => DEFAULT_SIZE,
            },
            None => DEFAULT_SIZE,
        },
        None,
    )
    .unwrap();

    format!("{}", result)
}
