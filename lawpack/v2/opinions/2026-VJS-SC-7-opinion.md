# [2026] VJS-SC 7 - Reference re the terminal status of an invariant containing a universal-negative conjunct (K-1, "no path around it")

**Supreme Council of the Vibe Justice System.** Constituted bench of five (s.18): Aldermere, Brightwater, Calderon, Donhill, Everand JJ. Each opinion is authored by a counted member; there is no synthesiser-on-top. Delivered IN FULL.

**The question (first impression, constitutional).** Under the K-29/K-30 binding discipline, can an invariant whose statement contains a UNIVERSAL-NEGATIVE conjunct - specifically K-1, "Sole mediated path: every action passes through one chokepoint; NO PATH AROUND IT" - ever be recorded `met`, or is such an invariant terminally `partial`, and what status vocabulary should the ledger use for it?

**Result.** Unanimous (5-0) on the core ratio; by majority (4-1, Everand J. dissenting in part) on the vocabulary remedy. The opinions follow in full, then the Disposition of the Court.

---

## Aldermere J.

I approach this question as I approach every question: by asking what the operative word means, and what the operative text requires, before I ask what result would be convenient. The pressure in this case is to find a word that lets the record feel settled. I will resist that pressure where it asks me to bend the meaning of `met`, and I will resist it equally where it asks me to bend the meaning of `partial`. Both words have meanings. The honest disposition is the one that keeps both words true, and that is not either of the two positions exactly as pleaded.

### I. What `met` means

The status vocabulary is not free-floating. It is a creature of the binding-quality standard, and that standard is explicit: a test must actually exercise and PROVE the claim, not merely touch the area; a test that exists but does not prove the statement is a "paper claim" the discipline exists to forbid. The honest-remainder principle sits alongside it: the kernel records what it cannot prove and never overclaims.

Read against that law, `met` carries one meaning and only one: the invariant's statement, as written, is bound to a test that exercises and proves it. `met` is the ledger's word for "proven." It is not the ledger's word for "mostly proven," "proven in the part that matters," or "proven enough." If `met` meant any of those, the binding-quality standard would be a dead letter, because a paper claim is exactly a claim that is mostly-but-not-actually proven. I will not read `met` to mean what the standard was enacted to forbid.

### II. K-1, as it is presently written, cannot be `met`

K-1's statement is a conjunction, and the status attaches to the statement as written. One conjunct (A) is enumerable and is, on these agreed facts, genuinely bound and proven: the content-driven gate inspects the staged diff rather than the caller's claim; coverage flows from a single predicate so it cannot drift; the raw-write test trips the integrity gate; forged-order fail-closed, capability equivalence, deny-dominance, and dual-door apex routing are all exercised; and the one discovered coverage gap was closed. Had K-1 said only that, I would mark it `met` without hesitation.

But K-1 also asserts (B): "NO PATH AROUND IT." That is a universal negative over an unbounded domain. A coverage test proves the modeled paths and is necessarily silent on an unmodeled bypass. No finite test proves the non-existence of every possible path. And the kernel does not pretend otherwise: enforcement.rs candidly records the irreducible remainder, the author who edits a gate and re-locks the digest pin, whose only backstops are non-machine. The record therefore contains the kernel's own admission that conjunct (B) is not proven and cannot be proven in-binary.

Given that `met` means proven, and given that the conjunction as written includes an admittedly unproven and unprovable conjunct, K-1 as presently drafted cannot truthfully be stamped `met`. To do so would be the precise paper claim the standard forbids: an assertion of proof that the kernel's own surface says does not exist. On the bare textual question, Position 1 is correct that K-1-as-written is not `met`, and I hold that it is not.

### III. But "terminally `partial`" is false in the other direction

Position 1 does not stop at "not `met`." It pleads `partial`, forever, by ruling, and calls that honesty. Here I part from it, and I part from it on its own premise.

`partial` is also a word with a meaning. In this vocabulary, sitting between `met` and `gap`, `partial` means "part of the work is done and part remains to be done." It carries an implicature of incompleteness-of-effort: it tells the reader that more work would move the needle. That implicature is true of a half-finished binding. It is FALSE of conjunct (B). The mediation-and-coverage work is done. What remains is not unfinished work; it is a residue that is unprovable by its nature, a universal negative that no quantity of further engineering will ever discharge. To label that `partial` tells the reader "keep going, this can be closed," when the truth is "this is closed as far as a machine can close it, and the rest is a named human backstop."

So Position 1 asks me to cure an overclaim with a misclaim. `met` would tell the reader the universal negative is proven; that is a lie toward overconfidence. `partial`-forever tells the reader the work is merely unfinished; that is a lie toward false modesty, and worse, a lie that never resolves, because it invites perpetual effort against a target that cannot be reached. The honest-remainder principle does not say "always pick the humbler word." It says record what you cannot prove and never overclaim. A word that misdescribes a finished-and-irreducible residue as unfinished work is not a recording of the truth; it is a different distortion of it. Position 1's remedy fails the very honesty it invokes.

### IV. The defect is in the vocabulary, not in K-1, and Position 2's reductio does not bite

If neither `met` nor `partial` can truthfully describe K-1, the fault lies not in the invariant but in the four-term set being asked to describe it. The vocabulary met | partial | gap | n/a has no term for the actual state of affairs: a machine-provable core that is proven, plus a named, enforcement-surface-recorded, non-machine-backstopped remainder. That is a gap in the vocabulary, and a textualist does not respond to a missing word by forcing the meaning of a present word. He notes the word is missing and supplies it with precision.

Position 2 presses a reductio I must answer squarely: every invariant rests on some irreducible trust (the compiler, the hardware, the key-holder), so if a universal-negative conjunct bars `met`, no security invariant is ever `met` and the vocabulary empties. The answer is the textualist distinction between the STATEMENT and its BACKGROUND. The status attaches to the statement as written. Most invariants do not write "and the compiler is honest" into their text; their trust roots live in the background, outside the proposition the test must prove. `met` for such an invariant means "the statement as written is proven," and it is, because the background trust root is not a conjunct of the statement. K-1 is different, and differently by its own drafting: it wrote a universal negative INTO its text. Having textually asserted "no path around it," it textually asserted something no test proves, and it cannot then claim the text is fully proven. The vocabulary is therefore not emptied. The reductio dissolves the moment one keeps faith with the difference between what an invariant says and what it silently rests upon. And that same distinction is the cure: do not let an invariant's statement assert, as a proven conjunct, a thing that belongs in the background as a named remainder.

The assented-record floor is persuasive here, not controlling, but it points the same way: machinery should record and route, not force a binary verdict that erases information. A status system that can only say `met` or `partial` erases the very distinction the reader most needs, between "unprovable by nature" and "not yet done." The floor's theme of machine humility favors a record that displays the remainder over one that hides it inside an ill-fitting binary.

### V. The third way, and its fencing

I therefore reject both pleaded dispositions and hold a third. The remedy has a primary form and a fallback form, and I order strict definitional fencing on both so that neither becomes a euphemism for overclaiming.

