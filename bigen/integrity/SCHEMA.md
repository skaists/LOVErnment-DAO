# BIGEN INTEGRITY LAYER — schema and law

**ROUTING** · **Destination:** `skaists/LOVErnment-DAO` → `bigen/integrity/SCHEMA.md`
**Delivery:** Code commits. Not kernel content — do not land in `beehive-nature`.
**Founder read required:** no. Ruling below is design-seat, within delegation.

---

## 1. WHY THIS EXISTS — the number that breaks Cochrane's model

**Lundh et al., Cochrane Database Syst Rev 2017;2:MR000033** ([DOI 10.1002/14651858.MR000033.pub3](https://doi.org/10.1002/14651858.MR000033.pub3)), 75 papers, retrieved via PubMed:

| Association | RR | 95% CI |
|---|---|---|
| Manufacturer-sponsored → favorable **efficacy results** | **1.27** | 1.17–1.37 |
| Manufacturer-sponsored → favorable **conclusions** | **1.34** | 1.19–1.51 |
| Manufacturer-sponsored → **LOW** risk of bias from blinding | **1.25** | 1.05–1.50 |
| Agreement between results and conclusions | **0.83** | 0.70–0.98 |

**Read rows 3 and 4 together.** Sponsored studies score *better* on standard methodology. The bias is not sloppy blinding — **it is the gap between what the table showed and what the abstract concluded.**

Lundh's own finding: *"an industry bias that cannot be explained by standard 'Risk of bias' assessments."*

> **Therefore: RoB 2 alone cannot detect sponsor bias. An implementation that ships only RoB 2 systematically misses the thing this library exists to catch.** The integrity layer is a separate dimension, not a RoB 2 domain.

---

## 2. THE SCHEMA — one `integrity:` block per included study

```yaml
integrity:
  funding:
    sources: []                          # verbatim from the funding statement
    manufacturer_funded: true|false|unclear
    # ^ "manufacturer" = whoever sells the intervention under test.
    #   Pharma, cannabis producer, supplement maker, device firm — identical flag.
    pubmed_support_flags: []             # auto: article_types matching "Research Support*"
    funding_statement_absent: bool       # absence is a finding, not a blank

  coi:
    declared: []                         # verbatim
    declaration_absent: bool
    undeclared_found: []                 # cite the source that revealed it

  retraction:
    status: none|retracted|expression_of_concern|corrected
    pubmed_flag: bool                    # auto: article_types contains "Retracted Publication"
    stated_cause: ""
    cause_disputed: bool                 # record the dispute; do NOT adjudicate intent
    last_checked: YYYY-MM-DD             # standing query — see §4

  registration:
    prereg_id: ""                        # NCT / ISRCTN / PROSPERO
    registered_before_enrollment: true|false|unclear
    outcome_switching: yes|no|unclear    # registry primary vs published primary
    switching_evidence: ""

  results_conclusions_gap:
    verdict: aligned|conclusion_overstates|conclusion_understates|not_assessed
    evidence: ""                         # quote the table value AND the claim sentence
```

**Every field defaults to `unclear` or `not_assessed`, never to a favourable value.** An unassessed study displays as unassessed. *Negative control: an integrity block that renders clean because nobody looked → fail.*

---

## 3. THE TWO LAWS

### Law A — the flag is symmetric or the library is worthless

**`manufacturer_funded` fires on whoever sells the thing being tested.** Lundh's mechanism is not "pharma is dishonest" — it is *manufacturers fund studies that favour their product.* The mechanism is structural and applies without regard to which side we like.

**This includes us.** A cannabis producer funding a cannabis trial earns the flag. A BNR- or SKAISTS-funded study earns the flag. A COA program we commission earns the flag.

> **Negative control: a corpus in which `manufacturer_funded: true` never appears on a study whose result we like → the detector is broken.** Run this check on every release.

**Rationale:** a bias detector that fires in one direction is not measuring bias, it is declaring allegiance, and a hostile reader identifies that in one pass. The asymmetry destroys the library's only asset.

### Law B — integrity is recorded as evidence, never applied as an automatic downgrade

**Article II, standing: attestation is *"evidence, not status."*** Same law here.

**The integrity block does NOT silently modify the GRADE rating.** It renders beside the estimate. A fork may weight it; the library does not weight it for you.

**Rationale:** if BIGEN auto-docks sponsored studies a GRADE level, the pooled estimate stops being a computation a stranger can re-run — it becomes our prior wearing a number's clothing. *The whole differentiator is that a BIGEN forest plot is recomputable. An automatic bias adjustment is the one thing that would make it not.*

---

## 4. AUTOMATION — retraction checking is a standing query, not a research task

PubMed exposes both signals as structured fields. Verified live on the Ricaurte case:

```
title:         "RETRACTED: Severe dopaminergic neurotoxicity in primates..."
article_types: ["Journal Article",
                "Research Support, U.S. Gov't, P.H.S.",
                "Retracted Publication"]
```

*(Ricaurte et al., Science 2002;297(5590):2260–3, [DOI 10.1126/science.1074501](https://doi.org/10.1126/science.1074501), PMID 12351788 — retrieved via PubMed.)*

**So both funding class and retraction status are lookups, not investigations.** Add to `register/queries.yaml`:

```yaml
- id: integrity-sweep
  type: retraction_check
  targets: all_included_studies       # re-check every PMID in the corpus
  schedule: weekly
  fields: [article_types, title_prefix]
  on_change: open_amendment           # a retraction is an AMENDMENTS.md event
```

**"Has anything in our corpus been retracted since last run" becomes a cron job.** That is the living-library property applied to integrity, and no paywalled review has it.

---

## 5. THE INSTRUMENT THAT DOES THE REAL WORK

**`results_conclusions_gap` is the differentiator, because it is what RoB 2 misses (§1).**

**Method:** read the results table. Read the concluding claim. Quote both. Judge whether the second is entailed by the first.

**It requires no special access** — the abstract and the table are enough — which means **a stranger can re-run it and check us.**

**Two worked examples already in hand, both marked `conclusion_overstates`:**

- **Hussein et al. 2014** ([DOI 10.1016/j.ajme.2014.02.003](https://doi.org/10.1016/j.ajme.2014.02.003)) — concludes the extract causes "down regulation of telomerase activity." **No telomerase activity assay was run.** TERT mRNA only; the activity claim is inference. *(The direction and magnitude are real — 116.13 → 2.29/3.08/9.60 copies — the overreach is in the mechanism sentence.)*

- **BIGEN's own ECS-telomerase evidence map** — attributed CBD's effect to an "AMPK→SIRT1/autophagy pathway." **AMPK appears nowhere** in the source's abstract, author keywords, or MeSH terms ([DOI 10.21873/invivo.14186](https://doi.org/10.21873/invivo.14186)). *Our own output failed the test first. Record it.*

> **The second example is the load-bearing one.** A library that only ever runs this instrument on other people's work has not implemented it.

---

*Sponsor bias survives good methodology. It lives in the sentence after the table. Catch it there, record it as evidence, never adjudicate intent, and point the instrument at yourself first.*
