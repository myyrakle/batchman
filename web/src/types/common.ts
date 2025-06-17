export interface ApiResponse<T> {
  data: T;
  error?: string;
}

export interface ErrorResponse {
  message: string;
  code: string;
} 