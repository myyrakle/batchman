use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

use crate::db::entities::{self, job::JobStatus};

pub async fn start_runner_loop(database_connection: DatabaseConnection) {
    let _ = tokio::spawn(async move {
        loop {
            let mut find_query = entities::job::Entity::find();

            find_query = find_query.filter(entities::job::Column::Status.eq(JobStatus::Pending));
            find_query = find_query.limit(5); // TODO: config 설정 가능한 값으로 빼기

            let Ok(pending_jobs) = find_query.all(&database_connection).await else {
                println!("Error fetching pending jobs");
                // TODO: 대기시간을 config 설정 가능한 값으로 빼기
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                continue;
            };

            if pending_jobs.is_empty() {
                // TODO: 대기시간을 config 설정 가능한 값으로 빼기
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                continue;
            }

            for pending_job in pending_jobs {
                // TODO: 리소스 제한이나 실행 제한 등에 걸리지 않는지 확인 (차후 개발)

                // TODO: job 상태를 START로 변경

                // TODO: 컨테이너 실행

                // TODO: 컨테이너 ID를 job에 업데이트

                // TODO: job 상태를 RUNNING으로 변경
            }
        }
    })
    .await;
}
