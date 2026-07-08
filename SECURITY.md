# Security Policy

## Reporting a vulnerability

**Please do not open a public Issue for security-relevant findings.**
Use a private channel so a fix can land before the details are public:

- **Preferred — GitHub Private Vulnerability Reporting**: the
  **Security** tab → **"Report a vulnerability."** Private by default,
  no email required.
- If you cannot use PVR, open a public Issue saying only
  *"security-relevant, requesting a private channel"* — no details —
  and a maintainer will arrange one.

Please include: affected path + commit hash, a minimal reproduction,
the impact you believe it has, and any suggested fix.

## What is in scope

This tree — the skaists governance workspace:

- `lovernment-core` (the cascade geometry module and the kernel
  consumption boundary — the pinned `escrow-core` dependency and the
  smoke test that exercises it)
- the research and lineage documents under `docs/` insofar as a
  defect there would mislead an implementation (wrong pinned
  parameter, wrong source attribution)

## What is out of scope

- **The kernel itself** — report kernel findings to
  [beehive-nature](https://github.com/beehive-nature/beehive-nature)
  (its own SECURITY.md governs; this tree consumes it as a pinned,
  never-vendored dependency).
- **Unbuilt, founder-gated work** — everything the kernel quarantine
  and this tree's STATUS.md name as captured-not-scheduled (emission,
  attestation, custody, personhood gates). Design feedback welcome via
  Issues; not vulnerabilities yet.
- **Third-party networks and providers** (GitHub, chain networks,
  model providers) — report to those projects.

## Good-faith safe harbor

Good-faith security research on your **own local instances** is
welcome and we won't pursue action for it. This safe harbor does not
extend to testing that touches other users or third-party funds.

## What happens after you report

Acknowledge → triage against STATUS.md and the test battery → fix → a
ledger line recording the finding, the fix, and — with your consent —
**credit by name**.
