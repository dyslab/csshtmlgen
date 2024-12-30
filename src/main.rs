use std::io::Write;

pub mod html_with_css;

fn main() {
  let mut _css = html_with_css::CssFramework::None;

  // prompt user to select a css framework
  let mut _input = String::new();
  while _css == html_with_css::CssFramework::None {
    println!("Select CSS framework:");
    println!("  1. Bulma (default)");
    println!("  2. SimpleCss");
    print!("  > ");
    std::io::stdout().flush().unwrap_or_default();
    _input.clear();
    match std::io::stdin().read_line(&mut _input) {
      Ok(_) => {
        match _input.trim() {
          "" => _css = html_with_css::CssFramework::Bulma, // default
          "1" => _css = html_with_css::CssFramework::Bulma,
          "2" => _css = html_with_css::CssFramework::SimpleCss,
          _ => _css = html_with_css::CssFramework::None,
        }
      },
      Err(e) => eprintln!("[ERROR] {}", e),
    }
  }
  let _selected_css_framework_name = html_with_css::get_css_framework_name(_css);
  println!("Selected CSS Framwork: {}", _selected_css_framework_name);

  let _save_file_name = format!("boilerplate_{}.html", _selected_css_framework_name);
  print!("Save to: {}", _save_file_name);
  std::io::stdout().flush().unwrap_or_default();
  match std::fs::write(_save_file_name, html_with_css::construct_html(_css)) {
    Ok(_) => println!("\t[OK]"),
    Err(e) => eprintln!("[ERROR] {}", e),
  }
}
