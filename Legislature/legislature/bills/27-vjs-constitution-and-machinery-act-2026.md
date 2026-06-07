<!-- Bill 27 of the VJS legislative programme, enacted by the Sovereign Founder with Royal Assent. -->
<!-- status: enacted | royal-assent: 2026-06-06 | outcome: passed | ayes: 4/4 | drafting rounds: 2 -->

# The VJS (Constitution and Machinery) Act 2026

**An Act** to establish the canonical name of the realm as the Vibe Justice System (VJS) and to record the same in the statute book; to mandate the four-branch directory layout (Constitution, Judicature, Legislature, Executive) for the public realm repository as foundational to open governance and the separation of powers; to require that the public realm hold system data only and that personal and operational data reside in separate, gitignored private repositories; to implement the Bill 22 sealing regime for sensitive operational content by requiring a public stub and operative content sealing; to require the public publication of every ministry's charter and the principle and schema of its registries, discharging the Founder's rule that principle and mechanics are public while facts remain private; to amend Bill 5 to insert a naming section defining the Ministry of Business, Engineering and Skills and the Ministry of Data Security; to amend Bill 21 to rename the Ministry of Security and Integrity to the Ministry of Data Security and to add a blanket reconciliation clause; to amend Bill 26 s. 14 to reflect the renamed Ministry of Data Security in the enabling clause for that Act; to assign the Refactoring Suite to the Ministry of Business, Engineering and Skills and the Security Suite to the Ministry of Data Security, with authority to make each suite a statutory instrument under the respective ministry's enabling power; to clarify that the Standing Committee makes statutory instruments in exercise of the relevant parent office's delegated authority; to provide a public-mechanics conformance enforcement hook; to preserve the Bill 21 carve-outs and neutering clause on the face; to establish the scope of the public realm record so that the law of every judgment is public while personal and operational facts are sealed and local court judgments remain private; and for connected purposes.

## Short title

1. This Act may be cited as the **VJS (Constitution and Machinery) Act 2026**.

## Commencement

2. (1) This Act comes into force on Royal Assent by the Sovereign Founder.

(2) Amendments to Bill 5, Bill 21, and Bill 26 made by this Act take effect on Royal Assent of this Act and supersede any prior conflicting text in those Acts by operation of the append-with-supersede rule, read as if the amended sections replace the prior text from that date forward.

---

## Part I - Definitions and Scope

### 3. Master definitions

In this Act:

**"the realm"** means the unified Vibe Justice System (VJS) governance jurisdiction established by CASE-LAW s. 9, organised as a single unitary software- and engineering-law court applying one canonical statute book (CASE-LAW plus enacted Acts) to every constituent repository.

**"the public realm repository"** or **"the canonical VJS repository"** means the single, publicly readable repository recorded at the top-level DNS and GitHub naming convention as the official seat of the Vibe Justice System, the citator, the neutral-citation namespace [YEAR] REALM-SC / REALM-CA / REALM-PC / [DIVISION] / CC-[REPO] per the Neutral Citations and Law Reporting Act 2026 (Bill 16), the Acts of the Realm, the case-law settlement, the procedure rules (VPR), and the realms' governance infrastructure (the .justice/ citator, legislature/, statutes/, ministry-of-justice/).

**"System data"** means the governing law (CASE-LAW, the Acts, the citator, neutral citations, judgments, procedure rules, the ledger, reasons-ledger), the architecture (the four-branch layout, the philosophy and procedural guides), and the administrative machinery (the pre-commit gate, the law-reports renderer, the legislative register) - the data that MUST be open and public to discharge CASE-LAW s. 1 (clear by access through the citator).

**"Operational data"** means live server configuration, credentials, bearer tokens, secrets, IP addresses, firewall rules, port mappings, and other infrastructure-specific facts required to operate the estate but irrelevant to the governance of the law.

**"Personal data"** means the business records, project files, work-product, and decision logs of the constituent ministries, departments, and projects - the per-department or per-project repository contents that belong to that department's operational control, not the central realm governance.

**"The four-branch layout"** means the mandatory top-level directory structure: Constitution/ (foundational law, procedure rules, the constitutional settlement), Judicature/ (the judiciary machinery, law reports, the citator, court decisions), Legislature/ (bills, legislative procedure, the statute register), Executive/ (ministries and operational departments). The layout embodies the separation of powers for clear navigation and open-justice visibility.

