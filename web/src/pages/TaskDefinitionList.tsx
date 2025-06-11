import React, { useState } from 'react';
import { Box, Button, Typography } from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import { TaskDefinition, TaskDefinitionSearchParams } from '../types/taskDefinition';
import TaskDefinitionTable from '../components/TaskDefinitionTable';
import TaskDefinitionSearch from '../components/TaskDefinitionSearch';
import CreateTaskDefinitionModal from '../components/CreateTaskDefinitionModal';
import CreateVersionModal from '../components/CreateVersionModal';

const TaskDefinitionList: React.FC = () => {
  const [searchParams, setSearchParams] = useState<TaskDefinitionSearchParams>({
    keyword: '',
    status: undefined,
    page: 0,
    size: 10,
  });

  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [isVersionModalOpen, setIsVersionModalOpen] = useState(false);
  const [selectedTask, setSelectedTask] = useState<TaskDefinition | null>(null);

  // 임시 데이터
  const taskDefinitions: TaskDefinition[] = [
    {
      id: '1',
      name: 'Sample Task',
      description: 'This is a sample task',
      version: '1.0.0',
      createdAt: '2024-03-20',
      updatedAt: '2024-03-20',
      status: 'ACTIVE',
      parameters: [],
    },
  ];

  const handleSearch = () => {
    // TODO: API 호출 구현
    console.log('Search with params:', searchParams);
  };

  const handleCreateTask = () => {
    setIsCreateModalOpen(true);
  };

  const handleCreateVersion = (task: TaskDefinition) => {
    setSelectedTask(task);
    setIsVersionModalOpen(true);
  };

  const handleCreateTaskSubmit = () => {
    // TODO: API 호출 구현
    setIsCreateModalOpen(false);
  };

  const handleCreateVersionSubmit = () => {
    // TODO: API 호출 구현
    setIsVersionModalOpen(false);
  };

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 3 }}>
        <Typography variant="h4">작업정의 목록</Typography>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={handleCreateTask}
        >
          새 작업정의
        </Button>
      </Box>

      <TaskDefinitionSearch
        searchParams={searchParams}
        onSearchParamsChange={setSearchParams}
        onSearch={handleSearch}
      />

      <TaskDefinitionTable
        taskDefinitions={taskDefinitions}
        onVersionCreate={handleCreateVersion}
      />

      <CreateTaskDefinitionModal
        open={isCreateModalOpen}
        onClose={() => setIsCreateModalOpen(false)}
        onSubmit={handleCreateTaskSubmit}
      />

      <CreateVersionModal
        open={isVersionModalOpen}
        onClose={() => setIsVersionModalOpen(false)}
        onSubmit={handleCreateVersionSubmit}
        taskDefinition={selectedTask}
      />
    </Box>
  );
};

export default TaskDefinitionList; 