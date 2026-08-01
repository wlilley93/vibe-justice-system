# County Court, sitting at first instance

**In the matter of two apex goal statements: `opbox-prod/GOAL-LONG-HORIZON.md` and `Jellytot/docs/GOAL-long-horizon.md`**

Submission `SUBMISSION-2026-08-01-174125`. Single judge. Jurisdiction: default. Citation ordinal to be minted by the clerk (ACT-004:s8, collisions fatal).

---

## 0. Findings on the pleaded facts, before anything else

I read both documents in full, the 2026-07-27 correction file, the 2026-07-31 handover, ACT-001, ACT-004, ACT-009 and ACT-010, and I checked the file system and the git record. The case file is substantially accurate. Six corrections, three of which matter to the disposal.

1. **"Doc 1 is cited by five other goal docs" is wrong.** It is cited by three goal documents (`opbox-prod/goals.md:3`, `GOAL-enforcement.md:3`, `GOAL-delivery.md:3`), each in the form "an increment of `GOAL-LONG-HORIZON.md`". The pleading understates the case in the way that matters: doc 1 is also cited by `fleet.yaml:3` (invariant 7, governing the three ledgers), by `docs/H1-REHEARSAL-2026-07-26.md:3`, and by two **executable** files on the tenant provisioning path, `boltrig-tenant/check-model-path.py:4` and `boltrig-tenant/configure-gateway-provider.py:5`, both citing criterion L5. Seven artefacts, two of them code that runs.

2. **"Doc 2 is cited by none" is wrong.** It is cited once: `boltrig/docs/findings/2026-07-26-readiness-says-disabled-about-a-live-dependency.md:38`, as `../../../Jellytot/docs/GOAL-long-horizon.md`. That path escapes the boltrig repository root into a directory that is not a repository. It resolves on this box and nowhere else. A citation that resolves only on the machine that wrote it is not a citation, and the single reference doc 2 has is of that kind.

3. **Fact 4 is imprecisely put but sound in substance.** Doc 2 calls `opbox-prod/docs/GOAL-boltrig-federation.md` "the current leg" and calls **itself** "the road it is on"; the pleading reads as though it called the federation file the road. The real clash is confirmed and is worse than pleaded: both apex documents claim parentage of the same third file.

4. **Fact 7 is correct, and incomplete in a way that cuts against the applicant's own preferred document.** Doc 2's 2026-07-26 appendices score test 1 MET and test 3 MET; the 2026-07-27 correction file finds test 1 NOT MET and test 3 half met; doc 2 was never annotated. But **doc 1 carries the same falsified claim**. Its "Where we actually are" section (lines 189 to 193) asserts Classical Visas "answering real agent turns through the shared kernel: `opbox.list_matters` returned the client's actual matter ... Verified from the audit tree." That rests on the same 2026-07-25 evidence the correction file impeaches ("the proof is from 25 Jul 13:57 and predates two replacements of the path it ran on; the five verb calls after it all returned 401"). Doc 1's last commit is `ae905e1`, 2026-07-26. Neither apex document was touched after the correction. The case file is otherwise symmetric; this is its one asymmetry, and it is not on the face of it.

5. **Fact 5 is correct and materially understated.** The Corporate Brain is not merely unretired. Its **criterion layer is lost.** `~/Projects/Opbox` now contains only `design-deliverables` and `frontend-v2`: no `goals.md`, no `DEFINITION-OF-DONE.md`. `opbox-prod/goals.md:56-61` records why: every `goals.md` in the tree was a symlink to a target that had been deleted, the loss was noticed on 2026-07-09, and the file at that path was recreated for a different purpose. The only saved copy in `~/Backups` is `goals.md.pre-corporate-brain.2026-06-23`, which is the checklist from **before** the re-point. The whitepaper survives (`opbox-docs/specs/the-corporate-brain.whitepaper.txt`, tracked in the `opbox-docs` submodule at `7316b54`) and is still cited by live tracked code (`boltrig/boltrig/fleet/prompt_stack.py:1`) and by `Jellytot/docs/HANDOVER-site-v1-2026-07-29.md`.

