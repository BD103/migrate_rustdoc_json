use serde::Deserialize;

#[derive(Deserialize)]
struct Crate {
    pub format_version: u32,
}

pub fn detect_version(json: &str) -> u32 {
    let Crate { format_version } = serde_json::from_str(json).unwrap();
    format_version
}