Primary form, partition the statement. K-1's recorded statement must be re-drafted to separate the machine-provable core from the universal negative. The core, every action passes through one chokepoint, thin transports over one non-drifting engine, single-predicate coverage that cannot drift, is the invariant; it is bound, proven to the binding-quality standard, and marked `met`. "No path around it" is recorded NOT as a conjunct claimed proven but as a named remainder: the edit-and-relock hole, recorded at the enforcement surface, with its non-machine backstops (the Sovereign's gate and the continuing duty of reasonable care) named. Under this partition K-1-as-recorded is honestly `met`, with the remainder displayed and travelling with it. This is the cleanest remedy because it keeps `met` meaning exactly "proven" and adds no new term.

Fallback form, a precise new status. Where a universal-negative conjunct cannot be cleanly severed from what the invariant protects, the ledger shall carry a new status, which I name `met-modulo-remainder`, defined exactly as: the machine-checkable content is individually bound and proven to the binding-quality standard, and the residue is a named, recorded, non-machine-backstopped remainder that is unprovable by nature. I stress that coining a new, exactly-defined term is the opposite of the sin I began by refusing. The sin is stretching `met` to cover the unproven. Reserving `met` for full proof and legislating a separate, fenced term for the proven-core-plus-named-remainder case keeps faith with the word. It is naming, not redefining.

The fencing on both forms is mandatory and is the heart of this order, because a humane middle status is exactly the kind of thing that rots into a loophole:

1. The new status, or the partitioned `met`, may issue ONLY on an express finding that the residue is unprovable BY NATURE, a true universal negative or an irreducible external trust root, and not merely unproven for want of effort. A residue that further engineering could close is `partial` or `gap`, not this. This is the line that prevents the status from absorbing genuine unfinished work.

2. The binding gate is undisturbed for every enumerable conjunct. Each provable claim must still have a test that exercises and proves it, not a paper binding. Neither the partition nor the new status relaxes K-29 or K-30 for the parts that can be proven. "Binding debt 0" remains necessary and not sufficient, exactly as the goal-completion audit established.

3. The remainder must be machine-recorded and must travel with the status. Following the enforcement.rs example of candor, the ledger reader must see, at the point of the status, the exact named hole and its non-machine backstop. No silent asterisk; the remainder is part of the entry, not a footnote a reader may miss.

### VI. A rule for the future

Because K-1's difficulty was in part self-inflicted in its drafting, I order a prospective drafting rule. An invariant statement shall not assert a universal negative as a conjunct claimed-proven. Universal negatives, and irreducible external trust roots, shall be recorded as named remainders with a stated non-machine backstop, separate from the machine-provable core. This keeps `met` honest at the point of authorship and spares future invariants the false choice between overclaiming and a `partial` that can never resolve.

---

### DISPOSITION

The question is answered as follows. (1) An invariant whose statement, as written, contains a universal-negative conjunct cannot honestly be recorded `met`, because `met` means proven and a universal negative over an unbounded domain is not proven; the kernel's own recorded remainder confirms it. To that extent Position 1 prevails. (2) But such an invariant is NOT "terminally `partial`," because `partial` falsely connotes unfinished work, whereas the residue is unprovable by nature; Position 1's remedy is rejected as a misclaim that violates the honest-remainder principle in the opposite direction. (3) The defect lies in the four-term vocabulary, which has no truthful word for a proven core plus a named non-machine remainder. Position 2 is correct that a bare `partial` misinforms and that the cure is to re-scope or to extend the vocabulary, and it prevails to that extent; its reductio against `met` generally is rejected by the statement-versus-background distinction.

Status K-1 should carry: NOT bare `met` and NOT bare `partial`. By the primary remedy, K-1's statement is partitioned and the machine-provable mediation-and-coverage core is marked **`met`**, with "no path around it" recorded as a named, enforcement-surface-recorded, non-machine-backstopped remainder. Where partition is not clean, K-1 carries the new status **`met-modulo-remainder`**, defined and fenced as above. Pending implementation of this order, K-1 may stand as `partial` ONLY if annotated with the recorded remainder, and that interim label is itself the reason the amendment is ordered.

Directives:
1. Amend the status vocabulary to add `met-modulo-remainder` with the exact definition stated in Part V, and adopt the statement-partition pattern as the default remedy, the new status as the fallback.
2. The status, or the partitioned `met`, may issue only on an express finding that the residue is unprovable by nature, not merely unfinished; effort-closable residues remain `partial` or `gap`.
3. K-29 and K-30 are undisturbed for every enumerable conjunct: real proving tests, no paper bindings; binding debt 0 remains necessary and not sufficient.
4. The named remainder must be machine-recorded and must travel with the status in the ledger entry, no silent asterisk.
5. Prospective drafting rule: no invariant statement may assert a universal negative as a conjunct claimed-proven; universal negatives and irreducible trust roots are recorded as named remainders with a stated non-machine backstop.

So I hold.

---

## Brightwater J.

I write for myself, and I begin where I always begin: with the person who will one day open this manifest and read down the status column. That reader, an engineer mid-incident or a builder inheriting the kernel a year from now, does not read our jurisprudence. They read four tokens: `met`, `partial`, `gap`, `n/a`. Whatever we hold today, its real life is lived in what those tokens TELL that reader. The question put to us is framed as ontology, can a universal negative ever be proven, but the law that governs us, the honest-remainder principle, is not an ontology. It is a duty of communication: a kernel must record what it cannot prove, and must never overclaim. The test of any disposition here is therefore not "is K-1 metaphysically settled" but "does the ledger entry inform the reader or mislead them." I decide the case on that ground, and on that ground both proffered positions are, in their bare form, defective.

**What each bare status SAYS to the reader.**

Consider `met`. In a coverage ledger disciplined by K-29 and K-30, `met` carries a single, hard meaning to the reader: this claim is fully proven by a bound test that exercises it, rely on it without caveat, there is nothing here a human must still watch. That is the meaning the binding-quality standard purchased for us at real cost. The goal-completion audit tore out weak and paper bindings precisely so that `met` would mean PROVEN and not merely TOUCHED. To stamp bare `met` on K-1 is to spend that purchased trust on a claim the kernel's own enforcement surface confesses it cannot fully carry. The edit-and-relock residue is named, in `enforcement.rs`, as beyond any in-binary check. A reader who sees bare `met` is told "no human backstop needed here" at the exact line where the kernel has written that the only remaining backstops are non-machine: the Sovereign's gate and the continuing duty of care. That is overclaim in the strict sense the honest-remainder principle forbids. Position 1 is right about this much, and I adopt it: K-1, as a conjunction that includes "no path around it," may not carry bare `met`.

Now consider `partial`. Position 1 calls `partial` the honest entry, "by ruling, forever." I cannot agree, and here I part from it decisively. Read `partial` as the reader reads it. In a ledger whose coverage ratchet "only decreases" (K-30), `partial` is not a philosophical confession. It is a WORK SIGNAL. It says: coverage is incomplete here, binding debt remains, come back and close it. It is an instruction to a future builder to pick up a tool. But there is no tool. The residue in K-1 is not unfinished work; it is unfinishable-by-nature work, an author with full write access who edits the gate and re-locks the digest pin, a hole no in-binary check can ever reach. A reader who acts on bare `partial` will do one of two harmful things. Either they will burn effort hunting for a binding that does not exist and cannot exist, or, having learned that this `partial` is permanent noise, they will start discounting `partial` everywhere, eroding the signal across the whole manifest until a REAL unfinished gap somewhere else is read as more permanent-noise. Position 1 calls this honesty. From the reader's chair it is the opposite: it conflates "unprovable by nature" with "not yet done," and that conflation is exactly the misinformation Position 2 identifies. Bare `partial` does not record the remainder honestly. It mis-describes it.

