import React from 'react';
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Box,
  Skeleton,
  Chip,
} from '@mui/material';
import { Job, JobStatus } from '../api';

interface JobTableProps {
  jobs: Job[];
  onRowClick?: (job: Job) => void;
  isLoading?: boolean;
}

const JobTable: React.FC<JobTableProps> = ({
  jobs,
  onRowClick,
  isLoading = false,
}) => {
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

  const formatDate = (dateString: string | null) => {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleString('ko-KR');
  };

  const renderSkeletonRow = () => (
    <TableRow>
      <TableCell><Skeleton variant="text" sx={{ bgcolor: 'grey.200' }} /></TableCell>
      <TableCell><Skeleton variant="text" sx={{ bgcolor: 'grey.200' }} /></TableCell>
      <TableCell><Skeleton variant="rectangular" width={60} height={24} sx={{ bgcolor: 'grey.200' }} /></TableCell>
      <TableCell><Skeleton variant="text" sx={{ bgcolor: 'grey.200' }} /></TableCell>
      <TableCell><Skeleton variant="text" sx={{ bgcolor: 'grey.200' }} /></TableCell>
      <TableCell><Skeleton variant="text" sx={{ bgcolor: 'grey.200' }} /></TableCell>
      <TableCell><Skeleton variant="text" sx={{ bgcolor: 'grey.200' }} /></TableCell>
      <TableCell><Skeleton variant="text" sx={{ bgcolor: 'grey.200' }} /></TableCell>
    </TableRow>
  );

  return (
    <TableContainer component={Paper}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>ID</TableCell>
            <TableCell>이름</TableCell>
            <TableCell>상태</TableCell>
            <TableCell>작업정의</TableCell>
            <TableCell>제출시간</TableCell>
            <TableCell>시작시간</TableCell>
            <TableCell>완료시간</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {isLoading ? (
            Array.from({ length: 5 }).map((_, index) => (
              <React.Fragment key={index}>
                {renderSkeletonRow()}
              </React.Fragment>
            ))
          ) : jobs.length === 0 ? (
            <TableRow>
              <TableCell colSpan={8} align="center" sx={{ py: 4, color: 'text.secondary' }}>
                조회된 작업이 없습니다.
              </TableCell>
            </TableRow>
          ) : (
            jobs.map((job) => (
              <TableRow
                key={job.id}
                hover
                onClick={() => onRowClick?.(job)}
                sx={{ cursor: onRowClick ? 'pointer' : 'default' }}
              >
                <TableCell>{job.id}</TableCell>
                <TableCell sx={{ fontWeight: 'medium' }}>{job.name}</TableCell>
                <TableCell>
                  <Chip
                    label={getStatusLabel(job.status)}
                    color={getStatusColor(job.status)}
                    size="small"
                  />
                </TableCell>
                <TableCell>{job.task_definition_name??''}</TableCell>
                <TableCell sx={{ fontSize: '0.875rem' }}>
                  {formatDate(job.submited_at)}
                </TableCell>
                <TableCell sx={{ fontSize: '0.875rem' }}>
                  {formatDate(job.started_at)}
                </TableCell>
                <TableCell sx={{ fontSize: '0.875rem' }}>
                  {formatDate(job.finished_at)}
                </TableCell>
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

export default JobTable;
