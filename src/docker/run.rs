use std::process::Command;

use crate::domain::task_definition::entities;

pub fn run_container(task_definition: entities::task_definition::Model) -> anyhow::Result<String> {
    // Docker 컨테이너 실행
    let image_name = &task_definition.image;

    let mut command = Command::new("docker");

    command.arg("run");
    command.arg("-d");

    // 메모리 제한 설정
    if let Some(memory_limit) = task_definition.memory_limit {
        command.arg("--memory");
        command.arg(format!("{}m", memory_limit));
    }

    // CPU 제한 설정
    if let Some(cpu_limit) = task_definition.cpu_limit {
        command.arg("--cpu-shares");
        command.arg(format!("{}", cpu_limit));
    }

    // 환경 변수 설정
    if let Some(env_vars) = &task_definition.env {
        for env_var in env_vars.split(',') {
            command.arg("-e");
            command.arg(env_var);
        }
    }

    command.arg(image_name);

    // CMD 설정
    if let Some(cmd) = &task_definition.command {
        let command_list = serde_json::from_str::<Vec<String>>(cmd)?;

        for cmd in command_list {
            command.arg(cmd);
        }
    }

    // Arguments 전달
    if let Some(args) = &task_definition.args {
        for arg in args.split(',') {
            command.arg(arg);
        }
    }

    let output = command.output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to start Docker container: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let container_id = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    Ok(container_id)
}
