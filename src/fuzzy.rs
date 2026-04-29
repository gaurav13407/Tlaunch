use crate::model::App;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn fuzzy_search<'a>(apps: &'a Vec<App>, query: &str) -> Vec<(&'a App, i64)> {
    let matcher = SkimMatcherV2::default();

    let mut results: Vec<_> = apps
        .iter()
        .filter_map(|app| {
            matcher
                .fuzzy_match(&app.name, query)
                .map(|score| (app, score))
        })
        .collect();

    // sort by score (descending)
    results.sort_by(|a, b| b.1.cmp(&a.1));

    results
}
