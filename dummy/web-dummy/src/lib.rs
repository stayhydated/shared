#[cfg(any(target_arch = "wasm32", test))]
mod cli;
mod components;
mod pages;
mod site;
mod terminal;

pub use site::app::App;

pub fn route_paths() -> Vec<String> {
    site::routing::all_routes()
        .into_iter()
        .map(|route| route.path().into_string())
        .collect()
}

pub fn sitemap_xml() -> String {
    site::render::render_sitemap()
}
