import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  Box,
  Typography,
  Paper,
  Grid,
  Button,
  Chip,
  Divider,
  IconButton,
  Menu,
  MenuItem,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
} from '@mui/material';
import {
  ArrowBack as ArrowBackIcon,
  MoreVert as MoreVertIcon,
} from '@mui/icons-material';
import Layout from '../components/Layout';
import { deleteTaskDefinition, ErrorResponse, listTaskDefinitions, TaskDefinition } from '../api';

const TaskDefinitionDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [taskDefinition, setTaskDefinition] = useState<TaskDefinition | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);

  useEffect(() => {
    const fetchTaskDefinition = async () => {
      if (!id) return;
      try {
        setIsLoading(true);
        const result = await listTaskDefinitions({
            task_definition_id: parseInt(id),
            page_number: 1,
            page_size: 1,
        });
        if (result.response instanceof ErrorResponse) {
            console.error('Failed to fetch task definition:', result.response.error_code, result.response.message);
        } else {
            let response = result.response;

            if(response.task_definitions.length > 0) {
               let taskDefinition = response.task_definitions[0];
               setTaskDefinition(taskDefinition);
            }
        }
      } catch (error) {
        console.error('Failed to fetch task definition:', error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchTaskDefinition();
  }, [id]);

  const handleMenuClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleMenuClose = () => {
    setAnchorEl(null);
  };

  const handleDeleteClick = () => {
    handleMenuClose();
    setIsDeleteDialogOpen(true);
  };

  const handleDeleteConfirm = async () => {
    if (!taskDefinition) return;

    try {
      setIsLoading(true);
      await deleteTaskDefinition(taskDefinition.id);
      navigate('/task-definitions');
    } catch (error) {
      console.error('Failed to delete task definition:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCreateVersion = () => {
    navigate(`/task-definitions/new?base=${taskDefinition?.id}`);
  };

  if (!taskDefinition) {
    return (
      <Layout isLoading={isLoading}>
        <Box sx={{ p: 3 }}>
          <Typography>작업 정의를 찾을 수 없습니다.</Typography>
        </Box>
      </Layout>
    );
  }

  return (
    <Layout isLoading={isLoading}>
      <Box sx={{ p: 3 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', mb: 3 }}>
          <IconButton
            onClick={() => {
              if (navigate) {
                navigate('/task-definitions');
              }
            }}
            sx={{ mr: 2 }}
          >
            <ArrowBackIcon />
          </IconButton>
          <Typography variant="h4" sx={{ flexGrow: 1 }}>
            {taskDefinition.name}
          </Typography>
          <IconButton onClick={handleMenuClick}>
            <MoreVertIcon />
          </IconButton>
        </Box>

        <Paper sx={{ p: 3, mb: 3 }}>
          <Box sx={{ display: 'grid', gap: 3 }}>
            <Box>
              <Typography variant="h6" gutterBottom>
                기본 정보
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Box sx={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: 2 }}>
                <Box>
                  <Typography variant="subtitle2" color="text.secondary">
                    버전
                  </Typography>
                  <Typography variant="body1">
                    {taskDefinition.version}
                  </Typography>
                </Box>
                <Box>
                  <Typography variant="subtitle2" color="text.secondary">
                    설명
                  </Typography>
                  <Typography variant="body1">
                    {taskDefinition.description || '-'}
                  </Typography>
                </Box>
              </Box>
            </Box>

            <Box>
              <Typography variant="h6" gutterBottom>
                실행 정보
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Box sx={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: 2 }}>
                <Box>
                  <Typography variant="subtitle2" color="text.secondary">
                    이미지
                  </Typography>
                  <Typography variant="body1">
                    {taskDefinition.image}
                  </Typography>
                </Box>
                <Box>
                  <Typography variant="subtitle2" color="text.secondary">
                    명령어
                  </Typography>
                  <Typography variant="body1">
                    {taskDefinition.command}
                  </Typography>
                </Box>
              </Box>
            </Box>

            

            <Box>
              <Typography variant="h6" gutterBottom>
                환경 변수
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Box>
                {taskDefinition.env && (JSON.parse(taskDefinition.env) as Array<{key:string;value:string}> ).map(e => (
                  <Chip
                    key={e.key}
                    label={`${e.key}=${e.value}`}
                    sx={{ mr: 1, mb: 1 }}
                    onClick={() => {}}
                  />
                ))}
                {(!taskDefinition.env || Object.keys(taskDefinition.env).length === 0) && (
                  <Typography variant="body2" color="text.secondary">
                    환경 변수가 없습니다.
                  </Typography>
                )}
              </Box>
            </Box>

            <Box>
              <Typography variant="h6" gutterBottom>
                리소스
              </Typography>
              <Divider sx={{ mb: 2 }} />
              <Box sx={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: 2 }}>
                <Box>
                  <Typography variant="subtitle2" color="text.secondary">
                    메모리 제한
                  </Typography>
                  <Typography variant="body1">
                    {taskDefinition.memory_limit || '-'}
                  </Typography>
                </Box>
                <Box>
                  <Typography variant="subtitle2" color="text.secondary">
                    CPU 제한
                  </Typography>
                  <Typography variant="body1">
                    {taskDefinition.cpu_limit || '-'}
                  </Typography>
                </Box>
              </Box>
            </Box>
          </Box>
        </Paper>

        <Menu
          anchorEl={anchorEl}
          open={Boolean(anchorEl)}
          onClose={handleMenuClose}
        >
          <MenuItem onClick={handleCreateVersion}>
            새 버전 만들기
          </MenuItem>
          <MenuItem onClick={handleDeleteClick}>
            삭제
          </MenuItem>
        </Menu>

        <Dialog
          open={isDeleteDialogOpen}
          onClose={() => setIsDeleteDialogOpen(false)}
        >
          <DialogTitle>작업 정의 삭제</DialogTitle>
          <DialogContent>
            <Typography>
              정말로 이 작업 정의를 삭제하시겠습니까?
            </Typography>
          </DialogContent>
          <DialogActions>
            <Button onClick={() => setIsDeleteDialogOpen(false)}>취소</Button>
            <Button onClick={handleDeleteConfirm} color="error">
              삭제
            </Button>
          </DialogActions>
        </Dialog>
      </Box>
    </Layout>
  );
};

export default TaskDefinitionDetail; 