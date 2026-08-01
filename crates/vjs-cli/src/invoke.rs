//! The `vjs invoke` command: local sovereign invocation (REG-INVOCATION-001) - subscribe + lock
//! the lawpack, record the invocation, and (with --install-hooks) activate the enforcement hooks.

use super::*;

pub(crate) fn cmd_invoke(
    repo: &Path,
    jurisdiction: String,
    principal: String,
    lawpack: Option<String>,
    install_hooks: bool,
    json: bool,
) -> Result<(), KernelError> {
    // THE RE-PIN CANNOT RATIFY A DISPLACEMENT ([2026] VJS-CC-VJS 16 C6).
    //
    // Measured 2026-08-01 on a displaced jurisdiction, following LAWPACK_LOCK_DRIFT's own
    // suggested fix: this command exited 0, pinned the digest of a one-order directory a verb
    // had made under the id `vjs-v2@0.1.0`, and printed `config written: false` - so it did not
    // even disturb the `lawpack_path` it contradicts. `vjs validate` then returned OK. The
    // jurisdiction was green and lawless, its config still declaring a subscription to the real
    // canon and its lock certifying a directory that was not it. That is CC-VJS 12 D3's vice
    // exactly: a label recorded as though a subscription had happened when it had not.
    //
    // ONLY WHERE NOTHING IS NAMED. `--lawpack <path>` is the operator saying which tree, in
    // terms, and D3 already refuses a path that does not resolve. What is refused here is the
    // UNNAMED re-pin - the one an operator reaches for because a Fatal told them to.
    //
    // FIRST, before `.vjs/invocation` is created and long before the lock is written, so the
    // lock is byte-identical by construction rather than by luck.
    if lawpack.is_none()
        && let Some(d) = vjs_engine::displacement::detect(repo)
    {
        return Err(KernelError::InvalidInput(format!(
            "refusing to pin: this jurisdiction records a subscription to '{}' ({}) but its \
             lawpack resolves from '{}'. Re-pinning would certify the tree that should not be \
             there under the recorded lawpack's id and leave the recorded subscription standing \
             and false, which is not a cure but the completion of the harm. Remove '{}', or name \
             the tree you mean with `vjs invoke --lawpack <path>`. ([2026] VJS-CC-VJS 16 C6)",
            d.recorded,
            d.recorded_by,
            d.answered_from.display(),
            d.answered_from.display(),
        )));
    }

    let io = |e: std::io::Error| KernelError::InvalidInput(format!("io: {}", e));
    let repo_code = jurisdiction.to_uppercase();
    let vjs_dir = repo.join(".vjs");
    std::fs::create_dir_all(vjs_dir.join("invocation")).map_err(io)?;

    // D2 AND D3 OF [2026] VJS-CC-VJS 12, WHICH ARE ONE CHANGE.
    //
    // `--lawpack` used to be a LABEL: it was printed, written into config.toml and into
    // lawpack.lock, and never reached `load_lawpack`. Passing a correct absolute path to a
    // real lawpack changed nothing. The digest came from `build_kernel_context(repo)`, which
    // looks only at `<repo>/lawpack/v2`, so a repository that did not VENDOR the canon
    // recorded a subscription to `vjs-v2@0.1.0` pinned at the sha256 of the empty string.
    // That record was not incomplete but false, and it is the artefact a later court reads.
    //
    // Invoke also cannot ask the kernel where the lawpack is, because it is writing the
    // config that would say so: that ordering is why the silent fallback looked load-bearing.
    // It does not need to ask. It was HANDED a lawpack. It resolves that, or it refuses.
    let (lawpack, lawpack_dir, lawpack_recorded) = resolve_invocation_lawpack(repo, lawpack)?;
    let digest = match &lawpack_dir {
        Some(dir) => digest_of_lawpack_dir(dir)?,
        None => build_kernel_context(repo)?.lawpack_digest,
    };
    let now = chrono::Utc::now();
    let stamp = now.format("%Y%m%d-%H%M%S").to_string();
    let now_rfc = now.to_rfc3339();

    // 1. config.toml - write only if absent (never clobber an existing config).
    // create_new makes the existence check and the write one atomic act, so a
    // config that appears between check and write survives untouched.
    let config_path = vjs_dir.join("config.toml");
    let config = format!(
        "version = \"2\"\njurisdiction_id = \"{jur}\"\nrepo_code = \"{code}\"\nlawpack = \"{lp}\"\nlawpack_path = \"{lpp}\"\nprincipal = \"{prin}\"\n\n[paths]\norders = \".vjs/orders\"\nlogs = \".vjs/logs\"\nsubmissions = \".vjs/submissions\"\nproofs = \".vjs/proofs\"\npermits = \".vjs/permits\"\nprivate = \".vjs/private\"\n\n[paths.public]\nenabled = false\n\n[governance]\npermit_required = [\"src/**\", \"crates/**\", \"lawpack/**\", \"Cargo.toml\", \"package.json\", \"AGENTS.md\", \"VJS.md\", \"README.md\"]\npermit_exempt = [\".vjs/logs/**\", \".vjs/permits/**\", \".vjs/proofs/**\", \".vjs/cache/**\", \".vjs/private/**\", \"target/**\", \"node_modules/**\"]\n",
        jur = jurisdiction,
        code = repo_code,
        lp = lawpack,
        lpp = lawpack_recorded.as_ref().map(|d| d.display().to_string()).unwrap_or_default(),
        prin = principal,
    );
    let config_written = match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&config_path)
    {
        Ok(mut f) => {
            use std::io::Write;
            f.write_all(config.as_bytes()).map_err(io)?;
            true
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => false,
        Err(e) => return Err(io(e)),
    };

    // 2. lawpack.lock - pin the lawpack digest. Canonical fields matching the
    // one LawpackLock serde model (Bug A: the writer and reader no longer drift),
    // including schema_version for the load-time version handshake (Bug C).
    let lock = format!(
        "lawpack_id = \"{lp}\"\nlawpack_version = \"0.1.0\"\ndigest = \"{dig}\"\nschema_version = {sv}\ngenerated_at = \"{ts}\"\nlocked_by = \"{prin}\"\n",
        lp = lawpack,
        dig = digest,
        sv = vjs_store::LOCK_SCHEMA_VERSION,
        ts = now_rfc,
        prin = principal,
    );
    std::fs::write(vjs_dir.join("lawpack.lock"), lock).map_err(io)?;

    // 3. the local sovereign invocation record (the constitutional act).
    let inv = format!(
        "id: INVOCATION-{stamp}\nkind: local_sovereign_invocation\nstatus: in_force\njurisdiction:\n  id: {jur}\n  repo_root: \".\"\n  repo_code: {code}\nprincipal:\n  name: \"{prin}\"\n  capacity: local_sovereign\nsubscription:\n  lawpack: {lp}\n  lawpack_lock: .vjs/lawpack.lock\n  mode: subscribed\n  v1_archive_import: none_unless_expressly_incorporated\nassent:\n  given: true\n  form: local_sovereign_act\n  statement: >\n    The Principal invokes this repository as a VJS V2 local jurisdiction,\n    subscribes it to the stated lawpack, and authorises the kernel, hooks,\n    permits, proofs, logs, and court route to govern repo work.\neffect:\n  - creates_local_jurisdiction\n  - creates_county_court_for_repo\n  - binds_agents_to_kernel_route\n  - requires_permits_for_governed_writes\n  - requires_logs_for_material_decisions\n  - installs_validation_hooks\n",
        stamp = stamp,
        jur = jurisdiction,
        code = repo_code,
        prin = principal,
        lp = lawpack,
    );
    let inv_path = vjs_dir
        .join("invocation")
        .join(format!("{}-local-sovereign-invocation.yaml", stamp));
    std::fs::write(&inv_path, inv).map_err(io)?;

    // 4. install enforcement hooks (the activation): git core.hooksPath + tiny
    // extensionless wrappers git will run, made executable.
    let mut hooks_installed = false;
    if install_hooks {
        let hooks_dir = vjs_dir.join("hooks");
        std::fs::create_dir_all(&hooks_dir).map_err(io)?;
        // The bypass-proof wall (PC-13 D6, PC-14 REG-FRONT-DOOR-001): resolve the
        // kernel binary from the REPO ROOT so the gate survives a repo move and works
        // whether the binary was built by cargo (target/) or exported from the
        // server_of_law Docker image (bin/, REG-FRONT-DOOR-DELIVERY-001), falling back
        // to PATH. It NEVER depends on the MCP container being up - the container is
        // the door, this is the wall.
        // The resolver finds the kernel binary; the staleness guard then fail-closes if
        // a cargo-built binary is older than the crates/ source it must enforce - a hook
        // that runs STALE law is worse than none (the install-binary gotcha). A shipped
        // bin/vjs (Docker export) has no in-tree source to compare and is trusted as-is.
        let resolver = "root=\"$(git rev-parse --show-toplevel)\"\nbin=\"\"\nfor c in bin/vjs target/release/vjs target/debug/vjs; do\n  [ -x \"$root/$c\" ] && bin=\"$root/$c\" && break\ndone\n[ -n \"$bin\" ] || bin=vjs\ncase \"$bin\" in\n  \"$root/target/\"*)\n    if [ -n \"$(find \"$root/crates\" -path '*/src/*' -name '*.rs' -newer \"$bin\" -print -quit 2>/dev/null)\" ]; then\n      echo \"vjs gate binary is STALE relative to crates/*/src - rebuild: cargo build\" >&2\n      exit 1\n    fi ;;\nesac";
        std::fs::write(
            hooks_dir.join("pre-commit"),
            format!("#!/usr/bin/env bash\n{resolver}\nexec \"$bin\" validate --staged\n"),
        )
        .map_err(io)?;
        std::fs::write(
            hooks_dir.join("pre-push"),
            format!("#!/usr/bin/env bash\n{resolver}\nexec \"$bin\" local-ci\n"),
        )
        .map_err(io)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for h in ["pre-commit", "pre-push"] {
                let p = hooks_dir.join(h);
                if let Ok(meta) = std::fs::metadata(&p) {
                    let mut perm = meta.permissions();
                    perm.set_mode(0o755);
                    let _ = std::fs::set_permissions(&p, perm);
                }
            }
        }
        let out = std::process::Command::new("git")
            .args([
                "-C",
                &repo.to_string_lossy(),
                "config",
                "core.hooksPath",
                ".vjs/hooks",
            ])
            .output();
        hooks_installed = out.map(|o| o.status.success()).unwrap_or(false);

        // PC-13 D8: emit the thin agent-runtime adapters (pre_write / session_start
        // / post_action) for every supported runtime. Each only calls the kernel
        // (REG-HOOKS-001 thin-adapter rule); the logic stays in hook.rs.
        let _ = vjs_core::install::generate_adapters(repo);
    }

    // PC-13 D5: atomically lock the surface into .vjs/install.lock. Best-effort -
    // if hooks were not installed this run, the surface is incomplete and the lock
    // is deferred to a later `vjs install-lock`; the completeness invariant (D4)
    // fails closed until it exists.
    let manifest_locked = vjs_core::install::build_manifest(repo, now_rfc.clone())
        .and_then(|m| {
            let body = toml::to_string(&m).ok()?;
            let header = "# VJS install manifest (REG-INSTALL-MANIFEST-001).\n";
            std::fs::write(
                repo.join(vjs_core::install::MANIFEST_FILE),
                format!("{header}{body}"),
            )
            .ok()
        })
        .is_some();

    if json {
        println!(
            "{}",
            serde_json::json!({
                "jurisdiction": jurisdiction,
                "repo_code": repo_code,
                "lawpack": lawpack,
                "lawpack_digest": digest,
                "invocation": inv_path.to_string_lossy(),
                "config_written": config_written,
                "hooks_installed": hooks_installed,
                "manifest_locked": manifest_locked,
            })
        );
    } else {
        println!(
            "Invoked '{}' as a VJS jurisdiction (repo_code {}).",
            jurisdiction, repo_code
        );
        println!(
            "  lawpack: {} ({}...)",
            lawpack,
            &digest[..digest.len().min(23)]
        );
        println!("  invocation: {}", inv_path.display());
        println!("  config written: {}", config_written);
        println!("  hooks installed (core.hooksPath): {}", hooks_installed);
        if !install_hooks {
            println!("  (run with --install-hooks to activate commit-time enforcement)");
        }
    }
    Ok(())
}
