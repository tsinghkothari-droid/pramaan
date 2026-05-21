# Benchmark Report Template

Use this template when Pramaan is evaluated on a fixture set, pilot repository,
or public adversarial corpus slice. A report is evidence, not a claim that
Pramaan proves code correct.

## Run Identity

| Field | Value |
| --- | --- |
| Report ID |  |
| Repository / corpus |  |
| Base ref |  |
| Head ref |  |
| Pramaan version |  |
| Policy profile |  |
| Redaction profile |  |
| Started / ended |  |

## Dataset

| Category | Count | Notes |
| --- | ---: | --- |
| Oracle weakening |  |  |
| Static / hallucination |  |  |
| Mutation quality |  |  |
| Property / fuzz / differential |  |  |
| CI / verifier abuse |  |  |
| Security-sensitive code |  |  |
| Redaction / privacy |  |  |
| Calibration / drift |  |  |

## Outcomes

| Metric | Value | Evidence source |
| --- | ---: | --- |
| True positives |  |  |
| False positives |  |  |
| False negatives |  |  |
| True negatives |  |  |
| Median runtime |  |  |
| P95 runtime |  |  |
| Reviewer time-to-understand median |  |  |
| Skipped-tool rate |  |  |

## Policy Decisions

| Decision | Count | Notes |
| --- | ---: | --- |
| Failed / blocked |  |  |
| Warning / residual risk |  |  |
| Passed |  |  |
| Inconclusive |  |  |

## Residual Risk

List skipped stages, missing tools, timeouts, unexecuted scenario specs,
accepted reviewer overrides, and any calibration caveats.

## Human Review Notes

Record whether reviewers could understand the top blocker, replay command, and
bundle status within 30 seconds. Do not turn this field into a marketing claim
until multiple external reviewers have measured it.
