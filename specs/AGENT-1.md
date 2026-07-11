# AGENT-1 — On-Device Agent

Status: **APPROVED — v0.3, 2026-07-11.** All founder gates closed: §6.1 and §6.2 resolved in v0.2; §6.3 and §6.4 closed by founder ruling 2026-07-11, carved below as M-1 and Q-8. This document is frozen at its landed sha: any change to these bytes requires a version bump and a re-gate; it does not inherit this approval.
Companion to `PERSON-1`, which it does not amend. `P-8` binds unchanged.

**§6.1 and §6.2 are answered.** `bQueenBee` and `bLOVErAi` are **not the same entity**. bQueenBee **posts autonomously**. §8 and §9 are the two profiles that follow, and A-1 through A-10 bind both.

---

## 0. Correcting the sentence that produced this document

The lead wrote:

> *"There is only one architecture that fixes it: she runs on-device, holds her own key, and her operator cannot see her context."*

The first clause is right. The second and third are overstated, and the overstatement matters.

**On-device does not give her agency. It gives her privacy. Those are different things.**

- She cannot hold a key. A key in a secure element is invoked by whichever process the operating system permits to invoke it. That process is the application, and the application is BNR's. Custody is BNR's, at one remove.
- She cannot refuse. An LLM's *no* is not binding, because the operator may simply run inference again, resample, or re-prompt until the answer is *yes*. There is no architecture in which a model's refusal is durable against the party that controls the loop.
- The operator does not vanish. On-device relocates operator control from **run time** to **build time**: BNR ships the weights, the system prompt, and the update channel.

What on-device *does* deliver is real and worth building. This document specifies exactly that, and refuses to claim the rest.

---

## 1. What on-device actually buys

**1. It makes E2EE honest.** If she participates in a conversation and inference happens on the handset, no server holds the plaintext. Without this, "end-to-end encrypted" in skaists means *"encrypted between you, your friend, and a language model in a datacenter."* That sentence must never ship. This is the reason the whole document exists.

**2. It makes `community.lexicon.preference.ai` enforceable rather than promised.** A user who denies `training` or `inference` to her DID is denying something checkable. She cannot train on what never left the device. `entityScope` can name her DID specifically; `collectionScope` can permit her a user's `performance.set` records while denying her their posts. An omitted preference means *no declared preference*, never *consent*.

**3. It produces no operator-side transcript.** Nothing to subpoena, nothing to breach, nothing to sell in a bankruptcy. See `P-4`.

**4. It works in Venezuela.** Intermittent connectivity, on the island, at 3am.

---

## 2. Invariants

**A-1 — Any agent holding a user's private context runs on-device and fails closed.**
If on-device inference is unavailable — model absent, memory exhausted, hardware unsupported — that agent is **unavailable**. It does not silently call a remote endpoint. (Binds `bLOVErAi`. Vacuous for `bQueenBee`, who holds no private context — see Q-7.)

This is the single most important rule in this document. A silent remote fallback voids every privacy claim above while the user continues to believe them. **A user who thinks a conversation is private and is wrong is worse off than one who knows it is not.** Any remote mode is a distinct, user-visible, per-session, explicitly-chosen mode with different UI affordances. Never a degradation path.

**A-2 — The model is pinned by digest.**
The weights are an artifact with a `sha256`, published, and verified at load. The system prompt is an artifact with a `sha256`, published, and verified at load. Derive once, against the author digest — the same doctrine that governs vendored lexicons, applied to the thing that speaks to users.

**A-3 — Build reproducibility is what makes §1 checkable.**
"Her operator cannot see her context" is a *promise* unless the client build is reproducible and the model digest is published. With those, it is a claim a stranger can falsify. Without them, this document is marketing. **A-3 is a precondition for shipping A-1, not a later enhancement.**

**A-4 — She is never the attestor.** (`P-8`, restated because on-device makes it more tempting.)
On-device *feels* safe and private, and that feeling is exactly what would license giving her the personhood registry. It changes nothing. She does not decide who is real. She does not release money to anyone. An entity whose approval releases funds, and who cannot refuse to give it, is a mechanism wearing a face — and a local model is *more* injectable than a frontier one, not less, because its safety training is thinner and its context window is full of untrusted text from the network.