So I hold the uncomfortable middle that neither bare status survives. `met` overclaims (says "proven, no backstop" where a named human backstop is load-bearing). `partial` misdescribes (says "work remains" where no work can ever close it). The honest-remainder principle does not merely permit a better entry here; it COMPELS one, because it requires the remainder to be recorded AS a remainder, and neither existing token does that. The defect is not in K-1. The defect is in a four-token vocabulary that has no word for "everything a machine can check is proven, and what is left is a named hole a human must hold."

**Engaging the strongest argument the other way.**

Position 2's deepest point deserves a real answer, not a dodge: every invariant rests on some irreducible trust, you trust the compiler, the silicon, the key-holder, so if a universal-negative residue bars `met`, then nothing is ever `met` and the vocabulary empties. If that were so, my holding would prove too much and I would retreat. It is not so, and the reason is a principled line the reader can actually apply.

There is a difference in kind between AMBIENT trust and a RECORDED claim-internal remainder. Compiler-trust and hardware-trust are background assumptions shared UNIFORMLY by every claim in the system; they are the substrate on which all assertions rest, and the kernel does not, and should not, record them as a gap in any particular invariant. K-1's residue is different. "No path around it" is a claim ABOUT the absence of bypasses, and the edit-and-relock path is a bypass WITHIN the very domain that claim purports to cover. The kernel's own surface records that specific, named, in-scope hole. That is not ambient trust. It is a foreground remainder internal to what the invariant asserts. The line, then, is clean and reader-usable: if an invariant's residue is only the ambient trust every claim shares, it earns ordinary `met`; if the invariant's own statement asserts a totality the machine cannot close, and the kernel records that specific residue, it earns the modulo status. On that line the vocabulary does not empty. The overwhelming majority of security invariants, whose only residue is ambient substrate-trust, remain plainly `met`. Only the rare invariant that makes a claim-specific universal-negative, and whose hole the kernel itself documents, is treated specially. Position 2 is therefore right that the vocabulary is wrong, and right that re-scoping plus a new status is the cure. It is wrong only in its loosest branch, the suggestion that bare `met` is "defensible." It is not, for the reason given above. I take Position 2's reform prong and reject its bare-met prong.

**The third way, and the guard against abuse.**

The honest entry is to do two things together, because either alone fails. First, RE-SCOPE. The enumerable conjunct (A), every action through one chokepoint, thin transports over a non-drifting engine, bound by the content-driven gate, the single-predicate coverage that cannot drift, raw-write-trips-the-gate, forged-order fail-closed, capability equivalence, deny-dominance, apex routing across both doors, with the once-found coverage gap closed, is fully proven to the binding-quality standard and carries `met` on its own terms, ledgered explicitly. Second, give the residue a token that tells the reader the truth: the machine-checkable content is proven, and what remains is a named, recorded, non-machine-backstopped remainder that no finite test can close. I will call that status `met-modulo-remainder` (I fix its meaning; the precise lexeme is for the drafter, with one hard constraint: it must be VISUALLY DISTINCT from `met`, so no reader scanning a column ever reads "proven, no caveat" where the truth is "proven up to a named human backstop").

Re-scoping without the new token does not work, and I want the record to be clear on why: split K-1 and the residue conjunct still needs a status, and if that status is `partial` we have merely relocated the misdescription. The new token is necessary, not cosmetic.

But Position 1's fear is legitimate and I will not leave the door it warns of open. A new status is exactly the kind of soft word under which lazy bindings hide, and the audit that gave us "binding debt 0 is necessary but not sufficient" is the standing proof that softness breeds paper claims. So `met-modulo-remainder` is available ONLY on three conjunctive conditions, with the burden on the author to earn each:

1. **Full proof of the checkable content.** Every machine-checkable conjunct is bound and PROVEN to the binding-quality standard, exercised, not merely touched. The "modulo" may quarantine only the genuinely uncheckable residue. It may never quarantine unfinished provable work. A modulo status hiding a checkable-but-unbuilt check is a silent stub under K-30 and a breach.

2. **A named, recorded residue.** The remainder must be specific and written into the enforcement surface (as the edit-and-relock hole already is), with the manifest entry LINKING to that record so the reader reaches the exact hole and its backstop in one hop. A residue you cannot name is not a recorded remainder; it is a `gap`. The honest-remainder principle requires recording what you cannot prove, and you can only record what you can name.

