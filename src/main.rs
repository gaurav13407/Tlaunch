mod config;
mod fuzzy;
mod model;
mod picker;
mod runner;
mod scanner;
mod search;
mod setup;

use runner::run_app;
use scanner::scan_apps;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !std::path::Path::new(&format!(
        "{}/.tlaunch/config.toml",
        std::env::var("HOME").unwrap()
    ))
    .exists()
    {
        println!("⚠️  First time setup required!");
        println!("👉 Run: tlaunch setup\n");
    }
    // 🔹 Setup command
    if args.len() >= 2 && args[1] == "setup" {
        setup::run_setup();
        return;
    }

    if args.len() < 2 {
        println!("Usage: tlaunch <command>");
        return;
    }

    // 🔹 Setup paths
    let home = std::env::var("HOME").unwrap();
    let local_path = format!("{}/.local/share/applications", home);

    let paths = vec!["/usr/share/applications".to_string(), local_path];

    // 🔹 List apps (now fuzzy)
    if args[1] == "list" {
        let apps = scan_apps(paths);

        if args.len() == 3 {
            let query = &args[2];
            let results = fuzzy::fuzzy_search(&apps, query);

            for (app, _) in results.into_iter().take(20) {
                println!("{} → {}", app.name, app.exec);
            }
        } else {
            for app in apps {
                println!("{} → {}", app.name, app.exec);
            }
        }

        return;
    }

    // 🔹 Picker
    if args[1] == "pick" {
        picker::run_picker(paths.clone());
        return;
    }

    // 🔹 Alias ADD (fuzzy-enabled)
    if args.len() >= 5 && args[1] == "alias" && args[2] == "add" {
        let alias = &args[3];
        let target = &args[4];

        let mut config = config::load_config();

        if config.aliases.contains_key(alias) {
            println!("Alias already exists");
            return;
        }

        let apps = scan_apps(paths.clone());
        let results = fuzzy::fuzzy_search(&apps, target);

        if let Some((app, _)) = results.first() {
            config.aliases.insert(alias.clone(), app.exec.clone());
            config::save_config(&config);
            println!("Alias '{}' added for {}", alias, app.name);
        } else {
            println!("App not found");
        }

        return;
    }

    // 🔹 Alias REMOVE
    if args.len() >= 4 && args[1] == "alias" && args[2] == "remove" {
        let alias = &args[3];

        let mut config = config::load_config();

        if config.aliases.remove(alias).is_some() {
            config::save_config(&config);
            println!("Alias '{}' removed", alias);
        } else {
            println!("Alias not found");
        }

        return;
    }

    // 🔹 Alias LIST
    if args.len() >= 3 && args[1] == "alias" && args[2] == "list" {
        let config = config::load_config();

        if config.aliases.is_empty() {
            println!("No aliases set");
        } else {
            let mut aliases: Vec<_> = config.aliases.iter().collect();
            aliases.sort_by_key(|(k, _)| *k);

            for (alias, cmd) in aliases {
                println!("{} → {}", alias, cmd);
            }
        }

        return;
    }

    // 🔹 Normal run
    let query = &args[1];
    let config = config::load_config();

    // Check alias first
    if let Some(exec) = config.aliases.get(query) {
        println!("Launching alias: {}", query);

        let parts: Vec<&str> = exec.split_whitespace().collect();

        if let Some((cmd, args)) = parts.split_first() {
            std::process::Command::new(cmd)
                .args(args)
                .spawn()
                .expect("Failed to launch");
        }

        return;
    }

    // 🔹 Fuzzy search fallback
    let apps = scan_apps(paths);

    let results = fuzzy::fuzzy_search(&apps, query);

    if let Some((app, _)) = results.first() {
        println!("Launching: {}", app.name);
        run_app(app);
    } else {
        println!("No match found");
    }
}