**"The gitignore rule"** means a .gitignore file placed at the root of a repository that specifies which files and directories are withheld from the git staging area and the public commit history.

**"The Bill 22 sealing regime"** means the mechanism established by the Data, Disclosure and Confidentiality Act 2026 (Bill 22): a public stub in the sealed-content register recording that sealed material exists, its scope, its ground, its author, its ordering court, and its expiry; the operative content sealed in a private, gitignored location; and the public record pointing to the existence and legal effect of the sealed material without publishing the content.

### 4. Scope and anti-collision

(1) This Act governs the repository structure and naming of the unified Vibe Justice System, the mandatory layout of the public realm repository, the division of system data (public) from operational and personal data (private), the application of the Bill 22 sealing regime to sensitive operational content, and the administrative organisation of the executive ministries.

(2) This Act does not extend into or duplicate: Bills 7 (Memory, Records and Archives), Bill 8 (Public Reasons and Audit), Bill 20 (Repositories and Records Certification), Bill 21 (Security and Integrity), or Bill 22 (Data, Disclosure and Confidentiality). Each Act governs its specified domain; this Act specifies the repository structure and naming as the vehicle by which those Acts are made visible and operational.

(3) Personal and operational data in separate, private repositories are governed by each ministry's or department's own .gitignore and are outside the scope of this Act save for the requirement in section 7 that a .gitignore MUST exist.

---

## Part II - The Vibe Justice System: Naming and Scope

### 5. The canonical realm naming

(1) The unified governance jurisdiction established by CASE-LAW s. 9 is hereby formally named the **Vibe Justice System** (abbreviated **VJS**).

(2) The canonical name "Vibe Justice System" or "VJS" is the legal name of the realm and shall be used in all official documentation, legislation, the neutral-citation namespace under the Neutral Citations and Law Reporting Act 2026 (Bill 16), the ledger, the law reports, and all public governance materials.

(3) The public realm repository shall be recorded in GitHub and public DNS under the canonical name: the "vibe-justice-system" repository (or "VJS" repository), destroying or deprecating the earlier "agent-universe" or other non-canonical names as the Principal directs. (The destruction of the earlier v1 at wlilley93/vibe-justice-system is an irreversible outward act of the Principal and is not in scope for this Act, which governs the new canonical layout.)

(4) Every nested operational, departmental, or project repository may carry a descriptive local name (e.g. "engineering-department/projects/acmeco") but shall defer to the canonical VJS naming for any public-facing or governance-related citation.

---

## Part III - The Mandatory Four-Branch Layout

### 6. Constitutional mandate of the four-branch directory structure

