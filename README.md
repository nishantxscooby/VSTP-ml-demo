#!/bin/bash

# ==============================================================================
# VSTP — REAL-TIME ML ANOMALY DETECTION 🚀
# ==============================================================================

echo "End-to-end Machine Learning–powered anomaly detection for the"
echo "Vishus Secure Transfer Protocol (VSTP)."
echo ""
echo "This project demonstrates how to monitor network flow metadata, run an"
echo "Isolation Forest model in real time, and instantly flag anomalies — all while"
echo "the Rust VSTP server & client exchange data."

# ------------------------------------------------------------------------------
# 📌 FEATURES
# ------------------------------------------------------------------------------
echo "⚡ Real-time anomaly scoring (Isolation Forest, scikit-learn)"
echo "🛰️ Seamless integration with VSTP TCP server + client"
echo "🧪 Synthetic anomaly injector for testing"
echo "📈 Offline analysis + real-time monitoring"
echo "🧰 Easy 4-terminal demo setup"
echo "📦 Runs fully locally — no cloud dependency"

# ------------------------------------------------------------------------------
# 🧠 ML MODEL
# ------------------------------------------------------------------------------
echo "The trained Isolation Forest model is stored as:"
echo "isoforest_vstp.joblib"
echo ""
echo "Extracted Features:"
echo "packets, bytes, duration, jitter, drop_rate, retrans_rate,"
echo "checksum_error_rate, bps, flags_syn, flags_ack"

# ------------------------------------------------------------------------------
# 🧩 FOLDER STRUCTURE
# ------------------------------------------------------------------------------
# VSTP-Vishus-Secure-Transfer-Protocol/
# │
# ├── realtime_isoforest.py        # Realtime anomaly detection script
# ├── inject_anomalies.py          # Synthetic high-severity anomaly generator
# ├── isoforest_vstp.joblib        # Trained Isolation Forest model
# ├── run_all.sh                   # Auto-launch (tmux) 4-pane demo runner
# │
# ├── examples/                    # Rust VSTP examples (tcp_server, tcp_client)
# ├── src/                         # Core Rust library code
# └── venv/                        # Python virtual environment

# ==============================================================================
# 🛠️ INSTALLATION
# ==============================================================================

# 1. Install Rust toolchain
# -------------------------
# curl https://sh.rustup.rs -sSf | sh

# 2. Install Python + venv + ML dependencies
# ------------------------------------------
# cd ~/VSTP-Vishus-Secure-Transfer-Protocol
# python3 -m venv venv
# source venv/bin/activate
# pip install -r requirements.txt

# (If requirements.txt is missing, install manually)
# pip install scikit-learn joblib pandas numpy

# ==============================================================================
# ▶️ FULL 4-TERMINAL DEMO
# ==============================================================================
echo "Run each of the following blocks in a separate terminal window."

# 🖥️ TERMINAL 1 — START VSTP TCP SERVER
# -------------------------------------
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
cargo run --example tcp_server

# 🖥️ TERMINAL 2 — START VSTP TCP CLIENT
# -------------------------------------
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
cargo run --example tcp_client

# 🖥️ TERMINAL 3 — REAL-TIME ML DETECTION (LIVE ALERTS)
# ----------------------------------------------------
echo "This tracks the log file and pipes data to the ML model."
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
source venv/bin/activate

PYTHONUNBUFFERED=1 \
tail -n0 -f /var/log/vstp_logs.jsonl \
| ./venv/bin/python ./realtime_isoforest.py

# Expected Output:
# ALERT,2025-11-20T12:13:05Z,demo_high,-0.24
# ALERT,2025-11-20T12:13:06Z,demo_high,-0.29

# 🖥️ TERMINAL 4 — HIGH-SEVERITY ANOMALY INJECTOR
# ----------------------------------------------
echo "Injects fake 'bad' traffic to trigger the ML alerts."
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
source venv/bin/activate

while true; do
  printf '{"timestamp":"now","flow_id":"demo_high","packets":1,"bytes":9000000,"duration":1.0,"mean_pkt_size":9000000,"std_pkt_size":0.0,"jitter":20.0,"drop_rate":0.4,"retrans_rate":0.0,"checksum_error_rate":0.1,"bps":9000000,"flags_syn":1,"flags_ack":0}\n' >> /var/log/vstp_logs.jsonl
  sleep 1
done

# ==============================================================================
# 🧪 TESTING
# ==============================================================================
echo "To replay or review previous demo sessions:"
# cat alerts_realtime.log
# cat vstp_features.csv

# ==============================================================================
# 📈 OUTPUT FILES
# ==============================================================================
# File                            | Purpose
# --------------------------------|---------------------------------------------
# alerts_realtime.log             | Raw log of all real-time anomaly alerts
# alerts_realtime_structured.csv  | CSV-friendly alert format for easy review
# alerts_offline.csv              | Dump of anomalies from offline analysis
# vstp_features.csv               | All extracted ML features used for detection

# ==============================================================================
# 🚀 AUTOMATIC RUNNER (TMUX)
# ==============================================================================
echo "If you have tmux installed, you can start all 4 windows automatically:"
# ./run_all.sh

# ==============================================================================
# 🙌 CREDITS
# ==============================================================================
echo "Developed by Nishant — VSTP ML realtime anomaly detection project."
