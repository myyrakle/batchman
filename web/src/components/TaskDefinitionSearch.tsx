import React from 'react';
import {
  Box,
  TextField,
  Button,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
} from '@mui/material';
import { TaskDefinitionSearchParams } from '../types/taskDefinition';

interface TaskDefinitionSearchProps {
  searchParams: TaskDefinitionSearchParams;
  onSearchParamsChange: (params: TaskDefinitionSearchParams) => void;
  onSearch: () => void;
}

const TaskDefinitionSearch: React.FC<TaskDefinitionSearchProps> = ({
  searchParams,
  onSearchParamsChange,
  onSearch,
}) => {
  return (
    <Box sx={{ display: 'flex', gap: 2, mb: 2 }}>
      <TextField
        label="검색어"
        value={searchParams.keyword}
        onChange={(e) => onSearchParamsChange({ ...searchParams, keyword: e.target.value })}
        size="small"
      />
      <FormControl size="small" sx={{ minWidth: 120 }}>
        <InputLabel>상태</InputLabel>
        <Select
          value={searchParams.status || ''}
          label="상태"
          onChange={(e) => onSearchParamsChange({ ...searchParams, status: e.target.value as 'ACTIVE' | 'INACTIVE' })}
        >
          <MenuItem value="">전체</MenuItem>
          <MenuItem value="ACTIVE">활성</MenuItem>
          <MenuItem value="INACTIVE">비활성</MenuItem>
        </Select>
      </FormControl>
      <Button variant="contained" onClick={onSearch}>
        검색
      </Button>
    </Box>
  );
};

export default TaskDefinitionSearch; 