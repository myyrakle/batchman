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
} from '@mui/material';
import {
    PlayArrow as PlayArrowIcon,
    Add as AddIcon,
} from '@mui/icons-material';
import { TaskDefinition } from '../api';
import JobSubmitModal from './JobSubmitModal';

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
    const [isJobSubmitModalOpen, setIsJobSubmitModalOpen] = useState(false);

    if (!taskDefinition) return null;

    const env = taskDefinition.env ? JSON.parse(taskDefinition.env) : [];

    const handleSubmitClick = () => {
        setIsJobSubmitModalOpen(true);
    };

    const handleJobSubmitModalClose = () => {
        setIsJobSubmitModalOpen(false);
    };

    const handleCreateVersion = () => {
        if (taskDefinition && onCreateVersion) {
            onCreateVersion(taskDefinition);
            onClose();
        }
    };

    return (
        <>
            <Dialog open={open} onClose={onClose} maxWidth="md" fullWidth>
                <DialogTitle>작업정의 상세 정보</DialogTitle>
                <DialogContent>
                    <Box
                        sx={{
                            display: 'flex',
                            flexDirection: 'column',
                            gap: 2,
                            mt: 2,
                        }}
                    >
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
                            value={
                                taskDefinition.command
                                    ? taskDefinition.command.replace(
                                          /\\"/g,
                                          '"'
                                      )
                                    : ''
                            }
                            fullWidth
                            InputProps={{ readOnly: true }}
                        />

                        <Box>
                            <Typography variant="subtitle1" sx={{ mb: 1 }}>
                                환경 변수
                            </Typography>
                            {env.length > 0 ? (
                                env.map(
                                    (
                                        item: { key: string; value: string },
                                        index: number
                                    ) => (
                                        <Box
                                            key={index}
                                            sx={{
                                                display: 'flex',
                                                gap: 1,
                                                mb: 1,
                                            }}
                                        >
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
                                    )
                                )
                            ) : (
                                <Typography
                                    variant="body2"
                                    color="text.secondary"
                                >
                                    환경 변수가 없습니다.
                                </Typography>
                            )}
                        </Box>

                        <Box sx={{ display: 'flex', gap: 2 }}>
                            <Box sx={{ flex: 1 }}>
                                <Box sx={{ display: 'flex', gap: 1 }}>
                                    <TextField
                                        label="메모리 제한"
                                        value={
                                            taskDefinition.memory_limit || ''
                                        }
                                        sx={{ flex: 1 }}
                                        InputProps={{ readOnly: true }}
                                    />
                                    <FormControl sx={{ minWidth: 100 }}>
                                        <InputLabel>단위</InputLabel>
                                        <Select value="m" label="단위" disabled>
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
                            value={new Date(
                                taskDefinition.created_at
                            ).toLocaleString()}
                            fullWidth
                            InputProps={{ readOnly: true }}
                        />
                    </Box>
                </DialogContent>
                <DialogActions>
                    <Button
                        onClick={handleSubmitClick}
                        startIcon={<PlayArrowIcon />}
                    >
                        작업 제출
                    </Button>
                    {onCreateVersion && (
                        <Button
                            onClick={handleCreateVersion}
                            startIcon={<AddIcon />}
                        >
                            새 버전 생성
                        </Button>
                    )}
                    <Button onClick={onClose}>닫기</Button>
                </DialogActions>
            </Dialog>

            {/* 작업 제출 모달 */}
            <JobSubmitModal
                open={isJobSubmitModalOpen}
                onClose={handleJobSubmitModalClose}
                taskDefinition={taskDefinition}
            />
        </>
    );
};

export default TaskDefinitionDetailModal;