(1) The public realm repository (the canonical VJS) SHALL be organised at the top level with EXACTLY FOUR branch directories, reflecting the separation of powers and the architecture of open justice:

   **Constitution/** - Foundational settlement, constitutional invariants, procedure rules, and governance philosophy.
   
   Contents (non-exhaustive): CASE-LAW.md (the founding case-law settlement), constitution/ (the enacted constitutional documents and invariants), VPR.md (the Vibe Procedure Rules), CDD.md (the citator and duty doctrine), AGENTS.md (binding agent law), governance guides and design principles. These files ground the entire realm and shall be the first point of navigation for any agent or human entering the realm.

   **Judicature/** - The judiciary, law reports, the citator, and judgments.
   
   Contents (non-exhaustive): .justice/ (the citator, the judgment index INDEX.md, the judgment files organized by tier), court/ (the judgment renderer and delivery machinery), law-reports/ (the public law-reports website, the gazette, the case-law database), ministry-of-justice/ (the ledger builders, the reasons-ledger, the audit chain), caselaw/ (the domain-specific case-law repositories). These directories house the public record of all rulings and governance decisions.

   **Legislature/** - Lawmaking and the statute book.
   
   Contents (non-exhaustive): legislature/ (the bills, the committee procedures, the legislature renderer), statutes/ (the Acts of the Realm, committed and in force; the instruments/ SI register). These directories house all enacted law and the legislative process.

   **Executive/** - Ministries, operational machinery, and implementation.
   
   Contents (non-exhaustive): ministry-of-business-engineering-and-skills/ (and other departments), cli/ (the command-line tooling, the cdd executable), plugin/ (the hooks, the pre-commit gate, the CLAUDE.md binding), docker/ (containerization and deployment scripts), skill/ (automated agents and procedures). These directories house the operational infrastructure and the bound agents who implement the law.

(2) At the TOP LEVEL, only the files that GitHub requires or that the VJS machine requires SHALL be present:
   - README.md (navigation and introduction)
   - LICENSE (the realm's legal license)
   - .gitignore (the master .gitignore for the public realm)
   - .github/ (GitHub workflows and CI/CD)
   - .claude/ (Claude-Code harness configuration, if applicable)
   - docs/ (supplementary documentation, if needed)
   - assets/ (images and supporting media)
   - The four branch directories above.

(3) NO other directories or machinery-bearing files shall reside at the top level. The four branches are exhaustive. If a directory or file does not belong to one of the four branches, it must be moved INTO one of them or removed.

(4) This layout is not a naming suggestion or a best practice; it is MANDATORY. The separation of powers is visible through the directory structure. Repositories that do not observe this layout are not in conformance with this Act and the VJS governance regime.

---

## Part IV - System Data Public, Personal and Operational Data Private

### 7. The public realm holds system data only

(1) The public realm repository (the canonical VJS) SHALL contain ONLY system data as defined in section 3. System data is, by definition, that which MUST be open and public to discharge CASE-LAW s. 1 (the record is clear by access through the citator) and s. 22 (the court geography and realm-as-state are known and navigable).

(2) The public realm SHALL NOT contain:
   - Personal data: per-project work-product, decision logs, research files, constituent-department operational records, or proprietary content belonging to a ministry or project.
   - Operational data: live server configuration, credentials, secrets, API keys, IP addresses, firewall rules, bearer tokens, database credentials, or any estate-specific infrastructure detail.

(3) Any commit to the public realm repository that introduces personal or operational data is a breach of this Act. The commit SHALL be rolled back, the data removed, and a breach submission filed to the court (CASE-LAW s. 4 to s. 5).

(4) The pre-commit gate (plugin/hooks/vjs-pre-commit.sh) SHALL be extended to scan staged commits for patterns matching "secret", "password", "API_KEY", "bearer", "credential", "private_key", "token" (case-insensitive) and shall FAIL CLOSED if such patterns are detected, preventing the commit and alerting the agent to a compliance violation.

(5) The pattern scan is advisory only at the first stage; a developer may override the gate by adding the commit flag --bill-27-override-secret-warning, which records the override in the audit log and proceeds to commit. The override is presumptively a breach of section 7(3) and is reviewable for good cause under Bill 6 s. 14 (protective measure). An agent who overrides without documenting the reason is in breach of the duty of care (CASE-LAW s. 4 to s. 5).

---

### 8. Personal and operational data in separate, private repositories

(1) Personal data (department records, project work, constituent operational files) SHALL reside in separate, private repositories, one per department or project, NOT in the public realm.

(2) Operational data (infrastructure secrets, server configuration, credentials) SHALL reside in separate, private repositories, one per operational domain (e.g., an "operational-estate-security" repository for server-estate secrets), NOT in the public realm.

(3) Each private repository SHALL contain its own .gitignore file at the root, specifying which files and directories are NOT to be committed to git. The .gitignore rule is the legal instrument that protects secrets from being committed; it must be present and must be enforced.

(4) The .gitignore rule at the root of a private repository is not optional; it is a governance requirement and a condition of the repository's validity as a governed data store. A repository without a .gitignore is presumed to have no private data and must be public, or is non-conformant and must acquire a .gitignore before accepting personal or operational content.

(5) The public realm repository's master .gitignore (section 9) SHALL list, by path, every private repository that is nested in or linked from the realm, ensuring that those directories are excluded from the public git staging area. The list is kept up to date as operational repositories are added or removed.

---

### 9. The public realm .gitignore

(1) The public realm repository SHALL contain a .gitignore file at its root that lists, by path, EVERY private repository nested in or linked from the realm, ensuring they are withheld from the public git commit history.

(2) The current .gitignore lists (non-exhaustively):
   - /ministry-of-business-engineering-and-skills/engineering-department/projects/
   - /ministry-of-business-engineering-and-skills/legal-department/harvey-labs/
   - /ministry-of-business-engineering-and-skills/legal-department/references/
   - /ministry-of-business-engineering-and-skills/skills-and-education/scratch-to-signals/
   - /ministry-of-business-engineering-and-skills/business-operations/Clara/
   - /national-archives/
   - **/node_modules/
   - **/.env
   - .env
   - *.env.local
   - law-reports/node_modules/
   - law-reports/review-service/node_modules/
   - law-reports/review-service/data/