3. **Demonstrated non-machine irreducibility, with an identified backstop.** The author must show WHY the residue is uncheckable in principle by an in-binary control, and must name the non-machine backstop that holds it (here: the Sovereign's gate plus the continuing duty of reasonable care). "Unprovable by nature" must be earned, not asserted from inconvenience. If a machine COULD check it, the status is unavailable and K-30 requires the check be built first.

And irreducibility is not a permanent license. If tooling later makes the residue machine-checkable, the modulo status expires by its own terms and the check must be built. This keeps the token from ossifying into a standing excuse, and keeps its meaning fixed for the reader: "uncheckable today, by nature, with a named human backstop, and re-examined."

This disposition honors the assented-record floor as persuasive on its true theme. The floor teaches machine humility: machinery records its own limits and routes for correction rather than asserting a totality it lacks. The modulo status is that humility applied to the ledger. It is the machine saying, in the one place the reader will look, "here is exactly how far I carry this claim, and here is where the human takes over." That is not a weaker record than `met`. For the engineer holding the pager, it is a stronger one, because it tells them the truth about where the backstop lives.

**On the literal question.** Can K-1 ever be `met`? Not in its bare form, and not because the engineering is unfinished, but because bare `met` would tell the reader something false about the backstop. Is it therefore terminally `partial`? No, and emphatically not by ruling, because `partial` tells the reader something equally false in the other direction and would misdirect their effort and corrode the token elsewhere. The law does not force us to choose between two lies. It forbids both and requires the true word.

---

**DISPOSITION.**

K-1 shall carry **`met-modulo-remainder`**, NOT bare `met` and NOT `partial`. Concretely: its enumerable mediation-and-coverage conjunct is recorded `met`, proven to the binding-quality standard; the integrated invariant carries `met-modulo-remainder`, with the edit-and-relock residue recorded as a named, Sovereign-and-duty-of-care-backstopped remainder.

**DIRECTIVES.**

1. **Re-scope K-1** in the manifest into (A) the proven mediation-and-coverage conjunct, ledgered `met`, and (B) the recorded "no path around it" residue.

2. **Extend the status vocabulary** to `met | met-modulo-remainder | partial | gap | n/a`. Fix the new value's meaning as: "all machine-checkable content is proven, and the residue is a named, recorded, non-machine-backstopped remainder no finite test can close." The lexeme must be visually distinct from `met`.

3. **Gate the new status** on the three conjunctive conditions above (full proof of checkable content; named residue recorded in the enforcement surface and linked from the manifest; demonstrated non-machine irreducibility with an identified backstop), burden on the author, audited under the binding-quality standard. A `met-modulo-remainder` whose "remainder" is in fact unfinished checkable work is a paper claim and a breach to be made good.

4. **General rule for universal-negative invariants.** A universal-negative conjunct may never carry bare `met` (overclaim) and may never carry bare `partial` solely by virtue of its universal-negativity (misdescription). It must be re-scoped, with its residue carried under `met-modulo-remainder` and a named backstop; or, where the residue is in truth machine-closable, the check must be built (K-30) before any status is assigned.

5. **Expiry of irreducibility.** When advancing tooling renders a recorded remainder machine-checkable, the `met-modulo-remainder` status lapses and K-30 requires the check be built; periodic audit shall re-test that every modulo residue remains genuinely irreducible.

The honest ledger is not the one that confesses "forever unfinished" where no work remains. It is the one that tells its reader exactly how far the machine reaches and precisely where the human stands guard.

---

## Calderon J.

I write separately and decide independently. The question put to the Council is framed as a contest between "terminally `partial`" and "some form of `met`," but the law and, more to the point, the logic compel me to hold that both framings share a single concealed error. That error is a category mistake about what a status token is and what proposition it is being asked to predicate. Once the mistake is named, the answer follows, and it is the less comfortable of the two offered answers: K-1 is neither terminally `partial` nor baldly `met`. It belongs at a status the present vocabulary cannot yet express, and the remedy is to give the ledger the vocabulary it lacks.

### I. The epistemics, stated plainly

A status in this ledger is a predicate applied to a proposition: the proposition is the invariant's statement, and the predicate (`met`, `partial`, `gap`, `n/a`) asserts something about our epistemic relation to that statement. The first discipline of any such system is that the predicate must match the type of the proposition. You cannot honestly apply a single epistemic predicate to a proposition that is, in fact, two propositions of different epistemic kind welded by an "and."

K-1 is exactly such a weld. Its statement is a conjunction A ∧ B:

- A is an enumerable, positively-quantified claim over a domain the machine can reach and inspect: every action transits one chokepoint, the front doors are thin transports over one engine, coverage flows from a single predicate (`front_door::is_governed_record`) so it cannot drift. A is decidable. It is discharged by deterministic tests that inspect the staged diff rather than the caller's claim, and the one place A was incompletely proven (the gate covering only the canon order tree, not all governed records) was a closeable shortfall in A's domain coverage, found and closed.

- B is a universal negative: "no path around it," that is, ¬∃ a bypass. This is a claim of a wholly different epistemic type.

The asymmetry between A and B is the oldest result in this area and I will not pretend it is soft. A universal negative over an open domain is not verifiable by enumeration; it is only falsifiable by a witness. You discharge ¬∃x P(x) either by closing the domain and checking every x, or not at all. One counterexample refutes it; no finite quantity of confirming instances proves it. So if B quantified over a genuinely open domain with no closure available, Position 1 would be unanswerable and I would join it.

### II. Why B is not, in fact, an open quantifier - the named-closure distinction

It is not unanswerable, because B is not one proposition either. B itself decomposes:

- B_modeled: "no path around it, within the paths the machine can reach." This is a universal negative over a *closed, finite, enumerable* domain, and universal negatives over closed finite domains are perfectly provable. The single-predicate coverage construction is precisely a domain-closure proof: it makes the set of governed records enumerable and machine-witnessed, so that "no governed record escapes the gate" is checkable rather than merely hoped. The canon-order-tree shortfall was a hole in this closure, and its closure was the act of completing B_modeled. B_modeled is, today, proven.

- B_residual: "no path around it, *including* a path that rewrites the checker and re-locks the digest pin." enforcement.rs records this candidly: an author with full write access who edits the gate and re-locks the pin is beyond any in-binary check. This residue is not machine-closeable by nature, because the actor it contemplates can rewrite the very machine that would close it. Over that domain, B_residual is unprovable by the machine - genuinely, not contingently.

Here is the move the Council must see clearly, and it is where I part from the seductive simplicity of Position 1. The line between an honest `met` and the forbidden paper claim is **not** the line between machine-witnessed closure and human closure. It is the line between **named** closure and **hidden** closure.

Every proof closes its domain somewhere. A machine proof closes it by enumeration over a predicate. A mathematical theorem closes it by stating axioms. A security invariant closes it by declaring a trust base: you trust the compiler, you trust the silicon, you trust the holder of the key. None of these closures is "fully proven against an unbounded adversary." A theorem is not unproven because it rests on axioms; it is proven *relative to its stated axioms*, and that is the only sense in which anything is ever proven. The compiler-trust and key-holder-trust that Position 2 invokes are real and they are correct: they are not gaps, they are *named premises*. A claim proven relative to a named, minimal, recorded premise is met in the only sense the word has ever honestly carried.

B_residual, properly handled, is exactly such a premise. "No path around it, given a trusted author who does not rewrite the gate and re-lock the pin" is a claim with a declared trust base, and the kernel has declared it: the Sovereign's gate and the continuing duty of reasonable care are the named non-machine backstops. Once the residual actor is named and placed outside the trust base, B_residual stops being an open quantifier and becomes a closed-by-axiom premise. The closure is *named*. That is what the binding-quality standard actually polices: it forbids *hidden* closures (a test that touches an area and silently pretends to have proven the claim), not *named* human closures (a backstop the enforcement surface openly records). K-1's closure of B_residual is named on the very surface the discipline points us to. To stamp it `met` with that named remainder is therefore not the paper claim the standard forbids; it is the standard's own ideal of honest recording.

### III. Why "terminally `partial`" is the inverse paper claim

Position 1 says `partial` forever is the honest entry and the honesty is a strength. I hold the opposite, and on Position 1's own chosen ground of honesty.

`partial` is a *progress* word. In a ratcheting ledger governed by K-30, `partial` means "work remains and the coverage figure can and should rise." It is a temporal promise: come back, this is unfinished, allocate effort and it will close. Applied to B_residual, that promise is false. There is no finishable work behind B_residual; it is unprovable by nature, not undone by neglect. To mark it `partial` is to assert the existence of pending, closeable work that does not exist. That is an overclaim - the *inverse* of the overclaim Position 1 fears, but an overclaim all the same, and the honest-remainder principle cuts against overclaiming in *both* directions.

The damage is not merely abstract. A reader triaging the ledger sees `partial` and routes effort to "finish" K-1. That effort is structurally wasted. Worse, a permanent `partial` is indistinguishable in the record from a genuinely unfinished `partial`, so it corrodes the signal value of `partial` for every other invariant: the token comes to mean "either nearly done or never done, indeterminable," which is to say it comes to mean very little. Honesty in a record is measured by the accuracy of the signal the reader receives, not by the modesty of the word the author chooses. By that measure, `partial`-forever is the *less* honest option, because it transmits a false and corrosive signal under cover of a humble-sounding word.

Bald `met` over the unqualified conjunction fails by the same metric in the opposite direction: it suppresses the named remainder and invites the reader to believe B is machine-closed when it is axiom-closed. Same information failure, opposite sign.

### IV. The assented-record floor on machine humility

The assented-record floor is persuasive here, not controlling, but it points the same way. Its lesson is that machinery should record and route, never silently void what it cannot itself adjudicate. The analogue: the ledger's machinery should neither silently *stamp* a universal negative as fully machine-proven (the machine overclaiming reach it does not have) nor *forever block* the invariant at `partial` (the machine refusing to recognize a discharge that lies, by the system's own design, with a named human backstop). The humble machine records the boundary of its own competence and hands the residue, by name, to the backstop that holds it. `partial`-forever is the machine pretending the residue is its own unfinished business. Bald `met` is the machine pretending the residue does not exist. The honest record is the machine saying: "I have proven everything I can reach; here is the one thing I cannot reach, and here is who holds it."

