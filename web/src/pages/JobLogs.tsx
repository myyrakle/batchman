import React, { useState, useEffect, useRef, useCallback } from 'react';
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
    Paper,
    Stack,
    Switch,
    FormControlLabel,
    Tooltip,
} from '@mui/material';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';
import RefreshIcon from '@mui/icons-material/Refresh';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import PauseIcon from '@mui/icons-material/Pause';
import ClearIcon from '@mui/icons-material/Clear';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import {
    Job,
    JobLog,
    listJobs,
    listJobLogs,
    countJobLogs,
    ErrorResponse,
} from '../api';
import { formatDate } from '../utils';

const INITIAL_LOAD_COUNT = 100;
const LOAD_MORE_COUNT = 50;
const POLLING_INTERVAL = 2000; // 2초마다 새로운 로그 확인

const JobLogs: React.FC = () => {
    const { jobId } = useParams<{ jobId: string }>();
    const navigate = useNavigate();
    const [job, setJob] = useState<Job | null>(null);
    const [logs, setLogs] = useState<JobLog[]>([]);
    const [totalLogCount, setTotalLogCount] = useState(0);
    const [isLoading, setIsLoading] = useState(true);
    const [isLoadingMore, setIsLoadingMore] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [isAutoRefreshEnabled, setIsAutoRefreshEnabled] = useState(true);
    const [hasMoreLogs, setHasMoreLogs] = useState(true);
    const [lastLogIndex, setLastLogIndex] = useState(-1);
    
    const logContainerRef = useRef<HTMLDivElement>(null);
    const shouldAutoScrollRef = useRef(true);
    const autoRefreshIntervalRef = useRef<NodeJS.Timeout | null>(null);

    const fetchJobDetail = async () => {
        if (!jobId) return;

        try {
            const jobResponse = await listJobs({
                page_number: 1,
                page_size: 1,
                job_id: parseInt(jobId),
            });

            if (jobResponse.status_code === 200 && 'jobs' in jobResponse.response) {
                const jobData = jobResponse.response.jobs[0];
                if (jobData) {
                    setJob(jobData);
                }
            }
        } catch (error) {
            console.error('Failed to fetch job detail:', error);
        }
    };

    const fetchLogCount = async (): Promise<number | null> => {
        if (!jobId) return null;

        try {
            const countResponse = await countJobLogs({
                job_id: parseInt(jobId),
            });

            if (countResponse.status_code === 200 && 'count' in countResponse.response) {
                const count = countResponse.response.count;
                setTotalLogCount(count);
                return count;
            }
            return null;
        } catch (error) {
            console.error('Failed to fetch log count:', error);
            return null;
        }
    };

    const fetchInitialLogs = async () => {
        if (!jobId) return;

        try {
            setIsLoading(true);
            setError(null);

            await fetchJobDetail();
            const actualCount = await fetchLogCount();

            // 최근 100개 로그를 가져옴
            const logsResponse = await listJobLogs({
                job_id: parseInt(jobId),
                offset: Math.max(0, (actualCount || 0) - INITIAL_LOAD_COUNT),
                limit: INITIAL_LOAD_COUNT,
            });

            if (logsResponse.status_code === 200 && 'logs' in logsResponse.response) {
                const logData = logsResponse.response.logs;
                setLogs(logData);
                if (logData.length > 0) {
                    setLastLogIndex(logData[logData.length - 1].index);
                }
                setHasMoreLogs((actualCount || 0) > INITIAL_LOAD_COUNT);
            } else if ('error_code' in logsResponse.response) {
                setError(logsResponse.response.message || 'Failed to load logs');
            }
        } catch (error) {
            setError('Failed to load logs');
            console.error('Failed to fetch initial logs:', error);
        } finally {
            setIsLoading(false);
        }
    };

    const loadMoreLogs = async () => {
        if (!jobId || isLoadingMore || !hasMoreLogs) return;

        try {
            setIsLoadingMore(true);
            
            const currentOffset = Math.max(0, totalLogCount - logs.length - LOAD_MORE_COUNT);
            const logsResponse = await listJobLogs({
                job_id: parseInt(jobId),
                offset: currentOffset,
                limit: LOAD_MORE_COUNT,
            });

            if (logsResponse.status_code === 200 && 'logs' in logsResponse.response) {
                const newLogs = logsResponse.response.logs;
                setLogs(prevLogs => [...newLogs, ...prevLogs]);
                setHasMoreLogs(currentOffset > 0);
            }
        } catch (error) {
            console.error('Failed to load more logs:', error);
        } finally {
            setIsLoadingMore(false);
        }
    };

    const fetchNewLogs = useCallback(async () => {
        if (!jobId || isLoading) return;

        try {
            // 새로운 로그 개수 확인
            const countResponse = await countJobLogs({
                job_id: parseInt(jobId),
            });

            if (countResponse.status_code === 200 && 'count' in countResponse.response) {
                const newTotalCount = countResponse.response.count;
                
                if (newTotalCount > totalLogCount) {
                    // 새로운 로그가 있으면 가져옴
                    const logsResponse = await listJobLogs({
                        job_id: parseInt(jobId),
                        offset: lastLogIndex + 1,
                        limit: newTotalCount - totalLogCount,
                    });

                    if (logsResponse.status_code === 200 && 'logs' in logsResponse.response) {
                        const newLogs = logsResponse.response.logs;
                        setLogs(prevLogs => [...prevLogs, ...newLogs]);
                        setTotalLogCount(newTotalCount);
                        
                        if (newLogs.length > 0) {
                            setLastLogIndex(newLogs[newLogs.length - 1].index);
                        }

                        // 자동 스크롤이 활성화되어 있으면 맨 아래로 스크롤
                        if (shouldAutoScrollRef.current && logContainerRef.current) {
                            setTimeout(() => {
                                logContainerRef.current?.scrollTo({
                                    top: logContainerRef.current.scrollHeight,
                                    behavior: 'smooth',
                                });
                            }, 100);
                        }
                    }
                }
            }
        } catch (error) {
            console.error('Failed to fetch new logs:', error);
        }
    }, [jobId, isLoading, totalLogCount, lastLogIndex]);

    const handleScroll = useCallback(() => {
        if (!logContainerRef.current) return;

        const container = logContainerRef.current;
        const isNearBottom = container.scrollHeight - container.scrollTop - container.clientHeight < 100;
        shouldAutoScrollRef.current = isNearBottom;
    }, []);

    const scrollToBottom = () => {
        if (logContainerRef.current) {
            logContainerRef.current.scrollTo({
                top: logContainerRef.current.scrollHeight,
                behavior: 'smooth',
            });
        }
        shouldAutoScrollRef.current = true;
    };

    const clearLogs = () => {
        setLogs([]);
        fetchInitialLogs();
    };

    useEffect(() => {
        fetchInitialLogs();
    }, [jobId]);

    useEffect(() => {
        if (isAutoRefreshEnabled) {
            autoRefreshIntervalRef.current = setInterval(fetchNewLogs, POLLING_INTERVAL);
        } else {
            if (autoRefreshIntervalRef.current) {
                clearInterval(autoRefreshIntervalRef.current);
                autoRefreshIntervalRef.current = null;
            }
        }

        return () => {
            if (autoRefreshIntervalRef.current) {
                clearInterval(autoRefreshIntervalRef.current);
            }
        };
    }, [isAutoRefreshEnabled, fetchNewLogs]);

    const getStatusColor = (status: string): "default" | "primary" | "secondary" | "error" | "info" | "success" | "warning" => {
        switch (status) {
            case 'Running':
                return 'info';
            case 'Finished':
                return 'success';
            case 'Failed':
                return 'error';
            case 'Pending':
                return 'warning';
            default:
                return 'default';
        }
    };

    const formatLogTime = (timeStr: string) => {
        const date = new Date(timeStr);
        return date.toLocaleString('ko-KR', {
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
        });
    };

    if (isLoading) {
        return (
            <Box sx={{ display: 'flex', justifyContent: 'center', p: 4 }}>
                <CircularProgress />
            </Box>
        );
    }

    if (error) {
        return (
            <Box sx={{ p: 3 }}>
                <Alert severity="error">{error}</Alert>
            </Box>
        );
    }

    return (
        <Box sx={{ p: 3 }}>
            <Box sx={{ display: 'flex', alignItems: 'center', mb: 3 }}>
                <IconButton onClick={() => navigate('/jobs')} sx={{ mr: 1 }}>
                    <ArrowBackIcon />
                </IconButton>
                <Typography variant="h4" component="h1">
                    작업 로그
                </Typography>
            </Box>

            {job && (
                <Card sx={{ mb: 3 }}>
                    <CardContent>
                        <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                            <Typography variant="h6" sx={{ mr: 2 }}>
                                {job.name}
                            </Typography>
                            <Chip
                                label={job.status}
                                color={getStatusColor(job.status)}
                                size="small"
                            />
                        </Box>
                        <Typography variant="body2" color="text.secondary">
                            작업 ID: {job.id} | 제출 시간: {formatDate(job.submited_at)}
                        </Typography>
                    </CardContent>
                </Card>
            )}

            <Card>
                <CardContent>
                    <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 2 }}>
                        <Typography variant="h6">
                            로그 ({logs.length.toLocaleString()} / {totalLogCount.toLocaleString()})
                        </Typography>
                        <Stack direction="row" spacing={1} alignItems="center">
                            <FormControlLabel
                                control={
                                    <Switch
                                        checked={isAutoRefreshEnabled}
                                        onChange={(e) => setIsAutoRefreshEnabled(e.target.checked)}
                                        size="small"
                                    />
                                }
                                label="자동 새로고침"
                            />
                            <Tooltip title="로그 새로고침">
                                <IconButton onClick={fetchNewLogs} size="small">
                                    <RefreshIcon />
                                </IconButton>
                            </Tooltip>
                            <Tooltip title="맨 아래로 스크롤">
                                <IconButton onClick={scrollToBottom} size="small">
                                    <PlayArrowIcon />
                                </IconButton>
                            </Tooltip>
                            <Tooltip title="로그 지우기">
                                <IconButton onClick={clearLogs} size="small">
                                    <ClearIcon />
                                </IconButton>
                            </Tooltip>
                        </Stack>
                    </Box>

                    <Divider sx={{ mb: 2 }} />

                    {hasMoreLogs && (
                        <Box sx={{ textAlign: 'center', mb: 2 }}>
                            <Button
                                variant="outlined"
                                onClick={loadMoreLogs}
                                disabled={isLoadingMore}
                                startIcon={isLoadingMore ? <CircularProgress size={16} /> : <ExpandMoreIcon />}
                            >
                                {isLoadingMore ? '로딩 중...' : '이전 로그 더보기'}
                            </Button>
                        </Box>
                    )}

                    <Paper
                        ref={logContainerRef}
                        onScroll={handleScroll}
                        sx={{
                            height: '600px',
                            overflow: 'auto',
                            backgroundColor: '#0d1117',
                            fontFamily: 'monospace',
                            fontSize: '14px',
                            lineHeight: 1.4,
                            p: 2,
                        }}
                    >
                        {logs.length === 0 ? (
                            <Box sx={{ textAlign: 'center', py: 4, color: 'text.secondary' }}>
                                로그가 없습니다.
                            </Box>
                        ) : (
                            logs.map((log) => (
                                <Box
                                    key={log.index}
                                    sx={{
                                        display: 'flex',
                                        borderBottom: '1px solid #21262d',
                                        py: 0.5,
                                        '&:hover': {
                                            backgroundColor: '#161b22',
                                        },
                                    }}
                                >
                                    <Box
                                        sx={{
                                            width: '140px',
                                            flexShrink: 0,
                                            color: '#7d8590',
                                            fontSize: '12px',
                                            mr: 2,
                                        }}
                                    >
                                        {formatLogTime(log.time)}
                                    </Box>
                                    <Box sx={{ flex: 1, color: '#f0f6fc', whiteSpace: 'pre-wrap' }}>
                                        {log.message}
                                    </Box>
                                </Box>
                            ))
                        )}
                    </Paper>
                </CardContent>
            </Card>
        </Box>
    );
};

export default JobLogs;
