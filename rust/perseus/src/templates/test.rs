use perseus::prelude::*;
use sycamore::prelude::*;

fn test_page<G: Html>(cx: Scope) -> View<G> {
    let count = vec![1, 2];

    let views = View::new_fragment(count.iter().map(|&x| view! { cx, li { (x) } }).collect());

    view! { cx,
        ul {
            (views)
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
  Template::build("test").view(test_page).head(head).build()
}

