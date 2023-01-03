#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    // Serialize for HashMap is available in std only.
    x: std::collections::HashMap<String, String>,
    y: i32,
}

#[cfg(feature = "serde")]
pub fn test() {
    let point = Point {
        x: Default::default(),
        y: 2,
    };

    let test = std::sync::Mutex::new(point);

    let serialized = serde_json::to_string(&test).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

fn main() {
    #[cfg(feature = "serde")]
    test();
}