6. **Facts 1, 2, 3 (the handover row and the invariant-7 citation), and 6 are verified as pleaded.** Doc 1: 8 commits, `f757e22` (2026-07-25) to `ae905e1` (2026-07-26), tracked, clean. Jellytot has no `.git`. The backup taken today exists and, because this program has twice been bitten by a zero-byte dump under a success-shaped name, I checked it rather than believed it: `~/Backups/jellytot-docs-scripts-20260801-174006.tgz`, 110,039 bytes, containing `docs/GOAL-long-horizon.md` at 17,267 bytes. It is a real backup.

**One further finding, of the court's own motion, on the "different in kind" hypothesis.** It does not survive reading the two texts. All six of doc 2's invariants appear in doc 1's eight (prose is not enforcement, near-verbatim in both; derive rather than store; running is not serving; fail closed and invalid indistinguishable from absent; committed is not delivered; consolidate). Doc 1 has two that doc 2 lacks. Doc 2's test 3 and doc 1's criterion L6 are the same criterion in different words. Doc 2's outcome statement and doc 1's "concretely, at the far end" paragraph describe the same end state. Both documents are outcome plus falsifiable criteria. They are not different in kind. They are a duplicate carrying two different scores of overlapping tests.

What is genuinely unique to doc 2 is narrow and identifiable: (a) the five binary acceptance tests as a numbered, citable set; (b) the ownership and walkaway limb ("the firm holds its own server, its own data and its own model keys", and test 5); (c) the "How this fails" list. That is transcribable content, not a different kind of document.

---

## 1. The question

Which of two documents, each written 2026-07-25 and each declaring itself the statement above all the others, is the authoritative apex record; how a second such record is prevented from appearing; and how a superseded goal is marked so that a reader cannot mistake it for live. Incidentally: whether any of this is reserved to the Principal as an objective rather than a record.

---

## 2. Decision

**Option A, varied.** Doc 1, `opbox-prod/GOAL-LONG-HORIZON.md`, is the apex record. Doc 2 is superseded by an explicit record under ACT-004:s9, is not deleted, and its unique content is transcribed into doc 1 **verbatim** before the supersession takes effect. Option B is refused. Option C is refused as to apex status, and the Corporate Brain is not retired by this court.

**Ratio.** An apex goal statement holds its authority from the record that carries it and not from its own claim to supremacy: as between competing candidates, the one that is version-controlled, citable at a stable path inside a subscribing repository, and reachable by a check governs, and the other is superseded by an explicit record under ACT-004:s9 with its unique content transcribed verbatim rather than rewritten. A court may decide **where** an objective is recorded and **which** record is citable; it may not decide **what** the objective is, so consolidation proceeds by transcription, and any wording a transcriber would change goes to the Principal instead.

---

## 3. Reasoning

### 3.1 Neither self-declaration counts for anything

Doc 1 says every other `GOAL*.md` is an increment of it. Doc 2 says everything else is a layer of it. ACT-001:s7 is decisive against both: adopted text derives force from the lawful organ, not from counsel's draft, and an agent's writing does not become binding by the fact of having been written. Doc 1 concedes the point itself: "It binds nothing on its own. VJS decides." The self-declarations are symmetric and worthless. What breaks the tie has to be external to both texts.

### 3.2 What is external: the record, not the prose

Three external differences, all measured above.

**Durability and diffability.** Doc 1 has eight commits, so its history is readable and any future edit is a diff. Doc 2 had, until 17:40:06 today, exactly one copy of itself on one nvme, in a directory with no `.git`. It is worth being precise about why that matters, because "unversioned" on its own is a weak argument. Doc 2's own stated convention is "Appended, not rewritten". That convention is **unverifiable**, because nothing records what the file said yesterday. A rule with no way to fail is doc 2's own first invariant, turned on itself. A goal document that cannot be diffed cannot prove even its own convention, let alone bind anything.

**Reachability.** In this program, authority is exercised through mechanisms that read the repository: `scripts/verify-all.sh`, the gate census, the seeded-failure selftests. Doc 1 is already load-bearing for two of them by name (L5 in `check-model-path.py` and `configure-gateway-provider.py`, invariant 7 in `fleet.yaml`). Doc 2 cannot be read by any gate without reaching outside the repository, which is exactly what its one citation does, and exactly why that citation is broken for anyone but this box.