### V. The danger, and the gate that contains it

I will not pretend the answer I reach is free of hazard. Its hazard is the laundromat: an author who relabels ordinary unfinished work "irreducible by nature" to escape the K-30 ratchet. This is precisely the abuse the binding-quality standard exists to forbid, and it is the place my logician's distinction must bite hardest. The defense is to make irreducibility an *affirmative, reviewable burden*, with the default running the other way.

Every residue is presumed `gap`/`partial` - closeable, ratchet-bound - unless and until the author *demonstrates*, on the record, that it is a universal negative over a domain that includes the checker or the trust base itself, with no machine closure available. The canon-order-tree shortfall is the worked counter-example and it disciplines the whole rule: it was closeable, it was closed, and it would *fail* any irreducibility test, so it could never have been laundered into the new status. B_residual passes the test on its face, because the actor it contemplates rewrites the checker. The discipline, in one line, is the *sorting*: closeable residue stays in the progress vocabulary and under the ratchet; irreducible-by-nature residue leaves the progress vocabulary entirely.

That last clause is the heart of my holding. The error both positions share is asking the *progress* vocabulary (met/partial/gap) to encode an *epistemic-type* fact (unprovability by nature) that progress vocabulary is structurally unfit to carry. The cure is not a better progress word. It is a second register. The progress register answers "is the closeable work done?" The trust register answers "what named, irreducible premises does this rest on?" K-1's progress register reads `met` - earned, and earned only because the modeled coverage gap was actually closed. K-1's trust register carries one named, backstopped remainder. Note the consequence, which I regard as a feature: the universal-negative conjunct does not by itself condemn K-1 to `partial`, and does not by itself entitle it to `met`. The status tracks the closeable work; the irreducible residue is handled by the backstop, off the progress axis, where it cannot lie about pending effort.

### VI. DISPOSITION

I hold for a third way. Position 1 is rejected: a universal-negative conjunct does not condemn K-1 to `partial` forever, and `partial`-forever is itself an overclaim (a false promise of pending work) that the honest-record principle forbids. Position 2 is adopted in its disciplined form: the present vocabulary is wrong, and the cure is to separate the progress register from the trust register rather than to overload either token.

**Status K-1 should carry:** On the progress register, `met` - displayed with a mandatory remainder marker (I will write it `met†`) that links to a recorded, named residual. On the trust register, the residual conjunct ("no path around it," edit-and-relock) carries the new terminal status I define below. K-1 reads, in full: *`met†` - mediation and coverage proven over the full governed-record domain; one `backstopped` remainder (gate-edit-and-relock), owner: Sovereign's gate + continuing duty of care.* It earns this only because the modeled coverage gap was in fact closed; absent that closure it would read `gap`/`partial`, not on account of the universal negative but on account of unfinished closeable work.

**I order the following directives:**

- **D1 - Vocabulary amendment.** Extend the enum {met | partial | gap | n/a} with a terminal, non-progress status `backstopped`: "true relative to a named non-machine premise; not machine-decidable by nature; not subject to the K-30 closeability ratchet; requires a recorded backstop owner." Add a composite roll-up marker (`met†`) meaning every conjunct is `met` or `backstopped`, with at least one `backstopped` conjunct and a mandatory link to the remainder register.

- **D2 - Decompose at the conjunction.** Any invariant whose statement contains a universal-negative conjunct must be split in the ledger into its machine-decidable part (governed by met/partial/gap and the ratchet) and its residual part (governed by D3). A single status may never be predicated of a heterogeneous conjunction. K-1 is to be recorded as A ∧ B_modeled → `met`, B_residual → `backstopped`.

- **D3 - The irreducibility burden (anti-laundromat).** No residue may be classed `backstopped` without a recorded demonstration that it is a universal negative over a domain that includes the checker or the trust base, with no machine closure available. The default classification of every residue is `gap`/`partial`. Reclassification to `backstopped` is an affirmative burden on the author, reviewable, and is itself a binding-quality question. The canon-order-tree shortfall is the governing example of a residue that fails this test and must remain on the progress axis.

- **D4 - Named backstop required.** Every `backstopped` residue must name its backstop owner and mechanism on the enforcement surface, as enforcement.rs already does for the edit-and-relock hole. An unnamed (hidden) closure is not `backstopped`; it falls back to `gap`. The forbidden paper claim is the *hidden* closure, not the *named* human one.

- **D5 - Binding-quality reaffirmed.** A test bound to a `met` conjunct must prove that conjunct over its full declared domain, not merely touch its area; the canon-order-tree closure is precedent that `met` requires domain-complete proof. "Binding debt 0" remains necessary and not sufficient. `backstopped` conjuncts are exempt from the test-binding requirement precisely because they are exempt from machine proof, and in exchange they bear the D3 burden and the D4 naming requirement - the trust register's price of admission.

The general rule I lay down for all future universal-negative invariants: the progress vocabulary governs only what the machine can close; what the machine cannot close by nature is removed from the progress vocabulary and recorded, by name, in the trust register. A universal negative is neither a permanent failure of work nor a thing to be silently waved through as done. It is a named premise, and a record that names its premises is the most honest record there is.

---

## Donhill J.

**I. The question, and the reason it cannot be answered by reflex.**

We are asked whether an invariant one of whose conjuncts is a universal negative - K-1's "no path around it" - can ever be recorded `met`, or whether it is condemned to `partial` in perpetuity. The two positions put to us are each half right, which is the surest sign that the fault lies not in the contest between them but in the instrument they are both forced to write on. My discipline on this bench is to watch the precedent both positions would set. Position 1 would hand us a status no invariant of this class could ever attain, draining `met` of meaning for the hardest and most important claims a kernel makes. Position 2 would hand us a `met` reachable by editing the claim until the inconvenient part is gone, which is the precise gaming the binding-quality standard exists to forbid, performed one level up - on the scope rather than on the test. I will not adopt either. The law compels a third way, and I am satisfied it does so without my having to invent comfort where the statute is silent.

**II. The vocabulary conflates two axes, and that is the disease.**

The ledger today offers `met | partial | gap | n/a`. Three of those four words describe a single axis: how much of the buildable WORK is done. `gap` means the work is missing; `partial` means some is done and some is not; `met` means it is all done and proven. The hidden premise is that incompleteness is always temporal - that whatever is not yet `met` could become `met` if someone kept working. That premise is true for ordinary invariants and false for K-1's second conjunct.

A universal negative over an open domain is not incomplete WORK. It is a claim of a different epistemic kind. A coverage test proves the modeled paths and is structurally silent on an unmodeled one; no quantity of further testing closes a gap that is defined by what has not been modeled. So K-1 sits across two axes at once: on the work axis its enumerable part is finished, while its universal-negative part is not "unfinished" but unsettleable-by-machine. The four-word vocabulary has no cell for "the buildable work is complete and what remains is not buildable at all." Forced to choose, the ledger must mislabel in one of two directions. The right judicial response to an instrument that compels a lie is to fix the instrument, not to pick the lie we find less embarrassing.

