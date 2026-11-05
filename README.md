# Telegraph Protocol H2 Evaluator Suite

This repository contains benchmark preference datasets (`eval_material.json`) and validator evaluation adapters (`adapter.py`) submitted for **Telegraph Protocol Season II (Evaluator Track)**.

## Supported Security & Compliance Intents

1. **`PORT_SCAN_AUDIT_GENERAL`**: Evaluates network port scan telemetry, service detection logic, and exposure risk scoring.
2. **`THREAT_IP_REPUTATION_GENERAL`**: Evaluates IP reputation classification, proxy/botnet identification, and threat actor telemetry.
3. **`MALWARE_DETECTION_HASH`**: Benchmarks file hash lookup precision, malware classification, and signature alignment.
4. **`EMAIL_SECURITY_GENERAL`**: Evaluates email header analysis, SPF/DKIM/DMARC verification, and phishing indicator detection.
5. **`VULNERABILITY_TRIAGE_GENERAL`** *(Bounty #90)*: Evaluates vulnerability triage, exploitability assessment, and false-positive filtering.
6. **`SANCTIONS_SCREENING_MATCH_GENERAL`** *(Bounty #78)*: Benchmarks entity resolution, sanctions list match validation, and PEP disambiguation.

## Evaluator Adapter & Scoring Engine

The centralized `adapter.py` module evaluates model outputs against preference pairs (`chosen` vs. `rejected`), enforcing precise risk categorization, actionable remediation guidance, and context-aware security reasoning.

## Platform Deployment
All datasets are deployed and admitted on the **Anjoman.ai Testnet** under wallet `0x1BD3...4000`.
