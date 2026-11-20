// Klein Firda-laagje bovenop de standaard mdBook-chrome:
// logo boven in de sidebar (zonder driehoek) en leesbare themanamen.
(function () {
  function padNaarWortel() {
    var script = document.querySelector('script[src*="book-"]');
    var src = script && script.getAttribute("src");
    if (!src || src.indexOf("/") === -1) return "";
    return src.slice(0, src.lastIndexOf("/") + 1);
  }

  function zetLogo() {
    var sidebar = document.getElementById("mdbook-sidebar");
    if (!sidebar || sidebar.querySelector(".firda-logo")) return;
    var wortel = padNaarWortel();
    var koppeling = document.createElement("a");
    koppeling.className = "firda-logo";
    koppeling.href = wortel + "index.html";
    var img = document.createElement("img");
    img.src = wortel + "assets/firda-logo.svg";
    img.alt = "Firda, naar de startpagina";
    koppeling.appendChild(img);
    sidebar.insertBefore(koppeling, sidebar.firstChild);
  }

  function hernoemThemas() {
    var licht = document.getElementById("mdbook-theme-light");
    var donker = document.getElementById("mdbook-theme-navy");
    if (licht) licht.textContent = "Firda licht";
    if (donker) donker.textContent = "Firda donker";
  }

  zetLogo();
  hernoemThemas();
  document.addEventListener("DOMContentLoaded", function () {
    zetLogo();
    hernoemThemas();
  });
})();
