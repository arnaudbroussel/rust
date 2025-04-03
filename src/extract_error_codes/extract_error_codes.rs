use regex::Regex;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Trouve tous les fichiers Mappings.cs dans un répertoire donné
fn find_mappings_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(find_mappings_files(&path));
            } else if let Some(ext) = path.extension() {
                if ext == "cs" && path.file_name().unwrap_or_default().to_string_lossy().contains("Mappings") {
                    files.push(path);
                }
            }
        }
    }
    files
}

/// Analyse un fichier Mappings.cs pour extraire les informations demandées
fn process_mappings_file(file_path: &Path) -> Vec<Vec<String>> {
    let content = fs::read_to_string(file_path).unwrap_or_default();
    let mut results = Vec::new();
    let re = Regex::new(r"ApiExceptionMapper\.RegisterException<([^>]+)>\(([^)]+)\)").unwrap();

    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            let exception_type = caps.get(1).unwrap().as_str().to_string();
            let params = caps.get(2).unwrap().as_str().replace(".", ";");
            let mut elements: Vec<String> = params.split(';').map(String::from).collect();
            elements.insert(0, exception_type);
            results.push(elements);
        }
    }
    results
}

/// Trouve un fichier .cs dans un répertoire donné
fn find_target_file(dir: &Path, filename: &str) -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_target_file(&path, filename) {
                    return Some(found);
                }
            } else if path.file_name().map(|n| n.to_string_lossy().as_ref() == filename).unwrap_or(false) {
                return Some(path);
            }
        }
    }
    None
}

/// Analyse un fichier pour extraire les informations demandées
fn process_target_file(file_path: &Path, search_value: &str) -> Option<(String, String)> {
    let content = fs::read_to_string(file_path).unwrap_or_default();
    let lines: Vec<&str> = content.lines().collect();
    let mut la = None;
    let mut ex = None;

    for (i, line) in lines.iter().enumerate() {
        if line.contains(search_value) {
            if let Some(caps) = Regex::new(r#""(.*?)""#).unwrap().captures(line) {
                ex = Some(caps.get(1).unwrap().as_str().to_string());
            }

            for j in (0..i).rev() {
                let prev_line = lines[j].trim();
                if !prev_line.starts_with("/// </summary>") && !prev_line.starts_with("/// <summary>") {
                    la = Some(prev_line.replace("/// ", "").trim_end_matches('.').to_string());
                    break;
                }
            }
            break;
        }
    }

    if let (Some(la_value), Some(ex_value)) = (la, ex) {
        Some((la_value, ex_value))
    } else {
        None
    }
}

/// Convertit un type d'exception en code HTTP
fn map_exception_to_http_code(exception: &str) -> String {
    match exception {
        "UnprocessableRequestException" => "422".to_string(),
        "ResourceNotFoundException" => "404".to_string(),
        "BadRequestException" => "400".to_string(),
        "ConflictException" => "409".to_string(),
        "UnauthorizedRequestException" => "401".to_string(),
        "ForbiddenRequestException" => "403".to_string(),
        _ => exception.to_string(),
    }
}

pub fn extract_error_codes() {
    let base_dir = Path::new("C:\\dev\\sbc.common.compliancecentre.service\\src\\SBC.Common.ComplianceCentre.Service.API\\Exceptions");
    let target_dir = Path::new("C:\\dev\\sbc.common.compliancecentre.service\\src\\SBC.Common.ComplianceCentre.Service.Domain.Core\\ErrorCodes7");

    let mapping_files = find_mappings_files(base_dir);
    let mut final_results = Vec::new();

    for mapping_file in mapping_files {
        let extracted_data = process_mappings_file(&mapping_file);

        for elements in extracted_data {
            if elements.len() < 3 {
                continue;
            }

            let firstofl = map_exception_to_http_code(&elements[0]);
            let filename = format!("{}.cs", elements[1]);
            let search_value = &elements[2];

            if let Some(target_file) = find_target_file(target_dir, &filename) {
                if let Some((la, ex)) = process_target_file(&target_file, search_value) {
                    let result = format!("{}-{}; {}-{} - {}", firstofl, ex, firstofl, ex, la);
                    final_results.push(result);
                }
            }
        }
    }

    // Génération du JSON
    let mut options: HashMap<String, Value> = HashMap::new();
    for (index, result) in final_results.iter().enumerate() {
        let parts: Vec<&str> = result.split(';').map(str::trim).collect();
        if parts.len() < 2 {
            continue;
        }
        let key = parts[0].to_string();
        let text = parts[1].to_string();

        let value = json!({
            "index": index,
            "text": text
        });

        options.insert(key, value);
    }

    let json_output = json!({ "options": options });

    // Sauvegarde du JSON
    let json_string = serde_json::to_string_pretty(&json_output).unwrap();
    fs::write("Results.json", json_string).expect("Erreur lors de l'écriture du fichier JSON");

    println!("Le fichier JSON a été généré avec succès.");
}