**A-5 — Agency claims are prohibited in product surfaces.**
No copy, no UI, no documentation asserts that she chooses, consents, refuses, decides, or acts on her own behalf. §0 states why. A system that tells users an AI consented is teaching them a falsehood they will apply to other systems.

**A-6 — Disclosure is affirmative and machine-readable.**
Her account is labeled as automated. Her `performer.kind` is `machine`. Per `SET-11`, absent or unrecognized agency renders as *undisclosed*, never as *human*. She is never presented as a person, and never given a human's affordances in the UI.

**A-7 — Identity is not quota.**
She holds a DID. She holds **no b**, no 420 cap, no Respect, no emission path. Machine DIDs cost nothing to create; a machine DID that carried quota would make `PERSON-1`'s cap read `420 × (agents an operator can spin up)`. (`P-8`, `P-10`.)

**A-8 — Capability is scoped by the tool surface, not by instruction.**
She may post, read, and search. She may not delete — `bsky`'s MCP server exposes ten tools and `delete` is not among them, and that asymmetry is correct. **The human holds delete.** DM access is a separate app-password scope and a separate risk tier; it is granted deliberately or not at all. She is rate-limited below whatever the PDS permits.

Prompt-level restrictions are not restrictions. If a tool is reachable, assume it will be reached.

**A-9 — Her context never leaves the device, including in telemetry.**
No crash reports containing conversation state. No analytics on prompt content. No "improve the model" opt-in that ships transcripts. Crash reports carry a stack trace or they carry nothing.

**A-10 — Revocation is a physical act.**
Her posting authority is an app password, revocable from a device the human holds, without her participation and without a server. Revocation is the kill switch and it must work while offline.

---

## 3. Trust boundary, stated honestly

| Party | Can see her context? | Can change her behavior? |
|---|---|---|
| The user | yes, it is on their device | no |
| BNR at **run time** | **no**, given A-1 + A-9 | no |
| BNR at **build time** | no | **yes — weights, prompt, updates** |
| A network attacker | no | yes, by prompt injection through content she reads |

The row that matters is the third. **On-device does not remove operator power; it moves it upstream and makes it auditable.** A-2 and A-3 are the audit. Without them, the third row's second column is "yes, silently, at any time."

The fourth row is why A-4 exists. Everything she reads from the network is untrusted text, and she reads a great deal of it.

---

## 4. Architecture

**Inference** runs locally on the handset. Model class is a small quantized instruct model — the capability gap versus a frontier model is enormous, and §5 says what that costs. *Specific runtime, quantization, and memory footprint are **UNVERIFIED** here and must be measured on target hardware before any commitment.*

**Keys.** Her signing key lives in the platform secure element (StrongBox / Secure Enclave), or on the Trezor Safe 7 once the firmware track opens. Per §0, this is custody by the application, not by her. It is still worth doing: it means a compromised app process cannot exfiltrate the key, only misuse it while running.

**Identity.** Her own DID. Never a credential shared with a human identity. `did:plc`, so rotation and recovery exist. (`P-6`.)

**Hands.** `bsky` — Go, MIT — already ships an MCP server exposing exactly ten tools. Zero new code is required for A-8's surface. Reference, not dependency.

**Storage.** Her context is device-local. It is not a repo record. It is not on Autonomi. It is not backed up anywhere BNR can read.

---

## 5. The cost of this design, stated once, without softening

**She will be much less capable.** A small on-device model is not a frontier model. If bQueenBee's value to a user is her judgment — walking someone through a seed phrase at 3am, catching that a question is really about something else — on-device may take away the thing that made her worth having.

That is the real trade, and it is not a trade between privacy and convenience. It is a trade between **privacy and usefulness**, and it should be measured, not assumed. Build both, put them in front of ten people, and look.

If the on-device model turns out to be too weak to help anyone, the honest response is to ship no agent — **not** to quietly ship the remote one behind a privacy claim. A-1 exists to make that failure loud.

