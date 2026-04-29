mod scanner;
mod model;
mod search;
mod runner;

use scanner::scan_apps;
use search::find_app;
use runner::run_app;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: tlaunch <app>");
        return;
    }

    let query = &args[1];

    let home = std::env::var("HOME").unwrap();
    let local_path = format!("{}/.local/share/applications", home);

    let paths = vec![
        "/usr/share/applications".to_string(),
        local_path,
    ];

    let apps = scan_apps(paths);

    if let Some(app) = find_app(&apps, query) {
        println!("Launching: {}", app.name);
        run_app(app);
    } else {
        println!("No match found");
    }
}
