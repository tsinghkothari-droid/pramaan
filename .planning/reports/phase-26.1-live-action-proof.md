# Phase 26.1 Live GitHub Action Proof

## Result

Phase 26.1 is complete as a live GitHub Actions proof.

- Workflow: Pramaan
- Event: `workflow_dispatch`
- Run URL: https://github.com/tsinghkothari-droid/pramaan/actions/runs/26229890652
- Job URL: https://github.com/tsinghkothari-droid/pramaan/actions/runs/26229890652/job/77187017437
- Run conclusion: `success`
- Head SHA: `3b089eedf6ec27d216351e2566839a7543008490`
- Artifact name: `pramaan-proof-bundle`
- Artifact ID: `7137454243`
- Artifact URL: https://github.com/tsinghkothari-droid/pramaan/actions/runs/26229890652/artifacts/7137454243
- Artifact digest from GitHub upload: `sha256:20bb80cd58f218748ab70e1beaa632dec54610cc88b27cf58ae94d93b92bd766`
- Downloaded artifact copy: `.planning/reports/phase-26.1-live-action-artifact/`
- Downloaded `bundle.manifest.json` file SHA-256: `d61fc95993b3592ff5e04b92949242737ef18aad04010ebc6b94c14a6755f445`
- Manifest-reported digest in rendered summary: `sha256:3cae6f3ca2d09c46526d2668df23ce1ba9f19419b8311d3b3aea790ced37a28b`

## Evidence Captured

The live run:

- checked out the repository with `fetch-depth: 0`;
- built `pramaan-cli` from source;
- ran `pramaan verify`;
- emitted local/offline VSA and in-toto attestations;
- rendered `github-step-summary.md`;
- uploaded `pramaan-proof-bundle`;
- applied the configured failure policy.

The bundle final status was `inconclusive`, with policy decision `warning`.
That is acceptable for this phase because the proof is about live Action
execution, artifact upload, and honest residual-risk rendering, not proving the
repository has zero residual risk.

## Bugs Found And Fixed During This Phase

The first live runs caught real Action/bundle issues:

1. `action.yml` had an unquoted attestation description containing `:` and
   failed GitHub action-manifest parsing.
2. The composite action referenced helper scripts as if they were at
   `$GITHUB_ACTION_PATH/*.mjs`; for local `uses: ./`, they are under
   `$GITHUB_ACTION_PATH/action/*.mjs`.
3. Static receipts declared command strings as artifacts, which bundle
   construction correctly rejected because they were not files.
4. Oracle/fuzz/mutation receipts used workspace-prefixed output paths instead
   of bundle-relative artifact paths, causing ambiguous artifact resolution.

Each bug was fixed and pushed before the successful live proof run.

## Residual Risk

- This was a `workflow_dispatch` run on `main`, not a pull-request event. It is
  a valid live Action proof, but Phase 26.4 / public-review work should still
  include the canonical PR-like demo path.
- The run used local/offline attestations; production Sigstore/cosign identity
  remains future hardening.
- GitHub emitted a Node.js 20 deprecation warning for upstream actions. This is
  not a Pramaan failure, but release docs should keep dependency versions under
  review before public launch.