**Jurisdiction.** ACT-001:s9: each repo is a local jurisdiction, and installed repos subscribe by default. ACT-009:s15 requires each repository to declare lineage in `.vjs/config.toml`. `~/Projects/Jellytot` is not a repository. It cannot subscribe, is not a local jurisdiction, and holds no governed surface. That does not put its files beyond this court's reach (Lexby owes duties wherever he writes), but it does mean nothing there can carry apex authority, because apex authority in this system is exercised by citation and by check, and both need a tracked path.

### 3.3 The strongest objection, which was not pleaded, and my answer

The strongest argument against my disposal is not either of the two the applicant put. It is this:

> **Doc 1 is impeached by the very correction relied on to impeach doc 2.** The applicant asks the court to prefer doc 1 as the more reliable record. But doc 1's headline claim about the live system is false on the same 2026-07-27 evidence, and has been uncorrected for five days. On the reliability criterion the applicant advances, doc 1 loses. Preferring it rewards the document nobody scored as hard.

I accept the premise entirely, and I found it myself rather than being shown it. I reject the conclusion, because **durability is not accuracy, and I do not hold doc 1 to be the more accurate document.** I hold it to be the more **correctable** one. Doc 1's false claim can be annotated in a place a stranger will find, the annotation lands as a diff that proves it landed, and a gate can require the shape of it. None of that is available against a file no gate can see and no history can compare. A record that can be shown to be wrong is worth more than a record that cannot be shown at all. And I do not spare doc 1: the annotation duty in condition 6 below falls on **both** documents, and on doc 1 first, because it is the one that will be read.

Two further objections, answered more shortly.

**"Invariant 7 decides it, and invariant 7 is doc 1's own invariant, so this is circular."** Correct as to circularity, which is why I do not rest on invariant 7 at all. Doc 1's invariants bind nothing of their own force (ACT-001:s7). I rest on ACT-004:s9, which is enacted law: supersession must be explicit, the old authority remains visible but is no longer binding, and corrections are new records. Invariant 7 tells me what this program believes about second paths. It is persuasive and it happens to point the same way. It is not my authority.

**"They are different in kind, so declare the relation (Option B) and lose nothing."** Tested against the texts, they are not different in kind (finding above: doc 2's six invariants are a proper subset of doc 1's eight). Option B would bless a duplicate and, worse, make it lawful, and the harm is already measured on the record: `HANDOVER-2026-07-31.md:39` lists both files in a single row headed "Long-horizon goal" for an agent picking up cold, while `:273` cites "GOAL-LONG-HORIZON invariant 7", an ordinal citation that resolves in only one of the two, doc 2's invariants being unnumbered. That is precisely the failure mode doc 1's criterion L7 and horizon H2 exist to prevent: hand it to a competent stranger with a written goal and they can proceed. A stranger handed two apex goals with divergent scores of overlapping tests cannot.

### 3.4 Why Option C fails, and what survives of it

The Corporate Brain cannot be the operative apex, but not for the reason pleaded ("it is a thesis, not a criterion"). It **had** a criterion layer: `DEFINITION-OF-DONE.md` and a 63-goal faculty checklist. That layer is gone from disk, was noticed missing on 2026-07-09 through a symlink whose target had been deleted, and the only backup is of the version before the re-point. Restoring it as apex would require reconstructing criteria that no longer exist. A record that cannot settle any claim cannot be the record that settles claims.

That is a finding of fact and a records ruling. It is not a retirement, and I have no power to retire it: see part 4.

---

## 4. Where the line falls

The applicant did not ask me to choose the objective. I hold that the line is drawn very nearly in the right place, and I move it in one respect: **the transcription must be verbatim, because editing during consolidation is an objective act wearing a records costume.**

**Within this court (records):**
- which file is the apex record, and where it lives;
- that supersession must be explicit, and the form it takes (ACT-004:s9);
- that the superseded record is not deleted (ACT-004:s9, `must_not: delete_old_records`);
- that consolidation is transcription, and what must be transcribed;
- that a score in a goal document is a claim about the live system, governed by [2026] CC-OPBOX 19;
- what mechanism prevents a third apex record inside a subscribing repository;
- the duty on Lexby to record a Principal re-point in the repository in the same session.

