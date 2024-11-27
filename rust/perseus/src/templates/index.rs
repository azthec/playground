use crate::components::layout::Layout;
use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
  view! { cx,
    Layout(title = "Index") {
      // Anything we put in here will be rendered inside the `<main>` block of the layout
      div(class= "flex-1 text-grey-darker text-center bg-grey-light px-4 py-2 m-2") { "Hello World!" }
      div(class= "flex-1 text-grey-darker text-center bg-grey-light px-4 py-2 m-2") { "Long page" }
      div(class= "flex-1 text-grey-darker text-center bg-grey-light px-4 py-2 m-2") { "Test 2" }
      div(class= "flex-1 text-grey-darker text-center bg-grey-light px-4 py-2 m-2") { "Test 3" }
    }
  }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
  view! { cx,
      title { "Index" }
      link(href="./tailwind.css", rel="stylesheet") {}
  }
}

pub fn get_template<G: Html>() -> Template<G> {
  Template::build("index").view(index_page).head(head).build()
}
