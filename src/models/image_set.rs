use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageSetElement {
    pub src: String,  // The file path or URL of the image
    pub width: u32,   // The width of the image in pixels
    pub density: f32, // The pixel density of the image, e.g. 1x, 1.5x, 2x, etc.
}