**Reserved to the Principal (ACT-001:s2, "may set objectives"):**
- **(i) Whether the objective is still what these documents say.** Not before me and not decided. I move text; I do not choose the destination.
- **(ii) Any wording changed during transcription.** A clause moved verbatim under a new heading is a records act. A clause reworded, merged, shortened or dropped is a change to the objective. Doc 2's five acceptance tests and its ownership and walkaway limb are content doc 1 does not state at apex level; folding them in with improvements would subtract or alter an objective by editing. Every clause a transcriber wishes to change is listed and goes up, unchanged, as a question.
- **(iii) The Corporate Brain.** Its installation as north star on 2026-06-23 was a Principal act. Nothing in the repository records it, and nothing records it retired. I cannot unmake it and do not. One question goes to the Principal with the evidence attached: **is The Corporate Brain still the objective, and if so, is the apex goal record a faithful restatement of it?** This is one of the narrow class of matters genuinely outside the process, because it is an objective and not an engineering fork.
- **(iv) Whether the sibling repositories subscribe.** Doc 1's table asserts supremacy over `vibe-justice-system/docs/GOAL.md`, `vibe-design-system/docs/GOAL.md` and two boltrig goal documents. I verified that none of them back-reference it. Under ACT-001:s9 that is each local jurisdiction's to subscribe, pin or decline, with Principal assent for a sovereignty change. I decline to decide it, and I scope every order below to `opbox-prod`.

---

## 5. Conditions and order

Each is checkable against the repository. Where I have already measured the starting state, I give it, so that no condition begins as an assertion.

**C1. The apex record.** `opbox-prod/GOAL-LONG-HORIZON.md` is the sole apex goal record for the `opbox-prod` tree.
*Check:* `git -C ~/Projects/opbox-prod ls-files --error-unmatch GOAL-LONG-HORIZON.md`. Passes today.

**C2. Transcription precedes supersession.** Doc 2's five acceptance tests, its ownership and walkaway clauses (the sentence beginning "The firm holds its own server" and test 5 entire), and its "How this fails" list are carried into the apex record verbatim. **Supersession does not take effect until this passes**; until then doc 2 remains the record of those clauses. This ordering is the whole protection against subtracting an objective.
*Check:* `grep -F` each of the five test sentences against the apex record; five hits.

**C3. One gate, five cases, one selftest.** `scripts/verify-one-apex-goal.sh` exits non-zero unless all of:
 (a) exactly one tracked file in the repository carries the apex marker token;
 (b) every tracked `GOAL*.md` and `goals.md` either carries the marker or names the apex path within its first ten lines;
 (c) each of the five acceptance tests in the apex record carries a score, a settling observation, and a **dated** pointer to the record that last settled it;
 (d) the apex record carries a line naming the date on which the Principal last set or re-pointed the objective;
 (e) the apex record names the file it supersedes.
`scripts/verify-one-apex-goal-selftest.sh` seeds at least: zero markers, two markers, a `GOAL*.md` with neither marker nor parent line, a test line with no date, a missing objective-set line, and asserts a non-zero exit in each direction. Both rows are added to `scripts/verify-all.sh` in the `--fast` tier, and `scripts/verify-gate-census.sh` passes.
*One gate, not two.* Two gates over one artefact is how a rule becomes a disagreement.
*Starting state, measured:* limb (b) is **RED today**. `GOAL.md`, `GOAL-control-plane.md`, `GOAL-ai-upgrades.md` and `docs/GOAL-boltrig-federation.md` contain no reference to the apex, although doc 1's table names all four. Only `goals.md`, `GOAL-enforcement.md` and `GOAL-delivery.md` back-reference it. The four back-references land in the same change, so the gate lands green and honest and no ratchet is opened.

