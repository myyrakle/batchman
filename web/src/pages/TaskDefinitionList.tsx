import React, { useState, useEffect } from 'react';
import { Box, Button, Typography, Alert, Snackbar, Pagination } from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import { CreateTaskDefinitionFormData } from '../types/taskDefinition';
import TaskDefinitionTable from '../components/TaskDefinitionTable';
import TaskDefinitionSearch from '../components/TaskDefinitionSearch';
import CreateTaskDefinitionModal from '../components/CreateTaskDefinitionModal';
import CreateVersionModal from '../components/CreateVersionModal';
import { createTaskDefinition, ErrorResponse, listTaskDefinitions, ListTaskDefinitionsParams, TaskDefinition } from '../api';
import { useSearchParams } from 'react-router-dom';

const TaskDefinitionList: React.FC = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [taskDefinitions, setTaskDefinitions] = useState<TaskDefinition[]>([]);
  const [total, setTotal] = useState(0);
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [isVersionModalOpen, setIsVersionModalOpen] = useState(false);
  const [selectedTask, setSelectedTask] = useState<TaskDefinition | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const currentPage = Number(searchParams.get('page_number')) || 1;
  const currentPageSize = Number(searchParams.get('page_size')) || 10;

  const fetchTaskDefinitions = async () => {
    try {
      setIsLoading(true);
      const params: ListTaskDefinitionsParams = {
        page_number: currentPage,
        page_size: currentPageSize,
        contains_name: searchParams.get('contains_name') || undefined,
        name: searchParams.get('name') || undefined,
        task_definition_id: searchParams.get('task_definition_id') ? Number(searchParams.get('task_definition_id')) : undefined,
      };
      const result = await listTaskDefinitions(params);

      if(result.response instanceof ErrorResponse) {
        setError('작업정의 목록을 불러오는데 실패했습니다.');
        console.error('Failed to fetch task definitions:', result.response.error_code, result.response.message);
      } else {
        setTaskDefinitions(result.response.task_definitions);
        setTotal(result.response.total_count);
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

  const handleSearch = (params: ListTaskDefinitionsParams) => {
    const newParams = new URLSearchParams(searchParams);
    Object.entries(params).forEach(([key, value]) => {
      if (value !== undefined) {
        newParams.set(key, String(value));
      } else {
        newParams.delete(key);
      }
    });
    setSearchParams(newParams);
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

  const handlePageChange = (_event: React.ChangeEvent<unknown>, value: number) => {
    const newParams = new URLSearchParams(searchParams);
    newParams.set('page_number', String(value));
    setSearchParams(newParams);
  };

  const handlePageSizeChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const newParams = new URLSearchParams(searchParams);
    newParams.set('page_size', event.target.value);
    newParams.set('page_number', '1'); // 페이지 크기가 변경되면 첫 페이지로 이동
    setSearchParams(newParams);
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
        searchParams={{
          page_number: currentPage,
          page_size: currentPageSize,
        }}
        onSearchParamsChange={handleSearch}
        onSearch={() => handleSearch({
          page_number: 1,
          page_size: currentPageSize,
        })}
      />

      <TaskDefinitionTable
        taskDefinitions={taskDefinitions}
        onVersionCreate={handleCreateVersion}
        isLoading={isLoading}
      />

      <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', mt: 3, gap: 2 }}>
        <select
          value={currentPageSize}
          onChange={handlePageSizeChange}
          style={{ padding: '8px', borderRadius: '4px' }}
        >
          <option value="10">10개씩 보기</option>
          <option value="20">20개씩 보기</option>
          <option value="50">50개씩 보기</option>
        </select>
        <Pagination
          count={Math.ceil(total / currentPageSize)}
          page={currentPage}
          onChange={handlePageChange}
          color="primary"
        />
      </Box>

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