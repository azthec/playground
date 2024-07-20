mod templates;

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
  PerseusApp::new()
    .template(crate::templates::index::get_template())
    .template(crate::templates::about::get_template())
    // .template(crate::templates::new_index::get_template())
}
