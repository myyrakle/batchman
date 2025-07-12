use std::fs::File;
use std::io::{BufRead, BufReader, SeekFrom};

/*
사용 예: 100번째 줄부터 500줄 가져오기
let lines = read_lines_range("large_file.txt", 100, 500)?;
*/
pub(crate) fn read_lines_range(
    file_path: &str,
    start: usize,
    count: usize,
) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .skip(start) // 시작 지점까지 스킵
        .take(count) // 필요한 개수만 가져오기
        .collect::<Result<Vec<_>, _>>()?;

    Ok(lines)
}

/*
파일의 총 줄 수를 계산합니다.
*/
pub(crate) fn count_lines(file_path: &str) -> Result<usize, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let count = reader.lines().count();

    Ok(count)
}

/*
Log Tailing을 위한 유틸리티 객체
*/
#[allow(dead_code)]
#[derive(Debug)]
pub struct Tailer {
    reader: tokio::io::BufReader<tokio::fs::File>,
}

#[allow(dead_code)]
impl Tailer {
    pub async fn new(file_path: &str) -> std::io::Result<Self> {
        use tokio::io::AsyncSeekExt;

        let file = tokio::fs::File::open(file_path).await?;
        let mut reader = tokio::io::BufReader::new(file);

        reader.seek(SeekFrom::End(0)).await?;

        Ok(Tailer { reader })
    }

    // 현재 Seek 위치부터 추가된 행이 더 있다면 최대 10행까지 읽어서 반환하고, Seek 위치를 업데이트합니다.
    pub async fn tail(&mut self, num_lines: usize) -> Result<Vec<String>, std::io::Error> {
        use tokio::io::AsyncBufReadExt;

        let mut lines = vec![];

        // 현재 Seek 위치부터 읽기 시작
        let mut line = String::new();
        while lines.len() < num_lines && self.reader.read_line(&mut line).await? > 0 {
            lines.push(line.trim().to_string());
            line.clear();
        }

        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    use tokio::io::AsyncWriteExt;

    use super::*;

    #[tokio::test]
    async fn test_tailing() {
        let log_file_path = "test_log.txt";

        // 로그 파일에 1초마다 3줄씩 추가하는 작업을 spawn합니다.
        let _write_log_task = tokio::spawn(async move {
            let file = tokio::fs::OpenOptions::new()
                .create(false)
                .write(true)
                .truncate(true)
                .open(log_file_path)
                .await
                .expect("Failed to open log file");

            let mut writer = tokio::io::BufWriter::new(file);

            for i in 0..20 {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                let log_entry = format!("Log entry {}\n", i);

                writer
                    .write(log_entry.as_bytes())
                    .await
                    .expect("Failed to write to log file");

                writer.flush().await.expect("Failed to flush log file");
            }
        });

        let _tail_task = tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            let mut tailer = Tailer::new(log_file_path).await.unwrap();

            // tailing을 시도합니다.
            loop {
                let lines = tailer.tail(3).await.unwrap();
                if !lines.is_empty() {
                    println!("Tailed lines: {:?}", lines);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