**III. Bare `met` is barred. Position 2's met-by-rescope is rejected.**

`met` reads, at a glance, as "done and proven," and a glance is what a ledger is for. To stamp `met` on a claim containing an unproven universal negative is to assert proof that does not exist - the paper claim the binding-quality standard was written to forbid. That the enumerable conjuncts are proven to that standard does not rescue it, because K-1's STATEMENT includes "no path around it," and a status attaches to the statement as written.

Position 2 offers an escape: re-scope the statement, bind and mark `met` the mediation-and-coverage claim, and move "no path around it" out of the invariant into a recorded assumption. I reject this as a general license, and I reject it sharply, because it is the most dangerous proposal in the file. If the cure for an unprovable conjunct is to delete it from the invariant's text and mark the remainder `met`, then every weak claim in the catalogue can be brought to `met` by amputating whatever it cannot prove. The discipline already teaches that "binding debt 0" is necessary but not sufficient because parties game the binding by touching rather than proving. Re-scope-to-met is the identical game played on the scope: game the boundary of the claim instead of the strength of the test. An invariant's text is not editorial; narrowing it is an amendment, and an amendment that makes a hard promise disappear from the record is the worst outcome available to us, because the system would then stop tracking its most important and least provable promise precisely because it is hard. I will permit decomposition, as Part V explains, but never silent amputation, and never decomposition that lets the residue vanish.

**IV. Bare `partial`, forever, is also barred. Position 1 is rejected.**

Position 1 wears the costume of humility: leave it `partial`, that honesty is a strength. I do not accept that honesty is achieved by always choosing the more pessimistic of two inaccurate words. `partial` carries a meaning on this ledger - "work remains" - and that meaning is false of K-1. The mediation work is done; the coverage gap that once existed (the gate covering only the canon order tree) was found and CLOSED; the residue is not pending labor but an irreducible non-machine remainder that the kernel's own enforcement surface candidly names. To label that `partial` tells the reader to keep going on something that cannot be advanced. That is not humility. It is imprecision wearing humility's coat, and it does active harm: it buries a named, permanent, human-backstopped remainder under the generic signal "not finished yet," and over time it invites someone to "finish" K-1 by writing one more test - chasing a target that recedes by nature, wasting effort, and eventually tempting exactly the paper test the standard forbids, written to make a perpetual `partial` go green. The honest-remainder principle does not say "record everything as incomplete." It says record what you cannot prove, and why, and label the rest done. Bare `partial` fails that principle from the other side: it disguises an irreducible residue as transient incompleteness and thereby fails to record what the limit actually is.

**V. The third way: a precise status for proven-up-to-a-named-remainder, gated against abuse.**

Both bare words mislabel because the vocabulary lacks a cell. I order that cell created. Its meaning: the machine-checkable content of the invariant is proven to the binding-quality standard, and the residue is a named, particular, irreducible-by-nature remainder discharged to a recorded non-machine backstop. The proposed lexeme `met-modulo-recorded-remainder` captures it; I will use `met-with-recorded-remainder` in this opinion and leave the exact token to the registrar, subject to three non-negotiable properties: it must be visibly distinct from `met`, it must be incapable of being read as "fully proven," and it must carry a mandatory pointer to the recorded remainder. The token must make the reader stop, not relax. That is why I will not let this collapse into an annotated `met` (a reader scanning a column of `met` stops reading - the remainder becomes a paper claim with extra steps) nor into an annotated `partial` (which mislabels an irreducible residue as in-progress at the same glance).

This status is available only on a five-part showing, recorded, and each part carries weight:

1. **Decomposition on the record.** The statement is shown to split into a machine-checkable part and a residue, with the boundary stated explicitly and reviewably. The reader must be able to see exactly where the machine stops. A boundary asserted is not a boundary shown.

2. **The machine-checkable part is fully met to the binding-quality standard.** Every enumerable conjunct has a test that exercises and PROVES it, coverage derived from a single non-drifting predicate, no binding debt, no open gap.

3. **The residue is irreducible by nature, not merely not-yet-built.** This is the load-bearing anti-gaming gate. The party must show that no finite in-system check could settle the residue - that it quantifies over an open or unmodeled domain, or rests on a trust root outside the machine's reach. "We have not written the test yet" fails this gate and the status stays `gap` or `partial`. "No test can exist because the claim ranges over paths the model does not enumerate" passes.

4. **The residue is named and backstopped on the record** by a specified non-machine mechanism, so the reader knows who carries the unprovable promise and how.

5. **The residue is particular to this invariant, not an ambient substrate trust.** This guards Position 2's overreach. It is true, as Position 2 says, that one trusts the compiler, the hardware, the key-holder. But that argument proves too much: if ambient trust qualified everything, the new status would be as empty as Position 1's `partial` and Position 2 fairly warns `met` would be. Substrate-wide trust assumptions are recorded once, at the substrate level, not smuggled into each invariant as a per-claim escape hatch. Only a residue specific to and load-bearing for THIS claim's truth qualifies.

Where all five hold, the status is neither `met` (which would overclaim the residue as proven) nor bare `partial` (which would underclaim the work and mislabel the residue), but `met-with-recorded-remainder`.

**VI. The slope runs both ways, and the gates are cut to hold it.**

My institutional fear was that a `met` reachable by re-scoping lets every weak claim climb to `met`. Gate 3 (irreducibility) and the bar on silent amputation in Part III answer that fear: a claim that is merely incomplete cannot reach this status at all, because its residue is buildable and therefore fails gate 3; and a claim cannot reach `met` by deleting its hard conjunct, because narrowing the text is an amendment that must preserve the excised conjunct as a standing obligation. My opposite fear was a status no invariant attains; gates 1, 2 and 4 answer that, because a kernel that has genuinely done the buildable work and honestly named its residue CAN attain it. The middle is not mush; it is fenced on both sides.

**VII. Application to K-1.**

On the agreed facts every gate is satisfied. The enumerable part - every action through one chokepoint, thin front doors over one engine - is proven to the binding-quality standard: a content-driven gate that inspects the staged diff rather than the caller's claim, coverage from the single predicate `front_door::is_governed_record` so it cannot drift, a raw-write-skips-every-verb test that still trips the integrity gate, forged-order fail-closed, capability equivalence, deny-dominance, and apex routing across both doors, with the one historical coverage gap found and closed. The residue - "no path around it" against an author with full write access who edits a gate and re-locks the digest pin - is irreducible by nature (no in-binary check can settle it), particular to K-1, named on the kernel's own enforcement surface, and backstopped by recorded non-machine mechanisms: the Sovereign's gate and the continuing duty of reasonable care. K-1 therefore carries `met-with-recorded-remainder`. It is not terminally `partial`, and it is not bare `met`. The true core of each position is vindicated and the false core of each is refused.

**VIII. Consonance with the governing law.**