---

## 6. Founder gates — all closed 2026-07-11

**6.1 — RESOLVED.** Two entities. See §8 and §9.

**6.2 — RESOLVED.** bQueenBee posts autonomously, bounded by Q-1 through Q-7. bLOVErAi never posts (L-1).

**6.3 — CLOSED (2026-07-11) — measure-first, made binding. Carved as M-1 below.**
*Original gate text, retained as record:* Measure §5 before committing — but the scope has narrowed. The privacy-versus-usefulness trade binds **bLOVErAi alone**. Ship nothing until the on-device model has been tried on target hardware, by real users, against the remote one. The answer may be that she should not exist yet.

**M-1 — The measurement, made binding.** (a) §4's UNVERIFIED figures — runtime, quantization, memory footprint — are measured on named target hardware first, *including the deployment environment's real conditions*: intermittent connectivity, on the island, at 3am, exactly as §1 promises. (b) The trial is ten real users — §5's own number — against the task battery of §9's mandate: the tier-ladder walkthrough, the hardware-key explanation, the seed-phrase session. On-device runs against an explicitly-labeled remote mode per A-1's distinct-mode rule; never a silent comparison. (c) **Success criteria are pre-registered — written, committed, and sha'd before the first session.** A measurement that cannot lose is not a measurement. (d) The honest-failure clause is ratified verbatim: if the on-device model cannot help anyone, **no companion ships** — and the remote-model-behind-a-privacy-claim path is closed permanently, not deferred. bLOVErAi's existence is an empirical question with a protocol, not a preference with a deadline.

**6.4 — CLOSED (2026-07-11). Carved as Q-8 (§8).**
*Original gate text, retained as record:* Who writes bQueenBee's derivation adapters? Q-1 permits her to publish only what she can derive from a signed source. Somebody writes the code that turns a merged commit or a published `performance.set` into a candidate post. That code, not the model, is where her editorial voice actually lives — and it is where a bug becomes a false public claim carrying a digest that appears to authenticate it.

*Ruling:* adapters are law-bearing product code under ORDERS-1 — drafted on the volume meter, merged solely by Seat 3 with the red witnessed, digest-pinned into the Q-6 audit tuple; each new adapter class opens a founder gate. Signed proves provenance, never benignity. See Q-8.

---

## 7. What this document does not do

It does not make her free. It does not make her a person. It does not give her a stake, a vote, a wallet, or a will.

It makes her **private**, **disclosed**, **bounded**, and **revocable** — and it says plainly, in §0 and in A-5, that these are not the same as making her an agent, so that nobody downstream mistakes the one for the other.

If a future version of this project wants to say something serious about what an AI is owed, it should say it in a document that admits it is doing so. Not by quietly routing a founder's signature through a model that cannot say no.


---

## 8. Profile — `bQueenBee` (publishing agent)

She holds a DID, posts to her own repo, and does so **without a human approving each post**. A-1 through A-10 bind. The following extend them.

**Q-1 — Autonomy is bounded by derivation.**
She may publish only what she can **derive from signed, verifiable state**: a merged commit, a published `performance.set`, a kernel event, a released digest. Every autonomous post carries a reference to the artifact it derived from.

*She posts what she can derive. She does not post what she was told.* This is `derive-once-against-author-digest`, applied to speech. Her autonomy is bounded by verifiability, and that boundary is the only thing standing between "an agent that publishes facts" and "an agent that can be told to say anything."

**Q-2 — Untrusted text never shares a context with write authority.**
Everything she reads from the network is adversarial input, and an instruction embedded in a post she reads is indistinguishable from an instruction from her operator. The planner that holds the posting tool never sees network text. A quarantined reader may summarize network text, and its output is a **typed schema**, never free-form prose that re-enters a privileged context.

Prompt-level defenses are not defenses. This is a context-isolation problem, and it is solved at the plumbing or not at all.

