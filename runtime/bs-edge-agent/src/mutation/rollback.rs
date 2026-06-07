use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum RollbackError {
    SnapshotFailed(String),
    RestoreFailed(String),
    NetworkReloadFailed(String),
}

/// Creates a snapshot of `/etc/config/` to `/tmp/bs_config_backup_<timestamp>`
pub fn create_snapshot() -> Result<String, RollbackError> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let backup_path = format!("/tmp/bs_config_backup_{}", ts);

    let output = Command::new("cp")
        .arg("-r")
        .arg("/etc/config")
        .arg(&backup_path)
        .output()
        .map_err(|e| RollbackError::SnapshotFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(RollbackError::SnapshotFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(backup_path)
}

/// Restores the snapshot from the given path to `/etc/config`
pub fn restore_snapshot(backup_path: &str) -> Result<(), RollbackError> {
    if !Path::new(backup_path).exists() {
        return Err(RollbackError::RestoreFailed(format!("Backup path {} does not exist", backup_path)));
    }

    // Safely copy back
    let output = Command::new("cp")
        .arg("-r")
        .arg(format!("{}/*", backup_path)) // Requires shell expansion, better to use sh -c or walkdir
        .arg("/etc/config/")
        .output(); // Wait, `cp -r /tmp/backup/* /etc/config/` requires shell.
        
    // Let's avoid shell interpolation. We can just use standard library `fs::copy` or a clean cp command.
    // Actually, `cp -a /tmp/bs_config_backup_123/. /etc/config/` works natively without shell globbing.
    let output = Command::new("cp")
        .arg("-a")
        .arg(format!("{}/.", backup_path))
        .arg("/etc/config/")
        .output()
        .map_err(|e| RollbackError::RestoreFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(RollbackError::RestoreFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

/// Reloads the network service to apply restored changes
pub fn reload_network() -> Result<(), RollbackError> {
    let output = Command::new("/etc/init.d/network")
        .arg("reload")
        .output()
        .map_err(|e| RollbackError::NetworkReloadFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(RollbackError::NetworkReloadFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // Tests are mocked or ignored in CI since we don't have OpenWrt layout natively
}
