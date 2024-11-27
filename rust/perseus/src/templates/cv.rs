use crate::components::layout::Layout;
use perseus::prelude::*;
use sycamore::prelude::*;

fn cv_page<G: Html>(cx: Scope) -> View<G> {
  view! { cx,
      Layout(title = "CV") {
      section {
          div(class = "max-w-5xl px-8 py-24 pb-12 mx-auto 2xl:max-w-7xl md:px-lg:px-24 lg:pt-32") {
              div(class = "flex items-center") {
                  div {
                      img(class = "inline-block object-cover w-24 h-48 rounded-full lg:w-64 lg:h-96", src = ".perseus/static/nyan.png", alt = "")
                  }
                  div(class = "ml-4 md:ml-8") {
                      p(class = "text-4xl font-semibold tracking-tighter text-gray-900 md:text-6xl lg:text-7xl") {
                          "Someone"
                          span(class = "block text-gray-500") { "Software Engineer" }
                      }
                      div(class = "flex mt-12 space-x-6 text-xs text-gray-500 uppercase") {
                          a(href = "#_", class = "duration-200 hover:text-gray-400") { "Twitter" }
                          a(href = "#_", class = "duration-200 hover:text-gray-400") { "Linkedin" }
                          a(href = "#_", class = "duration-200 hover:text-gray-400") { "GitHub" }
                          a(href = "#_", class = "duration-200 hover:text-gray-400") { "PostsCV" }
                      }
                  }
              }
          }
      }
      section {
          div(class = "max-w-5xl px-8 py-12 mx-auto 2xl:max-w-7xl md:px-lg:px-24") {
              div(class = "grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 lg:gap-12") {
                  div(class = "flex flex-col gap-12") {
                      div {
                          h2(class = "text-2xl font-bold tracking-tight text-gray-900") { "Contact" }
                          dl(class = "mt-4 space-y-4 text-sm text-gray-600") {
                              div(class = "flex items-center text-sm text-gray-500 gap-x-4") {
                                  dt(class = "flex-none") {
                                      span(class = "sr-only") { "Website" }
                                      svg(xmlns = "http://www.w3.org/2000/svg", class = "icon icon-tabler icon-tabler-world", width = "18", height = "18", viewBox = "0 0 24 24", stroke_width = "2", stroke = "currentColor", fill = "none", stroke_linecap = "round", stroke_linejoin = "round") {
                                          path(stroke = "none", d = "M0 0h24v24H0z", fill = "none")
                                          path(d = "M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0")
                                          path(d = "M3.6 9h16.8")
                                          path(d = "M3.6 15h16.8")
                                          path(d = "M11.5 3a17 17 0 0 0 0 18")
                                          path(d = "M12.5 3a17 17 0 0 1 0 18")
                                      }
                                  }
                                  dd {
                                      a(class = "hover:text-gray-900", href = "#_") { "localhost" }
                                  }
                              }
                              div(class = "flex items-center text-sm text-gray-500 gap-x-4") {
                                  dt(class = "flex-none") {
                                      span(class = "sr-only") { "Telephone" }
                                      svg(xmlns = "http://www.w3.org/2000/svg", class = "icon icon-tabler icon-tabler-device-mobile", width = "18", height = "18", viewBox = "0 0 24 24", stroke_width = "2", stroke = "currentColor", fill = "none", stroke_linecap = "round", stroke_linejoin = "round") {
                                          path(stroke = "none", d = "M0 0h24v24H0z", fill = "none")
                                          path(d = "M6 5a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-14z")
                                          path(d = "M11 4h2")
                                          path(d = "M12 17v.01")
                                      }
                                  }
                                  dd {
                                      a(class = "hover:text-gray-900", href = "tel:+1 (555) 000-0000") { "+1 (555) 000-0000" }
                                  }
                              }
                              div(class = "flex items-center text-sm text-gray-500 gap-x-4") {
                                  dt(class = "flex-none") {
                                      span(class = "sr-only") { "Email" }
                                      svg(xmlns = "http://www.w3.org/2000/svg", class = "icon icon-tabler icon-tabler-mail", width = "18", height = "18", viewBox = "0 0 24 24", stroke_width = "2", stroke = "currentColor", fill = "none", stroke_linecap = "round", stroke_linejoin = "round") {
                                          path(stroke = "none", d = "M0 0h24v24H0z", fill = "none")
                                          path(d = "M3 7a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v10a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-10z")
                                          path(d = "M3 7l9 6l9 -6")
                                      }
                                  }
                                  dd {
                                      a(class = "hover:text-gray-900", href = "mailto:hello@example.com") { "hello@example.com" }
                                  }
                              }
                          }
                      }
                      div {
                          h2(class = "text-2xl font-bold tracking-tight text-gray-900") { "Education" }
                          dl(class = "mt-4 space-y-4 text-sm text-gray-600") {
                              div(class = "flex flex-col gap-2") {
                                  dt(class = "flex flex-col") {
                                      span(class = "block") { "Master Computer Engineering" }
                                      span(class = "block") { "LSBLK" }
                                  }
                                  dd(class = "font-medium text-blue-500") { "2022 - 2023" }
                              }
                              div(class = "flex flex-col gap-2") {
                                  dt(class = "flex flex-col") {
                                      span(class = "block") { "Big Computer University" }
                                      span(class = "block") { "LSBLK" }
                                  }
                                  dd(class = "font-medium text-blue-500") { "2022 - 2023" }
                              }
                          }
                      }
                      div {
                          h2(class = "text-2xl font-bold tracking-tight text-gray-900") { "Expertise" }
                          ul(role = "list", class = "mt-4 space-y-2 text-sm text-gray-600") {
                              li { "Scala" }
                              li { "Rust" }
                          }
                      }
                      div {
                          h2(class = "text-2xl font-bold tracking-tight text-gray-900") { "Skills" }
                          ul(role = "list", class = "mt-4 space-y-2 text-sm text-gray-600") {
                              li { "Positive attribute" }
                              li { "Positive attribute" }
                              li { "Dubious attribute" }
                          }
                      }
                  }
                  div(class = "flex flex-col gap-12 lg:col-span-2") {
                      div {
                          h2(class = "text-2xl font-bold tracking-tight text-gray-900") { "About" }
                          p(class = "mt-4 text-sm text-gray-600") {
                              "Hello, I'm Someone, that does something."
                          }
                      }
                      div {
                          h2(class = "text-2xl font-bold tracking-tight text-gray-900") { "Work experience" }
                          div(class = "flex flex-col gap-8") {
                              div {
                                  p(class = "mt-4 text-lg font-medium text-blue-500") {
                                      "XYZ Tech Solutions -"
                                      span(class = "text-gray-500") { "Software Engineer" }
                                      span(class = "block text-xs text-gray-500") { "January 2019 - Present" }
                                  }
                                  p(class = "mt-4 text-sm font-medium text-gray-500") {
                                      "Developed a lot of tech!"
                                  }
                              }
                              div {
                                  p(class = "mt-4 text-lg font-medium text-blue-500") {
                                      "ABC Creative Agency -"
                                      span(class = "text-gray-500") { "Front-End Developer Intern" }
                                      span(class = "block text-xs text-gray-500") { "June 2018 - December 2018" }
                                  }
                                  p(class = "mt-4 text-sm font-medium text-gray-500") {
                                      "Built 3 new Javascript frameworks, wow."
                                  }
                              }
                              div {
                                  p(class = "mt-4 text-lg font-medium text-blue-500") {
                                      "Digital Innovations -"
                                      span(class = "text-gray-500") { "Software Engineer" }
                                      span(class = "block text-xs text-gray-500") { "March 2020 - March 2022" }
                                  }
                                  p(class = "mt-4 text-sm font-medium text-gray-500") {
                                      "Wrote a lot of unecessary reports."
                                  }
                              }
                          }
                      }
                  }
              }
          }
      }
      footer {
          div(class = "max-w-5xl px-8 py-12 mx-auto 2xl:max-w-7xl md:px-lg:px-24 lg:pt-48") {
              p(class = "max-w-2xl text-sm text-left text-gray-500") {
                  "© Localhost Design:"
                  a(href = "#_/", class = "underline underline-offset-2") { "Localhost Localhost" }
                  ". Demo Images: Respective owners."
              }
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
  Template::build("cv").view(cv_page).head(head).build()
}
