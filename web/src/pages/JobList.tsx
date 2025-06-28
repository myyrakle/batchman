import React, { useState, useEffect } from 'react';
import { Box, Button, Typography, Alert, Snackbar, Pagination, TextField, FormControl, InputLabel, Select, MenuItem } from '@mui/material';
import RefreshIcon from '@mui/icons-material/Refresh';
import SearchIcon from '@mui/icons-material/Search';
import JobTable from '../components/JobTable';
import { ErrorResponse, listJobs, ListJobsRequest, Job, JobStatus } from '../api';
import { useSearchParams, useNavigate } from 'react-router-dom';

const JobList: React.FC = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const navigate = useNavigate();
  const [jobs, setJobs] = useState<Job[]>([]);
  const [total, setTotal] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [searchText, setSearchText] = useState(searchParams.get('contains_name') || '');
  const [statusFilter, setStatusFilter] = useState<JobStatus | ''>((searchParams.get('status') as JobStatus) || '');

  const currentPage = Number(searchParams.get('page_number')) || 1;
  const currentPageSize = Number(searchParams.get('page_size')) || 10;

  const fetchJobs = async () => {
    try {
      setIsLoading(true);
      const request: ListJobsRequest = {
        page_number: currentPage,
        page_size: currentPageSize,
        job_id: searchParams.get('job_id') ? Number(searchParams.get('job_id')) : undefined,
        status: searchParams.get('status') as JobStatus || undefined,
        contains_name: searchParams.get('contains_name') || undefined,
      };
      console.log('API request:', request); // 디버깅용
      const result = await listJobs(request);
      
      if (result.response instanceof ErrorResponse) {
        setError('작업 목록을 불러오는데 실패했습니다.');
        console.error('Failed to fetch jobs:', result.response.error_code, result.response.message);
      } else {
        setJobs(result.response.jobs);
        setTotal(result.response.total_count);
      }
    } catch (err) {
      setError('작업 목록을 불러오는데 실패했습니다.');
      console.error('Failed to fetch jobs:', err);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchJobs();
  }, [currentPage, currentPageSize, searchParams]);

  const handleSearch = () => {
    const newParams = new URLSearchParams(searchParams);
    newParams.set('page_number', '1');
    
    if (searchText) {
      newParams.set('contains_name', searchText);
    } else {
      newParams.delete('contains_name');
    }
    
    if (statusFilter) {
      newParams.set('status', statusFilter);
    } else {
      newParams.delete('status');
    }
    
    setSearchParams(newParams);
  };

  const handleCloseError = () => {
    setError(null);
  };

  const handlePageChange = (_event: React.ChangeEvent<unknown>, value: number) => {
    const newParams = new URLSearchParams(searchParams);
    newParams.set('page_number', String(value));
    setSearchParams(newParams);
  };

  const handleJobClick = (job: Job) => {
    navigate(`/jobs/${job.id}`);
  };

  return (
    <Box sx={{ p: 3 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h5" component="h1">
          작업 목록
        </Typography>
        <Button
          variant="outlined"
          startIcon={<RefreshIcon />}
          onClick={fetchJobs}
          disabled={isLoading}
        >
          새로고침
        </Button>
      </Box>

      <Box sx={{ display: 'flex', gap: 2, mb: 3, alignItems: 'center' }}>
        <TextField
          label="검색"
          value={searchText}
          onChange={(e) => setSearchText(e.target.value)}
          sx={{ width: '300px' }}
        />
        <FormControl sx={{ minWidth: 120 }}>
          <InputLabel>상태</InputLabel>
          <Select
            value={statusFilter}
            label="상태"
            onChange={(e) => setStatusFilter(e.target.value as JobStatus | '')}
          >
            <MenuItem value="">전체</MenuItem>
            <MenuItem value="Pending">대기중</MenuItem>
            <MenuItem value="Starting">시작중</MenuItem>
            <MenuItem value="Running">실행중</MenuItem>
            <MenuItem value="Finished">완료</MenuItem>
            <MenuItem value="Failed">실패</MenuItem>
          </Select>
        </FormControl>
        <Button
          variant="contained"
          startIcon={<SearchIcon />}
          onClick={handleSearch}
          disabled={isLoading}
        >
          검색
        </Button>
      </Box>

      <Box sx={{ position: 'relative', minHeight: '400px' }}>
        <JobTable
          jobs={jobs}
          onRowClick={handleJobClick}
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

export default JobList;
