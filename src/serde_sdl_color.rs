use serde::{Deserializer, Deserialize, Serialize, Serializer};

use crate::point_and_color::Color;

pub fn serialize<S: Serializer>(color: &Color, serializer: S) -> Result<S::Ok, S::Error> {
    color.rgb().serialize(serializer)
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Color, D::Error> {
    let (r, g, b) = <(u8, u8, u8)>::deserialize(deserializer)?;
    Ok(Color::RGB(r, g, b))
}