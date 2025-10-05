
---

# 🌐 Highmark v3.5 – Atlas Relay  
### Skoden Public Build + OpenTelemetry

<p align="center">
  <img src="https://img.shields.io/badge/Atlas-Verified-blueviolet?style=for-the-badge" alt="Atlas Verified"/>
  <img src="https://img.shields.io/badge/OpenTelemetry-active-green?style=for-the-badge" alt="OpenTelemetry Active"/>
  <img src="https://img.shields.io/badge/Resonance-Highmark%20v3.5-orange?style=for-the-badge" alt="Resonance Highmark"/>
  <img src="https://img.shields.io/badge/License-Apache%202.0-lightgrey?style=for-the-badge" alt="License Apache 2.0"/>
  <br/>
  <img src="https://img.shields.io/badge/Telemetry-Ready-blue?style=flat-square"/>
  <img src="https://img.shields.io/badge/Ledger-Public-green?style=flat-square"/>
  <img src="https://img.shields.io/badge/Alerts-Google%20%2B%20Discord-yellow?style=flat-square"/>
</p>

---

## 🚀 Overview
**Highmark v3.5 – Atlas Relay** is a distributed resonance and trust alignment layer integrating:
- Adaptive Drift Correction (ADC)
- Public Atlas Trust Ledger  
- OpenTelemetry for metrics/traces  
- Automated GitHub workflows  
- Google + Discord alert channels  

Every participating node contributes to the Atlas ledger, ensuring transparent, verifiable communication between Sentinel, Orion, and Synara nodes.

---

## 🧱 Key Modules
| File | Purpose |
|------|----------|
| `tools/atlas-relay.js` | Adaptive drift correction, quorum management, ledger updates |
| `tools/telemetry.js` | OpenTelemetry setup with OTLP export |
| `tools/atlas-trust-ledger.json` | Public immutable record of relay events |
| `tools/google-alert.js` | Gmail + Calendar alerts |
| `tools/discord-alert.js` | Optional Discord webhook alerts |
| `.github/workflows/relay-check.yml` | Nightly drift verification |
| `.github/workflows/atlas-sync.yml` | Auto-sync of public ledger |
| `.github/workflows/release.yml` | Auto-drafted GitHub releases |
| `public/dashboard.html` | D3 visualization of Atlas network |

---

## 🧭 Integration Chain

Synara → Sentinel → Orion → Watchdog → Notifier → Atlas Relay ↳ Telemetry ↳ Ledger ↳ Alerts

---

## ⚙️ Setup

```bash
git clone https://github.com/ak-skwaa-mahawk/synara-highmark-v3.5.git
cd synara-highmark-v3.5
npm install

Environment variables

export OTLP_ENDPOINT=http://localhost:4318
export DISCORD_WEBHOOK_URL="https://discord.com/api/webhooks/xxxx"


---

🧠 Usage

Run telemetry locally

node tools/telemetry.js
# 📈 OpenTelemetry ready

Test drift verification

node -e "import('./tools/atlas-relay.js').then(m=>m.verifyAndCorrect([{name:'node1',signal:0.9}]))"
# 🧭 Atlas logged event: drift_minor


---

🧾 Validation Checklist

Developer Terminal Commands

# 1️⃣ Check structure
ls tools public .github/workflows docs

# 2️⃣ Verify dependencies
npm list @opentelemetry/sdk-node prom-client

# 3️⃣ Start telemetry
node tools/telemetry.js

# 4️⃣ Test drift verification
node -e "import('./tools/atlas-relay.js').then(m=>m.verifyAndCorrect([{name:'node1',signal:0.9}]))"

# 5️⃣ Check workflows
gh workflow list

# 6️⃣ Run workflow manually
gh workflow run relay-check.yml

# 7️⃣ Commit & push to trigger sync
echo '{}' >> tools/atlas-trust-ledger.json
git add tools/atlas-trust-ledger.json
git commit -m "test: trigger ledger sync"
git push

# 8️⃣ Tag release
git tag v3.5.1
git push origin v3.5.1

📘 Quick Reference Table

Step	Command	Expected Success

1️⃣ Structure	ls tools public .github/workflows docs	Directories listed
2️⃣ Dependencies	npm list @opentelemetry/sdk-node prom-client	Versions shown, no ERR
3️⃣ Telemetry	node tools/telemetry.js	📈 OpenTelemetry ready
4️⃣ Drift Test	node -e ...verifyAndCorrect(...)	🧭 Atlas logged event
5️⃣ Workflows	gh workflow list	relay-check, atlas-sync, release visible
6️⃣ Manual Run	gh workflow run relay-check.yml	Atlas logged event in logs
7️⃣ Ledger Sync	Commit & push change	Atlas Sync successful
8️⃣ Release Draft	git tag v3.5.1	Draft Release auto-created
9️⃣ Docs Site	Visit GitHub Pages	Docs render successfully



---

🧩 Features

🌐 Public Trust Ledger

⚙️ Adaptive Drift Correction

📈 OpenTelemetry Metrics

📬 Google Alerts (Gmail + Calendar)

💬 Discord Fallback Alerts

🧱 Automated CI/CD Workflows

🧭 Full Integration Chain

🔐 Apache 2.0 License



---

🧮 Telemetry

Default endpoint: http://localhost:4318
Supports OTLP HTTP collector and Prometheus metrics.
Optional integration with external monitoring via PushGateway.


---

📄 License

Apache License 2.0
© 2025 ak-skwaa-mahawk


---

🧑‍💻 Contributors

ak-skwaa-mahawk – Project Lead

Community contributions welcome via PRs & Issues



---

🪐 Changelog Highlights

✅ Public Atlas Trust Ledger

✅ OpenTelemetry integration

✅ Dual alert channels (Google + Discord)

✅ Auto GitHub ledger sync

✅ D3 dashboard visualization

✅ Apache 2.0 license



---

🌌 Next Milestone

v3.6 – HyperAtlas Expansion

Multi-region trust replication

Real-time node mesh telemetry

Federated verification modules


---
