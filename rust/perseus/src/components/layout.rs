use crate::components::topbar::{Topbar};
use sycamore::prelude::*;

// NOTE: None of the code in this file is Perseus-specific! This could easily be
// applied to any Sycamore app.

#[component]
pub fn Layout<'a, G: Html>(
  cx: Scope<'a>,
  LayoutProps { title, children }: LayoutProps<'a, G>,
) -> View<G> {
  let children = children.call(cx);

  view! { cx,
      // These elements are styled with bright colors for demonstration purposes
      Topbar(title = "Example") {} //, pages = pages) { }
      div(class = "flex-col items-stretch bg-grey-lighter min-h-screen") { (children) }
      // Later we can add a footer here if we want
  }
}

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
  /// The title of the page, which will be displayed in the header.
  pub title: &'a str,
  /// The content to put inside the layout.
  pub children: Children<'a, G>,
}
