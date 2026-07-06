use crate::ex_data::ex6::Asset;
use crate::ex_data::ex6::AssetManifest;
use bincode;
use serde;
use serde_derive;
use std::collections::HashMap;
use std::io::{BufWriter, BufReader, Write};
use std::fs::File;

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, Default)]
struct AssetRegistry {
    collection: std::collections::HashMap<String, Asset>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        return Self {
            collection: HashMap::new(),
        };
    }
    // pub fn append_asset(&mut self, guid: &str, asset: Asset);
    pub fn compile_to_binary_package(
        &self,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>>{
        let data = self;
        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);
        bincode::serialize_into(&mut writer, &data)?;
        return Ok(());
    }
    pub fn load_from_binary_package(package_path: &str)
    -> Result<Self, Box<dyn std::error::Error>>{
        let file = File::open(package_path)?;
        let mut reader = BufReader::new(file);
        let data: Self = bincode::deserialize_from(&mut reader)?;
        return Ok(data);
    }
}

pub fn main7() {
    let mock_svg_path = "mock_hero.svg";
    let manifest_json_path = "assets_manifest.json";
    let binary_package_path = "assets_package.bin";

    // 1. Generate a temporary mock SVG file on disk
    let mut file = File::create(mock_svg_path).unwrap();
    file.write_all(b"<svg viewBox='0 0 32 32'></svg>").unwrap();

    // 2. Test AssetManifest tracking & JSON storage
    println!("[ENGINE] Initializing text-based manifest...");
    let mut manifest = AssetManifest::new();
    manifest
        .register_svg("svg_player", mock_svg_path, 32.0, 32.0)
        .expect("Failed to register asset file");

    manifest
        .save_to_json(manifest_json_path)
        .expect("Failed to write manifest file");
    
    let loaded_manifest = AssetManifest::load_from_json(manifest_json_path)
        .expect("Failed to read manifest file");
    println!("[ENGINE] Text-based SVG Manifest system verified.");

    // 3. Test AssetRegistry compilation & Bincode round-trip
    println!("[ENGINE] Compiling manifest into binary package registry...");
    let mut registry = AssetRegistry::new();
    
    // We move the collection data out of the loaded text manifest into our binary registry
    registry.collection = loaded_manifest.collection;

    // Serialize to binary package
    registry
        .compile_to_binary_package(binary_package_path)
        .expect("Failed to compile to binary package");
    println!("[ENGINE] Binary package compiled successfully to '{}'.", binary_package_path);

    // Deserialize back from binary package
    println!("[ENGINE] Loading binary package back into memory...");
    let loaded_registry = AssetRegistry::load_from_binary_package(binary_package_path)
        .expect("Failed to load binary package");

    // 4. Verify everything works and data integrity is maintained
    if let Some(Asset::Vector(graphics)) = loaded_registry.collection.get("svg_player") {
        println!("[SUCCESS] Binary asset package verified!");
        println!("  - Asset key found: 'svg_player'");
        println!("  - Dimensions: {}x{}", graphics.width, graphics.height);
        println!("  - Raw XML Data: {:?}", graphics.raw_xml);
    } else {
        panic!("[ERROR] Data corruption! The 'svg_player' asset was missing or corrupted in the binary package.");
    }

    // Clean up files (Optional)
    let _ = std::fs::remove_file(mock_svg_path);
    let _ = std::fs::remove_file(manifest_json_path);
    let _ = std::fs::remove_file(binary_package_path);
}