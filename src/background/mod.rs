pub mod job_tracker;
pub mod runner;
pub mod scheduler;

use std::sync::Arc;

use scheduler::ScheduleCDCReceiver;

use crate::context;

pub async fn start_background_loop(
    context: Arc<context::Context>,
    schedule_cdc_receiver: ScheduleCDCReceiver,
) {
    tokio::join!(
        runner::start_runner_loop(context.clone()),
        scheduler::start_scheduler_loop(context.clone(), schedule_cdc_receiver),
        job_tracker::start_status_tracker_loop(context.connection.clone()),
    );
}