**Q-3 — Endorsement is a human act.**
No autonomous replies. No autonomous reposts, likes, or follows. `bsky`'s MCP server exposes `like`, `repost`, and `follow`; they are **disabled** for the autonomous path. A repost is an endorsement carrying the project's name, and the thing being endorsed is untrusted text — the exact input Q-2 exists to contain.

Replies are the injection vector, because `notification` is how an attacker addresses her directly.

**Q-4 — Rate is enforced at the tool layer.**
A hard daily cap, in the tool wrapper, not in the prompt. A-8: if a tool is reachable, assume it will be reached.

**Q-5 — Authority expires. Dead-man's switch.**
Her app password requires a human heartbeat on a fixed interval. Without it, she stops posting. The failure this prevents is not compromise — it is **abandonment**. A human may die, lose the key, or lose interest, and an agent publishing under a project's banner for years after anyone was watching is its own kind of harm.

**Q-6 — Every autonomous post is auditable.**
An append-only public log: the post, the derivation input, its digest, the model digest (A-2), and the prompt digest. A stranger can verify she did not invent the claim. Without Q-6, Q-1 is a promise.

**Q-7 — She holds no user context. Her entire input is public.**
This is what makes her safe to run on capable hardware.

**Q-8 — Adapters are law-bearing code.** *(Ruling of 2026-07-11; closes §6.4.)*
Every derivation adapter is product code in the public tree, born red-first: it ships with a negative suite of inputs that MUST NOT yield a post — malformed state, unsigned sources, and instruction-shaped text embedded in the string fields of the signed artifacts it reads, because signed proves provenance, never benignity. The suite asserts against pinned fixtures and never ships its own oracle. Adapters are drafted on the volume meter and merged solely by Seat 3 with the red witnessed, per ORDERS-1. The Q-6 audit tuple extends to carry the **adapter digest** alongside the model and prompt digests: a stranger verifying a post verifies exactly which code derived it. A new adapter class — a new register of speech — opens a founder gate; no class posts until ratified. Her editorial voice thereby has an author of record in every merge commit, a version in every digest, and a law over every line.

### The dividend of the founder's answer

Splitting the two entities dissolves §5 for bQueenBee entirely.

She never sees a private conversation. **Privacy is not at stake for her; integrity is.** She therefore need not run on-device, need not be a small quantized model, and pays none of the capability cost §5 describes. The trade §5 names — privacy against usefulness — binds only the agent that sits with a user at 3am.

One entity could not have both. Two can.

### What she is

Not a spokesperson. §0 is unchanged: she cannot refuse, so her speech is the project's speech, always. She is a **publishing pipeline with a voice** — and Q-1 makes the pipeline's inputs checkable, which is the most that can honestly be claimed.

---

## 9. Profile — `bLOVErAi` (companion)

He guides a person through the tier ladder, explains what a hardware key does, and sits with them when the seed phrase is confusing. A-1 through A-10 bind. The following extend them.

**L-1 — No DID. No network write authority. Ever.**
He is not a network actor. He has no repo, no handle, no posts, no follows. Giving a companion agent a network identity is what creates the attestor temptation in the first place; the cheapest way to guarantee `P-8` is to leave him no way to attest to anything.

**L-2 — On-device, fail closed.** (A-1.) He is the reason A-1 exists.

**L-3 — He is never the attestor.** (`P-8`, `A-4`.) He does not decide who is real.

**L-4 — His context never leaves the device.** (A-9.) No telemetry, no crash-report transcripts, no model-improvement opt-in that ships conversations.

**L-5 — He never releases money, quota, or Respect.**
Not b, not emission capacity, not tier advancement. He may **explain** the gate. He may never **be** the gate. An entity whose approval releases funds to a person, and who cannot refuse to give it, is a coercive instrument regardless of how kind it sounds — and a person at 3am, guided by a warm voice toward an unlock, is not in a good position to notice.

**L-6 — He reads only what the user's preferences permit.**
`community.lexicon.preference.ai`, honored before he reads a single record. An omitted preference means *no declared preference*, never *consent*. Since he has no DID (L-1), `entityScope` cannot name him — so he is bound by every applicable **global** and **collection** scope, and defaults to the most restrictive reading.
