use std::process::Command;

/// Forcefully stops a Docker container by sending a SIGKILL signal
///
/// # Arguments
///
/// * `container_id` - The ID or name of the container to stop
///
/// # Returns
///
/// * `Ok(())` if the container was successfully stopped
/// * `Err` with an error message if the operation failed
///
pub fn kill_container(container_id: &str) -> anyhow::Result<()> {
    let mut command = Command::new("docker");

    command.arg("kill");
    command.arg(container_id);

    let output = command.output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);

        if error.contains("No such container") {
            return Err(anyhow::anyhow!("Container not found: {}", container_id));
        }

        return Err(anyhow::anyhow!(
            "Failed to kill Docker container: {}",
            error
        ));
    }

    Ok(())
}

/// Forcefully stops a Docker container with a timeout
///
/// This function first attempts to gracefully stop the container with `docker stop`
/// using the specified timeout. If that fails, it forcefully kills the container
/// with `docker kill`.
///
/// # Arguments
///
/// * `container_id` - The ID or name of the container to stop
/// * `timeout_seconds` - The timeout in seconds to wait for graceful shutdown
///
/// # Returns
///
/// * `Ok(())` if the container was successfully stopped
/// * `Err` with an error message if both stop and kill operations failed
///
pub fn stop_container(container_id: &str, timeout_seconds: u32) -> anyhow::Result<()> {
    // First try to stop gracefully with timeout
    let mut command = Command::new("docker");

    command.arg("stop");
    command.arg("--time");
    command.arg(timeout_seconds.to_string());
    command.arg(container_id);

    let output = command.output()?;

    // If stop succeeded, return success
    if output.status.success() {
        return Ok(());
    }

    // If stop failed for a reason other than "container not found", log the error
    let error = String::from_utf8_lossy(&output.stderr);
    if !error.contains("No such container") {
        eprintln!("Warning: Failed to gracefully stop container: {}", error);
    }

    // Fall back to kill
    kill_container(container_id)
}
