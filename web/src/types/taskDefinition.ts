export interface TaskDefinitionSearchParams {
  keyword?: string;
  status?: 'ACTIVE' | 'INACTIVE';
  page: number;
  size: number;
}

export interface CreateTaskDefinitionFormData {
  name: string;
  description: string;
  image: string;
  command: string;
  env: {
    key: string;
    value: string;
  }[];
  resources: {
    memory: {
      value: number;
      unit: 'm' | 'g';
    };
    cpu: number;
  };
} 