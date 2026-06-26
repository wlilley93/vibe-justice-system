//! The `vjs permit` command: list / show / close permits.

use super::*;

pub(crate) fn cmd_permit(
    repo: &Path,
    subcmd: PermitCommands,
    json: bool,
) -> Result<(), KernelError> {
    match subcmd {
        PermitCommands::List => {
            let permits = Store::read_permits(repo)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&permits).unwrap());
            } else {
                if permits.is_empty() {
                    println!("No active permits");
                } else {
                    for permit in &permits {
                        println!(
                            "{} ({:?}): {} obligations",
                            permit.id.0,
                            permit.status,
                            permit.obligations.len()
                        );
                    }
                }
            }
            Ok(())
        }
        PermitCommands::Close { id, proof } => {
            let mut permits = Store::read_permits(repo)?;
            let permit = permits
                .iter_mut()
                .find(|p| p.id.0 == id)
                .ok_or(KernelError::PermitNotFound(id.clone()))?;
            permit.status = PermitStatus::Closed;

            if let Some(proof_content) = proof {
                use sha2::Digest;
                let digest = format!(
                    "sha256:{}",
                    hex::encode(sha2::Sha256::digest(proof_content.as_bytes()))
                );
                let proof = Proof {
                    id: ProofId(format!(
                        "PROOF-{}",
                        chrono::Utc::now().format("%Y%m%d-%H%M%S")
                    )),
                    permit_id: permit.id.clone(),
                    kind: ProofKind::DecisionLog,
                    status: ProofStatus::Passed,
                    digest: Some(digest),
                    captured_at: chrono::Utc::now().to_rfc3339(),
                };
                Store::write_proof(repo, &proof)?;
                if json {
                    println!("{}", serde_json::to_string_pretty(&proof).unwrap());
                }
            }

            Store::write_permit(repo, permit)?;
            if json {
                println!("{{ \"ok\": true, \"permit_id\": \"{}\" }}", id);
            } else {
                println!("Permit {} closed", id);
            }
            Ok(())
        }
    }
}
