pub fn template() -> String {
  String::from(r###"<!DOCTYPE html>
<html>
  <head>
    <title>
      Bulma Demo Page
    </title>
    <link rel="stylesheet" href="https://cdn.bootcdn.net/ajax/libs/bulma/1.0.1/css/bulma.min.css">
    <!--
      alternative CDN links: 
        - https://cdn.jsdelivr.net/npm/bulma@1.0.2/css/bulma.min.css
        - https://cdnjs.cloudflare.com/ajax/libs/bulma/1.0.3/css/bulma.min.css
    -->
    <!-- Favicons -->
    <link rel="apple-touch-icon" sizes="180x180" href="https://bulma.io/favicons/apple-touch-icon.png?v=201701041855">
    <link rel="icon" type="image/png" href="https://bulma.io/favicons/favicon-32x32.png?v=201701041855" sizes="32x32">
    <link rel="icon" type="image/png" href="https://bulma.io/favicons/favicon-16x16.png?v=201701041855" sizes="16x16">
    <!-- link rel="manifest" href="https://bulma.io/favicons/manifest.json?v=201701041855" -->
    <link rel="mask-icon" href="https://bulma.io/favicons/safari-pinned-tab.svg?v=201701041855" color="#00d1b2">
    <link rel="shortcut icon" href="https://bulma.io/favicons/favicon.ico?v=201701041855">
    <meta name="msapplication-config" content="https://bulma.io/favicons/browserconfig.xml?v=201701041855">
    <meta name="theme-color" content="#00d1b2">
  </head>
  <body>
    <section class="hero is-primary">
      <div class="hero-body">
        <p class="title">Hello, Bulma!</p>
        <p class="subtitle">Hero subtitle</p>
      </div>
    </section>
    <section>
      <div class="container is-fluid">
        <p class="is-size-3 mt-6 mb-3">
          This is a simple page for <a href="https://bulma.io/">Bulma</a> demonstration.
        </p>
        <p class="is-size-2 is-family-sans-serif mt-3 mb-6 has-text-warning has-text-weight-bold">Enjoy it!</p>
        <!-- Notification message below -->
        <article class="message is-info mb-6">
          <div class="message-header">
            <p>Message (info style)</p>
            <button class="delete" aria-label="delete"></button>
          </div>
          <div class="message-body">
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
            <strong>Pellentesque risus mi</strong>
            , tempus quis placerat ut, porta necnulla. Vestibulum rhoncus ac
            ex sit amet fringilla. Nullam gravida purus diam, et dictum
            <a>felis venenatis</a>
            efficitur. Aenean ac <em>\neleifend lacus\n</em>
            , in mollis lectus. Donec sodales, arcu etsollicitudin porttitor,
            tortor urna tempor ligula, id porttitor mi magna a neque. Donec 
            dui urna, vehicula et sem eget, facilisis sodales sem.
          </div>
        </article>
      </div>
    </section>
    <footer class="footer">
      <div class="content has-text-centered">
        <strong>Bulma</strong> by <a href="https://jgthms.com">Jeremy Thomas</a>. 
        The source code is licensed <a href="https://opensource.org/license/mit">MIT</a>. 
        The website content is licensed 
        <a href="https://creativecommons.org/licenses/by-nc-sa/4.0/">CC BY NC SA 4.0</a>.
      </div>
    </footer>
  </body>
</html>
"###)
}
