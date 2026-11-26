#!/usr/bin/env rust-script
//! Shared code regions for the student site (mdBook) and the decks (Slidev).
//!
//! Example code lives in real source files under `voorbeelden/` and marks the
//! parts that pages and slides may show with `#region name` / `#endregion name`.
//!
//! Subcommands (run from the repository root, via `mise run ...`):
//!
//! - `preprocess`: mdBook preprocessor. Replaces `{{#include path:name}}` with
//!   the lines of region `name`, without marker lines. Plain mdBook includes
//!   with line numbers (`path:2:10`) are left to mdBook itself.
//! - `check`: verifies that every region reference in `src/` and `slides/`
//!   points to an existing file and region, and that all markers are well formed.
//! - `--help`: short usage.
//!
//! Both mdBook and Slidev fail silently on a missing region (empty block or the
//! whole file with marker lines), which is why `check` is part of the build,
//! the CI and the pre-commit hook.
//!
//! ```cargo
//! [dependencies]
//! serde_json = "1"
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;

// ---------------------------------------------------------------------------
// Markers
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
enum Marker {
    Start(String),
    /// The name may be empty when the author left it out (the check rejects that).
    End(String),
}

/// Comment styles that Slidev understands: (opener, closer, `#` before the keyword is required).
const STYLES: [(&str, &str, bool); 5] = [
    ("//", "", false),
    ("#", "", false),
    ("--", "", true),
    ("/*", "*/", true),
    ("<!--", "-->", true),
];

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Recognises `// #region name`, `# region name`, `/* #region name */` and friends.
fn parse_marker(line: &str) -> Option<Marker> {
    let line = line.trim();
    for (open, close, hash_required) in STYLES {
        let Some(rest) = line.strip_prefix(open) else {
            continue;
        };
        let rest = rest.trim_start();
        let rest = match rest.strip_prefix('#') {
            Some(r) => r,
            None if hash_required => continue,
            None => rest,
        };
        let (is_end, rest) = if let Some(r) = rest.strip_prefix("endregion") {
            (true, r)
        } else if let Some(r) = rest.strip_prefix("region") {
            (false, r)
        } else {
            continue;
        };
        if rest.chars().next().is_some_and(is_word_char) {
            continue;
        }
        let rest = if close.is_empty() {
            rest
        } else {
            match rest.trim_end().strip_suffix(close) {
                Some(r) => r,
                None => continue,
            }
        };
        let name = rest.trim().to_string();
        return Some(if is_end {
            Marker::End(name)
        } else {
            Marker::Start(name)
        });
    }
    None
}

fn is_valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// A region as 0-based line numbers of its start and end marker.
#[derive(Debug, Clone)]
struct Region {
    name: String,
    start: usize,
    end: usize,
}

/// Finds all regions in a file. Problems come back as (1-based line, message).
fn scan(text: &str) -> (Vec<Region>, Vec<(usize, String)>) {
    let mut regions = Vec::new();
    let mut problems = Vec::new();
    let mut stack: Vec<(String, usize)> = Vec::new();
    let mut seen = BTreeSet::new();

    for (i, line) in text.lines().enumerate() {
        match parse_marker(line) {
            None => {}
            Some(Marker::Start(name)) => {
                if !is_valid_name(&name) {
                    problems.push((
                        i + 1,
                        format!(
                            "ongeldige regionaam '{name}' (gebruik letters, cijfers, - of _, beginnend met een letter)"
                        ),
                    ));
                } else if !seen.insert(name.clone()) {
                    problems.push((i + 1, format!("regionaam '{name}' komt twee keer voor")));
                }
                stack.push((name, i));
            }
            Some(Marker::End(name)) => match stack.pop() {
                None => problems.push((i + 1, "#endregion zonder bijbehorende #region".into())),
                Some((open, start)) => {
                    if name.is_empty() {
                        problems.push((
                            i + 1,
                            format!("noem de regionaam bij het einde: '#endregion {open}'"),
                        ));
                    } else if name != open {
                        problems.push((
                            i + 1,
                            format!("'#endregion {name}' sluit niet de open region '{open}'"),
                        ));
                    }
                    regions.push(Region {
                        name: open,
                        start,
                        end: i,
                    });
                }
            },
        }
    }
    for (name, start) in stack {
        problems.push((start + 1, format!("region '{name}' wordt niet afgesloten")));
    }
    (regions, problems)
}

