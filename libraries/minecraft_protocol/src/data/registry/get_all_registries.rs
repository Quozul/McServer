use crate::data::registry::registry_entry::RegistryEntry;
use crate::prelude::{Identifier, Nbt};
use crate::protocol_version::ProtocolVersion;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use walkdir::WalkDir;

const AVAILABLE_REGISTRIES: [&str; 9] = [
    "minecraft:banner_pattern",
    "minecraft:chat_type",
    "minecraft:damage_type",
    "minecraft:dimension_type",
    "minecraft:painting_variant",
    "minecraft:trim_material",
    "minecraft:trim_pattern",
    "minecraft:wolf_variant",
    "minecraft:worldgen/biome",
];

const REGISTRIES_TO_SEND: [&str; 9] = [
    "banner_pattern",
    "chat_type",
    "damage_type",
    "dimension_type",
    "painting_variant",
    "trim_material",
    "trim_pattern",
    "wolf_variant",
    "worldgen/biome",
];

pub fn get_grouped_registries(
    protocol_version: ProtocolVersion,
) -> HashMap<Identifier, Vec<RegistryEntry>> {
    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "./data/generated".to_string());
    let version_directory = PathBuf::from(data_dir)
        .join(protocol_version.to_string())
        .join("data/minecraft");
    let registries = get_all_registries(&version_directory);

    let mut grouped: HashMap<Identifier, Vec<RegistryEntry>> = HashMap::new();

    for registry in &registries {
        let entry = RegistryEntry {
            entry_id: Identifier::minecraft(&registry.entry_id),
            has_data: true,
            nbt: Some(registry.nbt.clone()),
        };
        grouped
            .entry(Identifier::from_str(&registry.registry_id).unwrap())
            .or_default()
            .push(entry);
    }

    grouped
}

pub fn get_overworld_codec(protocol_version: ProtocolVersion) -> Nbt {
    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "./data/generated".to_string());
    let overworld_path = PathBuf::from(data_dir)
        .join(protocol_version.to_string())
        .join("data/minecraft/dimension_type/overworld.json");
    get_json_files(&overworld_path)
        .map(|json| Nbt::from_json(&json, None).to_nameless_compound())
        .unwrap_or(Nbt::End)
}

pub fn get_registry_codec(protocol_version: ProtocolVersion) -> Nbt {
    let grouped = get_grouped_registries(protocol_version.clone());
    Nbt::Compound {
        name: None,
        value: grouped
            .iter()
            .map(|(registry_id, entries)| {
                let mut id = 0;
                Nbt::Compound {
                    name: Some(registry_id.to_string()),
                    value: vec![
                        Nbt::String {
                            name: Some(String::from("type")),
                            value: registry_id.to_string(),
                        },
                        Nbt::List {
                            name: Some(String::from("value")),
                            value: entries
                                .iter()
                                .map(|e| {
                                    let n = Nbt::Compound {
                                        name: None,
                                        value: vec![
                                            Nbt::String {
                                                name: Some("name".to_string()),
                                                value: e.entry_id.to_string(),
                                            },
                                            Nbt::Int {
                                                name: Some("id".to_string()),
                                                value: id,
                                            },
                                            e.nbt
                                                .clone()
                                                .unwrap()
                                                .to_named_compound("element".to_string()),
                                        ],
                                    };
                                    id = id + 1;
                                    if protocol_version >= ProtocolVersion::V1_20_2 {
                                        n.to_nameless_compound()
                                    } else {
                                        n
                                    }
                                })
                                .collect(),
                            tag_type: 10,
                        },
                    ],
                }
            })
            .collect::<Vec<_>>(),
    }
}

fn get_all_registries(root_directory: &Path) -> Vec<DataRegistryEntry> {
    WalkDir::new(root_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path().to_str().unwrap_or_default();
            REGISTRIES_TO_SEND.iter().any(|s| path.contains(s))
        })
        .flat_map(|entry| json_to_nbt(root_directory, entry.path()))
        .filter(|entry| AVAILABLE_REGISTRIES.contains(&entry.registry_id.as_str()))
        .collect::<Vec<_>>()
}

#[derive(Debug)]
pub struct DataRegistryEntry {
    pub registry_id: String,
    pub entry_id: String,
    pub nbt: Nbt,
}

fn json_to_nbt(
    root_directory: &Path,
    path: &Path,
) -> Result<DataRegistryEntry, Box<dyn std::error::Error>> {
    if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
        return Err(format!("{:?} not a json file", path).into());
    }

    let registry_id = get_registry_id(root_directory, path)?;
    let entry_id = get_entry_id(path)?;
    let json = get_json_files(path)?;
    let nbt = Nbt::from_json(&json, None).to_nameless_compound();

    Ok(DataRegistryEntry {
        registry_id,
        entry_id,
        nbt,
    })
}

fn get_registry_id(
    root_directory: &Path,
    path: &Path,
) -> Result<String, Box<dyn std::error::Error>> {
    let suffix = path.strip_prefix(root_directory)?;
    Ok(format!(
        "minecraft:{}",
        suffix
            .parent()
            .ok_or("failed to read suffix")?
            .to_string_lossy()
    ))
}

fn get_entry_id(path: &Path) -> Result<String, &str> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .ok_or("Failed to get file stem")
}

fn get_json_files(path: &Path) -> std::io::Result<Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}
