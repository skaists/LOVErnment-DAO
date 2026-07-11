# BIO-1 — Biometrics

Status: **APPROVED — v0.1, 2026-07-11.** Founder gate §4 closed same day: B-3 and B-4 ruled. Frozen at its landed sha: any change to these bytes requires a version bump and a re-gate; it does not inherit this approval.
Companion to `PERSON-1` (v0.1, ratified 2026-07-11), which load-bears on this document at P-3. Filed to close that citation; supersedes nothing.

---

## 0. The claim this document defends

**A biometric is a password you cannot rotate, printed on your body, visible to every camera you will ever walk past.** Handle accordingly.

Everything below follows from that sentence and from two already-ratified facts: a compromised key rotates (P-6); a compromised face does not, ever.

---

## 1. The one lawful role

**A local gate on a local key.** The FIDO / platform-passkey pattern, exactly and only: a 1:1 template, created on-device, matched on-device, stored inside the platform secure element, gating the use of a device-resident signing key. The template gates the key; **the key does the talking.** Nothing biometric ever represents the person to any other party — the secure element's signature does, and signatures rotate.

This is `PERSON-1` T1, unchanged. It raises what a session may *do* on one device. It unlocks nothing about earning, standing, or uniqueness.

---

## 2. Invariants

**B-1 — Raw biometrics never leave the device.**
Not as image, not as template, not as embedding, not as hash, not as an "irreversibly transformed" derivative, not in telemetry, not in crash reports, not encrypted-at-rest on any server. An embedding *is* the template; a hash *is* the template — the transformation excuse is the standard leak path, and "just iris codes" is how the last eye market described its inventory. If a bit was computed from a body, it stays on the body's device.

**B-2 — Biometrics are never uniqueness evidence.**
No 1:N comparison, at any gallery size, for any purpose, ever. A biometric proves *this body matches the template enrolled on this device* — nothing about how many devices, nothing about how many enrollments, nothing about personhood. The mathematics of why 1:N fails at scale, and why it fails even at `f = 0`, are owned by `PERSON-1` §2a–2b and are not repeated here. Uniqueness is the cascade's job (T3), and only the cascade's.

**B-3 — Biometrics never cross the seam.**
No `CanonicalEvent`, no `Evidence` object, no provenance class carries biometric data or any biometric-derived identifier. `DeviceAttestation` (0.90) attests that *a secure element signed* — never what unlocked it, never who stood in front of it. The BIND-1 provenance vocabulary is biometric-blind **by law, not by omission**: a future provenance class that would carry biometric-derived data is not a schema addition, it is a violation of this invariant, and the K-4 founder gate must refuse it.

**B-4 — Cameras may feed human judgment; they may never feed a matcher.**
The line that protects P-13 without opening the hole: remote synchronous attestation — humans seeing humans in a live small-group meeting, per the Eden precedent — is *peer attestation over a video channel*, and it is lawful because the judgment is human. The moment any algorithm compares faces, voices, or bodies across that channel — liveness scoring, face-match "verification," selfie-to-document checks — it has become 1:N-adjacent biometric processing, and B-1/B-2 forbid it. **The camera is a window, never an instrument.** If P-13's remote-presence parameter is adjudicated open (Article VI meta-tier, due within the first post-genesis year), it opens a window, not a scanner.

**B-5 — NO-DNA is total.**
P-4 binds here in full, by reference: genetic data is never collected, referenced, matched, or accepted, by any tier, under any gate, in any jurisdiction. Its five reasons live in `PERSON-1` and are not diluted by summary.

---

## 3. Failure honesty

When a platform secure element is compromised, the *key* rotates and the graph attests the rotation (P-6). The *biometric* does not rotate — which is precisely why it must never be the root of anything. The root is the key; the biometric is a convenience gate on one device, and losing it must never cost more than re-enrolling a fingerprint on a new phone.

---

## 4. Founder gate — closed 2026-07-11

B-1, B-2, and B-5 restate ratified doctrine (P-3, P-4, §2a–2b) and carry no new decision. **B-3** (the seam is biometric-blind by law) and **B-4** (the camera is a window, never an instrument) are new rulings: B-3 binds every future BIND-1 allowlist addition; B-4 draws the line P-13's remote-presence adjudication will stand on. Both approved; this document is closed.

---

## 5. What this document does not do

It adds no personhood mechanism — admission is `PERSON-1`'s alone. It does not forbid a person from using their face to unlock their own phone; it forbids everyone else from ever learning anything from that fact.
