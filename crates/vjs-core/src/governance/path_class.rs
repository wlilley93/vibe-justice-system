//! Path classification + the kernel's ONE glob semantics.
//!
//! `PathClassifier` decides whether a repo-relative path is Governed / Exempt / Ungoverned against
//! the permit rules, and owns `glob_matches` - the single, boundary-aware glob engine the whole
//! kernel shares (the permit `PathScope` vocabulary in `permit_gate` delegates to it, so a
//! capability-backed permit decision is identical to the legacy scope match by construction).

use std::path::Path;

/// Classify a path relative to repo root against governance rules
pub struct PathClassifier;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PathClassification {
    Governed,
    Exempt,
    Ungoverned,
}

impl PathClassifier {
    pub fn classify(
        path: &Path,
        permit_required: &[String],
        permit_exempt: &[String],
    ) -> PathClassification {
        let path_str = path.to_string_lossy();

        // Check exempt first
        if Self::matches_glob_any(&path_str, permit_exempt) {
            return PathClassification::Exempt;
        }

        // Check required
        if Self::matches_glob_any(&path_str, permit_required) {
            return PathClassification::Governed;
        }

        PathClassification::Ungoverned
    }

    fn matches_glob_any(path: &str, globs: &[String]) -> bool {
        globs.iter().any(|g| Self::glob_matches(g, path))
    }

    pub fn glob_matches(glob: &str, path: &str) -> bool {
        if let Some(prefix) = glob.strip_suffix("/**") {
            // Boundary-aware: "crates/**" covers crates and crates/..., never
            // crates-evil/... (a bare starts_with let sibling dirs through).
            path == prefix || path.starts_with(&format!("{}/", prefix))
        } else if glob.contains("/**/") {
            let parts: Vec<&str> = glob.split("/**/").collect();
            if parts.len() == 2 {
                let (prefix, suffix) = (parts[0], parts[1]);
                // "a/**/b" matches a/b and a/x/y/b, with both edges on a
                // path-separator boundary so "a2/b" and "a/xb" stay out.
                (path == format!("{}/{}", prefix, suffix))
                    || (path.starts_with(&format!("{}/", prefix))
                        && path.ends_with(&format!("/{}", suffix)))
            } else {
                false
            }
        } else if glob.contains('*') {
            regex::Regex::new(&Self::glob_to_regex(glob))
                .map(|re| re.is_match(path))
                .unwrap_or(false)
        } else {
            // A literal glob names exactly one path. starts_with here let
            // "Cargo.toml.bak" ride on a permit scoped to "Cargo.toml"; a
            // directory scope must be written as "dir/**".
            path == glob
        }
    }

    fn glob_to_regex(glob: &str) -> String {
        let mut re = String::from("^");
        let mut chars = glob.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '*' => {
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        re.push_str(".*");
                    } else {
                        re.push_str("[^/]*");
                    }
                }
                '?' => re.push_str("[^/]"),
                c if r"\.+()[]{}^$|".contains(c) => {
                    re.push('\\');
                    re.push(c);
                }
                c => re.push(c),
            }
        }
        re.push('$');
        re
    }
}
