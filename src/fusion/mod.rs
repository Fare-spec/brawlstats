use chrono::Local;
use serde_json::{Map, Value};
use std::fs::{self, File};
use std::io::{self, BufReader, Write};


fn read_json_object(file_path: &str) -> io::Result<Map<String, Value>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let json_value: Value = serde_json::from_reader(reader)?; 

    if let Value::Object(map) = json_value {
        Ok(map) 
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "This JSON file isn't an object"))
    }
}

fn merge_json_objects(file_paths: &[&str], output_file: &str) -> io::Result<()> {
    let mut merged_map = Map::new();

    for &file_path in file_paths {
        let json_map = read_json_object(file_path)?;
        for (key, value) in json_map {
            merged_map.entry(key).or_insert(value);
        }
    }

    let merged_json = Value::Object(merged_map);
    let merged_json_str = serde_json::to_string_pretty(&merged_json)?;

    let mut output = File::create(output_file)?;
    writeln!(output, "{}", merged_json_str)?;

    Ok(())
}

pub fn merge_files(files:Vec<&str>,path: &str) -> io::Result<()> {
    merge_json_objects(&files, path)?;
    for file in files {
        if let Err(e) = fs::remove_file(file) {
            eprintln!("Erreur lors de la suppression du fichier {}: {}", file, e);
        } else {
            println!("Fichier supprimé : {}", file);
        }
    }
    Ok(())
}