fn dedent(lines: &[&str]) -> Vec<String> {
    let indent = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|l| {
            if l.len() >= indent {
                l[indent..].to_string()
            } else {
                String::new()
            }
        })
        .collect()
}

/// Returns the lines of region `name`, without any marker lines, dedented.
fn extract(text: &str, name: &str) -> Result<String, String> {
    let (regions, problems) = scan(text);
    if let Some((line, msg)) = problems.first() {
        return Err(format!("regel {line}: {msg}"));
    }
    let Some(region) = regions.iter().find(|r| r.name == name) else {
        let known: Vec<&str> = regions.iter().map(|r| r.name.as_str()).collect();
        return Err(format!(
            "region '{name}' bestaat niet (aanwezig: {})",
            if known.is_empty() {
                "geen".to_string()
            } else {
                known.join(", ")
            }
        ));
    };
    let lines: Vec<&str> = text.lines().collect();
    let body: Vec<&str> = lines[region.start + 1..region.end]
        .iter()
        .copied()
        .filter(|l| parse_marker(l).is_none())
        .collect();
    Ok(dedent(&body).join("\n"))
}

// ---------------------------------------------------------------------------
// mdBook includes
// ---------------------------------------------------------------------------

/// One `{{#include ...}}` directive.
#[derive(Debug, PartialEq)]
struct Include {
    /// Byte range of the whole directive, braces included.
    span: (usize, usize),
    /// 1-based line number.
    line: usize,
    path: String,
    /// `Some` when the suffix is a region name, `None` for plain or line-number includes.
    region: Option<String>,
}