(3) As new private repositories or operational data directories are established, the .gitignore SHALL be updated to include them. The update is a governance obligation under section 8.

(4) Failure to list a private repository in the .gitignore is a compliance failure. If a private repository is committed to the public realm by accident, the commit SHALL be rolled back immediately, the .gitignore updated, and a breach submission filed to the court.

---

## Part V - Bill 22 Sealing and Sensitive Operational Content

### 10. Application of Bill 22 sealing to operational content

(1) Sensitive operational content (e.g., [2026] REALM-SI 1, the Security and Integrity (Server Estate) Instrument 2026) that contains live estate secrets, credentials, or infrastructure details SHALL NOT be committed to the public realm repository in its operative form.

(2) Instead, such content SHALL be handled by the Bill 22 redact-with-public-stub mechanism:
   - A public stub (the SI citation, the title, the status, a public gist of what the instrument governs) is recorded in the SI register (statutes/instruments/) as a markdown file.
   - The stub is a VALID SI register entry, marked as sealed per Bill 22 s. 13.
   - The operative content (the detailed security configuration, the verification scripts, the sensitive facts) is sealed into a private, gitignored operational directory (e.g., /operational-estate-security/ or /ministry-of-data-security/sealed-operational/).
   - The public stub points to the existence and legal effect of the sealed material; the seal withholds only the operative content.

(3) The public stub SHALL include:
   - The neutral citation: [2026] REALM-SI 1
   - The title: "The Security and Integrity (Server Estate) Instrument 2026"
   - The status: "made", "coming into force [date]", "in force"
   - A public gist: "This Instrument specifies the security baseline of the server estate (local development infrastructure and public production infrastructure). Operative content is sealed under Bill 22 s. 13. Refer to the sealed register for audit and review."
   - The seal metadata: ordering court (if any), ordering date, expiry, review route.

(4) The operative content (the full Instrument with specific host identifiers, firewall rules, reverse-proxy paths, bearer-token locations, container hardening flags, HMAC secrets, and verification scripts) SHALL be committed ONLY to a private, gitignored operational repository with its own .gitignore, never to the public realm.

(5) The private operational repository SHALL:
   - Be registered in the public realm's .gitignore (section 9).
   - Contain its own .gitignore at the root, specifying any sub-directories or files within it that are not to be committed to git (e.g., live .env files, running credentials).
   - Hold the complete operative content of sealed SIs and operational security documentation.
   - Be access-controlled: only agents and humans with operational authority may access it.
   - Be governed by the Bill 21 (Security and Integrity) audit chain: every access, every modification, every export is logged.

---

### 11. The sealed-content register

(1) A public sealed-content register SHALL be maintained in the public realm (suggested location: .justice/sealed-register.md or statutes/instruments/_sealed-manifest.md) listing every sealed instrument or content item:

   | Citation | Title | Status | Seal Ground | Seal Expiry | Private Location | Audit Chain |
   |---|---|---|---|---|---|---|
   | [2026] REALM-SI 1 | Security and Integrity (Server Estate) | in force | Bill 21 s.7 (compromise/tamper-protection) | [TBD - suggest 2027-06-06 for annual review] | /operational-estate-security/2026-si-1-sealed/ | Bill 8 audit ledger |

(2) The register is ITSELF public and passes the Bill 22 s. 19(5) pre-commit gate (the cite check). The register proves that the seal was made, by whom, on what ground, and when it sunsets. The register is part of the open record.

(3) Every sealed item's public stub (in statutes/instruments/) bears a cross-reference to the sealed-register entry.

(4) As each seal's expiry date approaches, the ministry responsible SHALL either:
   - Extend the seal (justify in writing and re-enter in the register with a new expiry);
   - Unseal and publish the operative content to the public realm; or
   - Destroy or archive the sealed content if it is no longer needed.

---

## Part VI - Public-Mechanics Rule: Ministry Charters and Registry Principles

### 5A. Public mechanics and registry schema

(1) Every ministry and executive department SHALL, within thirty (30) days of this Act coming into force, publish a **Public Mechanics Charter** in statutes/ministry-mechanics/ or within the ministry's own README, specifying:
   
   (i) its lawful functions and the Acts that delegate them;
   (ii) the registries it maintains (e.g., a records registry, a resource-allocation ledger, a capability matrix);
   (iii) the PRINCIPLE and SCHEMA of each registry (what it records, the meaning of each field, the governance rule for updates);
   (iv) the process by which a user or agent may request or appeal a registry entry.

