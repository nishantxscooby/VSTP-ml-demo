# VSTP — Real-Time ML Anomaly Detection 🚀
#
# End-to-end **Machine Learning–powered anomaly detection** for the Vishus Secure Transfer Protocol (VSTP).
#
# This project demonstrates how to monitor network flow metadata, run an **Isolation Forest** model in real time,
# and instantly flag anomalies — all while the **Rust VSTP server & client** exchange data.

# --------------------------------------------------------------------------

## 🔗 GitHub SEO Badges

# [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
# [![Project Status](https://img.shields.io/badge/Status-Active-brightgreen)](https://github.com/YourUsername/YourRepoName)
# [![Language: Rust](https://img.shields.io/badge/Language-Rust-orange)](https://www.rust-lang.org/)
# [![ML Model: scikit-learn](https://img.shields.io/badge/ML%20Model-scikit--learn-blue)](https://scikit-learn.org/)

# --------------------------------------------------------------------------

## 📌 Features

# * ⚡ Real-time anomaly scoring (Isolation Forest, scikit-learn)
# * 🛰️ Seamless integration with VSTP TCP server + client
# * 🧪 Synthetic anomaly injector for testing
# * 📈 Offline analysis + real-time monitoring
# * 🧰 Easy 4-terminal demo setup
# * 📦 Runs fully locally — no cloud dependency

# --------------------------------------------------------------------------

## 🧠 ML Model

# The trained **Isolation Forest** model is used for unsupervised anomaly detection.

# Model File: `isoforest_vstp.joblib`

# Extracted Features:
# packets, bytes, duration, jitter, drop_rate, retrans_rate, checksum_error_rate, bps, flags_syn, flags_ack

# --------------------------------------------------------------------------

## 🧩 Folder Structure

# VSTP-Vishus-Secure-Transfer-Protocol/
# │
# ├── realtime_isoforest.py        # Realtime anomaly detection
# ├── inject_anomalies.py          # Synthetic high-severity anomaly generator
# ├── isoforest_vstp.joblib        # Trained Isolation Forest model
# ├── run_all.sh                   # Auto-launch (tmux) 4-pane demo runner
# │
# ├── examples/                    # Rust VSTP examples (tcp_server, tcp_client)
# ├── src/                         # Core Rust library code
# └── venv/                        # Python virtual environment

# --------------------------------------------------------------------------

## 🛠️ Installation

### 1. Install Rust toolchain

echo "Installing Rust toolchain..."
# curl https://sh.rustup.rs -sSf | sh

### 2. Install Python + venv + ML dependencies

echo "Setting up Python environment and dependencies..."

# Navigate to project directory
# Assumes you are already in the project root if cloning
# cd ~/VSTP-Vishus-Secure-Transfer-Protocol

# python3 -m venv venv
# source venv/bin/activate
# pip install -r requirements.txt

# Note: If requirements.txt is missing, install manually:
# pip install scikit-learn joblib pandas numpy

# --------------------------------------------------------------------------

## ▶️ FULL 4-TERMINAL DEMO

# Below are the **exact 4 commands** you need to run the end-to-end demo.
# Each command should be executed in a **separate terminal** session.

# --------------------------------------------------------------------------

### 🖥️ Terminal 1 — Start VSTP TCP Server

echo "--- COMMAND FOR TERMINAL 1 (Server) ---"
echo "cd ~/VSTP-Vishus-Secure-Transfer-Protocol"
echo "cargo run --example tcp_server"

### 🖥️ Terminal 2 — Start VSTP TCP Client

echo "--- COMMAND FOR TERMINAL 2 (Client) ---"
echo "cd ~/VSTP-Vishus-Secure-Transfer-Protocol"
echo "cargo run --example tcp_client"

### 🖥️ Terminal 3 — Real-time ML Detection (live alerts)

echo "--- COMMAND FOR TERMINAL 3 (ML Detector) ---"
echo "cd ~/VSTP-Vishus-Secure-Transfer-Protocol"
echo "source venv/bin/activate"
echo "PYTHONUNBUFFERED=1 \\"
echo "tail -n0 -f /var/log/vstp_logs.jsonl \\"
echo "| ./venv/bin/python ./realtime_isoforest.py"

# Expected output will look like:
# ALERT,2025-11-20T12:13:05Z,demo_high,-0.24
# ALERT,2025-11-20T12:13:06Z,demo_high,-0.29

### 🖥️ Terminal 4 — High-Severity Anomaly Injector

echo "--- COMMAND FOR TERMINAL 4 (Injector) ---"
echo "cd ~/VSTP-Vishus-Secure-Transfer-Protocol"
echo "source venv/bin/activate"
echo ""
echo "while true; do"
echo "  printf '{\"timestamp\":\"now\",\"flow_id\":\"demo_high\",\"packets\":1,\"bytes\":9000000,\"duration\":1.0,\"mean_pkt_size\":9000000,\"std_pkt_size\":0.0,\"jitter\":20.0,\"drop_rate\":0.4,\"retrans_rate\":0.0,\"checksum_error_rate\":0.1,\"bps\":9000000,\"flags_syn\":1,\"flags_ack\":0}\\n' >> /var/log/vstp_logs.jsonl"
echo "  sleep 1"
echo "done"

# --------------------------------------------------------------------------

## 🧪 Testing

# To replay or review previous demo sessions:
# cat alerts_realtime.log
# cat vstp_features.csv

# --------------------------------------------------------------------------

## 📈 Output Files

# File | Purpose
# :--- | :---
# alerts_realtime.log | Raw log of all real-time anomaly alerts
# alerts_realtime_structured.csv | CSV-friendly alert format for easy review
# alerts_offline.csv | Dump of anomalies detected during any offline analysis
# vstp_features.csv | All extracted ML features used for detection

# --------------------------------------------------------------------------

## 🚀 Automatic 4-Pane Runner (tmux)

# If you have **tmux** installed, you can start all 4 windows automatically:
# ./run_all.sh

# --------------------------------------------------------------------------

## 🙌 Credits

# Developed by **Nishant** — VSTP ML realtime anomaly detection project.