fn is_region_suffix(s: &str) -> bool {
    s.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn find_includes(content: &str) -> Vec<Include> {
    const OPEN: &str = "{{#include";
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(rel) = content[from..].find(OPEN) {
        let start = from + rel;
        from = start + OPEN.len();
        if content[..start].ends_with('\\') {
            continue; // escaped, mdBook shows it literally
        }
        let Some(close) = content[from..].find("}}") else {
            break;
        };
        let end = from + close + 2;
        let inner = content[from..from + close].trim();
        let (path, region) = match inner.rsplit_once(':') {
            Some((p, suffix)) if is_region_suffix(suffix) => (p, Some(suffix.to_string())),
            _ => (inner.split(':').next().unwrap_or(inner), None),
        };
        found.push(Include {
            span: (start, end),
            line: content[..start].matches('\n').count() + 1,
            path: path.trim().to_string(),
            region,
        });
        from = end;
    }
    found
}

fn expand_includes(content: &str, base_dir: &Path) -> Result<String, String> {
    let mut out = String::with_capacity(content.len());
    let mut last = 0;
    for inc in find_includes(content) {
        let Some(region) = &inc.region else { continue };
        let file = base_dir.join(&inc.path);
        let text = fs::read_to_string(&file)
            .map_err(|e| format!("regel {}: kan {} niet lezen: {e}", inc.line, file.display()))?;
        let body = extract(&text, region)
            .map_err(|e| format!("regel {}: {}: {e}", inc.line, inc.path))?;
        out.push_str(&content[last..inc.span.0]);
        out.push_str(&body);
        last = inc.span.1;
    }
    out.push_str(&content[last..]);
    Ok(out)
}

// ---------------------------------------------------------------------------
// mdBook preprocessor
// ---------------------------------------------------------------------------

fn expand_chapters(
    value: &mut serde_json::Value,
    src_dir: &Path,
) -> Result<(), String> {
    match value {
        serde_json::Value::Array(items) => {
            for item in items {
                expand_chapters(item, src_dir)?;
            }
        }
        serde_json::Value::Object(map) => {
            if let Some(chapter) = map.get_mut("Chapter") {
                expand_chapter(chapter, src_dir)?;
            } else {
                for v in map.values_mut() {
                    expand_chapters(v, src_dir)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn expand_chapter(chapter: &mut serde_json::Value, src_dir: &Path) -> Result<(), String> {
    let source = chapter
        .get("source_path")
        .and_then(|v| v.as_str())
        .map(PathBuf::from);
    let name = chapter
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("?")
        .to_string();
    if let (Some(source), Some(content)) = (
        source,
        chapter.get("content").and_then(|v| v.as_str()).map(String::from),
    ) {
        let base = src_dir
            .join(&source)
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| src_dir.to_path_buf());
        let expanded = expand_includes(&content, &base)
            .map_err(|e| format!("hoofdstuk '{name}' ({}): {e}", source.display()))?;
        chapter["content"] = serde_json::Value::String(expanded);
    }
    if let Some(sub) = chapter.get_mut("sub_items") {
        expand_chapters(sub, src_dir)?;
    }
    Ok(())
}

fn preprocess(args: &[String]) -> Result<(), String> {
    // mdBook asks `<command> supports <renderer>` first; every renderer is fine.
    if args.first().map(String::as_str) == Some("supports") {
        return Ok(());
    }
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| format!("stdin lezen mislukt: {e}"))?;
    let mut parsed: serde_json::Value =
        serde_json::from_str(&input).map_err(|e| format!("ongeldige JSON van mdBook: {e}"))?;
    let ctx = parsed.get(0).cloned().ok_or("mdBook-context ontbreekt")?;
    let root = ctx.get("root").and_then(|v| v.as_str()).unwrap_or(".");
    let src = ctx
        .pointer("/config/book/src")
        .and_then(|v| v.as_str())
        .unwrap_or("src");
    let src_dir = Path::new(root).join(src);
    let book = parsed.get_mut(1).ok_or("boek ontbreekt in de invoer")?;
    expand_chapters(book, &src_dir)?;
    serde_json::to_writer(io::stdout(), book).map_err(|e| format!("uitvoer schrijven mislukt: {e}"))
}

// ---------------------------------------------------------------------------
// Check
// ---------------------------------------------------------------------------

const SKIP_DIRS: [&str; 6] = [".git", "node_modules", "build", "target", ".gradle", ".idea"];
const CODE_LANGS: [&str; 10] = [
    "kotlin", "kt", "rust", "rs", "python", "py", "java", "js", "javascript", "typescript",
];

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        let name = entry.file_name();
        if path.is_dir() {
            if !SKIP_DIRS.contains(&name.to_string_lossy().as_ref()) {
                walk(&path, out);
            }
        } else {
            out.push(path);
        }
    }
}

fn files_with_ext(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut all = Vec::new();
    walk(dir, &mut all);
    all.retain(|p| p.extension().is_some_and(|e| e == ext));
    all
}

/// Normalises `a/b/../c` without touching the file system.
fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other),
        }
    }
    out
}

#[derive(Default)]
struct Report {
    errors: Vec<String>,
    warnings: Vec<String>,
}

struct Checker<'a> {
    root: &'a Path,
    /// All regions per source file (path relative to the root).
    defined: BTreeMap<PathBuf, BTreeSet<String>>,
    used: BTreeSet<(PathBuf, String)>,
    report: Report,
}

