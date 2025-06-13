import React, { useState, useEffect } from 'react';
import { Box, Button, Typography, Alert, Snackbar } from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import { CreateTaskDefinitionFormData } from '../types/taskDefinition';
import TaskDefinitionTable from '../components/TaskDefinitionTable';
import TaskDefinitionSearch from '../components/TaskDefinitionSearch';
import CreateTaskDefinitionModal from '../components/CreateTaskDefinitionModal';
import CreateVersionModal from '../components/CreateVersionModal';
import { createTaskDefinition, ErrorResponse, listTaskDefinitions, ListTaskDefinitionsParams, TaskDefinition } from '../api';

const TaskDefinitionList: React.FC = () => {
  const [searchParams, setSearchParams] = useState<ListTaskDefinitionsParams>({
    contains_name: undefined,
    name: undefined,
    task_definition_id: undefined,
    page: 1,
    size: 10,
  });

  const [taskDefinitions, setTaskDefinitions] = useState<TaskDefinition[]>([]);
  const [total, setTotal] = useState(0);
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [isVersionModalOpen, setIsVersionModalOpen] = useState(false);
  const [selectedTask, setSelectedTask] = useState<TaskDefinition | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  console.log(total)

  const fetchTaskDefinitions = async () => {
    try {
      setIsLoading(true);
      const result = await listTaskDefinitions();

      if(result.response instanceof ErrorResponse) {
        setError('작업정의 목록을 불러오는데 실패했습니다.');
        console.error('Failed to fetch task definitions:', result.response.error_code, result.response.message);
      } else {
        setTaskDefinitions(result.response.task_definitions);
        setTotal(result.response.task_definitions?.length || 0);
      }
    } catch (err) {
      setError('작업정의 목록을 불러오는데 실패했습니다.');
      console.error('Failed to fetch task definitions:', err);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchTaskDefinitions();
  }, [searchParams]);

  const handleSearch = () => {
    setSearchParams(prev => ({ ...prev, page: 1 }));
  };

  const handleCreateTask = () => {
    setIsCreateModalOpen(true);
  };

  const handleCreateVersion = (task: TaskDefinition) => {
    setSelectedTask(task);
    setIsVersionModalOpen(true);
  };

  const handleCreateTaskSubmit = async (data: CreateTaskDefinitionFormData) => {
    try {
      setIsLoading(true);
      await createTaskDefinition({
        name: data.name,
        description: data.description,
        image: data.image,
        command: data.command,
        env: JSON.stringify(data.env),
        memory_limit: data.resources.memory.value,
        cpu_limit: data.resources.cpu,
        args: undefined,
      });
      setIsCreateModalOpen(false);
      fetchTaskDefinitions(); // 목록 새로고침
    } catch (err) {
      setError('작업정의 생성에 실패했습니다.');
      console.error('Failed to create task definition:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCreateVersionSubmit = () => {
    // TODO: API 호출 구현
    setIsVersionModalOpen(false);
  };

  const handleCloseError = () => {
    setError(null);
  };

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 3 }}>
        <Typography variant="h4">작업정의 목록</Typography>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={handleCreateTask}
          disabled={isLoading}
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
        isLoading={isLoading}
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

      <Snackbar
        open={!!error}
        autoHideDuration={6000}
        onClose={handleCloseError}
        anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
      >
        <Alert onClose={handleCloseError} severity="error" sx={{ width: '100%' }}>
          {error}
        </Alert>
      </Snackbar>
    </Box>
  );
};

export default TaskDefinitionList; 