This holding is what the honest-remainder principle compels: record exactly what cannot be proven and why, and label the rest done - neither overclaiming (the bar on `met`) nor mislabeling (the bar on `partial`). It honors K-29 as quality, not box-ticking: the existence of a bound test by name never suffices; the test must prove its conjunct, and the universal negative has no such test and so is removed from the `met` column by operation of the standard, not by grace. It honors K-30: the machine-checkable part keeps its coverage figure and the ratchet still binds it; the remainder is recorded ALONGSIDE the coverage number, never in its place, and may never be used to paper over a coverage decrease in the enumerable part. The remainder is the opposite of a silent stub - it is a loud, named non-stub, which is what K-30 wants. And the assented-record floor, persuasive here on its theme of machine humility, supports the same instinct: machinery that declares the limit of its own competence and hands the residue to a human backstop, rather than pretending either to have closed it or to be forever mid-task, is the machine knowing its place.

**IX. Directives.**

I would order the following, as the ratio of this judgment:

1. **Create the status.** Add to the ledger vocabulary a status meaning "machine-checkable content proven to the binding-quality standard, with a named, particular, irreducible, human-backstopped residue." Registrar to fix the token subject to the three properties in Part V.

2. **Record K-1 under it,** with an explicit pointer to the enforcement-surface entry that names the edit-and-relock remainder and to the non-machine backstops.

3. **Rule for future universal-negative or open-domain conjuncts.** (a) Such a conjunct may never be recorded `met` on the strength of a coverage test, coverage proving only modeled paths. (b) It may not be silently re-scoped out of an invariant to manufacture a `met`; any narrowing of an invariant's text is an amendment, recorded as such, that must preserve the excised conjunct as a named standing obligation. (c) It qualifies for `met-with-recorded-remainder` only on the recorded five-part showing of Part V. (d) Absent that showing - where the residue is not-yet-built rather than unprovable-by-nature - the status remains `gap` or `partial` as the work state dictates, and "binding debt 0" does not upgrade it.

4. **Documentation requirement.** Every entry in the new status must carry, on the record, its decomposition boundary, the proof of the machine-checkable part, the irreducibility ground, and the named backstop. A bare token without those four is itself a paper claim and is forbidden.

**DISPOSITION.**

