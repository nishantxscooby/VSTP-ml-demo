# VSTP — Real-Time ML Anomaly Detection 🚀  
A fully integrated pipeline combining **Virtual Secure Transfer Protocol (VSTP)** + **Rust server/client** + **Python ML anomaly detection using Isolation Forest**.

---

# 🎯 Perfect For  

- **Real-time Gaming:** Low-latency UDP/TCP with optional reliability  
- **IoT Systems:** Lightweight protocol with robust error handling  
- **Microservices:** Fast inter-service communication with rich metadata  
- **Streaming Applications:** Efficient binary messaging with fragmentation support  
- **Blockchain Networks:** Secure peer-to-peer communication  
- **Edge Computing:** Minimal overhead, maximum throughput  

---

# ⚡ Installation  

Add VSTP to your **Cargo.toml**:

```toml
[dependencies]
vstp = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

Create virtual environment for ML scripts:

```bash
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

---

# 🚀 Quick Start — 4-Terminal Demo (Full Pipeline)

Run VSTP **server**, **client**, **ML realtime detection**, **anomaly injector**.

---

## 🖥️ Terminal 1 — VSTP TCP Server

```bash
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
cargo run --example tcp_server
```

---

## 🖥️ Terminal 2 — VSTP TCP Client

```bash
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
cargo run --example tcp_client
```

---

## 🧠 Terminal 3 — Realtime ML Detection (live alerts)

```bash
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
source venv/bin/activate

PYTHONUNBUFFERED=1 \
tail -n0 -f /var/log/vstp_logs.jsonl \
| ./venv/bin/python ./realtime_isoforest.py
```

---

## 💥 Terminal 4 — High-Severity Anomaly Injector

```bash
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
source venv/bin/activate

while true; do
  printf '{"timestamp":"now","flow_id":"demo_high","packets":1,"bytes":9000000,"duration":1.0,"mean_pkt_size":9000000,"std_pkt_size":0.0,"jitter":20.0,"drop_rate":0.4,"retrans_rate":0.0,"checksum_error_rate":0.1,"bps":9000000,"flags_syn":1,"flags_ack":0}\n' >> /var/log/vstp_logs.jsonl
  sleep 1
done
```

---

# 📊 Alert Format  

Example output:

```text
ALERT,2025-11-20T21:13:03.522845+00:00,demo_high,-0.24646804413218
```

Each alert includes:

- **Timestamp**  
- **Flow ID**  
- **ML score (Isolation Forest)**  
- **Negative score = anomaly**  

---

# 🧬 ML Model (Isolation Forest)

- Model trained on **vstp_features.csv**  
- Saved as: **isoforest_vstp.joblib**  
- Used for **real-time inference**  
- Zero-latency scoring via streaming logs  

Replace model anytime:

```bash
cp new_model.joblib isoforest_vstp.joblib
```

---

# 🏗️ Project Structure  

```
VSTP-Vishus-Secure-Transfer-Protocol/
│── src/                 # Rust VSTP implementation
│── examples/            # tcp_server / tcp_client
│── venv/                # Python virtual environment
│── isoforest_vstp.joblib
│── realtime_isoforest.py
│── inject_anomalies.py
│── run_all.sh           # Auto-launch script (optional)
│── README.md
```

---

# 🔍 Realtime ML Pipeline  

1. **tcp_client** sends frames  
2. **tcp_server** receives & writes features into `/var/log/vstp_logs.jsonl`  
3. **tail -n0 -f** streams data  
4. **Isolation Forest** model scores live traffic  
5. Alerts printed instantly in terminal  

---

# 🛠️ Offline Analysis  

Generate offline alerts:

```bash
python3 analyze_offline.py ./vstp_features.csv
```

---

# 🧪 Testing  

Run tests:

```bash
cargo test
```

---

# 👨‍💻 Author  

**Nishant**  
B.Tech CSE  
GitHub: https://github.com/nishantxscooby  

---

# ⭐ Contribute  

PRs welcome!  
Fork → Add features → Open PR.

---

# 📜 License  

Dual licensed: MIT + Apache-2.0

