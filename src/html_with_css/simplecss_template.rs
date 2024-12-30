pub fn template() -> String {
  String::from(r#"<!DOCTYPE html>
<html>
  <head>
    <title>
      Simple.css Demo Page
    </title>
    <link rel="stylesheet" href="https://cdn.simplecss.org/simple.min.css">
  </head>
  <body>
    <h1>Hello, Simple.css!</h1>
    <p>This is a simple page for <a href="https://simplecss.org/">Simple.css</a> demonstration.</p>
    <p>
      <a class="button" href="https://simplecss.org/demo">I'm a button with a link to Simple.css demo</a>
    </p>
    <p>
      <mark>Enjoy it!</mark>
    </p>
    <p class="notice">This is a notice box. It's useful for calling out snippets of information. Cool, huh?</p>
  </body>
</html>
"#)
}