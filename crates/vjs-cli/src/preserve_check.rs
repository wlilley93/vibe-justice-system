//! `vjs preserve-check <before> <after>`: the ACT-RECTIFICATION-COMMISSION s5
//! content-preservation proof, as a command. A Commission may not act under s4(a)
//! or s4(f) without this runner; ss3-4 commence when it and the ratchet are in
//! force, so its existence is a commencement fact, not a convenience.
//!
//! THE TEST, in the statute's own terms: the comparison is over the FILE'S NODE
//! TREE and never over the loaded kernel structure (a `Statute` load silently
//! discards unmodelled keys, so a proof taken after a load cannot see a deletion).
//! Every mapping key with its value and every sequence item AT ITS INDEX, at every
//! depth, must be present after; the only permitted difference is scalar style,
//! quoting, indentation, line folding or KEY ORDER. A scalar whose PARSED TYPE
//! changes fails (the round-5 G-B seed: `appealable: "true"` is not `appealable:
//! true`). Every comment must be preserved.
//!
//! FAIL-CLOSED LIMITS, disclosed rather than hidden: a file that does not parse,
//! or that carries duplicate mapping keys (which the parser refuses), REFUSES the
//! proof - a refusal is not a pass, and s5 sends what this runner cannot compare
//! to manual proof. Comment extraction is a line-level detector (full-line and
//! trailing comments outside quoted and block scalars); on any line whose quote
//! state is ambiguous it refuses rather than guesses.

use super::*;

fn norm_scalar(v: &serde_yaml::Value) -> String {
    match v {
        serde_yaml::Value::String(s) => {
            format!("str:{}", s.split_whitespace().collect::<Vec<_>>().join(" "))
        }
        serde_yaml::Value::Bool(b) => format!("bool:{b}"),
        serde_yaml::Value::Number(n) => format!("num:{n}"),
        serde_yaml::Value::Null => "null".into(),
        _ => unreachable!("compound handled by walk"),
    }
}

/// Recursive node compare: mappings order-free by key, sequences by index.
/// Differences are pushed as (path, what) rows.
fn compare(
    path: &str,
    before: &serde_yaml::Value,
    after: &serde_yaml::Value,
    out: &mut Vec<String>,
) {
    use serde_yaml::Value::*;
    match (before, after) {
        (Mapping(b), Mapping(a)) => {
            for (k, bv) in b {
                let key = k
                    .as_str()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("{k:?}"));
                match a.get(k) {
                    Some(av) => compare(&format!("{path}.{key}"), bv, av, out),
                    None => out.push(format!("{path}.{key}: KEY PRESENT BEFORE, ABSENT AFTER")),
                }
            }
            for (k, _) in a {
                if b.get(k).is_none() {
                    let key = k
                        .as_str()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| format!("{k:?}"));
                    out.push(format!("{path}.{key}: KEY ABSENT BEFORE, PRESENT AFTER"));
                }
            }
        }
        (Sequence(b), Sequence(a)) => {
            if b.len() != a.len() {
                out.push(format!(
                    "{path}: SEQUENCE LENGTH CHANGED {} -> {} (an item lost, gained or moved)",
                    b.len(),
                    a.len()
                ));
            }
            for (i, (bv, av)) in b.iter().zip(a.iter()).enumerate() {
                compare(&format!("{path}[{i}]"), bv, av, out);
            }
        }
        (Tagged(b), Tagged(a)) => compare(path, &b.value, &a.value, out),
        _ => {
            let (bs, as_) = (
                std::mem::discriminant(before),
                std::mem::discriminant(after),
            );
            if bs != as_ {
                out.push(format!(
                    "{path}: PARSED TYPE CHANGED ({} -> {}) - a re-rendering that changes a \
                     scalar's parsed type is not content-preserving (s5; the G-B seed)",
                    type_name(before),
                    type_name(after)
                ));
            } else if norm_scalar(before) != norm_scalar(after) {
                out.push(format!(
                    "{path}: VALUE CHANGED after normalisation ({} -> {})",
                    norm_scalar(before),
                    norm_scalar(after)
                ));
            }
        }
    }
}

fn type_name(v: &serde_yaml::Value) -> &'static str {
    use serde_yaml::Value::*;
    match v {
        Null => "null",
        Bool(_) => "bool",
        Number(_) => "number",
        String(_) => "string",
        Sequence(_) => "sequence",
        Mapping(_) => "mapping",
        Tagged(_) => "tagged",
    }
}

