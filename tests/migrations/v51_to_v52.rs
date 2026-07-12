#[must_use]
pub fn without_message() -> u8 {
    0
}

#[must_use = "custom message"]
pub fn with_message() -> u8 {
    1
}
