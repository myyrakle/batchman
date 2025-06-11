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
}

export interface TaskDefinitionSearchParams {
  keyword?: string;
  status?: 'ACTIVE' | 'INACTIVE';
  page: number;
  size: number;
} 