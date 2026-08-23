//! Pure value-transformer helpers for the gazette command: YAML/JSON shaping, citation
//! derivation, text humanisation, and git-date lookups. No shared mutable state (nested fns,
//! lifted to module scope unchanged).

use std::path::Path;

const FOUNDING_ACT: &str = "ACT-COMPUTER-FIRST-REALM";
const SOURCES_ACT: &str = "ACT-001";
const COURTS_ORDER: &str = "2026-VJS-COURTS-CONSTITUTION-001";
const V1_UNION: &str = "BILL-1";
const V1_SI_ACT: &str = "BILL-14";
const V1_COURTS_ACT: &str = "BILL-16";
const V1_FOUNDING_BILL: &str = "BILL-32";

pub(crate) fn s(v: &serde_yaml::Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(|x| x.trim().to_string())
}

pub(crate) fn str_list(v: &serde_yaml::Value, key: &str) -> Vec<String> {
    match v.get(key) {
        // A sequence: collect its string items (the canonical form).
        Some(x) if x.is_sequence() => x
            .as_sequence()
            .map(|seq| {
                seq.iter()
                    .filter_map(|i| i.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default(),
        // Tolerate a bare scalar string as a one-element list: varies / affirms /
        // appeal_of are written as scalars across the order record.
        Some(x) => x
            .as_str()
            .filter(|s| !s.is_empty())
            .map(|s| vec![s.to_string()])
            .unwrap_or_default(),
        None => Vec::new(),
    }
}

pub(crate) fn first_sentence(text: &str) -> String {
    let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    match collapsed.find(". ") {
        Some(i) => collapsed[..i + 1].to_string(),
        None => collapsed,
    }
}

pub(crate) fn humanize(token: &str) -> String {
    token.replace('_', " ")
}

pub(crate) fn yaml_to_json(v: &serde_yaml::Value) -> serde_json::Value {
    serde_json::to_value(v).unwrap_or(serde_json::Value::Null)
}

pub(crate) fn pick(v: &serde_yaml::Value, keys: &[&str]) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    for k in keys {
        if let Some(x) = v.get(k)
            && !x.is_null()
        {
            obj.insert(k.to_string(), yaml_to_json(x));
        }
    }
    serde_json::Value::Object(obj)
}

pub(crate) fn text_body(kind: &str, v: &serde_yaml::Value) -> serde_json::Value {
    match kind {
        "statute" => {
            let mut body = pick(v, &["purpose"]);
            let enacted: Vec<serde_json::Value> = v
                .get("sections")
                .and_then(|x| x.as_sequence())
                .map(|secs| {
                    secs.iter()
                        .map(|sec| {
                            pick(sec, &["id", "title", "text", "commentary", "kernel_effect"])
                        })
                        .collect()
                })
                .unwrap_or_default();
            // Contiguous numbering: every ordinal from s1 to the highest
            // enacted section appears; absent ordinals are Reserved (the
            // positive drafting convention), so the count never jumps.
            let act_id = v.get("id").and_then(|x| x.as_str()).unwrap_or_default();
            let ordinal = |sec: &serde_json::Value| -> Option<u32> {
                sec["id"].as_str()?.rsplit(":s").next()?.parse().ok()
            };
            let max = enacted.iter().filter_map(&ordinal).max().unwrap_or(0);
            let mut by_ord: std::collections::HashMap<u32, serde_json::Value> = enacted
                .into_iter()
                .filter_map(|sec| ordinal(&sec).map(|n| (n, sec)))
                .collect();
            let sections: Vec<serde_json::Value> = (1..=max)
                .map(|n| {
                    by_ord.remove(&n).unwrap_or_else(|| {
                        serde_json::json!({
                            "id": format!("{}:s{}", act_id, n),
                            "title": "Reserved",
                            "reserved": true,
                        })
                    })
                })
                .collect();
            body["sections"] = serde_json::Value::Array(sections);
            body
        }
        "regulation" => pick(v, &["authority", "text", "kernel_effect"]),
        "order" => pick(
            v,
            &[
                "question",
                "holding",
                "directives",
                "forbidden",
                "exceptions",
                "runtime_summary",
                "source_opinion",
            ],
        ),
        "decision" => pick(
            v,
            &[
                "decision",
                "reason",
                "basis",
                "consequences",
                "review_triggers",
                "scope",
            ],
        ),
        "invariant" => pick(v, &["severity", "rule", "remedy", "basis"]),
        "obligation" => pick(v, &["text", "kind", "due", "required", "basis"]),
        "spec" => pick(
            v,
            &[
                "purpose",
                "scope",
                "decisions",
                "invariants",
                "obligations",
                "review_triggers",
            ],
        ),
        "rule" => pick(
            v,
            &["summary", "effect", "scope", "exceptions", "rank", "source"],
        ),
        _ => serde_json::Value::Null,
    }
}

pub(crate) fn textual_refs(content: &str) -> Vec<String> {
    let mut out = Vec::new();
    let id_re = regex::Regex::new(
        r"\b((?:ACT|DEC|INV|OBL|SPEC|REG)-[A-Z0-9][A-Za-z0-9-]*[A-Za-z0-9](?::s\d+)?)",
    )
    .expect("static regex");
    let cite_re =
        regex::Regex::new(r"\[(\d{4})\]\s+(VJS|REALM)-([A-Z]{2})\s+(\d+)").expect("static regex");
    // Directive `must:` fields are long snake_case strings; their citations appear as
    // "2026_vjs_pc_6" / "2026_vjs_sc_2", never the human "[YYYY] XXX N" form, so cite_re
    // misses them (e.g. PC-19 D5's mandated VJS-PC 6, cited only inside its must:).
    let snake_re =
        regex::Regex::new(r"(\d{4})_(vjs|realm)_([a-z]{2})_(\d+)").expect("static regex");
    for line in content.lines() {
        for m in id_re.find_iter(line) {
            if !line[..m.start()].trim_end().ends_with("no") {
                out.push(m.as_str().to_string());
            }
        }
        for c in cite_re.captures_iter(line) {
            let (year, realm, court, n) = (&c[1], &c[2], &c[3], &c[4]);
            out.push(match realm {
                // "[2026] VJS-PC 5" -> the order id "2026-VJS-PC-005"
                "VJS" => vjs_order_id(year, court, n),
                // "[2026] REALM-SC 10" -> the archive item id "REALM-SC-10"
                _ => format!("REALM-{}-{}", court, n),
            });
        }
        for c in snake_re.captures_iter(line) {
            let (year, realm, court, n) = (&c[1], &c[2], &c[3], &c[4]);
            let court = court.to_ascii_uppercase();
            out.push(match realm {
                "vjs" => vjs_order_id(year, &court, n),
                _ => format!("REALM-{}-{}", court, n),
            });
        }
    }
    out
}

/// The order id a VJS citation resolves to, applying the one descriptive-id alias: VJS-SC 2
/// is recorded under [2026] VJS-COURTS-CONSTITUTION-001, not "2026-VJS-SC-002", so a bare
/// "[2026] VJS-SC 2" / "2026_vjs_sc_2" would otherwise resolve to a non-existent id and drop.
fn vjs_order_id(year: &str, court: &str, n: &str) -> String {
    let id = format!("{}-VJS-{}-{:0>3}", year, court, n);
    if id == "2026-VJS-SC-002" {
        COURTS_ORDER.to_string()
    } else {
        id
    }
}

pub(crate) fn title_case(t: &str) -> String {
    const SMALL: [&str; 14] = [
        "a", "an", "and", "as", "at", "but", "by", "for", "in", "of", "on", "or", "the", "to",
    ];
    let words: Vec<&str> = t.split(' ').collect();
    let last = words.len().saturating_sub(1);
    words
        .iter()
        .enumerate()
        .map(|(i, w)| {
            if w.chars().skip(1).any(|c| c.is_uppercase()) || w.chars().any(|c| c.is_ascii_digit())
            {
                return w.to_string();
            }
            let lower = w.to_lowercase();
            // a bracket or a dash opens a new phrase: the first word of a
            // subtitle ("... - the Founding Settlement") is capitalised
            let after_break = i > 0 && words[i - 1] == "-";
            if !after_break
                && !w.starts_with('(')
                && i != 0
                && i != last
                && SMALL.contains(&lower.trim_matches(|c: char| !c.is_alphanumeric()))
            {
                return lower;
            }
            let mut cs = w.chars();
            match cs.next() {
                Some(f) if f.is_alphabetic() => f.to_uppercase().collect::<String>() + cs.as_str(),
                Some('(') => {
                    // capitalise inside an opening bracket: "(the order)" -> "(The Order)"
                    let rest = cs.as_str();
                    let mut rc = rest.chars();
                    match rc.next() {
                        Some(g) => format!("({}{}", g.to_uppercase(), rc.as_str()),
                        None => w.to_string(),
                    }
                }
                _ => w.to_string(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn derive_order_citation(id: &str) -> Option<String> {
    let parts: Vec<&str> = id.split('-').collect();
    if parts.len() == 4
        && parts[0].len() == 4
        && parts[0].chars().all(|c| c.is_ascii_digit())
        && parts[1] == "VJS"
        && matches!(parts[2], "SC" | "PC" | "CC" | "BOOT")
    {
        let n: u32 = parts[3].parse().ok()?;
        return Some(format!("[{}] VJS-{} {}", parts[0], parts[2], n));
    }
    None
}

pub(crate) fn derive_v1_citation(id: &str, date: &str) -> Option<String> {
    let year = date
        .get(0..4)
        .filter(|y| y.chars().all(|c| c.is_ascii_digit()))?;
    let (series, n) = id.rsplit_once('-')?;
    if n.is_empty() || !n.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let series = match series.strip_prefix("REALM-") {
        Some(rest) => format!("REALM-{}", rest),
        None => format!("REALM-{}", series),
    };
    Some(format!("[{}] {} {}", year, series, n))
}

pub(crate) fn git_dates(repo: &Path, extra: &[&str]) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let mut args = vec![
        "-C".to_string(),
        repo.to_string_lossy().to_string(),
        "log".to_string(),
    ];
    args.extend(extra.iter().map(|a| a.to_string()));
    args.extend(["--name-only".to_string(), "--format=\u{1}%cI".to_string()]);
    if let Ok(out) = std::process::Command::new("git")
        .args(&args)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_OBJECT_DIRECTORY")
        .output()
    {
        let text = String::from_utf8_lossy(&out.stdout);
        let mut current = String::new();
        for line in text.lines() {
            if let Some(ts) = line.strip_prefix('\u{1}') {
                // keep the full committer timestamp (%cI); the day is derived
                // where a date-only display is wanted
                current = ts.to_string();
            } else if !line.is_empty() && !current.is_empty() {
                map.entry(line.to_string())
                    .or_insert_with(|| current.clone());
            }
        }
    }
    map
}

pub(crate) fn lineage_anchor(estate: &str, kind: &str, id: &str) -> Option<&'static str> {
    let anchor = match (estate, kind) {
        ("v2", "statute") => {
            if id == FOUNDING_ACT {
                V1_FOUNDING_BILL
            } else {
                FOUNDING_ACT
            }
        }
        ("v2", "regulation") => "", // its authority field already names the parent
        ("v2", "order") => COURTS_ORDER,
        ("v2", _) => SOURCES_ACT,
        ("v1", "act") => V1_UNION,
        ("v1", "instrument") => V1_SI_ACT,
        ("v1", "judgment") => V1_COURTS_ACT,
        _ => "",
    };
    (!anchor.is_empty() && anchor != id).then_some(anchor)
}

pub(crate) fn xml_esc(t: &str) -> String {
    t.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
