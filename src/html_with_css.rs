pub mod simplecss_template;
pub mod bulma_template;

#[derive(Copy, Clone, PartialEq)]
pub enum CssFramework {
  Bulma,
  SimpleCss,
  // SemanticUi,
  // Tailwind,
  // Bootstrap,
  // Materialize,
  None
}

pub struct Html {
  pub head: String,
  pub body: String,
}

pub fn get_css_framework_name(css_framework: CssFramework) -> String {
  match css_framework {
    CssFramework::Bulma => String::from("Bulma"),
    CssFramework::SimpleCss => String::from("SimpleCss"),
    CssFramework::None => String::from("None")
  }
}

pub fn construct_html(css_framework: CssFramework) -> String {
  match css_framework {
    CssFramework::Bulma => bulma_template::template(),
    CssFramework::SimpleCss => simplecss_template::template(),
    CssFramework::None => String::from("No framework applied.")
  }
}
