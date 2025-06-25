import React, { useState } from 'react';
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
  TextField,
  Box,
  Typography,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Snackbar,
  Alert,
} from '@mui/material';
import { PlayArrow as PlayArrowIcon, Add as AddIcon } from '@mui/icons-material';
import { TaskDefinition, ErrorResponse, submitJob } from '../api';
import { useNavigate } from 'react-router-dom';

interface TaskDefinitionDetailModalProps {
  open: boolean;
  onClose: () => void;
  taskDefinition: TaskDefinition | null;
  onCreateVersion?: (task: TaskDefinition) => void;
}

const TaskDefinitionDetailModal: React.FC<TaskDefinitionDetailModalProps> = ({
  open,
  onClose,
  taskDefinition,
  onCreateVersion,
}) => {
  const navigate = useNavigate();
  const [isSubmitModalOpen, setIsSubmitModalOpen] = useState(false);
  const [jobName, setJobName] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  if (!taskDefinition) return null;

  const env = taskDefinition.env ? JSON.parse(taskDefinition.env) : [];

  const handleSubmitClick = () => {
    setIsSubmitModalOpen(true);
  };

  const handleSubmitClose = () => {
    setIsSubmitModalOpen(false);
    setJobName('');
  };

  const handleCreateVersion = () => {
    if (taskDefinition && onCreateVersion) {
      onCreateVersion(taskDefinition);
      onClose();
    }
  };

  const handleSubmitConfirm = async () => {
    if (!jobName.trim()) {
      setErrorMessage('작업 이름을 입력해주세요.');
      return;
    }

    try {
      setIsSubmitting(true);
      setErrorMessage(null);
      
      const result = await submitJob({
        task_definition_id: taskDefinition.id,
        job_name: jobName.trim(),
      });

      if (result.response instanceof ErrorResponse) {
        setErrorMessage(`작업 제출에 실패했습니다: ${result.response.message || '알 수 없는 오류'}`);
        console.error('Failed to submit job:', result.response.error_code, result.response.message);
      } else {
        setSuccessMessage(`작업 "${jobName}"이 성공적으로 제출되었습니다.`);
        handleSubmitClose();
        // 2초 후 작업 목록 페이지로 이동
        setTimeout(() => {
          navigate('/jobs');
        }, 2000);
      }
    } catch (error) {
      setErrorMessage('작업 제출 중 오류가 발생했습니다.');
      console.error('Failed to submit job:', error);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleCloseSuccessAlert = () => {
    setSuccessMessage(null);
  };

  const handleCloseErrorAlert = () => {
    setErrorMessage(null);
  };

  return (
    <>
      <Dialog open={open} onClose={onClose} maxWidth="md" fullWidth>
        <DialogTitle>작업정의 상세 정보</DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, mt: 2 }}>
            <TextField
              label="이름"
              value={taskDefinition.name}
              fullWidth
              InputProps={{ readOnly: true }}
            />

            <TextField
              label="설명"
              value={taskDefinition.description}
              fullWidth
              InputProps={{ readOnly: true }}
            />

            <TextField
              label="이미지"
              value={taskDefinition.image}
              fullWidth
              InputProps={{ readOnly: true }}
            />

            <TextField
              label="명령어"
              value={taskDefinition.command ? taskDefinition.command.replace(/\\"/g, '"') : ''}
              fullWidth
              InputProps={{ readOnly: true }}
            />

            <Box>
              <Typography variant="subtitle1" sx={{ mb: 1 }}>환경 변수</Typography>
              {env.length > 0 ? (
                env.map((item: { key: string; value: string }, index: number) => (
                  <Box key={index} sx={{ display: 'flex', gap: 1, mb: 1 }}>
                    <TextField
                      label="키"
                      value={item.key}
                      size="small"
                      sx={{ flex: 1 }}
                      InputProps={{ readOnly: true }}
                    />
                    <TextField
                      label="값"
                      value={item.value}
                      size="small"
                      sx={{ flex: 1 }}
                      InputProps={{ readOnly: true }}
                    />
                  </Box>
                ))
              ) : (
                <Typography variant="body2" color="text.secondary">
                  환경 변수가 없습니다.
                </Typography>
              )}
            </Box>

            <Box sx={{ display: 'flex', gap: 2 }}>
              <Box sx={{ flex: 1 }}>
                <Box sx={{ display: 'flex', gap: 1 }}>
                  <TextField
                    label="메모리 제한"
                    value={taskDefinition.memory_limit || ''}
                    sx={{ flex: 1 }}
                    InputProps={{ readOnly: true }}
                  />
                  <FormControl sx={{ minWidth: 100 }}>
                    <InputLabel>단위</InputLabel>
                    <Select
                      value="m"
                      label="단위"
                      disabled
                    >
                      <MenuItem value="m">MB</MenuItem>
                    </Select>
                  </FormControl>
                </Box>
              </Box>
              <Box sx={{ flex: 1 }}>
                <TextField
                  label="CPU 제한"
                  value={taskDefinition.cpu_limit || ''}
                  fullWidth
                  InputProps={{ readOnly: true }}
                />
              </Box>
            </Box>

            <TextField
              label="생성일"
              value={new Date(taskDefinition.created_at).toLocaleString()}
              fullWidth
              InputProps={{ readOnly: true }}
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleSubmitClick} startIcon={<PlayArrowIcon />}>
            작업 제출
          </Button>
          {onCreateVersion && (
            <Button onClick={handleCreateVersion} startIcon={<AddIcon />}>
              새 버전 생성
            </Button>
          )}
          <Button onClick={onClose}>닫기</Button>
        </DialogActions>
      </Dialog>

      <Dialog
        open={isSubmitModalOpen}
        onClose={handleSubmitClose}
      >
        <DialogTitle>작업 제출</DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            margin="dense"
            label="작업 이름"
            fullWidth
            value={jobName}
            onChange={(e) => setJobName(e.target.value)}
            required
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={handleSubmitClose} disabled={isSubmitting}>취소</Button>
          <Button
            onClick={handleSubmitConfirm}
            variant="contained"
            disabled={!jobName.trim() || isSubmitting}
          >
            {isSubmitting ? '제출 중...' : '제출'}
          </Button>
        </DialogActions>
      </Dialog>

      {/* 성공 알림 */}
      <Snackbar
        open={!!successMessage}
        autoHideDuration={6000}
        onClose={handleCloseSuccessAlert}
        anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
      >
        <Alert onClose={handleCloseSuccessAlert} severity="success" sx={{ width: '100%' }}>
          {successMessage}
        </Alert>
      </Snackbar>

      {/* 에러 알림 */}
      <Snackbar
        open={!!errorMessage}
        autoHideDuration={6000}
        onClose={handleCloseErrorAlert}
        anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
      >
        <Alert onClose={handleCloseErrorAlert} severity="error" sx={{ width: '100%' }}>
          {errorMessage}
        </Alert>
      </Snackbar>
    </>
  );
};

export default TaskDefinitionDetailModal; 