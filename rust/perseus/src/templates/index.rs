use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
  view! { cx,
    div(class = "flex min-h-full flex-col justify-center px-6 py-12 lg:px-8") {
      div(class = "sm:mx-auto sm:w-full sm:max-w-sm") {
        h1(class = "mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900") {
          "Welcome to perseus!"
        }
        p(class="mt-1 max-w-2xl text-center text-sm leading-6 text-gray-500") {
          "This is just an example app. Try changing some code inside "
          code { "src/templates/index.rs" }
          " and you'll be able to see the results here!"
        }
        a(href = "about", id = "about-link",
            class = "inline-flex items-center rounded-md bg-blue-50 px-2 py-1 text-xs font-medium text-blue-700 ring-1 ring-inset ring-blue-700/10") {
          "About!"
        }
      }
    }
  }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
  view! { cx,
      title { "Welcome to Perseus!" }
      link(href="./tailwind.css", rel="stylesheet") {}
  }
}

pub fn get_template<G: Html>() -> Template<G> {
  Template::build("index").view(index_page).head(head).build()
}
