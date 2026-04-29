mod scanner;
mod model;
mod search;
mod runner;
mod config;
mod setup;

use scanner::scan_apps;
use search::find_app;
use runner::run_app;

fn main() {
    let args: Vec<String> = std::env::args().collect();
     
     if args.len() >= 2 && args[1] == "setup" {
    setup::run_setup();
    return;
}
    if args.len() < 2 {
        println!("Usage: tlaunch <app>");
        return;
    }

    // setup paths
    let home = std::env::var("HOME").unwrap();
    let local_path = format!("{}/.local/share/applications", home);

    let paths = vec![
        "/usr/share/applications".to_string(),
        local_path,
    ];

if args.len() >= 2 && args[1] == "list" {
    let apps = scan_apps(paths);

    if args.len() == 3 {
        let query = args[2].to_lowercase();

        for app in apps {
            if app.name.to_lowercase().contains(&query) {
                println!("{} → {}", app.name, app.exec);
            }
        }
    } else {
        for app in apps {
            println!("{} → {}", app.name, app.exec);
        }
    }

    return;
}
    if args.len() >= 5 && args[1] == "alias" && args[2] == "add" {
        let alias = &args[3];
        let target = &args[4];

        let mut config = config::load_config();

        if config.aliases.contains_key(alias) {
            println!("Alias already exists");
            return;
        }

        let apps = scan_apps(paths.clone());

        if let Some(app) = find_app(&apps, target) {
            config.aliases.insert(alias.clone(), app.exec.clone());
            config::save_config(&config);
            println!("Alias '{}' added for {}", alias, app.name);
        } else {
            println!("App not found");
        }

        return;
    }


    let query = &args[1];
    let config = config::load_config();

    // check alias first
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

    // fallback to search
    let apps = scan_apps(paths);

    if let Some(app) = find_app(&apps, query) {
        println!("Launching: {}", app.name);
        run_app(app);
    } else {
        println!("No match found");
    }
}
