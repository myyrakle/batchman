use std::sync::Arc;

use crate::{
    context::{self},
    domain::{
        job::dto::{SubmitJobBody, SubmitJobRequest},
        schedule::{
            dao::PatchScheduleParams,
            entities::{self, schedule::ScheduleWithStates},
        },
    },
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

pub type _ScheduleCDCSender = tokio::sync::mpsc::Sender<ScheduleCDCEvent>;
pub type ScheduleCDCReceiver = tokio::sync::mpsc::Receiver<ScheduleCDCEvent>;

pub async fn start_scheduler_loop(
    context: Arc<context::Context>,
    mut receiver: tokio::sync::mpsc::Receiver<ScheduleCDCEvent>,
) {
    let _ = tokio::spawn(async move {
        let mut schedules = context
            .schedule_repository
            .list_schedules(Default::default())
            .await
            .expect("Failed to load schedules")
            .into_iter()
            .flat_map(ScheduleWithStates::try_from)
            .collect::<Vec<_>>();

        // 스케줄링 루프
        loop {
            let now = chrono::Utc::now();

            // 스케줄 데이터가 변경되면 스케줄을 다시 로드
            // TODO: 추후에는 정보 기반으로 변경된 스케줄만 로드하도록 개선
            if let Ok(_) = receiver.try_recv() {
                schedules = context
                    .schedule_repository
                    .list_schedules(Default::default())
                    .await
                    .expect("Failed to load schedules")
                    .into_iter()
                    .flat_map(ScheduleWithStates::try_from)
                    .collect::<Vec<_>>();
            }

            // 없으면 일단 적당히 대기
            if schedules.is_empty() {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                continue;
            }

            for schedule in schedules.iter() {
                if schedule.is_time_to_trigger(&now) {
                    if let Err(error) = context
                        .job_service
                        .submit_job(SubmitJobRequest {
                            request_body: SubmitJobBody {
                                task_definition_id: schedule.model.task_definition_id,
                                job_name: schedule.model.job_name.clone(),
                            },
                        })
                        .await
                    {
                        log::error!(
                            "Failed to submit job for schedule {}: {}",
                            schedule.model.id,
                            error
                        );
                    }

                    if let Err(error) = context
                        .schedule_repository
                        .patch_schedule(PatchScheduleParams {
                            schedule_id: schedule.model.id,
                            last_triggered_at: Some(now),
                            ..Default::default()
                        })
                        .await
                    {
                        log::error!(
                            "Failed to update last triggered time for schedule {}: {}",
                            schedule.model.id,
                            error
                        );
                    }
                }
            }
        }
    })
    .await;
}
