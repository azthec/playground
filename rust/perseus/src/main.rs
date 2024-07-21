mod error_views;
mod templates;

use perseus::{plugins::Plugins, prelude::*};

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
  PerseusApp::new()
    .plugins(Plugins::new().plugin(
      perseus_tailwind::get_tailwind_plugin,
      perseus_tailwind::TailwindOptions {
        in_file: Some("src/tailwind.css".into()),
        out_file: "dist/tailwind.css".into(),
        config_path: None,
      },
    ))
    .static_alias("/tailwind.css", "dist/tailwind.css")
    .template(crate::templates::index::get_template())
    .template(crate::templates::about::get_template())
    .template(crate::templates::new_index::get_template())
    .error_views(crate::error_views::get_error_views())
}
