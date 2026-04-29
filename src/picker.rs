use crate::config;
use crate::scanner::scan_apps;
use crate::search::find_app;
use std::{
    io::{self, Write},
    usize,
};

pub fn run_picker(paths: Vec<String>) {
    let apps = scan_apps(paths);

    // 🔹 Step 1: ask search query
    print!("Search app: ");
    io::stdout().flush().unwrap();

    let mut query = String::new();
    io::stdin().read_line(&mut query).unwrap();
    let query = query.trim().to_lowercase();

    // 🔹 Step 2: filter apps
    let mut filtered: Vec<_> = apps
        .iter()
        .filter(|app| app.name.to_lowercase().contains(&query))
        .collect();

    // 🔹 Step 3: sort by relevance
    filtered.sort_by_key(|app| {
        let name = app.name.to_lowercase();

        if name.starts_with(&query) {
            0
        } else {
            1
        }
    });

    // 🔹 Step 4: limit results
    let filtered: Vec<_> = filtered.into_iter().take(10).collect();

    if filtered.is_empty() {
        println!("No apps found");
        return;
    }

    // 🔹 Step 3: show list
    println!("\nResults:");
    for (i, app) in filtered.iter().enumerate() {
        println!("{}. {}", i + 1, app.name);
    }

    // 🔹 Step 4: select number
    print!("\nSelect number: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let index: usize = match choice.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= filtered.len() => num - 1,
        _ => {
            println!("Invalid selection");
            return;
        }
    };

    let selected = filtered[index];

    // 🔹 Step 5: alias input
    print!("Alias name: ");
    io::stdout().flush().unwrap();

    let mut alias = String::new();
    io::stdin().read_line(&mut alias).unwrap();
    let alias = alias.trim();

    // 🔹 Step 6: save alias
    let mut config = config::load_config();

    if config.aliases.contains_key(alias) {
        println!("Alias already exists");
        return;
    }

    config
        .aliases
        .insert(alias.to_string(), selected.exec.clone());

    config::save_config(&config);

    println!("✅ Alias '{}' added for {}", alias, selected.name);
}
