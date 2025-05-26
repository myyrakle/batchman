use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SubmitJobBody {
    pub task_definition_id: i64,
    pub job_name: String,
}

#[derive(Debug, Clone)]
pub struct SubmitJobRequest {
    pub request_body: SubmitJobBody,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StopJobBody {
    pub job_id: i64,
}

#[derive(Debug, Clone)]
pub struct StopJobRequest {
    pub request_body: StopJobBody,
}
