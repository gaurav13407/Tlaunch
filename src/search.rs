use crate::model::App;

pub fn find_app<'a>(apps: &'a Vec<App>, query: &str) -> Option<&'a App> {
    let query = query.to_lowercase();

   apps.iter()
    .find(|app| app.name.to_lowercase().starts_with(&query))
    .or_else(|| {
        apps.iter()
            .find(|app| app.name.to_lowercase().contains(&query))
    }) 
}
