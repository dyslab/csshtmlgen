pub mod simplecss_template;
pub mod bulma_template;
pub mod tailwind_template;
pub mod bootstrap_template;

#[derive(Copy, Clone, PartialEq)]
pub enum CssFramework {
  Bulma,
  Tailwind,
  Bootstrap,
  SimpleCss,
  None
}

impl CssFramework {
  pub fn get_name(&self) -> String {
    match self {
      CssFramework::Bulma => String::from("Bulma"),
      CssFramework::Tailwind => String::from("Tailwind"),
      CssFramework::Bootstrap => String::from("Bootstrap"),
      CssFramework::SimpleCss => String::from("SimpleCss"),
      CssFramework::None => String::from("None"),
    }
  }
  pub fn construct_html(&self) -> String {
    match self {
      CssFramework::Bulma => bulma_template::template(),
      CssFramework::Tailwind => tailwind_template::template(),
      CssFramework::Bootstrap => bootstrap_template::template(),
      CssFramework::SimpleCss => simplecss_template::template(),
      CssFramework::None => String::from("No framework applied.")
    }
  }
}


