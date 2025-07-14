import axios, { AxiosError } from "axios";
import config from "./config";

export class ErrorResponse {
  error_code?: string;
  message?: string;

  constructor(error_code: string, message: string) {
    this.error_code = error_code;
    this.message = message;
  }
}

// API 응답 타입 정의
export interface ApiResponse<T> {
  response: T | ErrorResponse;
  status_code: number;
}

// Task Definition 관련 타입
export interface TaskDefinition {
  id: number;
  name: string;
  description: string;
  version: number;
  image: string;
  command: string | null;
  args: string | null;
  env: string | null;
  memory_limit: number | null;
  cpu_limit: number | null;
  created_at: string;
  enabled: boolean;
  is_latest: boolean;
}

export interface ListTaskDefinitionsParams {
  page_number: number;
  page_size: number;
  name?: string;
  task_definition_id?: number;
  contains_name?: string;
  is_latest_only?: boolean;
}

export interface ListTaskDefinitionsResponse {
  task_definitions: TaskDefinition[];
  total_count: number;
}

export interface CreateTaskDefinitionRequest {
  name: string;
  description: string;
  image: string;
  command?: string;
  args?: string;
  env?: string;
  memory_limit?: number;
  cpu_limit?: number;
}

export interface CreateTaskDefinitionResponse {
  task_definition_id: number;
}

export interface PatchTaskDefinitionRequest {
  image?: string;
  command?: string;
  args?: string;
  env?: string;
  memory_limit?: number;
  cpu_limit?: number;
}

// Job 관련 타입
export type JobStatus =
  | "Pending"
  | "Starting"
  | "Running"
  | "Finished"
  | "Failed";

export type ContainerType = "Docker";

export interface Job {
  id: number;
  name: string;
  task_definition_id: number;
  task_definition_name?: string;
  status: JobStatus;
  submited_at: string | null;
  started_at: string | null;
  finished_at: string | null;
  container_type: ContainerType;
  container_id: string | null;
  exit_code: number | null;
  error_message: string | null;
  log_expire_after: string | null;
  log_expired: boolean;
}

export interface SubmitJobRequest {
  task_definition_id: number;
  job_name: string;
  log_expire_after?: string;
}

export interface StopJobRequest {
  job_id: number;
}

export interface SubmitJobResponse {
  job_id: number;
}

export interface ListJobsRequest {
  page_number: number;
  page_size: number;
  job_id?: number;
  status?: JobStatus;
  contains_name?: string;
}

export interface ListJobsResponse {
  jobs: Job[];
  total_count: number;
}

// Job Log 관련 타입
export interface JobLog {
  index: number;
  time: string;
  message: string;
}

export interface ListJobLogsRequest {
  job_id: number;
  offset: number;
  limit: number;
}

export interface ListJobLogsResponse {
  logs: JobLog[];
}

export interface CountJobLogsRequest {
  job_id: number;
}

export interface CountJobLogsResponse {
  count: number;
}

// Schedule 관련 타입
export interface Schedule {
  id: number;
  name: string;
  job_name: string;
  cron_expression: string;
  task_definition_id: number;
  command: string | null;
  timezone: string;
  timezone_offset: number;
  enabled: boolean;
}

export interface CreateScheduleRequest {
  name: string;
  job_name: string;
  cron_expression: string;
  task_definition_id: number;
  command?: string;
  timezone: string;
  timezone_offset: number;
  enabled: boolean;
}

export interface PatchScheduleRequest {
  name?: string;
  job_name?: string;
  cron_expression?: string;
  task_definition_id?: number;
  command?: string;
  timezone?: string;
  timezone_offset?: number;
  enabled?: boolean;
}

// API 클라이언트 생성
const api = axios.create({
  baseURL: config.apiBaseUrl,
  headers: {
    "Content-Type": "application/json",
  },
});

