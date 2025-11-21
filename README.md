#!/bin/bash

# ==============================================================================
# VSTP — Real-Time ML Anomaly Detection 🚀
# ==============================================================================
#
# End-to-end Machine Learning–powered anomaly detection for the
# Vishus Secure Transfer Protocol (VSTP).
#
# This project demonstrates how to:
# 1. Monitor network flow metadata
# 2. Run an Isolation Forest model in real time
# 3. Instantly flag anomalies
#
# ...all while the Rust VSTP server & client exchange data.
#
# 🔗 BADGES (Metadata)
# --------------------
# License:  MIT
# Language: Rust
# ML Model: scikit-learn (Isolation Forest)
# Status:   Active
#
# ==============================================================================
# 📌 FEATURES
# ==============================================================================
# * ⚡ Real-time anomaly scoring
# * 🛰️ Seamless integration with VSTP TCP server + client
# * 🧪 Synthetic anomaly injector for testing
# * 📈 Offline analysis + real-time monitoring
# * 🧰 Easy 4-terminal demo setup
# * 📦 Runs fully locally — no cloud dependency

# ==============================================================================
# 🧠 ML MODEL & DATA
# ==============================================================================
# Model File: isoforest_vstp.joblib
# Features:   packets, bytes, duration, jitter, drop_rate, retrans_rate,
#             checksum_error_rate, bps, flags_syn, flags_ack

# ==============================================================================
# 🛠️ INSTALLATION
# ==============================================================================

# --- Step 1: Install Rust Toolchain ---
# curl https://sh.rustup.rs -sSf | sh

# --- Step 2: Install Python + Dependencies ---
# cd ~/VSTP-Vishus-Secure-Transfer-Protocol
# python3 -m venv venv
# source venv/bin/activate
# pip install -r requirements.txt

# (Manual install if requirements.txt is missing)
# pip install scikit-learn joblib pandas numpy

# ==============================================================================
# ▶️ FULL 4-TERMINAL DEMO (Run in separate tabs)
# ==============================================================================

# ------------------------------------------------------------------------------
# 🖥️ TERMINAL 1: Start VSTP Server
# ------------------------------------------------------------------------------
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
cargo run --example tcp_server

# ------------------------------------------------------------------------------
# 🖥️ TERMINAL 2: Start VSTP Client
# ------------------------------------------------------------------------------
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
cargo run --example tcp_client

# ------------------------------------------------------------------------------
# 🖥️ TERMINAL 3: Real-time ML Detection (Live Alerts)
# ------------------------------------------------------------------------------
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
source venv/bin/activate

# Tail the logs and pipe to the ML detector
PYTHONUNBUFFERED=1 \
tail -n0 -f /var/log/vstp_logs.jsonl \
| ./venv/bin/python ./realtime_isoforest.py

# Expected Output:
# ALERT,2025-11-20T12:13:05Z,demo_high,-0.24
# ALERT,2025-11-20T12:13:06Z,demo_high,-0.29

# ------------------------------------------------------------------------------
# 🖥️ TERMINAL 4: High-Severity Anomaly Injector
# ------------------------------------------------------------------------------
cd ~/VSTP-Vishus-Secure-Transfer-Protocol
source venv/bin/activate

# Inject fake "bad" traffic loop
while true; do
  printf '{"timestamp":"now","flow_id":"demo_high","packets":1,"bytes":9000000,"duration":1.0,"mean_pkt_size":9000000,"std_pkt_size":0.0,"jitter":20.0,"drop_rate":0.4,"retrans_rate":0.0,"checksum_error_rate":0.1,"bps":9000000,"flags_syn":1,"flags_ack":0}\n' >> /var/log/vstp_logs.jsonl
  sleep 1
done

# ==============================================================================
# 🧪 TESTING & OUTPUT FILES
# ==============================================================================
#
# | File                           | Purpose                                   |
# |--------------------------------|-------------------------------------------|
# | alerts_realtime.log            | Raw log of all real-time anomaly alerts   |
# | alerts_realtime_structured.csv | CSV-friendly alert format for easy review |
# | alerts_offline.csv             | Dump of anomalies (offline analysis)      |
# | vstp_features.csv              | All extracted ML features used            |

# To review results:
# cat alerts_realtime.log

# ==============================================================================
# 🚀 AUTOMATIC RUNNER (TMUX)
# ==============================================================================
# If you have tmux installed, launch everything at once:
# ./run_all.sh

# ==============================================================================
# 🙌 CREDITS
# ==============================================================================
# Developed by Nishant — VSTP ML realtime anomaly detection project.
