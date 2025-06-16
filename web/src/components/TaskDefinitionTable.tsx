import React from 'react';
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  IconButton,
  CircularProgress,
  Box,
} from '@mui/material';
import ContentCopyIcon from '@mui/icons-material/ContentCopy';
import { TaskDefinition } from '../api';
import { formatDate } from '../utils';

interface TaskDefinitionTableProps {
  taskDefinitions: TaskDefinition[];
  onVersionCreate: (task: TaskDefinition) => void;
  onRowClick: (task: TaskDefinition) => void;
  isLoading?: boolean;
}

const TaskDefinitionTable: React.FC<TaskDefinitionTableProps> = ({
  taskDefinitions,
  onVersionCreate,
  onRowClick,
  isLoading = false,
}) => {
  return (
    <TableContainer component={Paper}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>ID</TableCell>
            <TableCell>이름</TableCell>
            <TableCell>버전</TableCell>
            <TableCell>이미지</TableCell>
            <TableCell>생성일</TableCell>
            <TableCell>작업</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {isLoading ? (
            <TableRow>
              <TableCell colSpan={6}>
                <Box sx={{ display: 'flex', justifyContent: 'center', p: 3 }}>
                  <CircularProgress />
                </Box>
              </TableCell>
            </TableRow>
          ) : taskDefinitions.length === 0 ? (
            <TableRow>
              <TableCell colSpan={6} align="center">
                작업정의가 없습니다.
              </TableCell>
            </TableRow>
          ) : (
            taskDefinitions.map((task) => (
              <TableRow
                key={task.id}
                hover
                onClick={() => onRowClick(task)}
                sx={{ cursor: 'pointer' }}
              >
                <TableCell>{task.id}</TableCell>
                <TableCell>{task.name}</TableCell>
                <TableCell>{task.version}</TableCell>
                <TableCell>{task.image}</TableCell>
                <TableCell>{formatDate(task.created_at)}</TableCell>
                <TableCell>
                  <IconButton
                    onClick={(e) => {
                      e.stopPropagation();
                      onVersionCreate(task);
                    }}
                    size="small"
                  >
                    <ContentCopyIcon />
                  </IconButton>
                </TableCell>
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

export default TaskDefinitionTable; 