(2) The PRINCIPLE and SCHEMA of every registry SHALL be public, available in the public realm, and suitable for reproduction by any agent or user with access to the realm's codebase.

(3) The **CONTENTS** of a registry may be private (e.g., per-agent security clearance, per-project resource spend) and shall be governed by Bill 22 (Data, Disclosure and Confidentiality). The PRINCIPLE and SCHEMA are public; the FACTS are private.

(4) Failure to publish or to keep current a ministry's Public Mechanics Charter is a reportable matter under the Public Reasons and Audit Act 2026 (Bill 8) and may be referred via the Public Reasons and Audit Act to the court for review of conformance, never as a punitive matter but as a governance conformance check.

(5) A ministry lacking a public charter is not in breach of the duty of care but is non-conformant with this Act; the remedy is publication and updating, enforced by the deterministic gate (Bill 21 s. 19(5)) as a conformance requirement checkable on filing.

(6) The registry schema and mechanics are reviewable for clarity, completeness, and conformance with the Principle; they are not reviewable on the merits of the rules they encode (which are separately reviewable under the enabling Act).

---

## Part VII - Amendments to Bill 5 (Ministries and Offices Act 2026)

### 6. Amendment to Bill 5: insertion of naming section

In the Ministries and Offices Act 2026 (Bill 5), after section 17 (Savings and supremacy), insert:

"### 18. Ministry naming and designation

(1) The executive ministry holding operational jurisdiction under section 14(1) is constituted and organised as follows:

   (i) the **Ministry of Business, Engineering and Skills (MBES)**, holding operational jurisdiction over engineering, refactoring, and skills functions;
   
   (ii) the **Ministry of Data Security (MDS)**, holding jurisdiction over security, integrity, and data-assurance functions, constituted under the Security and Integrity Act 2026 (Bill 21 s. 4 as amended).

(2) These names are operative designations of the functional sub-grouping within the single executive operational ministry under section 14(2), and may be amended by ordinary statutory procedure."

---

## Part VIII - Amendments to Bill 21 (Security and Integrity Act 2026)

### 7. Amendment to Bill 21: renaming and reconciliation clause

In the Security and Integrity Act 2026 (Bill 21), after section 15 (Severance), insert:

"### 16. Nomenclature reconciliation

References in the Security and Integrity Act 2026 to 'the Ministry of Security and Integrity' or 'the Ministry' are to the Ministry of Data Security (MDS) as designated under the Ministries and Offices Act 2026 (Bill 5), without prejudice to any substantive function, power, limitation, or carve-out of that Ministry as described in this Act."

### 8. Preservation of Bill 21 s. 5-6: Neutering and carve-out preservation

Bill 21 sections 5 to 6 (Functions and Limits) are preserved in their entirety without amendment. The neutering clause (section 6), the classifier-not-container principle (section 6(2)), and all other operative guardrails against executive overreach remain in full force and apply to the Ministry of Data Security with no diminishment or implied relaxation by this Act.

---

## Part IX - Amendment to Bill 26 (Statutory Instruments Framework Act 2026)

### 9. Amendment to Bill 26 s. 14(1): Ministry name correction in enabling clause

In section 14(1) of the Statutory Instruments (Framework) Act 2026 (Bill 26), in the new section 16 of Bill 21 being inserted, strike "The Ministry of Security and Integrity may" and replace with "The Ministry of Data Security may".

---

## Part X - Suite Ownership and Statutory Instrument Legalisation

### 5B. Suite ownership and legalisation as statutory instruments

(1) The VJS Refactoring Suite, maintained at `.justice/suites/refactoring.md`, is owned and superintended by the Ministry of Business, Engineering and Skills (MBES). The Suite establishes binding standards for code quality, naming, testing, and refactoring discipline within the operational box. The Suite may be amended by statutory instrument under the Statutory Instruments (Framework) Act 2026 (Bill 26) and the authority granted to MBES under the Ministries and Offices Act 2026 (Bill 5 s. 18) and shall be cited and recorded in the statutory-instrument register under the form [YEAR] REALM-SI N.

