//! Per-device claim check before mutating ddb commands.
//!
//! Shells out to the `device-claim` binary (see substrate-distro/device-claim/).
//! When acquire succeeds, also bumps `last_seen` in the device registry's
//! TOML so operator's enrollment record stays current automatically
//! (operator directive 2026-06-23 msg `4ovb3n`).
//!
//! If `device-claim` is not on PATH, the wrapper degrades gracefully:
//! prints a single stderr warning and lets the call proceed. This keeps
//! ddb usable during the v0.1.b rollout window before the binary is
//! universally installed.

use crate::registry::{Device, DeviceMap, Registry};
use std::process::Command;

/// Try to acquire the claim for the resolved device. Returns Ok(()) when:
///   - claim acquired (exit 0)
///   - claim already owned by this handle (exit 0, same-handle refresh)
///   - prior crashed/stale claim stolen (exit 3) — proceed with warning
///   - device-claim binary missing (graceful skip, stderr warning)
///   - DEVICE_CLAIM_OVERRIDE=1 (operator break-glass)
///
/// Returns Err(msg) when:
///   - exit 2 (claim held by a different live handle)
///
/// `dev_name` is the short name from --device or registry auto-pick.
/// If lookup fails entirely (no devices enrolled, fresh device), the
/// wrapper skips claim with a warning rather than blocking ddb.
pub fn check_or_acquire(dev_name: Option<&str>) -> Result<(), String> {
    // Operator break-glass — env var bypasses claim entirely.
    if std::env::var("DEVICE_CLAIM_OVERRIDE").is_ok() {
        eprintln!("ddb: DEVICE_CLAIM_OVERRIDE=1 — skipping claim check");
        return Ok(());
    }

    // Resolve device → serial. Skip-with-warn if the registry can't.
    let Some((_name, device)) = resolve_device(dev_name) else {
        eprintln!("ddb: no device resolved; skipping claim check");
        return Ok(());
    };
    let claim_id = &device.serial;

    // Shell to device-claim. Skip-with-warn if the binary is missing.
    let output = match Command::new("device-claim")
        .arg("acquire")
        .arg(claim_id)
        .output()
    {
        Ok(o) => o,
        Err(_) => {
            eprintln!("ddb: device-claim binary not found; skipping claim check");
            return Ok(());
        }
    };

    match output.status.code() {
        Some(0) => {
            // Acquired or same-handle refresh. Also bump last_seen.
            let _ = bump_last_seen(claim_id);
            Ok(())
        }
        Some(3) => {
            // Stolen (crashed or stale prior owner). Warn but proceed.
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("ddb: {}", stderr.trim());
            let _ = bump_last_seen(claim_id);
            Ok(())
        }
        Some(2) => {
            // Refused — another live handle owns this device.
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!(
                "ddb refused: {}. To break-glass override: DEVICE_CLAIM_OVERRIDE=1 ddb …",
                stderr.trim()
            ))
        }
        _ => {
            // Other failure — log but don't block.
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("ddb: device-claim error: {}", stderr.trim());
            Ok(())
        }
    }
}

fn resolve_device(dev_name: Option<&str>) -> Option<(String, Device)> {
    let devices: DeviceMap = Registry::load().ok()?;
    Registry::resolve(dev_name, &devices).ok()
}

/// Bump `last_seen` in the device's enrollment record. Best-effort:
/// failure is logged but doesn't block the call.
fn bump_last_seen(serial: &str) -> Result<(), String> {
    let mut devices: DeviceMap = Registry::load()?;
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let mut found = false;
    for dev in devices.values_mut() {
        if dev.serial == serial {
            dev.last_seen = Some(now.clone());
            found = true;
            break;
        }
    }
    if found {
        Registry::save(&devices)?;
    }
    Ok(())
}
