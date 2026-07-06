use std::collections::HashMap;

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, Default)]
pub struct VectorGraphics {
    pub raw_xml: String,
    pub width: f32,
    pub height: f32,
}
impl VectorGraphics {
    pub fn new(raw_xml: String, width: f32, height: f32) -> Self {
        return VectorGraphics {
            raw_xml,
            width,
            height,
        };
    }
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub enum Asset {
    Vector(VectorGraphics),
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, Default)]
pub struct AssetManifest {
    pub collection: std::collections::HashMap<String, Asset>,
}

impl AssetManifest {
    pub fn new() -> Self {
        return AssetManifest {
            collection: HashMap::new(),
        };
    }
    pub fn register_svg(
        &mut self,
        guid: &str,
        file_path: &str,
        w: f32,
        h: f32,
    ) -> std::io::Result<()> {
        let data = std::fs::read_to_string(file_path)?;
        println!("{}", data);
        let asset = VectorGraphics::new(data, w, h);
        self.collection
            .insert(guid.to_string(), Asset::Vector(asset));
        return Ok(());
    }
    pub fn save_to_json(&self, path: &str) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        let _ = std::fs::write(path, data);
        return Ok(());
    }
    pub fn load_from_json(path: &str) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let manifest = serde_json::from_str(&content)?;
        Ok(manifest)
    }
}

pub fn main6() {
    // Example usage:
    let mut manifest = AssetManifest::new();

    // Create a dummy SVG file for demonstration purposes
    std::fs::write("test.svg", "<svg></svg>").unwrap();

    if let Err(e) = manifest.register_svg("icon_01", "test.svg", 512.0, 512.0) {
        eprintln!("Error registering SVG: {}", e);
    }

    if let Err(e) = manifest.save_to_json("manifest.json") {
        eprintln!("Error saving JSON: {}", e);
    }
}