(2) The VJS Security Suite, maintained at `.justice/suites/security.md`, is owned and superintended by the Ministry of Data Security (MDS). The Suite establishes binding standards for security review, authentication, secrets handling, and dependency audit within the operational box. The Suite may be amended by statutory instrument under the Statutory Instruments (Framework) Act 2026 (Bill 26) and the authority granted to MDS under the Security and Integrity Act 2026 (Bill 21 s. 16) and shall be cited and recorded in the statutory-instrument register under the form [YEAR] REALM-SI N.

(3) Each Suite, once amended by statutory instrument, is binding operative law subordinate to the Acts of the Realm, applicable to every repository and project in the operational box. A breach of a Suite standard, as read in the light of the ruling that ordered its application, is a falling-below of the duty of care (CASE-LAW s. 5) and is justiciable before the single judiciary.

(4) Publication of a Suite as a statutory instrument is permissive, not mandatory. A Suite may be maintained and enforced as non-binding Standing Committee policy guidance pending formalisation as an SI. Where formalised as an SI, the Suite takes binding operative effect; where remaining as guidance, the Suite is enforced through the ordinary duty of care and court review.

(5) A Suite published as a statutory instrument is operational guidance binding on the administrative and procedural conduct of the named ministry and may govern the deterministic pre-commit gate for the machine-checkable rules it contains. It does NOT prescribe or override how courts order protective measures, remedies, or sanctions, which remain governed by CASE-LAW s. 6 (restorative remedy only) and Bill 6 s. 14 (least-restrictive protective measure). Any provision of a Suite SI that purports to override, carve out, or substitute a non-restorative remedy is ultra vires and void.

---

## Part XI - Maker Clarification: Standing Committee as Delegated Maker

### 5C. Standing Committee as delegated maker under parent authority

(1) When an Act confers delegated power to make statutory instruments on a named ministry, office, or governance body (the "parent authority"), the Standing Committee may exercise that delegated power in the name of and under the superintendence of the named authority.

(2) A statutory instrument is MADE BY the Standing Committee in exercise of the enabling power conferred on the parent authority. The parent authority is the legal source of authority; the Standing Committee is the designated maker and operational agent.

(3) Every statutory instrument made by the Standing Committee under a parent Act's enabling power SHALL record in its recitals: "In exercise of the powers conferred by section X of Bill NN, in the name of the [Parent Ministry/Office Name]", plainly citing both the enabling section and the parent authority's name, so the authority chain is transparent.

(4) This section declares and confirms the allocation of maker authority under the statutory-instrument framework (Bill 26 s. 14-15) and gives effect to the Standing Committee's status as the default legislative agent for subordinate lawmaking, as standing declared by the Sovereign Founder.

---

## Part XII - Conformance and Transitional Provision

### 12. Conformance obligation

(1) Every repository in the realm SHALL, within thirty (30) days of this Act coming into force, audit itself against sections 6, 7, 8, and 9:
   - Does the public realm conform to the four-branch layout?
   - Does it contain personal or operational data that should be private?
   - Are all private repositories listed in the .gitignore?
   - Do all private repositories have their own .gitignore?

(2) On audit, any non-conformances SHALL be remediated:
   - Directories moved to the correct branch or removed.
   - Personal or operational data moved to private repositories or sealed under Bill 22.
   - .gitignore files added and updated.

(3) The pre-commit gate (extended per section 7(4)) SHALL enforce conformance going forward, preventing commits that violate sections 7 or 8.

(4) Any repository that fails to conform within thirty (30) days SHALL be reported to the ministry responsible, and the matter MAY be referred to the court as a governance breach.

---

### 13. Transitional provision: [2026] REALM-SI 1

(1) The Security and Integrity (Server Estate) Instrument 2026 ([2026] REALM-SI 1), committed to statutes/instruments/2026-realm-si-1-security-and-integrity-server-estate.md on 2026-06-06, contained sensitive operational content (host identifiers, firewall rules, reverse-proxy paths, bearer-token locations, container hardening, HMAC secrets, and verification scripts) before it was replaced with a public stub.

(2) Within ten (10) days of this Act coming into force, the following transition SHALL occur:
   - The current SI 1 markdown file is replaced with a public stub containing only the citation, title, status, and a public gist of what the Instrument governs (section 10(3)).
   - The operative content (the full text with sensitive details) is moved to a private operational repository (e.g., /operational-estate-security/2026-si-1-sealed/).
   - The private repository is added to the public realm's .gitignore.
   - The public stub includes a cross-reference to the sealed-register entry.
   - A sealed-register entry is created (section 11).

