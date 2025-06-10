import axios, { AxiosError } from 'axios';

const API_BASE_URL = 'http://localhost:13939/api';

export interface ErrorResponse {
    error_code?: string;
    message?: string;
}

// API 응답 타입 정의
export interface ApiResponse<T>  {
    response: T | ErrorResponse
    status_code: number;
}


// Task Definition 관련 타입
export interface TaskDefinition {
  id: number;
  name: string;
  description: string;
  command: string;
  created_at: string;
  updated_at: string;
}

// Job 관련 타입
export interface Job {
  id: number;
  task_definition_id: number;
  status: 'pending' | 'running' | 'completed' | 'failed';
  started_at: string;
  ended_at: string | null;
  output: string | null;
  error: string | null;
}

// Schedule 관련 타입
export interface Schedule {
  id: number;
  task_definition_id: number;
  cron_expression: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

// API 클라이언트 생성
const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// 에러 처리 헬퍼 함수
const handleApiError = (error: AxiosError): ApiResponse<ErrorResponse> => {
  if (error.response) {
    return {
      response: error.response.data as ErrorResponse,
      status_code: error.response.status
    };
  }
  return {
    response: {
      message: error.message || 'Unknown error occurred'
    },
    status_code: 500
  };
};

// Task Definition API
export const listTaskDefinitions = async (): Promise<ApiResponse<TaskDefinition[] | ErrorResponse>> => {
  try {
    const response = await api.get('/task-definitions');
    return {
      response: response.data,
      status_code: response.status
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const createTaskDefinition = async (taskDefinition: Omit<TaskDefinition, 'id' | 'created_at' | 'updated_at'>): Promise<ApiResponse<TaskDefinition | ErrorResponse>> => {
  try {
    const response = await api.post('/task-definitions', taskDefinition);
    return {
      response: response.data,
      status_code: response.status
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const patchTaskDefinition = async (id: number, taskDefinition: Partial<TaskDefinition>): Promise<ApiResponse<TaskDefinition | ErrorResponse>> => {
  try {
    const response = await api.patch(`/task-definitions/${id}`, taskDefinition);
    return {
      response: response.data,
      status_code: response.status
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const deleteTaskDefinition = async (id: number): Promise<ApiResponse<void | ErrorResponse>> => {
  try {
    const response = await api.delete(`/task-definitions/${id}`);
    return {
      response: response.data,
      status_code: response.status
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

// Job API
export const submitJob = async (taskDefinitionId: number): Promise<ApiResponse<Job | ErrorResponse>> => {
  try {
    const response = await api.post('/jobs/submit', { task_definition_id: taskDefinitionId });
    return {
      response: response.data,
      status_code: response.status
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const stopJob = async (jobId: number): Promise<ApiResponse<void | ErrorResponse>> => {
  try {
    const response = await api.post('/jobs/stop', { job_id: jobId });
    return {
      response: response.data,
      status_code: response.status
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

// Schedule API
export const listSchedules = async (): Promise<ApiResponse<Schedule[] | ErrorResponse>> => {
  try {
    const response = await api.get('/schedules');
    return {
      response: response.data,
      status_code: response.status
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const createSchedule = async (schedule: Omit<Schedule, 'id' | 'created_at' | 'updated_at'>): Promise<ApiResponse<Schedule | ErrorResponse>> => {
  try {
    const response = await api.post('/schedules', schedule);
    return {
      response: response.data,
      status_code: response.status
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const patchSchedule = async (id: number, schedule: Partial<Schedule>): Promise<ApiResponse<Schedule | ErrorResponse>> => {
  try {
    const response = await api.patch(`/schedules/${id}`, schedule);
    return {
      response: response.data,
      status_code: response.status
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

export const deleteSchedule = async (id: number): Promise<ApiResponse<void | ErrorResponse>> => {
  try {
    const response = await api.delete(`/schedules/${id}`);
    return {
      response: response.data,
      status_code: response.status
    };
  } catch (error) {
    return handleApiError(error as AxiosError);
  }
};

