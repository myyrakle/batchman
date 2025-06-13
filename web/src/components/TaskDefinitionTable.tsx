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

interface TaskDefinitionTableProps {
  taskDefinitions: TaskDefinition[];
  onVersionCreate: (task: TaskDefinition) => void;
  isLoading?: boolean;
}

const TaskDefinitionTable: React.FC<TaskDefinitionTableProps> = ({
  taskDefinitions,
  onVersionCreate,
  isLoading = false,
}) => {
  return (
    <TableContainer component={Paper}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>이름</TableCell>
            <TableCell>설명</TableCell>
            <TableCell>버전</TableCell>
            <TableCell>상태</TableCell>
            <TableCell>생성일</TableCell>
            <TableCell>수정일</TableCell>
            <TableCell>작업</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {isLoading ? (
            <TableRow>
              <TableCell colSpan={7}>
                <Box sx={{ display: 'flex', justifyContent: 'center', p: 3 }}>
                  <CircularProgress />
                </Box>
              </TableCell>
            </TableRow>
          ) : taskDefinitions.length === 0 ? (
            <TableRow>
              <TableCell colSpan={7} align="center">
                작업정의가 없습니다.
              </TableCell>
            </TableRow>
          ) : (
            taskDefinitions.map((task) => (
              <TableRow key={task.id}>
                <TableCell>{task.name}</TableCell>
                <TableCell>{task.description}</TableCell>
                <TableCell>{task.version}</TableCell>
                <TableCell>{"task.status"}</TableCell>
                <TableCell>{"task.createdAt"}</TableCell>
                <TableCell>{"task.updatedAt"}</TableCell>
                <TableCell>
                  <IconButton
                    size="small"
                    onClick={() => onVersionCreate(task)}
                    title="새 버전 생성"
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