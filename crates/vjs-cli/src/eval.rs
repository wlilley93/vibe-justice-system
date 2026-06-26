//! The `vjs eval` command: run the agent-harness evaluation suites.

use super::*;

pub(crate) fn cmd_eval(repo: &Path, suite: Option<String>, json: bool) -> Result<(), KernelError> {
    let suite = suite.unwrap_or_else(|| "all".into());
    let lawpack = load_lawpack(repo)?;
    // The route suite needs a kernel context; build it best-effort.
    let ctx = build_kernel_context(repo).ok();
    let reports = vjs_core::evals::run_suite(&suite, &lawpack.invariants, ctx.as_ref(), repo);

    let total_failed: usize = reports.iter().map(|r| r.failed).sum();
    let total_passed: usize = reports.iter().map(|r| r.passed).sum();

    if json {
        println!("{}", serde_json::to_string_pretty(&reports).unwrap());
    } else if reports.is_empty() {
        println!(
            "No eval suite matched '{}'. Try: agent-harness | prompts | route | all",
            suite
        );
    } else {
        for report in &reports {
            println!(
                "suite {}: {} passed, {} failed",
                report.suite, report.passed, report.failed
            );
            for c in &report.results {
                let mark = if c.passed { "PASS" } else { "FAIL" };
                println!("  [{}] {} - {}", mark, c.case, c.description);
                if !c.passed {
                    println!("        expected {}, got {}", c.expected, c.actual);
                    if let Some(fix) = &c.fix {
                        println!("        fix: {}", fix);
                    }
                }
            }
        }
        println!("TOTAL: {} passed, {} failed", total_passed, total_failed);
    }

    if total_failed > 0 {
        std::process::exit(1);
    }
    Ok(())
}
