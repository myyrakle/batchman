import React from 'react';
import {
  Box,
  TextField,
  Button,
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
     
      <Button variant="contained" onClick={onSearch}>
        검색
      </Button>
    </Box>
  );
};

export default TaskDefinitionSearch; 