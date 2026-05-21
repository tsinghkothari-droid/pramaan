# Phase 35.7: Machine Verification and Human Sign-Off Gate

## Goal

Adopt a mandatory split between what coding agents can self-verify and what a
human must explicitly approve after code is written.

## Why This Phase Exists

Pramaan's own development process should model the product. Agents can produce
mechanical evidence, but they should not be the final judge of claim truth,
security acceptability, reviewer usefulness, release readiness, or business
value.

## Tasks

1. Add `docs/human-signoff.md` with the machine/human boundary.
2. Add `.planning/templates/MACHINE_VERIFICATION.md`.
3. Add `.planning/templates/HUMAN_SIGNOFF.md`.
4. Update `TASKS.md`, `.planning/ROADMAP.md`, and `.planning/STATE.md` so each
   meaningful GSD phase expects both artifacts.
5. Update autonomous continuation instructions so agents prepare sign-off
   evidence but do not self-approve human-only decisions.

## Exit Criteria

- Future GSD phases know which evidence an agent must produce.
- Future GSD phases know where human approval is required.
- Public claims, releases, Marketplace publishing, and Serious v1 decisions
  are blocked until human sign-off exists.
- The templates are lightweight enough to use every phase, not ceremonial.
