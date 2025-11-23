# 🚀 VSTP — Real-Time ML Anomaly Detection
# =======================================

# Virtual Secure Transfer Protocol (VSTP) is a Rust-based high-performance
# network transfer protocol paired with real-time Machine Learning anomaly
# detection using Python (Isolation Forest).

# This repository showcases:
# - A Rust TCP server & client (full working example)
# - Real-time ML anomaly detection on live traffic
# - A streaming JSONL log pipeline
# - Dashboard-ready alert outputs
# - High-severity anomaly injector for simulation


# 🏗️ Directory Structure
# ----------------------
# .
# ├── src/                      # Rust source code for VSTP
# ├── examples/                 # tcp_server & tcp_client
# ├── realtime_isoforest.py     # Python real-time anomaly detector
# ├── inject_anomalies.py       # High-severity anomaly injector
# ├── isoforest_vstp.joblib     # Pretrained Isolation Forest model
# ├── run_all.sh                # Automates full 4-terminal setup
# ├── alerts_realtime.log       # Example output
# ├── alerts_realtime_structured.csv
# ├── Cargo.toml                # Rust project config
# └── README.md


# ⚙️ Requirements
# ---------------
# Rust (latest stable)
# Python 3.11+
# pip install joblib pandas scikit-learn numpy scipy


# 🧠 Real-Time ML Architecture
# ----------------------------
#    tcp_client  --->  tcp_server  --->  /var/log/vstp_logs.jsonl
#                                       | (streaming log file)
#                                       ↓
#                           realtime_isoforest.py
#                                  |
#                                  ↓
#                          🔔 ALERT, flow_id, score


# 🔥 Full 4-Terminal Demo (Manual Run)
# ====================================

# ------------------------------
# Terminal 1 — Start VSTP Server
# ------------------------------
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
cargo run --example tcp_server


# -----------------------------
# Terminal 2 — Start VSTP Client
# -----------------------------
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
cargo run --example tcp_client


# -----------------------------------------------------------------
# Terminal 3 — Realtime ML Detection (prints alerts instantly)
# -----------------------------------------------------------------
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
source venv/bin/activate

PYTHONUNBUFFERED=1 \
tail -n0 -f /var/log/vstp_logs.jsonl \
| ./venv/bin/python ./realtime_isoforest.py


# -----------------------------------------------------------------
# Terminal 4 — High Severity Anomaly Injector (simulated attacks)
# -----------------------------------------------------------------
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
source venv/bin/activate

while true; do
  printf '{"timestamp":"now","flow_id":"demo_high","packets":1,"bytes":9000000,"duration":1.0,"mean_pkt_size":9000000,"std_pkt_size":0.0,"jitter":20.0,"drop_rate":0.4,"retrans_rate":0.0,"checksum_error_rate":0.1,"bps":9000000,"flags_syn":1,"flags_ack":0}\n' \
  >> /var/log/vstp_logs.jsonl
  sleep 1
done


# 🛡️ Alert Format
# ----------------
# The ML system emits alerts in CSV-friendly format:

# ALERT,<timestamp>,<flow_id>,<anomaly_score>

# Example:
# ALERT,2025-11-21T01:41:56.223Z,demo_high,-0.24648


# 📊 Model Details
# ----------------
# - Isolation Forest (unsupervised)
# - Trained on real VSTP network traffic
# - Saved as: isoforest_vstp.joblib
# - Realtime scoring speed: <1ms per event
# - CSV reconstruction supported


# 📈 Offline Analysis
# -------------------
# python3 inject_anomalies.py > alerts_offline.csv
# python3 realtime_isoforest.py < sample.jsonl > realtime.csv


# 🛠️ run_all.sh — Start everything automatically
# ----------------------------------------------
# ./run_all.sh
#
# Creates:
#  - tmux session
#  - Server window
#  - Client window
#  - Detection window
#  - Injection window


# 🧪 Example Output
# -----------------
# ALERT,2025-11-20T21:13:09.784689+00:00,demo_high,-0.220393
# ALERT,2025-11-20T21:13:10.890316+00:00,demo_high,-0.221392
# ALERT,2025-11-20T21:13:11.931511+00:00,demo_high,-0.243815


# 🧩 Why This Exists
# -------------------
# - Research-grade anomaly detection for network security
# - Demonstrates how Rust + Python can integrate seamlessly
# - Real-time ML at production speeds
# - Extendable for IDS/IPS, DevSecOps pipelines, or SOC automation


# 🏁 Contribute
# -------------
# PRs welcome — ML enhancements, VSTP extensions, documentation, dashboards.


# 🌐 SEO / GitHub Badges
# ----------------------
# ![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
# ![Python](https://img.shields.io/badge/Python-3776AB?logo=python&logoColor=white)
# ![Machine Learning](https://img.shields.io/badge/ML-Isolation_Forest-brightgreen)
# ![Security](https://img.shields.io/badge/Security-Network_Anomaly_Detection-red)
# ![VSTP](https://img.shields.io/badge/VSTP-Live_Traffic_Analysis-blue)
# ![Logs](https://img.shields.io/badge/Logs-Streaming-orange)


# ✨ Author
# ----------
# Nishant — B.Tech CSE, Bennett University  
# Rust + ML + Systems Programming Enthusiast  
