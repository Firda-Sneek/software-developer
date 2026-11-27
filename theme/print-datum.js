// Zet de printdatum in de footer op het moment van printen (Europe/Amsterdam).
// De QR en de canonieke URL worden bij de build al in de HTML gezet.
(function () {
  function zetDatum() {
    var el = document.getElementById("firda-print-datum");
    if (!el) return;
    el.textContent = new Intl.DateTimeFormat("nl-NL", {
      dateStyle: "long",
      timeZone: "Europe/Amsterdam",
    }).format(new Date());
  }

  window.addEventListener("beforeprint", zetDatum);
  if (window.matchMedia) {
    window.matchMedia("print").addEventListener("change", function (e) {
      if (e.matches) zetDatum();
    });
  }
})();
