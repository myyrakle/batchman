import React from 'react';
import { useNavigate } from 'react-router-dom';
import {
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableRow,
    Paper,
    IconButton,
    Box,
    Skeleton,
    Link,
} from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import DeleteIcon from '@mui/icons-material/Delete';
import { TaskDefinition } from '../api';

interface TaskDefinitionTableProps {
    taskDefinitions: TaskDefinition[];
    onRowClick: (task: TaskDefinition) => void;
    onDelete: (task: TaskDefinition) => void;
    onVersionCreate: (task: TaskDefinition) => void;
    isLoading?: boolean;
}

const TaskDefinitionTable: React.FC<TaskDefinitionTableProps> = ({
    taskDefinitions,
    onRowClick,
    onDelete,
    onVersionCreate,
    isLoading = false,
}) => {
    const navigate = useNavigate();

    const renderSkeletonRow = () => (
        <TableRow>
            <TableCell>
                <Skeleton variant="text" sx={{ bgcolor: 'grey.200' }} />
            </TableCell>
            <TableCell>
                <Skeleton variant="text" sx={{ bgcolor: 'grey.200' }} />
            </TableCell>
            <TableCell>
                <Skeleton variant="text" sx={{ bgcolor: 'grey.200' }} />
            </TableCell>
            <TableCell>
                <Skeleton variant="text" sx={{ bgcolor: 'grey.200' }} />
            </TableCell>
            <TableCell>
                <Skeleton variant="text" sx={{ bgcolor: 'grey.200' }} />
            </TableCell>
            <TableCell>
                <Box sx={{ display: 'flex', gap: 1 }}>
                    <Skeleton
                        variant="circular"
                        width={32}
                        height={32}
                        sx={{ bgcolor: 'grey.200' }}
                    />
                    <Skeleton
                        variant="circular"
                        width={32}
                        height={32}
                        sx={{ bgcolor: 'grey.200' }}
                    />
                </Box>
            </TableCell>
        </TableRow>
    );

    return (
        <TableContainer component={Paper}>
            <Table>
                <TableHead>
                    <TableRow>
                        <TableCell>ID</TableCell>
                        <TableCell>이름</TableCell>
                        <TableCell>Version</TableCell>
                        <TableCell>이미지</TableCell>
                        <TableCell>명령어</TableCell>
                        <TableCell>생성일</TableCell>
                        <TableCell>작업</TableCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {isLoading
                        ? Array.from({ length: 5 }).map((_, index) => (
                              <React.Fragment key={index}>
                                  {renderSkeletonRow()}
                              </React.Fragment>
                          ))
                        : taskDefinitions.map(task => (
                              <TableRow
                                  key={task.id}
                                  onClick={() => onRowClick(task)}
                                  sx={{
                                      cursor: 'pointer',
                                      '&:hover': {
                                          backgroundColor:
                                              'rgba(0, 0, 0, 0.04)',
                                      },
                                  }}
                              >
                                  <TableCell>{task.id}</TableCell>
                                  <TableCell>
                                      <Link
                                          component="button"
                                          variant="body2"
                                          onClick={e => {
                                              e.stopPropagation(); // 상위 TableRow의 onClick 이벤트 전파 방지
                                              navigate(
                                                  `/task-definitions/${task.id}`
                                              ); // 작업 정의 상세 페이지로 이동
                                          }}
                                          sx={{
                                              fontWeight: 'medium',
                                              textAlign: 'left',
                                              cursor: 'pointer',
                                              '&:hover': {
                                                  textDecoration: 'underline',
                                              },
                                          }}
                                      >
                                          {task.name}
                                      </Link>
                                  </TableCell>
                                  <TableCell>{task.version}</TableCell>
                                  <TableCell>{task.image}</TableCell>
                                  <TableCell>{task.command}</TableCell>
                                  <TableCell>
                                      {new Date(
                                          task.created_at
                                      ).toLocaleString()}
                                  </TableCell>
                                  <TableCell>
                                      <Box sx={{ display: 'flex', gap: 1 }}>
                                          <IconButton
                                              size="small"
                                              onClick={e => {
                                                  e.stopPropagation();
                                                  onVersionCreate(task);
                                              }}
                                          >
                                              <AddIcon />
                                          </IconButton>
                                          <IconButton
                                              size="small"
                                              onClick={e => {
                                                  e.stopPropagation();
                                                  onDelete(task);
                                              }}
                                              sx={{ color: 'error.main' }}
                                          >
                                              <DeleteIcon />
                                          </IconButton>
                                      </Box>
                                  </TableCell>
                              </TableRow>
                          ))}
                </TableBody>
            </Table>
        </TableContainer>
    );
};

export default TaskDefinitionTable;
