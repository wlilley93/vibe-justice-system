//! The refusal must teach its own cure (Operation Watertight WS2).
//!
//! On 2026-08-05 two decision logs failed the permit gate for carrying citations in
//! `basis` instead of the permit id, because the discharge contract (containment of
//! the permit id in the log's id, basis or issue) was discoverable only by reading
//! the gate's source. The message now states the contract; this test pins the words
//! so a future rewording cannot silently drop the contract from the refusal.

use vjs_core::PermitGate;

#[test]
fn obligation_missing_message_states_the_discharge_contract() {
    let msg = PermitGate::obligation_missing_message("PERMIT-123");
    // The permit id the log must contain.
    assert!(msg.contains("PERMIT-123"), "names the permit: {msg}");
    // The contract: containment in id, basis or issue.
    assert!(
        msg.contains("id, basis or issue CONTAINS the permit id"),
        "states the containment contract: {msg}"
    );
    // The canonical command form, so the cure is copy-pasteable.
    assert!(
        msg.contains("vjs log decision") && msg.contains("--basis <permit-id>"),
        "gives the canonical form: {msg}"
    );
}
