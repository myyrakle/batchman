mod db;

use std::fs::File;
use std::io::Write;
use std::process::Command;

use db::setup_schema;

#[tokio::main]
async fn main() {
    let connection = db::create_database_connection().await.unwrap();

    setup_schema(&connection).await;

    // // Docker 컨테이너 실행
    // let container_name = "my_container";
    // let image_name = "ubuntu:latest";

    // let output = Command::new("docker")
    //     .arg("run")
    //     .arg("--name")
    //     .arg(container_name)
    //     .arg("-d")
    //     .arg(image_name)
    //     .output()
    //     .expect("Failed to start Docker container");

    // if !output.status.success() {
    //     eprintln!(
    //         "Failed to start Docker container: {}",
    //         String::from_utf8_lossy(&output.stderr)
    //     );
    //     return;
    // }

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
