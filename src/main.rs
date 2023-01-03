#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Default)]
struct Point {
    // Serialize for HashMap is only available in std.
    x: std::collections::HashMap<String, String>,
}

#[cfg(feature = "serde")]
pub fn test() {
    let point = Point::default();

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
}

fn main() {
    #[cfg(feature = "serde")]
    test();
}
