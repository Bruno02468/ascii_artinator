//! This is a simple implementation of a web interface for the API.

/// This will return the API endpoint, which can be set via an environment
/// variable, defaulting to same host, same port, "/braille".
fn get_endpoint() -> &'static str {
    return option_env!("AA_ENDPOINT").unwrap_or("/braille");
}

fn main() {
    println!("Hello, world!");
}