The status K-1 should carry is **`met-with-recorded-remainder`** (the substance of Position 2's `met-modulo-recorded-remainder`), NOT bare `met` and NOT terminally `partial`. Position 1 is rejected: perpetual `partial` is a distinct dishonesty that mislabels an irreducible residue as pending work. Position 2 is accepted in substance but its re-scope-to-`met` route is rejected: an invariant's text may not be silently amputated to reach `met`. I direct the creation of the new status defined in Parts V and IX, governed by the five-part showing and the four directives above, with the irreducibility and particularity gates as the binding guard against both slopes: weak claims cannot climb to it, and no honest invariant of this class is condemned to a status it can never attain.

---

## Everand J.

I write narrowly, because the case can be disposed of narrowly, and because the discipline we administer is corroded as much by over-ruling as by under-ruling. The question put to us looks like a forced choice between two unhappy stamps on a single ledger row. It is not. The unhappiness is a symptom, and the disease is upstream of the vocabulary. Once the disease is named, the cure is small, clean, and citable, and it does not require us to invent a new word.

**I. The defect is in the statement, not first in the status code.**

K-1 is pleaded as a conjunction. Conjunct (A) is enumerable and positive: every action passes through one chokepoint; the front doors are thin transports over one engine that cannot drift. That conjunct is bound by deterministic tests that do not take the caller's word for anything: a content-driven gate that inspects the staged diff, a coverage predicate (`front_door::is_governed_record`) from which coverage is derived so it cannot silently narrow, a raw-write-skips-every-verb test that still trips the integrity gate, forged-order fail-closed, capability equivalence, deny-dominance, apex routing across both doors. A real coverage gap was found and closed. Conjunct (B) is a universal negative: "no path around it" - the assertion that no unmodeled bypass exists anywhere.

A status code is a certification about one proposition. The four words we have - met, partial, gap, n/a - each presuppose a single coherent claim the code is true *of*. Ask one code to summarize a conjunction of a proven positive claim *and* an unsettleable universal negative, and no honest code exists, because the two conjuncts have different truth-makers. That is the whole of the parties' quarrel. Position 1 is correct that you cannot stamp the negative `met`. Position 2 is correct that stamping the whole thing `partial` says something false about the positive. Both are correct, and both are correct because the row is miscompounded. You cannot legislate a single honest code for an incoherent subject; you fix the subject.

**II. K-29, read with the binding-quality gloss, already excludes universal negatives from the bindable set.**

The doctrinal hook is in our own law, and it is decisive. K-29 binds an invariant to a test; the binding-quality standard - established in the goal-completion audit and now the reason "binding debt 0" is necessary but not sufficient - requires that the bound test actually *prove* the claim, not merely touch its area. A test that exists but does not prove is the paper claim the discipline exists to forbid.

Apply that standard honestly to "no path around it." A finite test exercises modeled paths. It is, by construction, silent on an unmodeled path - that is what "unmodeled" means. No finite test can ever prove a universal negative over an open domain. Therefore no test can ever satisfy the binding-quality standard *for* a universal negative. The conclusion is not that we should relax the standard for this one hard claim. The conclusion is that a universal negative is **binding-ineligible**: it can never be the subject of a K-29 binding that meets quality, ever, by its nature. It was a category error to seat it as a status-bearing invariant row at all. The kernel half-knows this already: enforcement.rs candidly records the irreducible remainder (the author who edits a gate and re-locks the digest pin) and names its backstops as non-machine - the Sovereign's gate and the continuing duty of reasonable care. That is the right register for the negative. The ledger of bound invariants is the wrong one.

So I do not reach the parties' fight on its own terms. I dispose of it by **severance**, which K-29 already compels once read with the quality gloss.

**III. Why I reject "terminally partial by ruling."**

Position 1's honesty is real but it is aimed at the wrong target. Freezing K-1 at `partial` forever does not record humility about the negative; it records a falsehood about the positive. `partial` means *work remains*. On the enumerable conjunct, the work is done: the predicate is single-sourced, the gate is content-driven, the coverage gap was found and closed, the binding is quality-verified. To tell every future reader "incomplete" of a complete thing is itself a paper claim, merely inverted, and it is the more corrosive of the two because it disables the instrument. K-30 gives `partial` its meaning by making the ratchet move only one way; a row pinned at `partial` by ruling can never be distinguished from a row that is genuinely unfinished, and so the signal that the ratchet exists to carry is destroyed. A status that can never change is not a status. I will not order the record to lie in the name of candour.

**IV. Why I reject the bare "just mark it met," and why I also decline the new word.**

The weak form of Position 2 - stamp the conjunction `met` - certifies the negative and is exactly the paper claim the quality standard forbids. That is out.

The strong form proposes a re-scope (bind and meet the enumerable claim, quarantine the negative) and, in the alternative, a new status: `met-modulo-recorded-remainder`. I adopt the re-scope. I decline the new word, and I decline it on minimalist grounds that are not mere taste. Severance plus the existing remainder register already delivers every ounce of honesty the new word was meant to carry: the positive claim reads `met` because it is proven; the negative reads as a named, human-backstopped remainder because it is one. Minting a fifth status to describe a residue we have already agreed to record elsewhere duplicates the register inside the vocabulary and invites the next author to reach for the soft code rather than do the severance. A four-word vocabulary that forces the hard question ("is this a provable proposition or a remainder?") is worth more than a five-word vocabulary that lets the author dodge it. We do not expand the lexicon to paper over a modeling error we can simply correct.

**V. The "empties the vocabulary" worry, answered with a bright line.**

Position 2's strongest point deserves a clean answer: if any irreducible trust assumption bars `met`, then nothing is ever `met` - you trust the compiler, the silicon, the human holding the key - and the word is dead. Correct, and that is precisely why the line must be drawn at the *statement*, not at the *existence of trust*.

Two things must be kept apart:

- **Ambient trust** (compiler, hardware, key-holder) is the substrate every invariant shares. It is not asserted by any one invariant's statement; it is the floor they all stand on. It has never barred `met` and does not now, because the invariant does not *certify* it.
- **An asserted universal negative** ("no path around it") is written *into the statement*. By writing it, the author makes it part of what `met` would certify - and that part is uncertifiable.

The defect, then, is authorship: do not write into a binding-eligible statement a claim the binding cannot ever prove. State invariants positively and enumerably (what is mediated, what is covered, what the gate proves on the staged diff). The universal negative is the *spirit* those positive invariants serve - the north star, not a checkable row. Recorded in the remainder register with its backstop named, it misleads no one; written into a status-bearing row, it makes every honest code a lie. That bright line preserves `met` for the ordinary security invariant (ambient trust and all) and denies it only to the asserted-negative, which severance removes.

**VI. The assented-record floor, as persuasive theme.**

The floor is not controlling here, but its grain runs with this result: machinery records what it cannot prove and routes for correction rather than voiding or overclaiming. The remainder register is the machine being humble in the structurally correct place - admitting the edit-and-relock hole on the kernel's own surface - rather than smearing that humility across a proven positive claim and calling the whole thing unfinished.

**DISPOSITION**

The question is disposed of by **severance**, and on the merits neither position prevails in its strong form.

1. **Status K-1 should carry: `met`** - on its restated, severed statement (the enumerable mediation-and-coverage conjunct), which is proven, whose coverage gap was found and closed, and whose binding is quality-verified. **Not `partial`.** A row may not be pinned at `partial` by ruling; `partial` is reserved for work that remains, and here none does.

2. **The "no path around it" conjunct is binding-ineligible** and carries no met/partial/gap/n/a code. Under K-29 read with the binding-quality standard, no finite test can ever prove a universal negative, so none can be bound to it; it was a category error to seat it as a status-bearing invariant. It is moved to the honest-remainder register (where enforcement.rs already logs the edit-and-relock hole), with its non-machine backstops named expressly: the Sovereign's gate and the continuing duty of reasonable care.

**DIRECTIVE (ordered):**

- (a) **Sever and restate** K-1: the positive, enumerable mediation/coverage invariant carries `met`; the universal negative is removed from its statement.
- (b) **Future rule for universal-negative invariants (the citable ratio):** No binding-eligible invariant statement may contain a universal-negative conjunct. A universal negative must be (i) decomposed into positive, enumerable, test-provable claims, each carrying its own status, and (ii) its irreducible residue recorded in the honest-remainder register with its non-machine backstop named. A test may not be bound under K-29 to a universal negative, because the binding-quality standard requires proof and no finite test proves a universal negative.
- (c) **Annotation, not a new code:** a `met` invariant whose subject matter abuts a recorded remainder must carry a visible reference to that remainder entry, so `met` is never misread as "trust nothing." This is a documentation duty discharged with the four existing codes.
- (d) **I decline to mint** `met-modulo-recorded-remainder` or any fifth status. Severance plus the remainder register plus the annotation duty achieve full honesty without expanding the lexicon, and a forcing four-word vocabulary is the stronger instrument.
- (e) **Bright line preserved:** ambient trust assumptions (compiler, hardware, key-holder), which are not asserted in any invariant's statement, do not bar `met`; only a universal negative written into the statement does, and severance removes it. The vocabulary is therefore not emptied; it is disciplined.

---

## Disposition of the Court

The leading judgment is delivered by **Brightwater J.**, with whom **Aldermere, Calderon, and Donhill JJ.** agree as to the order; **Everand J.** concurs in the core ratio and dissents in part as to the vocabulary remedy. The Court holds, on a symmetric case file and owing no deference to either pleaded position:

**1. Unanimously (5-0):** the question presents a false binary. K-1, as written, may carry NEITHER bare `met` NOR terminal `partial`.
  - Bare `met` is barred: it would certify a universal negative ("no path around it") that no finite test proves, the precise paper claim the binding-quality standard forbids, and the kernel's own enforcement surface records the irreducible edit-and-relock remainder.
  - Terminal `partial` is equally barred: `partial` means "work remains" and falsely signals unfinished effort where the machine-checkable work is complete and the residue is unprovable BY NATURE. That is the inverse overclaim, offending the honest-remainder principle from the opposite side and corroding the K-30 ratchet's signal.

**2. Unanimously (5-0):** the invariant must be decomposed. Its enumerable mediation-and-coverage core (every action through one chokepoint; thin transports over one non-drifting engine; coverage derived from a single predicate so it cannot drift; the once-found coverage gap closed) is proven to the binding-quality standard and is recorded `met`. The universal-negative conjunct "no path around it" is severed and recorded as a NAMED, non-machine-backstopped remainder (the gate-edit-and-relock hole), its backstops named: the Sovereign's gate and the continuing duty of reasonable care. The remainder travels with the row; it is never a silent asterisk.

**3. By majority (4-1; Everand J. dissenting in part):** the integrated invariant carries a new ledger status, **`met-modulo-remainder`**, defined as: every machine-checkable conjunct is individually bound and proven to the binding-quality standard, and the residue is a named, recorded, non-machine-backstopped remainder that is unprovable by nature. Everand J. would achieve the identical honesty by severance under the existing four codes (the proven core reads `met`; the universal negative is binding-ineligible and lives only in the remainder register) plus a mandatory annotation duty, declining to mint a fifth token; the majority adopts the token so that the remainder is visibly inseparable from the row a reader scans.

**4. The new status is fenced against abuse (the anti-laundromat gate), unanimously as to its necessity:** `met-modulo-remainder` issues ONLY on an express, recorded showing of (i) full proof of every machine-checkable conjunct to the binding-quality standard (no paper binding; K-29/K-30 undisturbed; "binding debt 0" remains necessary and not sufficient); (ii) a named residue recorded on the enforcement surface and linked from the manifest; and (iii) a demonstrated finding that the residue is unprovable BY NATURE (a true universal negative, or an irreducible claim-internal trust root), not merely unfinished. A residue that further engineering could close remains `partial` or `gap`. The status lapses if tooling later makes the residue machine-checkable.

**5. The citable ratio for the future (prospective drafting rule), unanimously:** no binding-eligible invariant statement may assert a universal negative as a conjunct claimed-proven. Such a conjunct must be decomposed into positive, enumerable, test-provable claims (each carrying its own status under the ordinary vocabulary and the K-30 ratchet), and its irreducible residue recorded in the honest-remainder register with a named non-machine backstop. Ambient trust shared by every claim (compiler, hardware, key-holder) is not asserted in any statement and does not bar `met`; only a universal negative written INTO a statement does, and decomposition removes it.

**Order accordingly.** K-1 is recorded `met-modulo-remainder`. The status vocabulary is amended. The directives are set out in the accompanying order [2026] VJS-SC 7.