/// Line-level comment extractor. Returns Err on ambiguity (fail closed).
/// Skips block-scalar interiors (their `#` is content); tracks in-line quote
/// state; a quote left open at end of line makes the NEXT lines part of a
/// multi-line quoted scalar, which this detector refuses to scan for trailing
/// comments (full-line comments cannot occur inside one anyway at column 0 of
/// a well-indented record; we refuse rather than guess).
fn comments(text: &str) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let mut block_indent: Option<usize> = None; // indent of the block-scalar introducer
    for (n, line) in text.lines().enumerate() {
        let indent = line.len() - line.trim_start().len();
        let trimmed = line.trim_start();
        if let Some(bi) = block_indent {
            if trimmed.is_empty() || indent > bi {
                continue; // block scalar content: # here is content, not comment
            }
            block_indent = None;
        }
        if trimmed.starts_with('#') {
            out.push(trimmed.to_string());
            continue;
        }
        // Trailing comment: a ` #` outside quotes. Track quote state.
        let mut in_sq = false;
        let mut in_dq = false;
        let mut prev_space = true;
        let bytes = trimmed.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let c = bytes[i] as char;
            match c {
                '\'' if !in_dq => in_sq = !in_sq,
                '"' if !in_sq => in_dq = !in_dq,
                '#' if !in_sq && !in_dq && prev_space => {
                    out.push(trimmed[i..].to_string());
                    break;
                }
                _ => {}
            }
            prev_space = c == ' ' || c == '\t';
            i += 1;
        }
        if in_sq || in_dq {
            return Err(format!(
                "line {}: a quoted scalar is left open at end of line (a multi-line quoted \
                 scalar); this detector refuses to guess where its comments are - prove the \
                 comparison manually (s5 sends the unprovable to manual proof)",
                n + 1
            ));
        }
        // A block-scalar introducer ends the line with | or > (with optional +- and
        // indent digit): its following more-indented lines are content.
        let after_key = trimmed.split(" #").next().unwrap_or(trimmed).trim_end();
        if let Some(last) = after_key.split(':').next_back() {
            let tok = last.trim();
            if tok == "|"
                || tok == ">"
                || (tok.len() <= 3
                    && (tok.starts_with('|') || tok.starts_with('>'))
                    && tok[1..]
                        .chars()
                        .all(|c| c == '+' || c == '-' || c.is_ascii_digit()))
            {
                block_indent = Some(indent);
            }
        }
    }
    Ok(out)
}

pub(crate) fn cmd_preserve_check(
    before: &Path,
    after: &Path,
    json: bool,
) -> Result<(), KernelError> {
    let read = |p: &Path| {
        std::fs::read_to_string(p).map_err(|e| KernelError::Io(format!("{}: {e}", p.display())))
    };
    let bt = read(before)?;
    let at = read(after)?;
    // Loadability is part of the test: the after-file must load by the same parser.
    // A parse refusal (including on duplicate keys) REFUSES the proof - not a pass.
    let bv: serde_yaml::Value = serde_yaml::from_str(&bt).map_err(|e| {
        KernelError::InvalidInput(format!(
            "REFUSED: before-file does not parse ({e}); manual proof"
        ))
    })?;
    let av: serde_yaml::Value = serde_yaml::from_str(&at).map_err(|e| {
        KernelError::InvalidInput(format!(
            "REFUSED: after-file does not parse ({e}) - a re-rendering that renders the \
             record unloadable is not content-preserving"
        ))
    })?;
    let mut diffs = Vec::new();
    compare("$", &bv, &av, &mut diffs);
    let bc =
        comments(&bt).map_err(|e| KernelError::InvalidInput(format!("REFUSED (before): {e}")))?;
    let ac =
        comments(&at).map_err(|e| KernelError::InvalidInput(format!("REFUSED (after): {e}")))?;
    // Comments are preserved as a multiset; order may move with key order.
    let mut missing = bc.clone();
    for c in &ac {
        if let Some(pos) = missing.iter().position(|m| m == c) {
            missing.remove(pos);
        }
    }
    for m in &missing {
        diffs.push(format!("comment PRESENT BEFORE, ABSENT AFTER: {m}"));
    }
    let ok = diffs.is_empty();
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "content_preserved": ok,
                "before": before.display().to_string(),
                "after": after.display().to_string(),
                "differences": diffs,
            }))
            .unwrap()
        );
    } else {
        for c in &diffs {
            println!("FAIL {c}");
        }
        println!(
            "preserve-check: {}",
            if ok {
                "CONTENT PRESERVED (node tree, parsed types, comments)"
            } else {
                "NOT PRESERVED"
            }
        );
    }
    if !ok {
        return Err(KernelError::InvalidInput(format!(
            "preserve-check: {} difference(s) - the rectification is not content-preserving \
             (ACT-RECTIFICATION-COMMISSION:s5)",
            diffs.len()
        )));
    }
    Ok(())
}
