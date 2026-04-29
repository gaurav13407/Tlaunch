use crate::model::App;
use std::process::Command;

pub fn run_app(app: &App) {
    let parts: Vec<&str> = app.exec.split_whitespace().collect();

    if let Some((cmd, args)) = parts.split_first() {
        Command::new(cmd)
            .args(args)
            .spawn()
            .expect("Failed to launch app");
    }
}