**C4. The supersession banner** (ACT-004:s9: explicit, old visible, not binding). The first non-blank line of `~/Projects/Jellytot/docs/GOAL-long-horizon.md` states that it is superseded, names the canonical path, this ruling and the date. The same sentence is recorded inside the apex record under a "Supersedes" heading, because the Jellytot copy sits outside every repository and no gate can reach it. The in-repo half is what makes it durable; the banner is what stops a human reading a dead goal as live.
*Check:* read line 1 of the file; `grep -F 'Jellytot/docs/GOAL-long-horizon.md'` in the apex record; C3(e).

**C5. Nothing is deleted.** Doc 2 is not deleted, moved or rewritten beyond the banner in C4 and the annotation in C6 (ACT-004:s9 `must_not: delete_old_records`; and its own append-only convention, which the court respects even though the document cannot prove it).
*Check:* the file exists, and its 2026-07-25 and 2026-07-26 sections are byte-identical to the copy inside `~/Backups/jellytot-docs-scripts-20260801-174006.tgz` apart from the two added lines.

**C6. The scores.** See part 6, which is an operative condition and not commentary.

**C7. The Corporate Brain, recorded not retired.** The apex record names `opbox-docs/specs/the-corporate-brain.whitepaper.txt` (tracked in the `opbox-docs` submodule at `7316b54`) as its source paper; records that the paper is retained as a source and is not an operative criterion record; and records the finding that its criterion layer is not on disk and is not backed up, the only saved copy in `~/Backups` being the pre-re-point checklist. The Principal question at part 4(iii) is filed with that evidence attached.
*Check:* the paths in that passage resolve, or are recorded as missing; `ls ~/Projects/Opbox` returns only `design-deliverables` and `frontend-v2`.

**C8. The handover index.** The "Long-horizon goal" row of `Jellytot/docs/HANDOVER-2026-07-31.md` names one path.
*Check:* read the row. Today it names two.

**C9. What prevents a third apex, honestly stated.** C3(a) closes the in-repo case, which is the case a gate can close. Two others were the ones that actually happened here, and each has its own closure:
 - **Outside any repository** (the doc 2 case). An apex goal document not tracked in a subscribing repository has no apex authority, whatever it says about itself. This is self-executing rather than gated, and that is the point: authority here is exercised by citation and by check, and both need a tracked path. A gate that pretended to police the whole file system would be a check that cannot fail honestly.
 - **A Principal re-point recorded only in an agent's memory** (the Corporate Brain case). When the Principal sets or re-points the objective, Lexby records it in the apex record in the same session, with the date. C3(d) makes its absence a red gate. The 2026-06-23 re-point existed only in a memory file, which is why nothing recorded it retired: nothing recorded it at all.

**C10. Scope.** Every order above is confined to `opbox-prod` and to the two documents named. Nothing here binds `vibe-justice-system`, `vibe-design-system`, `boltrig` or `Balmoral`, whose goal documents remain their own jurisdictions' business.

---

## 6. The two corrected acceptance-test scores

The corrections are in `Jellytot/docs/2026-07-27-cv-chat-outage-and-state.md` under "Corrections to the record": goal test 1 **NOT MET** though scored met (the proof is from 2026-07-25 13:57 and predates two replacements of the path it ran on; the five verb calls after it returned 401; the newest question produced a plausible answer with zero tool calls), and goal test 3 **half met** (the pricing arithmetic is real, but the billing ledger has never taken a number from the audit tree, so "derived from the record" is not established).

[2026] CC-OPBOX 19 is on all fours as to status claims in a document and I follow it rather than invent a rule: the live system is authoritative, a document that contradicts it loses, and the remedy is a staleness annotation, not a mandatory rewrite. ACT-004:s9 supplies the rest: corrections are new records.

**Ordered:**