impl Checker<'_> {
    fn rel(&self, p: &Path) -> String {
        p.strip_prefix(self.root).unwrap_or(p).display().to_string()
    }

    fn error(&mut self, file: &Path, line: usize, msg: impl Into<String>) {
        let at = self.rel(file);
        self.report.errors.push(format!("{at}:{line}: {}", msg.into()));
    }

    fn warn(&mut self, file: &Path, line: usize, msg: impl Into<String>) {
        let at = self.rel(file);
        self.report.warnings.push(format!("{at}:{line}: {}", msg.into()));
    }

    /// Scans all example sources for malformed markers and collects the regions.
    fn scan_examples(&mut self) {
        let mut files = Vec::new();
        walk(&self.root.join("voorbeelden"), &mut files);
        for file in files {
            if file.extension().is_some_and(|e| e == "md") {
                continue; // README's may describe the markers in prose
            }
            let Ok(text) = fs::read_to_string(&file) else { continue };
            for (i, line) in text.lines().enumerate() {
                if line.contains("ANCHOR:") || line.contains("ANCHOR_END:") {
                    self.error(
                        &file,
                        i + 1,
                        "ANCHOR-marker gevonden: gebruik '#region naam' en '#endregion naam'",
                    );
                }
            }
            let (regions, problems) = scan(&text);
            for (line, msg) in problems {
                self.error(&file, line, msg);
            }
            let key = file.strip_prefix(self.root).unwrap_or(&file).to_path_buf();
            self.defined
                .insert(key, regions.into_iter().map(|r| r.name).collect());
        }
    }

    /// Checks one reference to `target#region` found in `from` at `line`.
    fn check_reference(&mut self, from: &Path, line: usize, target: &Path, region: Option<&str>) {
        let target = normalize(target);
        if !target.is_file() {
            self.error(from, line, format!("bestand bestaat niet: {}", self.rel(&target)));
            return;
        }
        let key = target.strip_prefix(self.root).unwrap_or(&target).to_path_buf();
        let in_examples = key.starts_with("voorbeelden");
        match region {
            Some(name) => {
                let known = match self.defined.get(&key) {
                    Some(set) => Some(set.clone()),
                    None => fs::read_to_string(&target).ok().map(|t| {
                        scan(&t).0.into_iter().map(|r| r.name).collect::<BTreeSet<_>>()
                    }),
                };
                match known {
                    Some(set) if set.contains(name) => {
                        self.used.insert((key, name.to_string()));
                    }
                    Some(set) => {
                        let list = set.iter().cloned().collect::<Vec<_>>().join(", ");
                        let list = if list.is_empty() { "geen".to_string() } else { list };
                        self.error(
                            from,
                            line,
                            format!(
                                "region '{name}' bestaat niet in {} (aanwezig: {list})",
                                key.display()
                            ),
                        );
                    }
                    None => self.error(from, line, format!("{} is geen tekstbestand", key.display())),
                }
            }
            None if in_examples => self.error(
                from,
                line,
                format!(
                    "{} wordt zonder region gebruikt: markers blijven dan zichtbaar. Voeg ':naam' toe",
                    key.display()
                ),
            ),
            None => {}
        }
    }

    /// Fenced code blocks in Markdown that hold real code instead of an include.
    fn warn_inline_code(&mut self, file: &Path, text: &str) {
        let mut fence: Option<(usize, String, usize)> = None; // (start line, lang, code lines)
        for (i, line) in text.lines().enumerate() {
            let t = line.trim_start();
            if let Some(rest) = t.strip_prefix("```") {
                match fence.take() {
                    None => fence = Some((i + 1, rest.trim().to_lowercase(), 0)),
                    Some((start, lang, lines)) => {
                        if CODE_LANGS.contains(&lang.as_str()) && lines > 0 {
                            self.warn(
                                file,
                                start,
                                format!("code staat inline ({lang}): zet het in voorbeelden/ en include een region"),
                            );
                        }
                    }
                }
            } else if let Some((_, _, lines)) = fence.as_mut() {
                if !t.is_empty() && !t.starts_with("{{#include") {
                    *lines += 1;
                }
            }
        }
    }

    fn check_book(&mut self) {
        for file in files_with_ext(&self.root.join("src"), "md") {
            let Ok(text) = fs::read_to_string(&file) else { continue };
            let dir = file.parent().unwrap_or(self.root).to_path_buf();
            for inc in find_includes(&text) {
                self.check_reference(&file, inc.line, &dir.join(&inc.path), inc.region.as_deref());
            }
            self.warn_inline_code(&file, &text);
        }
    }

    fn check_decks(&mut self) {
        for file in files_with_ext(&self.root.join("slides"), "md") {
            let Ok(text) = fs::read_to_string(&file) else { continue };
            let dir = file.parent().unwrap_or(self.root).to_path_buf();
            let mut in_fence = false;
            for (i, line) in text.lines().enumerate() {
                let t = line.trim_start();
                if t.starts_with("```") {
                    in_fence = !in_fence;
                }
                if in_fence {
                    continue;
                }
                let Some(rest) = t.strip_prefix("<<<") else { continue };
                let token = rest.split_whitespace().next().unwrap_or("");
                if token.is_empty() {
                    self.error(&file, i + 1, "lege '<<<'-import");
                    continue;
                }
                if token.starts_with("@/") {
                    self.error(
                        &file,
                        i + 1,
                        "gebruik een pad relatief aan het deck, niet '@/' (dat wijst naar de deckmap)",
                    );
                    continue;
                }
                let (path, region) = match token.rsplit_once('#') {
                    Some((p, r)) => (p, Some(r)),
                    None => (token, None),
                };
                self.check_reference(&file, i + 1, &dir.join(path), region);
            }
            self.warn_inline_code(&file, &text);
        }
    }

    fn warn_unused(&mut self) {
        let unused: Vec<(PathBuf, String)> = self
            .defined
            .iter()
            .flat_map(|(f, set)| set.iter().map(move |n| (f.clone(), n.clone())))
            .filter(|k| !self.used.contains(k))
            .collect();
        for (file, name) in unused {
            self.report
                .warnings
                .push(format!("{}: region '{name}' wordt nergens gebruikt", file.display()));
        }
    }
}

