import React, { useState } from 'react';
import {
    Dialog,
    DialogTitle,
    DialogContent,
    DialogActions,
    Button,
    TextField,
    Snackbar,
    Alert,
} from '@mui/material';
import { PlayArrow as PlayArrowIcon } from '@mui/icons-material';
import { TaskDefinition, ErrorResponse, submitJob } from '../api';
import { useNavigate } from 'react-router-dom';

interface JobSubmitModalProps {
    open: boolean;
    onClose: () => void;
    taskDefinition: TaskDefinition | null;
}

const JobSubmitModal: React.FC<JobSubmitModalProps> = ({
    open,
    onClose,
    taskDefinition,
}) => {
    const navigate = useNavigate();
    const [jobName, setJobName] = useState('');
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [successMessage, setSuccessMessage] = useState<string | null>(null);
    const [errorMessage, setErrorMessage] = useState<string | null>(null);

    const handleClose = () => {
        setJobName('');
        setErrorMessage(null);
        onClose();
    };

    const handleSubmitConfirm = async () => {
        if (!taskDefinition || !jobName.trim()) {
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
                setErrorMessage(
                    `작업 제출에 실패했습니다: ${
                        result.response.message || '알 수 없는 오류'
                    }`
                );
                console.error(
                    'Failed to submit job:',
                    result.response.error_code,
                    result.response.message
                );
                return;
            }

            const jobID = result.response.job_id;

            setSuccessMessage(`작업 "${jobName}"이 성공적으로 제출되었습니다.`);
            handleClose();
            // 2초 후 작업 목록 페이지로 이동
            setTimeout(() => {
                console.log('Navigating to job detail page with ID:', jobID);
                if (jobID) {
                    navigate(`/jobs/${jobID}`);
                } else {
                    navigate('/jobs');
                }
            }, 2000);
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
            <Dialog open={open} onClose={handleClose}>
                <DialogTitle>작업 제출</DialogTitle>
                <DialogContent>
                    <TextField
                        autoFocus
                        margin="dense"
                        label="작업 이름"
                        fullWidth
                        value={jobName}
                        onChange={e => setJobName(e.target.value)}
                        required
                        placeholder="작업에 대한 고유한 이름을 입력하세요"
                    />
                </DialogContent>
                <DialogActions>
                    <Button onClick={handleClose} disabled={isSubmitting}>
                        취소
                    </Button>
                    <Button
                        onClick={handleSubmitConfirm}
                        variant="contained"
                        startIcon={<PlayArrowIcon />}
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
                <Alert
                    onClose={handleCloseSuccessAlert}
                    severity="success"
                    sx={{ width: '100%' }}
                >
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
                <Alert
                    onClose={handleCloseErrorAlert}
                    severity="error"
                    sx={{ width: '100%' }}
                >
                    {errorMessage}
                </Alert>
            </Snackbar>
        </>
    );
};

export default JobSubmitModal;