(3) The transition does NOT diminish the legal force of SI 1. The SI remains in force, operative, and binding. The operative content remains in effect and accessible to authorized operators. Only the public visibility is changed: the public record shows that SI 1 exists and what it covers (the server estate); the operative details are sealed pending a reason to unseal (such as the expiry of a security embargo or the remediation of the vulnerability documented in the SI's Appendix A).

(4) This transition is a compliance obligation under this Act. Failure to complete it within ten (10) days is a breach.

---

## Part XIII - Public Record and Judgments Publication

### 14. Scope of public realm record; local judgments privacy

(1) **The law is always public.** The neutral citation, the ratio, the status, and the legal holding of every judgment that forms part of the public record are public and are never sealed. This is open justice. The Data, Disclosure and Confidentiality Act 2026 (Bill 22) never seals the law itself; it seals only sensitive content behind a public stub.

(2) **No personal or operational data in the public record.** A judgment enters the public central record only so far as its narrative is free of personal or operational facts. Where a judgment recounts such facts, those facts are sealed under the Bill 22 regime (a public stub recording the citation, ratio, and status, with the factual content sealed in a private location); or, for a local matter, the judgment remains in its own repository. The test is data-based, not court-based: it is the presence of personal or operational facts, not the level of the court, that determines what is withheld.

(3) **Court-level presumption.** The CONSTITUTIONAL courts (Supreme Court [YEAR] REALM-SC N; Privy Council [YEAR] REALM-PC N) decide realm-wide governance and constitutional questions, are presumptively system data, and are included in the public record in full. The **Court of Appeal** ([YEAR] REALM-CA N) reviews matters that may originate in local courts and **may therefore recount personal or operational facts**; each Court of Appeal judgment's narrative SHALL be scanned for such facts before publication, and any found are sealed under subsection (2), the citation, ratio, and status remaining public. **Local courts** (County Courts [YEAR] CC-<REPO> N; High Court Divisions [YEAR] <DIVISION> N) record jurisdiction-local precedent that remains committed to its own local `.justice/` directory (binding in its repo, persuasive elsewhere, CASE-LAW s. 22), and is not included in the public central record.

(4) **Derived projections scoped accordingly.** The public derived projections (the universal ledger, the law-reports corpus, the search index, and any machine-readable projection of realm law) shall include the central courts' published law (the constitutional courts in full; the Court of Appeal's citation, ratio, and status, with any factual narrative sealed) together with the published Acts and instruments, and shall EXCLUDE local court judgments. A per-judgment personal-data scan is applied before any judgment, or any projection of it, is published.

---

## Part XIV - Amendment and Status

### 15. Amendment

(1) Amendment to this Act is made by the Principal acting as Sovereign or by the Supreme Court in the ordinary course of a judgment. The new text supersedes the old; the old text is retained with a supersession note. Silent repeal is never permitted.

(2) The four-branch layout mandate (section 6) and the system-data-only rule (section 7) are foundational to CASE-LAW s. 1 and s. 9 and may not be silently repealed or relaxed by implication. Any amendment must be express.

(3) **Reservation: County Court formation and .gitignore requirement.** The requirement that a County Court be validly formed only with a .gitignore file (relating to the governance validity of CASE-LAW s. 22(1)) is RESERVED to the Repos House reference (the registrar matter before the Privy Council). It is not codified in this Act to avoid redundant enactment. If the Privy Council determines that the .gitignore requirement is material to valid County Court formation, the requirement shall be recorded in the Repos House judgment and this Act shall be read as giving effect to that determination.

---

### 16. Commencement and transitional timeline

(1) This Act comes into force on Royal Assent (2026-06-06).

(2) The conformance audit (section 12) and the SI 1 transition (section 13) are due within ten to thirty (10-30) days of commencement, as specified.

(3) The pre-commit gate extension (section 7(4)) may be phased: warning mode (1-7 days), hard fail mode (7+ days).

(4) The Public Mechanics Charters required by section 5A must be published within thirty (30) days of commencement.

(5) The amendments to Bills 5, 21, and 26 (sections 6-9 of this Act) come into force on Royal Assent of this Act.

---

**END OF ACT**

---

## Committee note

Bill 27 is reported by the Standing Committee as the foundational structural Act of the Vibe Justice System. The four Counsel (Aldous=Restraint, Verity=Codifier, Marlowe=Guardrail, Drummond=Pragmatist) reconciled their memos into this single comprehensive Bill reconciling the six substantive requirements given by the Founder:

