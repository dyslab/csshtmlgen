pub fn template() -> String {
  String::from(r###"<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Bootstrap Demo Page</title>
    <!-- Bootstrap CSS -->
    <link href="https://cdn.bootcdn.net/ajax/libs/twitter-bootstrap/5.3.3/css/bootstrap.min.css" rel="stylesheet">
    <!-- Alternatively, Official CDN link
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-QWTKZyjpPEjISv5WaRU9OFeRpok6YctnYmDr5pNlyT2bRjXh0JMhjY6hW+ALEwIH" crossorigin="anonymous">
    -->
    <!-- Favicons -->
    <link rel="apple-touch-icon" href="https://getbootstrap.com/docs/5.3/assets/img/favicons/apple-touch-icon.png" sizes="180x180">
    <link rel="icon" href="https://getbootstrap.com/docs/5.3/assets/img/favicons/favicon-32x32.png" sizes="32x32" type="image/png">
    <link rel="icon" href="https://getbootstrap.com/docs/5.3/assets/img/favicons/favicon-16x16.png" sizes="16x16" type="image/png">
    <link rel="manifest" href="https://getbootstrap.com/docs/5.3/assets/img/favicons/manifest.json">
    <link rel="mask-icon" href="https://getbootstrap.com/docs/5.3/assets/img/favicons/safari-pinned-tab.svg" color="#712cf9">
    <link rel="icon" href="https://getbootstrap.com/docs/5.3/assets/img/favicons/favicon.ico">
    <meta name="theme-color" content="#712cf9">
  </head>
  <body>
    <div id="carouselExample" class="carousel slide" data-bs-ride="carousel">
      <div class="carousel-inner">
        <div class="carousel-item active" data-bs-interval="10000">
          <!--img src="https://picsum.photos/seed/picsum/500/100?grayscale" class="d-block w-100" alt="..."-->
          <svg class="bd-placeholder-img bd-placeholder-img-lg img-fluid" width="100%" height="250" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Placeholder: Responsive image" preserveAspectRatio="xMidYMid slice" focusable="false"><title>Placeholder</title><rect width="100%" height="100%" fill="#868eb6"></rect></svg>
          <div class="carousel-caption d-none d-md-block text-primary-emphasis">
            <h1 class="text-uppercase">Hello, Bootstrap!</h1>
            <h5 class="text-capitalize">First slide label</h5>
            <p>Some representative placeholder content for the first slide.</p>
          </div>
        </div>
        <div class="carousel-item">
          <svg class="bd-placeholder-img bd-placeholder-img-lg img-fluid" width="100%" height="250" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Placeholder: Responsive image" preserveAspectRatio="xMidYMid slice" focusable="false"><title>Placeholder</title><rect width="100%" height="100%" fill="#86ae96"></rect></svg>
          <div class="carousel-caption d-none d-md-block text-info-emphasis">
            <h1 class="text-uppercase">Come on! We are getting stronger.</h1>
            <h5 class="text-capitalize">Second slide label</h5>
            <p>Some representative placeholder content for the second slide.</p>
          </div>
        </div>
        <div class="carousel-item">
          <svg class="bd-placeholder-img bd-placeholder-img-lg img-fluid" width="100%" height="250" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Placeholder: Responsive image" preserveAspectRatio="xMidYMid slice" focusable="false"><title>Placeholder</title><rect width="100%" height="100%" fill="#a68e96"></rect></svg>
          <div class="carousel-caption d-none d-md-block text-danger-emphasis">
            <h1 class="text-uppercase">Fight! Fight! Fight!</h1>
            <h5 class="text-capitalize">Third slide label</h5>
            <p>Some representative placeholder content for the third slide.</p>
          </div>
        </div>
      </div>
      <button class="carousel-control-prev" type="button" data-bs-target="#carouselExample" data-bs-slide="prev">
        <span class="carousel-control-prev-icon" aria-hidden="true"></span>
        <span class="visually-hidden">Previous</span>
      </button>
      <button class="carousel-control-next" type="button" data-bs-target="#carouselExample" data-bs-slide="next">
        <span class="carousel-control-next-icon" aria-hidden="true"></span>
        <span class="visually-hidden">Next</span>
      </button>
    </div>
    <div class="container mx-auto py-5 fs-5">
      <p class="text-center mb-5">
        This is a simple page for <a class="link-offset-4 link-underline-opacity-25 link-underline-opacity-100-hover" href="https://getbootstrap.com/" target="_blank">Bootstrap</a> demonstration.
        <span class="badge text-white text-bg-info">Enjoy it!</span>
      </p>
      <p class="mb-5">
        <a class="btn btn-success fs-4" href="https://getbootstrap.com/docs/">I'm a button with a link to Bootstrap docs</a>
      </p>
      <div class="alert alert-success mb-5" role="alert">
        <h4 class="alert-heading fw-semibold">Well done!</h4>
        <p>Aww yeah, you successfully read this important alert message. This example text is going to run a bit longer so that you can see how spacing within an alert works with this kind of content.</p>
        <hr>
        <p class="mb-0">Whenever you need to, be sure to use margin utilities to keep things nice and tidy.</p>
      </div>
    </div>
    <footer class="footer mt-auto py-4 bg-light text-end">
      <div class="container">
        <span class="text-muted">©2024 Bootstrap Demo Page</span>
      </div>
    </footer>
    <!-- Bootstrap JS -->
    <script src="https://cdn.bootcdn.net/ajax/libs/twitter-bootstrap/5.3.3/js/bootstrap.bundle.min.js"></script>
    <!-- Alternatively, Official CDN link
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js" integrity="sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz" crossorigin="anonymous"></script>
    -->
  </body>
</html>
"###)
}
