# Spec-driven development (SDD) — Opitar

Every change starts with an observable problem and acceptance criteria. Then the plan is written, the agreed scope is built and the evidence is recorded. A local test does not prove a real install or a release.

Level: **standard** (it has a UI, it stores the user's selection, it is published as open source).

## Index

- [Prior research](research.md)
- [Product and journeys](product.md)
- [Experience and look](experience.md)
- [Module map](modules.md)
- [Cross-cutting contracts](contracts.md)
- [Decisions](decisions.md)
- [Change template](change-template.md)
- Area specs: [IDX-01 — File index](IDX-01-index.md), [SEL-01 — Selection and saving](SEL-01-selection.md), [UI-01 — Interface, viewer and CLI](UI-01-interface.md)
- [Traceability and evidence](traceability.md)

Changes: [CAM-001 — First version](changes/CAM-001/spec.md) (done) · [CAM-002 — Groups that understand folders and prefixes](changes/CAM-002/spec.md) (done) · [CAM-003 — Everything in English, format v2](changes/CAM-003/spec.md) (done).

## How to read requirements

- **Current:** what the code does at the stated commit, with evidence.
- **Target:** what a change must achieve; it does not claim it works today.
- **Gap:** a known difference or limit. Never deleted to make the spec look met.
- **Open decision:** a product choice still to make.

## Evidence levels

| Level | Proves | Does not prove |
|---|---|---|
| Document | Intent | That it is built |
| Code read | Behaviour visible in the code | That it works when run |
| Automated check | That case, at that commit and environment | Screens, other environments |
| Real walk-through | What was observed in that setup | Other platforms |

## Drift check

`python3 ~/.claude/skills/sdd/scripts/sdd-deriva.py .`