1. **No silent edit.** The 2026-07-26 scores in doc 2 stay exactly as written. Editing them to read as though they had always been right would destroy the correction, offend ACT-004:s9, and break doc 2's own convention.
2. **Annotate doc 2 where the wrong score is read.** A one-line staleness annotation at the head of its 2026-07-26 section pointing at the 2026-07-27 correction file, per CC-OPBOX 19 ratio 3. Proportional, low overhead, and it means a reader of the superseded record cannot reach the wrong score without meeting the correction.
3. **Annotate doc 1 too**, at lines 189 to 193, for the "answering real agent turns ... Verified from the audit tree" bullet, which the same correction falsifies on the same evidence. Not pleaded; raised of the court's own motion. Doc 1 is the document that will actually be read, so this is the more urgent of the two.
4. **The tests land corrected.** When the five tests are transcribed into the apex record under C2, they land at their corrected scores: **test 1 NOT MET**, **test 3 half met**, each naming the 2026-07-27 correction and its date. No score may be transcribed as MET where a later record says otherwise. Transcribing the sentence verbatim and the score as corrected is not a contradiction: the test is objective content and moves unchanged; the score is a claim about the live system and is governed by CC-OPBOX 19.
5. **Bring the correcting text inside a repository.** The correction file is itself untracked and unbacked, so the two corrected findings are transcribed verbatim into the apex record beside the tests they correct, citing the source file and date. A correction that lives only where the defect lived is not a correction.
6. **Forward rule, narrow: an observation is spent when the path it ran on is replaced.** Test 1 was not mis-scored through carelessness; it was scored from evidence that was true when taken and had expired by the time it was cited. So a score in the apex record carries the date of the observation that settled it (C3(c)), and a score whose observation predates a replacement of the path it ran on is not MET and must be re-taken. Re-taking is a command, not an opinion.

---

## 7. Obiter (clearly marked, and binding on nobody)

- **`../../../` is not a citation.** The single reference to doc 2, in `boltrig/docs/findings/2026-07-26-readiness-says-disabled-about-a-live-dependency.md:38`, escapes the boltrig repository into a non-repository and resolves only on this box. Cross-repo relative links are a class, not an instance. Boltrig's own jurisdiction may want a gate for them. Not before me.
- **Doc 1's table is an assertion about other jurisdictions.** It claims supremacy over the VJS, VDS and boltrig goal documents, and none of the four back-reference it. Under ACT-001:s9 the cleaner shape is a subscription line inside each of those repositories rather than a claim asserted from outside one.
- **The Corporate Brain loss is the more serious defect on these facts than the duplication.** An operative criterion set vanished because a non-git stack root held it behind a symlink whose target was deleted; the loss surfaced three weeks later; and the only backup is of the version before it. This program has already written down, twice, what happens when the thing whose job is surviving the loss of a box is itself unbacked. The duplication wasted a reader's time. This destroyed a record.
- **The word "apex" in ACT-009:s11 is about courts, not documents.** Nothing in this opinion borrows from it, and a future court should not read apex-singleness of the judiciary as authority for apex-singleness of goal documents. Different subject, different reasons.
- **If anything survives a future consolidation, it should be the five binary acceptance tests.** They are the only part of either document that can fail. Doc 1 is the better record; doc 2 wrote the better test.

---

**Order:** Option A as varied. Conditions C1 to C10 and the six orders at part 6 stand. The Principal questions at part 4(ii) and 4(iii) are filed, not decided. Doc 2 remains visible and non-binding. Nothing is deleted."}],"stop_reason":"end_turn","stop_sequence":null,"stop_details":null,"usage":{"input_tokens":2,"cache_creation_input_tokens":2334,"cache_read_input_tokens":124658,"output_tokens":16883,"server_tool_use":{"web_search_requests":0,"web_fetch_requests":0},"service_tier":"standard","cache_creation":{"ephemeral_1h_input_tokens":0,"ephemeral_5m_input_tokens":2334},"inference_geo":"not_available","iterations":[{"input_tokens":2,"output_tokens":16883,"cache_read_input_tokens":124658,"cache_creation_input_tokens":2334,"cache_creation":{"ephemeral_5m_input_tokens":2334,"ephemeral_1h_input_tokens":0},"type":"message"}],"speed":"standard"},"diagnostics":null},"requestId":"req_011CdcT5E4GdWuP7NzU5pb4i","attributionAgent":"general-purpose","type":"assistant","uuid":"a519e585-4256-4382-ad99-fcab3ba70c07","timestamp":"2026-08-01T17:52:57.020Z","effort":"xhigh","userType":"external","entrypoint":"cli","cwd":"~/Projects/vibe-justice-system","sessionId":"4f50ef0c-95a9-46da-a8a9-b5a00de7f3a9","version":"2.1.220","gitBranch":"HEAD"}
