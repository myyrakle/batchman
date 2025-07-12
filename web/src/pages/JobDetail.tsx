import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
    Box,
    Button,
    Card,
    CardContent,
    Typography,
    Chip,
    Divider,
    IconButton,
    Alert,
    CircularProgress,
    Table,
    TableBody,
    TableCell,
    TableRow,
    Link,
} from '@mui/material';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';
import RefreshIcon from '@mui/icons-material/Refresh';
import StopIcon from '@mui/icons-material/Stop';
import VisibilityIcon from '@mui/icons-material/Visibility';
import {
    Job,
    JobStatus,
    TaskDefinition,
    listJobs,
    listTaskDefinitions,
    stopJob,
    ErrorResponse,
} from '../api';
import { formatDate } from '../utils';

const JobDetail: React.FC = () => {
    const { jobId } = useParams<{ jobId: string }>();
    const navigate = useNavigate();
    const [job, setJob] = useState<Job | null>(null);
    const [taskDefinition, setTaskDefinition] = useState<TaskDefinition | null>(
        null
    );
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [isStoppingJob, setIsStoppingJob] = useState(false);

    const fetchJobDetail = async () => {
        if (!jobId) return;

        try {
            setIsLoading(true);
            setError(null);

            // 특정 작업 조회
            const jobResult = await listJobs({
                page_number: 1,
                page_size: 1,
                job_id: parseInt(jobId),
            });

            if (jobResult.response instanceof ErrorResponse) {
                setError('작업 정보를 불러오는데 실패했습니다.');
                return;
            }

            if (jobResult.response.jobs.length === 0) {
                setError('작업을 찾을 수 없습니다.');
                return;
            }

            const jobData = jobResult.response.jobs[0];
            setJob(jobData);

            // 작업 정의 정보 조회
            const taskDefResult = await listTaskDefinitions({
                page_number: 1,
                page_size: 1,
                task_definition_id: jobData.task_definition_id,
            });

            if (taskDefResult.response instanceof ErrorResponse) {
                console.error(
                    'Failed to fetch task definition:',
                    taskDefResult.response
                );
            } else if (taskDefResult.response.task_definitions.length > 0) {
                setTaskDefinition(taskDefResult.response.task_definitions[0]);
            }
        } catch (err) {
            console.error('Failed to fetch job detail:', err);
            setError('작업 정보를 불러오는데 실패했습니다.');
        } finally {
            setIsLoading(false);
        }
    };

    const handleStopJob = async () => {
        if (!job) return;

        try {
            setIsStoppingJob(true);
            const result = await stopJob({ job_id: job.id });

            if (result.response instanceof ErrorResponse) {
                setError('작업 중지에 실패했습니다.');
            } else {
                // 작업 정보 다시 조회
                fetchJobDetail();
            }
        } catch (err) {
            console.error('Failed to stop job:', err);
            setError('작업 중지에 실패했습니다.');
        } finally {
            setIsStoppingJob(false);
        }
    };

    useEffect(() => {
        fetchJobDetail();
    }, [jobId]);

    const getStatusColor = (status: JobStatus) => {
        switch (status) {
            case 'Pending':
                return 'warning';
            case 'Starting':
                return 'info';
            case 'Running':
                return 'primary';
            case 'Finished':
                return 'success';
            case 'Failed':
                return 'error';
            default:
                return 'default';
        }
    };

    const getStatusLabel = (status: JobStatus) => {
        switch (status) {
            case 'Pending':
                return '대기중';
            case 'Starting':
                return '시작중';
            case 'Running':
                return '실행중';
            case 'Finished':
                return '완료';
            case 'Failed':
                return '실패';
            default:
                return status;
        }
    };

    const canStopJob =
        job &&
        (job.status === 'Pending' ||
            job.status === 'Starting' ||
            job.status === 'Running');

    if (isLoading) {
        return (
            <Box
                sx={{
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    height: '400px',
                }}
            >
                <CircularProgress />
            </Box>
        );
    }

    if (error) {
        return (
            <Box sx={{ p: 3 }}>
                <Alert severity="error" sx={{ mb: 2 }}>
                    {error}
                </Alert>
                <Button
                    variant="outlined"
                    startIcon={<ArrowBackIcon />}
                    onClick={() => navigate('/jobs')}
                >
                    작업 목록으로 돌아가기
                </Button>
            </Box>
        );
    }

    if (!job) {
        return (
            <Box sx={{ p: 3 }}>
                <Alert severity="warning" sx={{ mb: 2 }}>
                    작업을 찾을 수 없습니다.
                </Alert>
                <Button
                    variant="outlined"
                    startIcon={<ArrowBackIcon />}
                    onClick={() => navigate('/jobs')}
                >
                    작업 목록으로 돌아가기
                </Button>
            </Box>
        );
    }

    return (
        <Box sx={{ p: 3 }}>
            {/* 헤더 */}
            <Box sx={{ display: 'flex', alignItems: 'center', mb: 3 }}>
                <IconButton onClick={() => navigate('/jobs')} sx={{ mr: 1 }}>
                    <ArrowBackIcon />
                </IconButton>
                <Typography variant="h5" component="h1" sx={{ flexGrow: 1 }}>
                    작업 상세 - {job.name}
                </Typography>
                <Box sx={{ display: 'flex', gap: 1 }}>
                    <Button
                        variant="outlined"
                        startIcon={<VisibilityIcon />}
                        onClick={() => navigate(`/jobs/${job.id}/logs`)}
                    >
                        로그 보기
                    </Button>
                    <Button
                        variant="outlined"
                        startIcon={<RefreshIcon />}
                        onClick={fetchJobDetail}
                        disabled={isLoading}
                    >
                        새로고침
                    </Button>
                    {canStopJob && (
                        <Button
                            variant="contained"
                            color="error"
                            startIcon={<StopIcon />}
                            onClick={handleStopJob}
                            disabled={isStoppingJob}
                        >
                            {isStoppingJob ? '중지 중...' : '작업 중지'}
                        </Button>
                    )}
                </Box>
            </Box>

            <Box
                sx={{
                    display: 'grid',
                    gridTemplateColumns: { xs: '1fr', md: '1fr 1fr' },
                    gap: 3,
                }}
            >
                {/* 작업 정보 */}
                <Box>
                    <Card>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                작업 정보
                            </Typography>
                            <Divider sx={{ mb: 2 }} />
                            <Table size="small">
                                <TableBody>
                                    <TableRow>
                                        <TableCell
                                            component="th"
                                            scope="row"
                                            sx={{
                                                fontWeight: 'bold',
                                                width: '40%',
                                            }}
                                        >
                                            작업 ID
                                        </TableCell>
                                        <TableCell>{job.id}</TableCell>
                                    </TableRow>
                                    <TableRow>
                                        <TableCell
                                            component="th"
                                            scope="row"
                                            sx={{ fontWeight: 'bold' }}
                                        >
                                            작업 이름
                                        </TableCell>
                                        <TableCell>{job.name}</TableCell>
                                    </TableRow>
                                    <TableRow>
                                        <TableCell
                                            component="th"
                                            scope="row"
                                            sx={{ fontWeight: 'bold' }}
                                        >
                                            상태
                                        </TableCell>
                                        <TableCell>
                                            <Chip
                                                label={getStatusLabel(
                                                    job.status
                                                )}
                                                color={getStatusColor(
                                                    job.status
                                                )}
                                                size="small"
                                                onClick={() => {}}
                                            />
                                        </TableCell>
                                    </TableRow>
                                    <TableRow>
                                        <TableCell
                                            component="th"
                                            scope="row"
                                            sx={{ fontWeight: 'bold' }}
                                        >
                                            종료 코드
                                        </TableCell>
                                        <TableCell>
                                            {job.exit_code !== null
                                                ? job.exit_code
                                                : '-'}
                                        </TableCell>
                                    </TableRow>
                                    {job.error_message && (
                                        <TableRow>
                                            <TableCell
                                                component="th"
                                                scope="row"
                                                sx={{ fontWeight: 'bold' }}
                                            >
                                                오류 메시지
                                            </TableCell>
                                            <TableCell
                                                sx={{ color: 'error.main' }}
                                            >
                                                {job.error_message}
                                            </TableCell>
                                        </TableRow>
                                    )}
                                    <TableRow>
                                        <TableCell
                                            component="th"
                                            scope="row"
                                            sx={{ fontWeight: 'bold' }}
                                        >
                                            작업 정의
                                        </TableCell>
                                        <TableCell>
                                            <Link
                                                component="button"
                                                variant="body2"
                                                onClick={e => {
                                                    e.stopPropagation(); // 상위 TableRow의 onClick 이벤트 전파 방지
                                                    navigate(
                                                        `/task-definitions/${job.task_definition_id}`
                                                    ); // 작업 정의 상세 페이지로 이동
                                                }}
                                                sx={{
                                                    fontWeight: 'medium',
                                                    textAlign: 'left',
                                                    cursor: 'pointer',
                                                    '&:hover': {
                                                        textDecoration:
                                                            'underline',
                                                    },
                                                }}
                                            >
                                                {job.task_definition_name ??
                                                    'DELETED'}
                                                :{taskDefinition?.version ?? ''}
                                            </Link>
                                        </TableCell>
                                    </TableRow>
                                    <TableRow>
                                        <TableCell
                                            component="th"
                                            scope="row"
                                            sx={{ fontWeight: 'bold' }}
                                        >
                                            제출 시간
                                        </TableCell>
                                        <TableCell>
                                            {formatDate(job.submited_at)}
                                        </TableCell>
                                    </TableRow>
                                    <TableRow>
                                        <TableCell
                                            component="th"
                                            scope="row"
                                            sx={{ fontWeight: 'bold' }}
                                        >
                                            시작 시간
                                        </TableCell>
                                        <TableCell>
                                            {formatDate(job.started_at)}
                                        </TableCell>
                                    </TableRow>
                                    <TableRow>
                                        <TableCell
                                            component="th"
                                            scope="row"
                                            sx={{ fontWeight: 'bold' }}
                                        >
                                            완료 시간
                                        </TableCell>
                                        <TableCell>
                                            {formatDate(job.finished_at)}
                                        </TableCell>
                                    </TableRow>
                                    <TableRow>
                                        <TableCell
                                            component="th"
                                            scope="row"
                                            sx={{ fontWeight: 'bold' }}
                                        >
                                            컨테이너 타입
                                        </TableCell>
                                        <TableCell>
                                            {job.container_type}
                                        </TableCell>
                                    </TableRow>
                                    <TableRow>
                                        <TableCell
                                            component="th"
                                            scope="row"
                                            sx={{ fontWeight: 'bold' }}
                                        >
                                            컨테이너 ID
                                        </TableCell>
                                        <TableCell>
                                            {job.container_id || '-'}
                                        </TableCell>
                                    </TableRow>
                                </TableBody>
                            </Table>
                        </CardContent>
                    </Card>
                </Box>
            </Box>
        </Box>
    );
};

export default JobDetail;
