use sycamore::prelude::*;

#[component]
pub fn TopbarButton<G: Html>(
  cx: Scope,
  TopbarButtonProps { title, href, selected }: TopbarButtonProps,
) -> View<G> {
    // TODO: take into account selected to change the strength of the colour
    let class = "text-gray-500 hover:text-gray-800 dark:hover:text-white px-3 py-2 rounded-md text-sm font-medium";
    view! { cx, a(class=class, href=href) { (title) } }
}

#[derive(Prop)]
pub struct TopbarButtonProps {
  title: &'static str,
  href: &'static str,
  selected: bool
}
