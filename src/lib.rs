mod utils;

use cfg_if::cfg_if;
use qrcode::{render::svg::Color, EcLevel, QrCode};
use std::{collections::HashMap, str::Split};
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

struct Config {
    pub min_size: Option<u32>,
    pub max_size: Option<u32>,
    pub ec_level: EcLevel,
    pub bg: String,
    pub fg: String,
    pub quiet_zone: bool,
}

// https://example.com/fg=000000/bg=ffffff/min=128/max=256/ec=m/qz=1?data
fn parse_config(segments: &mut Split<char>) -> Config {
    const DEFAULT_EC_LEVEL: EcLevel = EcLevel::M;
    const DEFAULT_BG: &str = "#ffffff";
    const DEFAULT_FG: &str = "#000000";
    const DEFAULT_QUIET_ZONE: bool = true;

    let map = segments.fold(HashMap::new(), |mut acc, e| {
        let pair = e.split('=').collect::<Vec<_>>();

        if pair.len() < 2 {
            return acc;
        }

        acc.insert(pair[0], pair[1]);
        acc
    });

    let min_size = match map.get("min") {
        Some(size_str) => size_str.parse().ok(),
        None => None,
    };

    let max_size = match map.get("max") {
        Some(size_str) => size_str.parse().ok(),
        None => None,
    };

    let ec_level = match map.get("ec") {
        Some(&"l") => EcLevel::L,
        Some(&"m") => EcLevel::M,
        Some(&"q") => EcLevel::Q,
        Some(&"h") => EcLevel::H,
        Some(_) => DEFAULT_EC_LEVEL,
        None => DEFAULT_EC_LEVEL,
    };

    let bg = match map.get("bg") {
        Some(color) => format!("#{}", color),
        None => DEFAULT_BG.to_owned(),
    };

    let fg = match map.get("fg") {
        Some(color) => format!("#{}", color),
        None => DEFAULT_FG.to_owned(),
    };

    let quiet_zone = match map.get("qz") {
        Some(&"1") => true,
        Some(&"0") => false,
        Some(_) => DEFAULT_QUIET_ZONE,
        None => DEFAULT_QUIET_ZONE,
    };

    Config {
        min_size,
        max_size,
        ec_level,
        bg,
        fg,
        quiet_zone,
    }
}

#[wasm_bindgen]
pub fn handle_request(url: String) -> Result<String, JsValue> {
    let url =
        Url::parse(&url).map_err(|e| JsValue::from_str(&format!("unable to parse url: {}", e)))?;

    let text = match url.query() {
        Some(text) => text.to_owned(),
        None => url.clone().into(),
    };

    let mut segments = url
        .path_segments()
        .ok_or(JsValue::from_str("cannot-be-a-base URL"))?;

    let cfg = parse_config(&mut segments);

    let code = QrCode::with_error_correction_level(text.as_bytes(), cfg.ec_level)
        .map_err(|e| JsValue::from_str(&format!("unable to create qr code: {}", e)))?;

    let mut image_builder = code.render();

    let mut image_builder = image_builder
        .dark_color(Color(&cfg.fg))
        .light_color(Color(&cfg.bg))
        .quiet_zone(cfg.quiet_zone);

    if let Some(size) = cfg.min_size {
        image_builder = image_builder.min_dimensions(size, size);
    }

    if let Some(size) = cfg.max_size {
        image_builder = image_builder.max_dimensions(size, size);
    }

    Ok(image_builder.build().to_string())
}
