use std::sync::Arc;

use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{
    actions::{self, submit_job::SubmitJobRequest},
    context,
    db::entities,
    routes::jobs::SubmitJobBody,
};

#[derive(Debug)]
pub enum ScheduleCDCEvent {
    New(NewSchedule),
    Update(UpdateSchedule),
    Delete(DeleteSchedule),
}

#[derive(Debug)]
pub struct NewSchedule {
    pub id: i64,
    pub model: entities::schedule::Model,
}

#[derive(Debug)]
pub struct UpdateSchedule {
    pub id: i64,
    pub model: entities::schedule::Model,
}

#[derive(Debug)]
pub struct DeleteSchedule {
    pub id: i64,
}

pub type ScheduleCDCSender = tokio::sync::mpsc::Sender<ScheduleCDCEvent>;
pub type ScheduleCDCReceiver = tokio::sync::mpsc::Receiver<ScheduleCDCEvent>;

pub async fn list_schedules(
    database_connection: &DatabaseConnection,
) -> anyhow::Result<Vec<entities::schedule::Model>> {
    let schedules = entities::schedule::Entity::find()
        .all(database_connection)
        .await?;

    Ok(schedules)
}

pub async fn start_scheduler_loop(
    context: Arc<context::Context>,
    mut receiver: tokio::sync::mpsc::Receiver<ScheduleCDCEvent>,
) {
    let _ = tokio::spawn(async move {
        let mut schedules = list_schedules(&context.connection)
            .await
            .expect("Failed to load schedules");

        // 스케줄링 루프
        loop {
            // 스케줄 데이터가 변경되면 스케줄을 다시 로드
            // TODO: 추후에는 정보 기반으로 변경된 스케줄만 로드하도록 개선
            if let Ok(_) = receiver.try_recv() {
                schedules = list_schedules(&context.connection)
                    .await
                    .expect("Failed to load schedules");
            }

            // 없으면 일단 적당히 대기
            if schedules.is_empty() {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                continue;
            }

            for schedule in schedules.iter() {
                if is_time_to_trigger(schedule) {
                    if let Err(error) = submit_job_by_schedule(&context.connection, schedule).await
                    {
                        log::error!(
                            "Failed to submit job for schedule {}: {}",
                            schedule.id,
                            error
                        );
                    }
                }
            }
        }
    })
    .await;
}

fn is_time_to_trigger(_schedule: &entities::schedule::Model) -> bool {
    // TODO: cron expression을 파싱해서 현재 시간과 비교

    return false;
}

pub async fn submit_job_by_schedule(
    database_connection: &DatabaseConnection,
    schedule: &entities::schedule::Model,
) -> anyhow::Result<()> {
    actions::submit_job::submit_job(SubmitJobRequest {
        connection: database_connection,
        request_body: SubmitJobBody {
            task_definition_id: schedule.task_definition_id,
            job_name: schedule.job_name.clone(),
        },
    })
    .await?;

    Ok(())
}
