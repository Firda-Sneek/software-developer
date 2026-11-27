#!/usr/bin/env rust-script
//! Puts a print footer with the canonical URL and a QR code in every mdBook page.
//!
//! Runs after `mdbook build` (see `mise run qr`). Idempotent: an existing footer is
//! replaced, so a rebuild keeps clean URLs. No external QR API and no CDN: the QR is
//! drawn as inline SVG. The footer is only visible when printing (`theme/css/print.css`).
//!
//! Run from the repository root: `rust-script scripts/print-qr.rs`
//!
//! ```cargo
//! [dependencies]
//! qrcode = { version = "0.14", default-features = false }
//! ```

use qrcode::{Color, EcLevel, QrCode};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SITE: &str = "https://firda-sneek.github.io/software-developer/";
const FOOTER_START: &str = "<footer id=\"firda-print-footer\"";
const FOOTER_END: &str = "</footer>";
/// Quiet zone around the QR, in modules (the footer is small).
const BORDER: usize = 1;
/// Pages that are not real content pages.
const SKIP_FILES: [&str; 2] = ["toc.html", "print.html"];

fn canonical_url(book: &Path, page: &Path) -> String {
    let rel = page
        .strip_prefix(book)
        .unwrap_or(page)
        .to_string_lossy()
        .replace('\\', "/");
    if rel == "index.html" {
        SITE.to_string()
    } else {
        format!("{SITE}{rel}")
    }
}

/// Compact inline SVG: one path, black modules, white background.
fn qr_svg(url: &str) -> Result<String, String> {
    let code = QrCode::with_error_correction_level(url.as_bytes(), EcLevel::M)
        .map_err(|e| format!("QR maken mislukt voor {url}: {e}"))?;
    let n = code.width();
    let size = n + 2 * BORDER;
    let mut path = String::new();
    for (i, color) in code.to_colors().into_iter().enumerate() {
        if color == Color::Dark {
            let (x, y) = (i % n + BORDER, i / n + BORDER);
            path.push_str(&format!("M{x} {y}h1v1h-1z"));
        }
    }
    Ok(format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {size} {size}\" shape-rendering=\"crispEdges\">\
<rect width=\"{size}\" height=\"{size}\" fill=\"#fff\"/><path d=\"{path}\" fill=\"#000\"/></svg>"
    ))
}

fn footer(url: &str, svg: &str) -> String {
    format!(
        "<footer id=\"firda-print-footer\" hidden>\n  <div class=\"firda-print-inner\">\n    <div class=\"firda-print-qr\" aria-hidden=\"true\">{svg}</div>\n    <p>Nieuwste versie: <span class=\"firda-print-url\">{url}</span> — gedrukt op <span id=\"firda-print-datum\">…</span>. Controleer online of deze pagina is bijgewerkt.</p>\n  </div>\n</footer>\n"
    )
}

/// Returns the page with exactly one footer, just before `</body>`.
fn add_footer(html: &str, url: &str) -> Result<String, String> {
    let mut html = html.to_string();
    if let Some(start) = html.find(FOOTER_START) {
        let end = html[start..]
            .find(FOOTER_END)
            .map(|e| start + e + FOOTER_END.len())
            .ok_or("bestaande printfooter is onvolledig")?;
        let rest = html[end..].trim_start_matches('\n').to_string();
        html = format!("{}{}", &html[..start], rest);
    }
    if !html.contains("</body>") {
        return Err("geen </body> gevonden".into());
    }
    Ok(html.replacen("</body>", &format!("{}</body>", footer(url, &qr_svg(url)?)), 1))
}

fn collect_pages(dir: &Path, book: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            // The Slidev decks live in book/slides/ and get no footer.
            if path != book.join("slides") {
                collect_pages(&path, book, out);
            }
        } else if path.extension().is_some_and(|e| e == "html")
            && !SKIP_FILES.iter().any(|s| path.file_name().is_some_and(|n| n == *s))
        {
            out.push(path);
        }
    }
}

fn run() -> Result<(), String> {
    let book = std::env::current_dir().map_err(|e| e.to_string())?.join("book");
    if !book.is_dir() {
        return Err(format!("{} ontbreekt. Draai eerst: mdbook build", book.display()));
    }
    let mut pages = Vec::new();
    collect_pages(&book, &book, &mut pages);
    if pages.is_empty() {
        return Err("geen HTML-pagina's in book/".into());
    }
    for page in &pages {
        let html = fs::read_to_string(page).map_err(|e| format!("{}: {e}", page.display()))?;
        let url = canonical_url(&book, page);
        let new = add_footer(&html, &url).map_err(|e| format!("{}: {e}", page.display()))?;
        fs::write(page, new).map_err(|e| format!("{}: {e}", page.display()))?;
    }
    println!("Klaar: printfooter in {} pagina's.", pages.len());
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("print-qr.rs: FOUT: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_urls() {
        let book = Path::new("book");
        assert_eq!(canonical_url(book, &book.join("index.html")), SITE);
        assert_eq!(
            canonical_url(book, &book.join("jaren/sd1.html")),
            format!("{SITE}jaren/sd1.html")
        );
    }

    #[test]
    fn footer_is_added_once_and_replaced_on_rebuild() {
        let html = "<html><body><p>x</p></body></html>";
        let once = add_footer(html, "https://example.invalid/a.html").unwrap();
        assert_eq!(once.matches(FOOTER_START).count(), 1);
        assert!(once.contains("https://example.invalid/a.html"));
        let twice = add_footer(&once, "https://example.invalid/b.html").unwrap();
        assert_eq!(twice.matches(FOOTER_START).count(), 1);
        assert!(twice.contains("b.html") && !twice.contains("a.html"));
        assert!(twice.ends_with("</body></html>"));
    }

    #[test]
    fn qr_svg_has_modules() {
        let svg = qr_svg(SITE).unwrap();
        assert!(svg.starts_with("<svg") && svg.contains("<path d=\"M"));
    }

    #[test]
    fn page_without_body_is_an_error() {
        assert!(add_footer("<html></html>", SITE).is_err());
    }
}
