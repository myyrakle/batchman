import React, { useState, useEffect } from 'react';
import {
    Box,
    Button,
    Typography,
    Alert,
    Snackbar,
    Pagination,
    CircularProgress,
    TextField,
    FormControlLabel,
    Switch,
} from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import { CreateTaskDefinitionFormData } from '../types/taskDefinition';
import TaskDefinitionTable from '../components/TaskDefinitionTable';
import CreateTaskDefinitionModal from '../components/CreateTaskDefinitionModal';
import CreateVersionModal from '../components/CreateVersionModal';
import TaskDefinitionDetailModal from '../components/TaskDefinitionDetailModal';
import DeleteConfirmationModal from '../components/DeleteConfirmationModal';
import {
    createTaskDefinition,
    ErrorResponse,
    listTaskDefinitions,
    ListTaskDefinitionsParams,
    TaskDefinition,
    deleteTaskDefinition,
} from '../api';
import { useSearchParams } from 'react-router-dom';
import SearchIcon from '@mui/icons-material/Search';

const TaskDefinitionList: React.FC = () => {
    const [searchParams, setSearchParams] = useSearchParams();
    const [taskDefinitions, setTaskDefinitions] = useState<TaskDefinition[]>(
        []
    );
    const [total, setTotal] = useState(0);
    const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
    const [isVersionModalOpen, setIsVersionModalOpen] = useState(false);
    const [isDetailModalOpen, setIsDetailModalOpen] = useState(false);
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
    const [selectedTask, setSelectedTask] = useState<TaskDefinition | null>(
        null
    );
    const [error, setError] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState(false);
    const [searchText, setSearchText] = useState(
        searchParams.get('contains_name') || ''
    );
    const [showLatestOnly, setShowLatestOnly] = useState(false);

    const currentPage = Number(searchParams.get('page_number')) || 1;
    const currentPageSize = Number(searchParams.get('page_size')) || 10;

    const fetchTaskDefinitions = async () => {
        try {
            setIsLoading(true);
            const params: ListTaskDefinitionsParams = {
                page_number: currentPage,
                page_size: currentPageSize,
                name: searchParams.get('name') || undefined,
                task_definition_id: searchParams.get('task_definition_id')
                    ? Number(searchParams.get('task_definition_id'))
                    : undefined,
                contains_name: searchParams.get('contains_name') || undefined,
                is_latest_only: searchParams.get('is_latest_only') === 'true',
            };
            console.log('API params:', params); // 디버깅용
            const result = await listTaskDefinitions(params);

            if (result.response instanceof ErrorResponse) {
                setError('작업정의 목록을 불러오는데 실패했습니다.');
                console.error(
                    'Failed to fetch task definitions:',
                    result.response.error_code,
                    result.response.message
                );
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
        const isLatestOnly = searchParams.get('is_latest_only') === 'true';
        setShowLatestOnly(isLatestOnly);
        fetchTaskDefinitions();
    }, [currentPage, currentPageSize, searchParams]);

    const handleSearch = (params: ListTaskDefinitionsParams) => {
        const newParams = new URLSearchParams(searchParams);
        newParams.set('page_number', '1');
        if (params.contains_name) {
            newParams.set('contains_name', params.contains_name);
        } else {
            newParams.delete('contains_name');
        }
        if (params.is_latest_only) {
            newParams.set('is_latest_only', 'true');
        } else {
            newParams.delete('is_latest_only');
        }
        setSearchParams(newParams);
    };

    const handleCreateTask = () => {
        setSelectedTask(null);
        setIsCreateModalOpen(true);
    };

    const handleCreateVersion = (task: TaskDefinition) => {
        setSelectedTask(task);
        setIsCreateModalOpen(true);
    };

    const handleCreateTaskSubmit = async (
        data: CreateTaskDefinitionFormData
    ): Promise<number | void> => {
        try {
            setIsLoading(true);

            // 메모리 단위 변환: GB를 MB로 변환
            const memoryLimitInMB =
                data.resources.memory.unit === 'g'
                    ? data.resources.memory.value * 1024
                    : data.resources.memory.value;

            console.log('Memory conversion:', {
                originalValue: data.resources.memory.value,
                unit: data.resources.memory.unit,
                convertedToMB: memoryLimitInMB,
            });

            const result = await createTaskDefinition({
                name: data.name,
                description: data.description,
                image: data.image,
                command: data.command,
                env: JSON.stringify(data.env),
                memory_limit: memoryLimitInMB,
                cpu_limit: data.resources.cpu,
                args: undefined,
            });

            if (result.response instanceof ErrorResponse) {
                setError('작업정의 생성에 실패했습니다.');
                console.error(
                    'Failed to create task definition:',
                    result.response.error_code,
                    result.response.message
                );
                return;
            }

            const task_definition_id = result.response?.task_definition_id;

            setIsCreateModalOpen(false);
            fetchTaskDefinitions(); // 목록 새로고침

            return task_definition_id; // 생성된 작업정의 ID 반환
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

    const handlePageChange = (
        _event: React.ChangeEvent<unknown>,
        value: number
    ) => {
        const newParams = new URLSearchParams(searchParams);
        newParams.set('page_number', String(value));
        setSearchParams(newParams);
    };

    const handleRowClick = (task: TaskDefinition) => {
        setSelectedTask(task);
        setIsDetailModalOpen(true);
    };

    const handleDelete = (task: TaskDefinition) => {
        setSelectedTask(task);
        setIsDeleteModalOpen(true);
    };

    const handleDeleteConfirm = async () => {
        if (!selectedTask) return;

        try {
            setIsLoading(true);
            const result = await deleteTaskDefinition(selectedTask.id);

            if (result.response instanceof ErrorResponse) {
                setError('작업정의 삭제에 실패했습니다.');
                console.error(
                    'Failed to delete task definition:',
                    result.response.error_code,
                    result.response.message
                );
            } else {
                setIsDeleteModalOpen(false);
                fetchTaskDefinitions(); // 목록 새로고침
            }
        } catch (err) {
            setError('작업정의 삭제에 실패했습니다.');
            console.error('Failed to delete task definition:', err);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <Box sx={{ p: 3 }}>
            <Box
                sx={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    mb: 3,
                }}
            >
                <Typography variant="h5" component="h1">
                    작업정의 목록
                </Typography>
                <Button
                    variant="contained"
                    startIcon={<AddIcon />}
                    onClick={handleCreateTask}
                    disabled={isLoading}
                >
                    새 작업정의 생성
                </Button>
            </Box>

            <Box sx={{ display: 'flex', gap: 2, mb: 3, alignItems: 'center' }}>
                <TextField
                    label="검색"
                    value={searchText}
                    onChange={e => setSearchText(e.target.value)}
                    sx={{ width: '300px' }}
                />
                <Button
                    variant="contained"
                    startIcon={<SearchIcon />}
                    onClick={() =>
                        handleSearch({
                            page_number: 1,
                            page_size: currentPageSize,
                            contains_name: searchText,
                            is_latest_only: showLatestOnly,
                        })
                    }
                >
                    검색
                </Button>
                <FormControlLabel
                    control={
                        <Switch
                            checked={showLatestOnly}
                            onChange={e => {
                                const newValue = e.target.checked;
                                setShowLatestOnly(newValue);
                                handleSearch({
                                    page_number: 1,
                                    page_size: currentPageSize,
                                    contains_name: searchText,
                                    is_latest_only: newValue,
                                });
                            }}
                        />
                    }
                    label="최신 버전만 보기"
                />
            </Box>

            <Box sx={{ position: 'relative', minHeight: '400px' }}>
                <TaskDefinitionTable
                    taskDefinitions={taskDefinitions}
                    onRowClick={handleRowClick}
                    onDelete={handleDelete}
                    onVersionCreate={handleCreateVersion}
                    isLoading={isLoading}
                />
            </Box>

            <Box sx={{ display: 'flex', justifyContent: 'center', mt: 2 }}>
                <Pagination
                    count={Math.ceil(total / currentPageSize)}
                    page={currentPage}
                    onChange={handlePageChange}
                    color="primary"
                />
            </Box>

            <CreateTaskDefinitionModal
                open={isCreateModalOpen}
                onClose={() => {
                    setIsCreateModalOpen(false);
                    setSelectedTask(null);
                }}
                onSubmit={handleCreateTaskSubmit}
                baseTaskDefinition={selectedTask || undefined}
                isVersion={!!selectedTask}
            />

            <CreateVersionModal
                open={isVersionModalOpen}
                onClose={() => setIsVersionModalOpen(false)}
                onSubmit={handleCreateVersionSubmit}
                taskDefinition={selectedTask}
            />

            <TaskDefinitionDetailModal
                open={isDetailModalOpen}
                onClose={() => setIsDetailModalOpen(false)}
                taskDefinition={selectedTask}
                onCreateVersion={handleCreateVersion}
            />

            <DeleteConfirmationModal
                open={isDeleteModalOpen}
                onClose={() => setIsDeleteModalOpen(false)}
                onConfirm={handleDeleteConfirm}
                title="작업정의 삭제"
                message={`정말로 "${selectedTask?.name}" 작업정의를 삭제하시겠습니까?`}
            />

            <Snackbar
                open={!!error}
                autoHideDuration={6000}
                onClose={handleCloseError}
                anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
            >
                <Alert
                    onClose={handleCloseError}
                    severity="error"
                    sx={{ width: '100%' }}
                >
                    {error}
                </Alert>
            </Snackbar>
        </Box>
    );
};

export default TaskDefinitionList;
