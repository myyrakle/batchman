import React, { useState, useEffect } from 'react';
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
    Alert,
    Switch,
    FormControlLabel,
    Autocomplete,
} from '@mui/material';
import {
    CreateScheduleRequest,
    TaskDefinition,
    listTaskDefinitions,
    ListTaskDefinitionsResponse,
    ErrorResponse,
} from '../api';

interface ScheduleCreateModalProps {
    open: boolean;
    onClose: () => void;
    onSubmit: (data: CreateScheduleRequest) => Promise<void>;
}

const ScheduleCreateModal: React.FC<ScheduleCreateModalProps> = ({
    open,
    onClose,
    onSubmit,
}) => {
    const [formData, setFormData] = useState<CreateScheduleRequest>({
        name: '',
        job_name: '',
        cron_expression: '',
        task_definition_id: 0,
        command: '',
        timezone: 'Asia/Seoul',
        timezone_offset: 540, // 9 hours * 60 minutes
        enabled: true,
    });
    
    const [taskDefinitions, setTaskDefinitions] = useState<TaskDefinition[]>([]);
    const [error, setError] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState(false);

    // 미리 정의된 cron 표현식들
    const cronPresets = [
        { label: '매분', value: '* * * * *' },
        { label: '매시 정각', value: '0 * * * *' },
        { label: '매일 자정', value: '0 0 * * *' },
        { label: '매일 오전 9시', value: '0 9 * * *' },
        { label: '매주 월요일 자정', value: '0 0 * * 1' },
        { label: '매월 1일 자정', value: '0 0 1 * *' },
        { label: '직접 입력', value: '' },
    ];

    const timezones = [
        { label: 'Asia/Seoul (UTC+9)', value: 'Asia/Seoul', offset: 540 },
        { label: 'UTC (UTC+0)', value: 'UTC', offset: 0 },
        { label: 'America/New_York (UTC-5)', value: 'America/New_York', offset: -300 },
        { label: 'Europe/London (UTC+0)', value: 'Europe/London', offset: 0 },
    ];

    useEffect(() => {
        if (open) {
            fetchTaskDefinitions();
        }
    }, [open]);

    const fetchTaskDefinitions = async () => {
        try {
            const result = await listTaskDefinitions({
                page_number: 1,
                page_size: 100,
                is_latest_only: true,
            });

            if (result.status_code === 200) {
                const response = result.response as ListTaskDefinitionsResponse;
                setTaskDefinitions(response.task_definitions);
            }
        } catch (err) {
            console.error('Failed to fetch task definitions:', err);
        }
    };

    const handleInputChange = (field: keyof CreateScheduleRequest, value: any) => {
        setFormData(prev => ({
            ...prev,
            [field]: value,
        }));
    };

    const handleTimezoneChange = (timezone: string) => {
        const selectedTimezone = timezones.find(tz => tz.value === timezone);
        if (selectedTimezone) {
            setFormData(prev => ({
                ...prev,
                timezone: selectedTimezone.value,
                timezone_offset: selectedTimezone.offset,
            }));
        }
    };

    const handleSubmit = async () => {
        try {
            setError(null);
            setIsLoading(true);

            // 유효성 검사
            if (!formData.name.trim()) {
                setError('스케줄명을 입력해주세요.');
                return;
            }
            if (!formData.job_name.trim()) {
                setError('작업명을 입력해주세요.');
                return;
            }
            if (!formData.cron_expression.trim()) {
                setError('Cron 표현식을 입력해주세요.');
                return;
            }
            if (!formData.task_definition_id) {
                setError('태스크 정의를 선택해주세요.');
                return;
            }

            await onSubmit(formData);
            handleClose();
        } catch (err) {
            setError('스케줄 생성 중 오류가 발생했습니다.');
        } finally {
            setIsLoading(false);
        }
    };

    const handleClose = () => {
        setFormData({
            name: '',
            job_name: '',
            cron_expression: '',
            task_definition_id: 0,
            command: '',
            timezone: 'Asia/Seoul',
            timezone_offset: 540,
            enabled: true,
        });
        setError(null);
        onClose();
    };

    const selectedTaskDefinition = taskDefinitions.find(
        td => td.id === formData.task_definition_id
    );

    return (
        <Dialog 
            open={open} 
            onClose={handleClose} 
            maxWidth="md" 
            fullWidth
            PaperProps={{
                sx: { minHeight: '600px' }
            }}
        >
            <DialogTitle>
                <Typography variant="h5">스케줄 생성</Typography>
            </DialogTitle>
            <DialogContent>
                <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 1 }}>
                    {error && (
                        <Alert severity="error" onClose={() => setError(null)}>
                            {error}
                        </Alert>
                    )}

                    {/* 기본 정보 */}
                    <Typography variant="h6" sx={{ mt: 2, mb: 1 }}>
                        기본 정보
                    </Typography>
                    
                    <TextField
                        fullWidth
                        label="스케줄명"
                        value={formData.name}
                        onChange={(e) => handleInputChange('name', e.target.value)}
                        placeholder="예: 일일 데이터 백업"
                        required
                    />

                    <TextField
                        fullWidth
                        label="작업명"
                        value={formData.job_name}
                        onChange={(e) => handleInputChange('job_name', e.target.value)}
                        placeholder="예: daily-backup-job"
                        required
                        helperText="실행될 때 생성되는 작업의 이름 패턴"
                    />

                    {/* 태스크 정의 선택 */}
                    <FormControl fullWidth required>
                        <InputLabel>태스크 정의</InputLabel>
                        <Select
                            value={formData.task_definition_id}
                            label="태스크 정의"
                            onChange={(e) => handleInputChange('task_definition_id', e.target.value)}
                        >
                            {taskDefinitions.map((td) => (
                                <MenuItem key={td.id} value={td.id}>
                                    {td.name} (v{td.version}) - {td.image}
                                </MenuItem>
                            ))}
                        </Select>
                    </FormControl>

                    {/* 선택된 태스크 정의 정보 표시 */}
                    {selectedTaskDefinition && (
                        <Box sx={{ p: 2, bgcolor: 'action.hover', borderRadius: 1 }}>
                            <Typography variant="subtitle2">선택된 태스크 정의:</Typography>
                            <Typography variant="body2">
                                이미지: {selectedTaskDefinition.image}
                            </Typography>
                            {selectedTaskDefinition.command && (
                                <Typography variant="body2">
                                    기본 명령어: {selectedTaskDefinition.command}
                                </Typography>
                            )}
                        </Box>
                    )}

                    {/* 추가 명령어 */}
                    <TextField
                        fullWidth
                        label="추가 명령어 (선택사항)"
                        value={formData.command}
                        onChange={(e) => handleInputChange('command', e.target.value)}
                        placeholder="태스크 정의의 기본 명령어를 덮어쓸 경우에만 입력"
                        helperText="비워두면 태스크 정의의 기본 명령어가 사용됩니다"
                    />

                    {/* 스케줄링 설정 */}
                    <Typography variant="h6" sx={{ mt: 2, mb: 1 }}>
                        스케줄링 설정
                    </Typography>

                    {/* Cron 표현식 프리셋 */}
                    <FormControl fullWidth>
                        <InputLabel>Cron 표현식 프리셋</InputLabel>
                        <Select
                            value=""
                            label="Cron 표현식 프리셋"
                            onChange={(e) => {
                                if (e.target.value) {
                                    handleInputChange('cron_expression', e.target.value);
                                }
                            }}
                        >
                            {cronPresets.map((preset) => (
                                <MenuItem key={preset.label} value={preset.value}>
                                    {preset.label}
                                </MenuItem>
                            ))}
                        </Select>
                    </FormControl>

                    <TextField
                        fullWidth
                        label="Cron 표현식"
                        value={formData.cron_expression}
                        onChange={(e) => handleInputChange('cron_expression', e.target.value)}
                        placeholder="0 0 * * *"
                        required
                        helperText="분 시 일 월 요일 형식 (예: 0 0 * * * = 매일 자정)"
                    />

                    {/* 시간대 설정 */}
                    <FormControl fullWidth>
                        <InputLabel>시간대</InputLabel>
                        <Select
                            value={formData.timezone}
                            label="시간대"
                            onChange={(e) => handleTimezoneChange(e.target.value)}
                        >
                            {timezones.map((tz) => (
                                <MenuItem key={tz.value} value={tz.value}>
                                    {tz.label}
                                </MenuItem>
                            ))}
                        </Select>
                    </FormControl>

                    {/* 활성화 상태 */}
                    <FormControlLabel
                        control={
                            <Switch
                                checked={formData.enabled}
                                onChange={(e) => handleInputChange('enabled', e.target.checked)}
                            />
                        }
                        label="스케줄 활성화"
                    />
                </Box>
            </DialogContent>
            <DialogActions sx={{ p: 3 }}>
                <Button onClick={handleClose} disabled={isLoading}>
                    취소
                </Button>
                <Button 
                    onClick={handleSubmit} 
                    variant="contained" 
                    disabled={isLoading}
                >
                    {isLoading ? '생성 중...' : '생성'}
                </Button>
            </DialogActions>
        </Dialog>
    );
};

export default ScheduleCreateModal;
