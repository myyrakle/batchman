export interface TaskDefinition {
  id: string;
  name: string;
  description: string;
  version: string;
  createdAt: string;
  updatedAt: string;
  status: 'ACTIVE' | 'INACTIVE';
  parameters: {
    name: string;
    type: string;
    required: boolean;
    description: string;
  }[];
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

export interface TaskDefinitionSearchParams {
  keyword?: string;
  status?: 'ACTIVE' | 'INACTIVE';
  page: number;
  size: number;
}

export interface CreateTaskDefinitionFormData {
  name: string;
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