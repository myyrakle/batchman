use std::fs::File;
use std::io::{BufRead, BufReader};

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
