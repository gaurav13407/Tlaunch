use crate::model::App;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_apps(paths: Vec<String>) -> Vec<App> {
    let mut apps = Vec::new();

    for path in paths {
        scan_directory(&path, &mut apps);
    }

    apps
}

fn scan_directory(dir: &str, apps: &mut Vec<App>) {
    if !Path::new(dir).exists() {
        return;
    }

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
            parse_desktop_file(path, apps);
        }
    }
}

fn parse_desktop_file(path: &Path, apps: &mut Vec<App>) {
    if let Ok(content) = fs::read_to_string(path) {
        let mut name = None;
        let mut exec = None;

        for line in content.lines() {
            if line.starts_with("Name=") && name.is_none() {
                name = Some(line.replace("Name=", ""));
            }

            if line.starts_with("Exec=") && exec.is_none() {
                let cleaned = line.replace("Exec=", "");
                exec = Some(clean_exec(&cleaned));
            }

            if name.is_some() && exec.is_some() {
                break;
            }
        }

        if let (Some(name), Some(exec)) = (name, exec) {
            apps.push(App { name, exec });
        }
    }
}

fn clean_exec(exec: &str) -> String {
    exec.split_whitespace()
        .filter(|p| !p.starts_with('%'))
        .collect::<Vec<&str>>()
        .join(" ")
}