fn check(root: &Path) -> Result<(), String> {
    if !root.join("book.toml").is_file() {
        return Err(format!(
            "{} is niet de wortel van de repo (book.toml ontbreekt). Draai: mise run check-includes",
            root.display()
        ));
    }
    let mut c = Checker {
        root,
        defined: BTreeMap::new(),
        used: BTreeSet::new(),
        report: Report::default(),
    };
    c.scan_examples();
    c.check_book();
    c.check_decks();
    c.warn_unused();

    for w in &c.report.warnings {
        eprintln!("waarschuwing: {w}");
    }
    for e in &c.report.errors {
        eprintln!("FOUT: {e}");
    }
    let regions: usize = c.defined.values().map(BTreeSet::len).sum();
    if c.report.errors.is_empty() {
        println!(
            "check-includes: ok ({regions} region(s), {} gebruikt, {} waarschuwing(en))",
            c.used.len(),
            c.report.warnings.len()
        );
        Ok(())
    } else {
        Err(format!("{} fout(en) in region-verwijzingen", c.report.errors.len()))
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

const USAGE: &str = "Gebruik: rust-script scripts/regions.rs <preprocess|check>\n\
  preprocess   mdBook-preprocessor (leest JSON van stdin)\n\
  check        controleert alle region-verwijzingen in src/ en slides/";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("preprocess") => preprocess(&args[1..]),
        Some("check") => std::env::current_dir()
            .map_err(|e| format!("werkmap onbekend: {e}"))
            .and_then(|d| check(&d)),
        Some("--help") | Some("-h") => {
            println!("{USAGE}");
            Ok(())
        }
        _ => Err(USAGE.to_string()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("regions.rs: FOUT: {e}");
            ExitCode::FAILURE
        }
    }
}