// 에러 처리 헬퍼 함수
const handleApiError = (error: AxiosError): ApiResponse<ErrorResponse> => {
  if (error.response) {
    return {
      response: new ErrorResponse(
        (error.response.data as any)?.error_code,
        (error.response.data as any)?.message,
      ),
      status_code: error.response.status,
    };
  }
  return {
    response: new ErrorResponse("", error.message || "Unknown error occurred"),
    status_code: 500,
  };
};

// Task Definition API
export const listTaskDefinitions = async (
  params: ListTaskDefinitionsParams,
): Promise<ApiResponse<ListTaskDefinitionsResponse | ErrorResponse>> => {
  try {
    const response = await api.get("/task-definitions", { params });
    console.log("API request params:", params); // 디버깅용
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const createTaskDefinition = async (
  taskDefinition: CreateTaskDefinitionRequest,
): Promise<ApiResponse<CreateTaskDefinitionResponse | ErrorResponse>> => {
  try {
    const response = await api.post("/task-definitions", taskDefinition);
    console.log("API response:", response); // 디버깅용
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const patchTaskDefinition = async (
  id: number,
  taskDefinition: PatchTaskDefinitionRequest,
): Promise<ApiResponse<void | ErrorResponse>> => {
  try {
    const response = await api.patch(`/task-definitions/${id}`, taskDefinition);
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const deleteTaskDefinition = async (
  id: number,
): Promise<ApiResponse<void | ErrorResponse>> => {
  try {
    const response = await api.delete(`/task-definitions/${id}`);
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

// Job API
export const submitJob = async (
  request: SubmitJobRequest,
): Promise<ApiResponse<SubmitJobResponse | ErrorResponse>> => {
  try {
    const response = await api.post("/jobs/submit", request);
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const stopJob = async (
  request: StopJobRequest,
): Promise<ApiResponse<void | ErrorResponse>> => {
  try {
    const response = await api.post("/jobs/stop", request);
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const listJobs = async (
  request: ListJobsRequest,
): Promise<ApiResponse<ListJobsResponse | ErrorResponse>> => {
  try {
    const queryParams = new URLSearchParams();
    queryParams.append("page_number", String(request.page_number));
    queryParams.append("page_size", String(request.page_size));

    if (request.job_id) {
      queryParams.append("job_id", String(request.job_id));
    }
    if (request.status) {
      queryParams.append("status", request.status);
    }
    if (request.contains_name) {
      queryParams.append("contains_name", request.contains_name);
    }

    const response = await api.get(`/jobs?${queryParams.toString()}`);
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

// Job Log API
export const listJobLogs = async (
  request: ListJobLogsRequest,
): Promise<ApiResponse<ListJobLogsResponse | ErrorResponse>> => {
  try {
    const queryParams = new URLSearchParams();
    queryParams.append("offset", String(request.offset));
    queryParams.append("limit", String(request.limit));

    const response = await api.get(
      `/jobs/${request.job_id}/logs?${queryParams.toString()}`,
    );
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const countJobLogs = async (
  request: CountJobLogsRequest,
): Promise<ApiResponse<CountJobLogsResponse | ErrorResponse>> => {
  try {
    const response = await api.get(`/jobs/${request.job_id}/logs/count`);
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

// Schedule API
export const listSchedules = async (): Promise<
  ApiResponse<Schedule[] | ErrorResponse>
> => {
  try {
    const response = await api.get("/schedules");
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const createSchedule = async (
  schedule: CreateScheduleRequest,
): Promise<ApiResponse<number | ErrorResponse>> => {
  try {
    const response = await api.post("/schedules", schedule);
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const patchSchedule = async (
  id: number,
  schedule: PatchScheduleRequest,
): Promise<ApiResponse<void | ErrorResponse>> => {
  try {
    const response = await api.patch(`/schedules/${id}`, schedule);
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const deleteSchedule = async (
  id: number,
): Promise<ApiResponse<void | ErrorResponse>> => {
  try {
    const response = await api.delete(`/schedules/${id}`);
    return {
      response: response.data,
      status_code: response.status,
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};
