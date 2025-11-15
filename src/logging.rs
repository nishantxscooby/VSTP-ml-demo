use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize)]
pub struct VstpLog {
    pub timestamp: String,
    pub flow_id: String,
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: String,
    pub packets: u32,
    pub bytes: u64,
    pub duration: f64,
    pub checksum_errors: u32,
    pub dropped_packets: u32,
    pub retransmissions: u32,
    pub flags: Vec<String>,
    pub packet_sizes: Option<Vec<u32>>,
    pub inter_arrivals: Option<Vec<f64>>,
}

pub fn write_log_line(entry: &VstpLog) {
    let path = "/var/log/vstp_logs.jsonl";
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    let payload = serde_json::to_string(entry).unwrap();
    writeln!(f, "{}", payload).unwrap();
}
