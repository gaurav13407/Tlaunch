use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let home=std::env::var("HOME").unwrap();
    let local_path=format!("{}/.local/share/applications",home);
    let paths=vec![
        "/usr/share/applications".to_string(),
        local_path,
    ];

    for path in &paths{
        scan_directory(path);
    }
}

fn scan_directory(dir:&str){
    if!Path::new(dir).exists(){
        return;
    }
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()){
        let path=entry.path();
        if path.extension().and_then(|s| s.to_str())==Some("desktop"){
            parse_desktop_file(path);
        }
    }
}

fn parse_desktop_file(path:&Path){
    if let Ok(content)=fs::read_to_string(path){
        let mut name=None;
        let mut exec=None;

        for line in content.lines(){
            if line.starts_with("Name=") && name.is_none(){
                name=Some(line.replace("Name=", ""));
            }
            if line.starts_with("Exec=") && exec.is_none(){
                let cleaned=line.replace("Exec=", "");
                exec=Some(clean_exec(&cleaned));
            }
            if name.is_some() && exec.is_some(){
                break;
            }
        }
        if let (Some(name),Some(exec))=(name,exec){
            println!("{} -> {}",name,exec);
        }
    }
}

fn clean_exec(exec:&str)->String{
    exec.split_whitespace()
        .filter(|part| !part.starts_with('%'))
        .collect::<Vec<&str>>()
        .join(" ")
}
