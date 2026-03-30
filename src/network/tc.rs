use crate::error::{NetavarkError, NetavarkResult};
use crate::network::types::BandwidthOptions;
use log::debug;
use std::process::{Command, Stdio};

/// apply_bandwidth_limit applies bandwidth limits to a network interface using tc-tbf.
pub fn apply_bandwidth_limit(iface: &str, bw: &BandwidthOptions) -> NetavarkResult<()> {
    debug!("Applying bandwidth limit to {}: {:?}", iface, bw);

    // 1. Delete existing qdisc (it might not exist, so we ignore errors)
    let _ = Command::new("tc")
        .args(["qdisc", "del", "dev", iface, "root"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    // 2. Add TBF qdisc
    // tc qdisc add dev <iface> root tbf rate <rate>bit burst <burst>b latency <latency>ms
    let output = Command::new("tc")
        .args([
            "qdisc",
            "add",
            "dev",
            iface,
            "root",
            "tbf",
            "rate",
            &format!("{}bit", bw.rate),
            "burst",
            &format!("{}b", bw.burst),
            "latency",
            &format!("{}ms", bw.latency),
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(NetavarkError::msg(format!(
            "failed to apply bandwidth limit to {iface}: {stderr}"
        )));
    }

    Ok(())
}