// ---------------------------------------------------------------------------
// Tests: `mise run test-scripts`
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markers_in_all_taught_languages() {
        assert_eq!(parse_marker("// #region greeting"), Some(Marker::Start("greeting".into())));
        assert_eq!(parse_marker("    // #endregion greeting"), Some(Marker::End("greeting".into())));
        assert_eq!(parse_marker("# region greeting"), Some(Marker::Start("greeting".into())));
        assert_eq!(parse_marker("# endregion greeting"), Some(Marker::End("greeting".into())));
        assert_eq!(parse_marker("//region greeting"), Some(Marker::Start("greeting".into())));
        assert_eq!(parse_marker("/* #region greeting */"), Some(Marker::Start("greeting".into())));
        assert_eq!(parse_marker("<!-- #endregion greeting -->"), Some(Marker::End("greeting".into())));
        assert_eq!(parse_marker("let region = 1;"), None);
        assert_eq!(parse_marker("// regional notes"), None);
        assert_eq!(parse_marker("#[derive(Debug)]"), None);
    }

    const KOTLIN: &str = "package x\n\n// #region greeting\nfun greeting(name: String): String =\n    \"Hello, $name\"\n// #endregion greeting\n\nfun main() {\n    println(greeting(\"x\"))\n}\n";
    const PYTHON: &str = "# region greeting\ndef greeting(name):\n    return f\"Hello, {name}\"\n# endregion greeting\n";
    const RUST: &str = "// #region outer\nfn a() {\n    // #region inner\n    let x = 1;\n    // #endregion inner\n    let y = 2;\n}\n// #endregion outer\n";

    #[test]
    fn extract_kotlin_has_no_markers() {
        let got = extract(KOTLIN, "greeting").unwrap();
        assert_eq!(got, "fun greeting(name: String): String =\n    \"Hello, $name\"");
    }

    #[test]
    fn extract_python_dedents_and_strips() {
        let got = extract(PYTHON, "greeting").unwrap();
        assert_eq!(got, "def greeting(name):\n    return f\"Hello, {name}\"");
    }

    #[test]
    fn nested_regions_keep_the_inner_code_but_not_its_markers() {
        let outer = extract(RUST, "outer").unwrap();
        assert_eq!(outer, "fn a() {\n    let x = 1;\n    let y = 2;\n}");
        let inner = extract(RUST, "inner").unwrap();
        assert_eq!(inner, "let x = 1;");
    }

    #[test]
    fn missing_region_is_an_error() {
        assert!(extract(KOTLIN, "nope").unwrap_err().contains("greeting"));
    }

    #[test]
    fn malformed_markers_are_reported() {
        let (_, p) = scan("// #region a\ncode\n");
        assert!(p[0].1.contains("niet afgesloten"));
        let (_, p) = scan("// #region a\n// #endregion\n");
        assert!(p[0].1.contains("noem de regionaam"));
        let (_, p) = scan("// #endregion a\n");
        assert!(p[0].1.contains("zonder bijbehorende"));
        let (_, p) = scan("// #region 1bad\n// #endregion 1bad\n");
        assert!(p[0].1.contains("ongeldige regionaam"));
        let (_, p) = scan("// #region a\n// #endregion a\n// #region a\n// #endregion a\n");
        assert!(p[0].1.contains("twee keer"));
    }

    #[test]
    fn include_parsing_leaves_line_ranges_alone() {
        let md = "a {{#include f.kt:greeting}} b\n{{#include f.kt:2:10}}\n{{#include f.kt}}\n\\{{#include f.kt:x}}\n";
        let found = find_includes(md);
        assert_eq!(found.len(), 3);
        assert_eq!(found[0].region.as_deref(), Some("greeting"));
        assert_eq!(found[1].region, None);
        assert_eq!(found[1].path, "f.kt");
        assert_eq!(found[2].region, None);
    }

    #[test]
    fn expand_replaces_only_region_includes() {
        let dir = std::env::temp_dir().join("regions-rs-test");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("a.kt"), KOTLIN).unwrap();
        let md = "```kotlin\n{{#include a.kt:greeting}}\n```\n{{#include a.kt:1:2}}\n";
        let out = expand_includes(md, &dir).unwrap();
        assert!(out.contains("fun greeting(name: String): String =\n    \"Hello, $name\"\n```"));
        assert!(out.contains("{{#include a.kt:1:2}}"));
        assert!(!out.contains("#region"));
        assert!(expand_includes("{{#include a.kt:nope}}", &dir).is_err());
        assert!(expand_includes("{{#include missing.kt:x}}", &dir).is_err());
    }
}
