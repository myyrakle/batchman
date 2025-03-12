pub(crate) mod actions;
pub(crate) mod db;
pub(crate) mod docker;

use std::fs::File;
use std::io::Write;
use std::process::Command;

use db::{entities, setup_schema};
use docker::run_container;

#[tokio::main]
async fn main() {
    let connection = db::create_database_connection().await.unwrap();

    setup_schema(&connection).await;

    // // Docker 로그 가져오기
    // let log_output = Command::new("docker")
    //     .arg("logs")
    //     .arg(container_name)
    //     .output()
    //     .expect("Failed to get Docker logs");

    // if !log_output.status.success() {
    //     eprintln!(
    //         "Failed to get Docker logs: {}",
    //         String::from_utf8_lossy(&log_output.stderr)
    //     );
    //     return;
    // }

    // // 로그를 파일에 저장
    // let mut file = File::create("docker_logs.txt").expect("Failed to create log file");
    // file.write_all(&log_output.stdout)
    //     .expect("Failed to write logs to file");

    // println!("Docker logs saved to docker_logs.txt");
}