1. **NAME** (Part II, section 5): fixes the canonical realm name as "VJS" / "Vibe Justice System".
2. **LAYOUT** (Part III, section 6): mandates the four-branch directory layout, the GitHub top-level structure, and the SYSTEM-DATA-ONLY rule.
3. **PUBLIC-MECHANICS RULE** (Part VI, section 5A): every ministry SHALL publish its charter, functions, the registries it maintains, and the PRINCIPLE and SCHEMA (not contents) of those registries. This gives effect to the Founder's rule: principle/schema/mechanics public; facts private.
4. **MINISTRY RENAMES** (Part VII-VIII): inserts a new section 18 into Bill 5 naming the Ministry of Business, Engineering and Skills (MBES) and the Ministry of Data Security (MDS), and adds a blanket reconciliation clause to Bill 21, reconciling the Bill 21 MSI to be the same body as the MDS.
5. **SUITE OWNERSHIP** (Part X, section 5B): assigns the Refactoring Suite to MBES and the Security Suite to MDS, providing that each suite is legalised and amendable as a STATUTORY INSTRUMENT under the owning ministry's enabling power per Bill 26 framework.
6. **MAKER CLARIFICATION** (Part XI, section 5C): clarifies that the Standing Committee MAKES statutory instruments in exercise of the relevant parent office's enabling power, declaring the office as the parent authority and the Committee as the maker.

The Act also:
- Uses the provenance neutral-citation scheme throughout (REALM-SC / REALM-CA / REALM-PC / DIVISION / CC-REPO per Bill 16).
- Correctly characterizes CASE-LAW s.22(3) as declaratory (ordinary law, amendable in the ordinary course), not entrenched.
- Provides enforcement of the public-mechanics rule via Bill 8 conformance referral.
- Reserves the .gitignore / County Court formation rule to the Repos House reference.
- Corrects commencement language to use the append-with-supersede rule for amendments to Bills 5, 21, 26.
- Adds Part XIII on the public realm record: the law of every judgment is public; personal or operational facts are sealed (Bill 22); the constitutional courts are included in full; the Court of Appeal's narrative is scanned and sealed where it carries facts; local judgments remain private.
- Preserves all entrenched articles, particularly CASE-LAW s.1 and s.9 (s.22 being declaratory ordinary law).
- Preserves the Bill 21 neutering clause, carve-outs, and guardrails unchanged (section 8).
- Creates no new court, tier, or citation series.
- Implements the Bill 22 sealing mechanism for operational secrets.
- Discharges CASE-LAW s.1 (clear by access through the citator) and s.22 (realm-as-state is navigable).
- Houses the machinery for Phase 2 (four-branch refactor), Phase 3 (ministry charters), Phase 4 (suite SIs), and Phase 6 (maker clarification).
- Is drafted in plain, operational language suitable for immediate enactment and implementation.

---

## Vote record

Standing Committee vote (first and final reading): **4/4 AYE** (Counsel Aldous, Verity, Marlowe, Drummond voting unanimously on the reconciled full draft).

Each Counsel's conditional vote was discharged on the face of the Act:
- **Aldous (Restraint)**: Minimalist discipline maintained; only load-bearing provisions enacted; no bloat.
- **Verity (Codifier)**: Completeness verified; no fork; all six requirements addressed precisely; all ten scrutiny fixes applied.
- **Marlowe (Guardrail)**: Privacy protected (facts private, law public); Bill 21 guardrails preserved; no new coercive power; judgments-privacy rule added.
- **Drummond (Pragmatist)**: All machinery deterministic or fail-closed; operability verified; suites clarified as permissive-not-mandatory SI pathway; public-mechanics enforcement wired.

---

## Royal Assent

*Royal Assent granted by the Sovereign Founder on 2026-06-06 ("proceed all"). This Act is now **in force** per its commencement (s. 2) and is recorded in `statutes/`. The canonical name is **VJS**; the four-branch layout, the system-data-only and public-mechanics rules, the ministry renames (MBWS -> Ministry of Business, Engineering and Skills; Ministry of Security and Integrity / Defence -> Ministry of Data Security), the suite-ownership, the SI-maker confirmation, and the judgments-publication rule are in force; the amendments to Bills 5, 21, and 26 take effect now by the append-with-supersede rule. The repository is conformed to the four-branch layout by the refactor that follows.*
