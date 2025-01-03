pub fn template() -> String {
  String::from(r###"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Tailwind CSS Demo Page</title>
    <link rel="apple-touch-icon" sizes="180x180" href="https://tailwindcss.com/favicons/apple-touch-icon.png?v=3"/>
    <link rel="icon" type="image/png" sizes="32x32" href="https://tailwindcss.com/favicons/favicon-32x32.png?v=3"/>
    <link rel="icon" type="image/png" sizes="16x16" href="https://tailwindcss.com/favicons/favicon-16x16.png?v=3"/>
    <link rel="manifest" href="https://tailwindcss.com/favicons/site.webmanifest?v=3"/>
    <link rel="mask-icon" href="https://tailwindcss.com/favicons/safari-pinned-tab.svg?v=3" color="#38bdf8"/>
    <link rel="shortcut icon" href="https://tailwindcss.com/favicons/favicon.ico?v=3"/>
    <script src="https://cdn.tailwindcss.com"></script>
  </head>
  <body>
    <!-- header -->
    <div class="bg-gray-700 text-white text-center py-4">
      <h1 class="text-3xl text-white-700 font-bold my-6">
        Hello, <span class="text-teal-300">Tailwind CSS</span>
      </h1>
    </div>
    <!-- main content -->
    <div class="container mx-auto px-4">
      <div class="my-10 text-lg">
        <p>This is a simple page for <a class="text-fuchsia-500 hover:underline underline-offset-4" href="https://tailwindcss.com/">Tailwind CSS</a> demonstration.</p>
      </div>
      <div class="my-10 text-lg">
        <p class="font-mono text-4xl antialiased font-bold text-yellow-700">Enjoy it!</p>
      </div>
      <div class="grid gap-4 grid-cols-2 text-lg mb-10">
        <div class="bg-orange-100 p-6 text-center shadow hover:shadow-lg">grid 01</div>
        <div class="bg-red-100 p-6 text-center shadow hover:shadow-lg">grid 02</div>
        <div class="bg-teal-100 p-6 text-center shadow hover:shadow-lg">grid 03</div>
        <div class="bg-lime-100 p-6 text-center shadow hover:shadow-lg">grid 04</div>
      </div>
      <div class="text-center px-8 py-8 mt-10 mb-10 sm:mt-16 md:mt-20">
        <h2 class="text-slate-900 text-4xl tracking-tight font-extrabold sm:text-5xl dark:text-white">
          “Best practices” don’t actually work.
        </h2>
        <figure>
          <blockquote>
            <p class="mt-6 max-w-3xl mx-auto text-lg">
              I’ve written <a href="https://adamwathan.me/css-utility-classes-and-separation-of-concerns/" class="text-sky-500 font-semibold dark:text-sky-400">a few thousand words</a> on why traditional “semantic class names” are the reason CSS is hard to maintain, but the truth is you’re never going to believe me until you actually try it. If you can suppress the urge to retch long enough to give it a chance, I really think you’ll wonder how you ever worked with CSS any other way.
            </p>
          </blockquote>
          <figcaption class="mt-6 flex items-center justify-center space-x-4 text-left">
            <img src="https://tailwindcss.com/_next/static/media/adam.f69b0b90.jpg" alt="" class="w-14 h-14 rounded-full" loading="lazy" decoding="async">
            <div>
              <div class="text-slate-900 font-semibold dark:text-white">Adam Wathan</div>
              <div class="mt-0.5 text-sm leading-6">Creator of Tailwind CSS</div>
            </div>
          </figcaption>
        </figure>
      </div>
    </div>
    <!-- footer -->
    <div class="text-center py-6 bg-gray-100 text-gray-400">
      <p class="text-lg">&copy;2024 Get started with Tailwind CSS.</p>
    </div>
  </body>
</html>
"###)
}
