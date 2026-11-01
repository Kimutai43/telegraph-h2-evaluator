# Telegraph H2 Evaluator | Track 2: Evaluators

A specialized benchmarking and preference-pair curation repository built for the **Telegraph Protocol H2 Hackathon**. This repository contains high-precision, security-focused evaluation datasets designed to test, benchmark, and score large language model (LLM) reasoning across critical cybersecurity domains on the Anjoman testnet.

---

## 📂 Repository Structure

The dataset is strictly categorized into isolated intent pools matching the Anjoman testnet schema:

\\\	ext
telegraph-h2-evaluator/
├── data/
│   ├── PORT_SCAN_AUDIT_GENERAL/
│   │   └── eval_material.json
│   ├── THREAT_IP_REPUTATION_GENERAL/
│   │   └── eval_material.json
│   ├── MALWARE_DETECTION_HASH/
│   │   └── eval_material.json
│   └── EMAIL_SECURITY_GENERAL/
│       └── eval_material.json
└── README.md
\\\

---

## 🎯 Evaluator Intent Categories

1. **Port Scan Audit (PORT_SCAN_AUDIT_GENERAL)**
   * *Focus:* Evaluating LLM capability to analyze Nmap outputs, identify exposure risks (such as public RDP or Telnet), and differentiate critical vs. benign infrastructure footprints.
2. **Threat IP Reputation (THREAT_IP_REPUTATION_GENERAL)**
   * *Focus:* Assessing threat intelligence analysis on IP telemetry, identifying malicious indicators (e.g., Tor exit nodes, active C2 channels) versus trusted public infrastructure.
3. **Malware Detection Hash (MALWARE_DETECTION_HASH)**
   * *Focus:* Testing cryptographic checksum analysis, validating SHA-256 hashes against known threat signatures, and avoiding false positives on benign binaries.
4. **Email Security & Authentication (EMAIL_SECURITY_GENERAL)**
   * *Focus:* Parsing email headers, DMARC/SPF/DKIM evaluation failure states, and detecting phishing typosquatting or malicious disk image attachments.

---

## 📋 Data Schema Specification

All dataset files follow a rigorous preference-pair JSON format optimized for automated testnet evaluation:

\\\json
{
  "prompt": "Input security scenario or telemetry query",
  "chosen": "Accurate, high-risk or secure analytical reasoning (Preferred)",
  "rejected": "Flawed, overly dismissive, or incorrect security assessment"
}
\\\

---

## 🚀 Author & Submission

* **Developer:** Brian Kimutai ([Kimutai43](https://github.com/Kimutai43))
* **Event:** Telegraph Protocol H2 Hackathon (Track 2: Evaluators)
* **Target Network:** Anjoman Testnet
