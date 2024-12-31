use std::io::Write;

pub mod html_with_css;

fn main() {
  let mut _css = html_with_css::CssFramework::None;

  // prompt user to select a css framework
  let mut _input = String::new();
  while _css == html_with_css::CssFramework::None {
    println!("Available CSS frameworks:");
    println!("1. Bulma (default)");
    println!("2. Tailwind");
    println!("3. Bootstrap");
    println!("4. SimpleCss");
    print!("Select [1/2/3/4] > ");
    std::io::stdout().flush().unwrap_or_default();
    _input.clear();
    match std::io::stdin().read_line(&mut _input) {
      Ok(_) => {
        match _input.trim() {
          "" => _css = html_with_css::CssFramework::Bulma, // default
          "1" => _css = html_with_css::CssFramework::Bulma,
          "2" => _css = html_with_css::CssFramework::Tailwind,
          "3" => _css = html_with_css::CssFramework::Bootstrap,
          "4" => _css = html_with_css::CssFramework::SimpleCss,
          _ => _css = html_with_css::CssFramework::None,
        }
      },
      Err(e) => eprintln!("[ERROR] {}", e),
    }
  }
  let _selected_css_framework_name = _css.get_name();
  println!("Selected CSS Framwork: {}", _selected_css_framework_name);

  let _save_file_name = format!("boilerplate_{}.html", _selected_css_framework_name);
  print!("Save to: {}", _save_file_name);
  std::io::stdout().flush().unwrap_or_default();
  match std::fs::write(_save_file_name, _css.construct_html()) {
    Ok(_) => println!("\t[OK]"),
    Err(e) => eprintln!("[ERROR] {}", e),
  }
}
