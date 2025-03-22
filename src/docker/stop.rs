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
