import React from 'react';
import { Box, TextField, Button } from '@mui/material';
import { ListTaskDefinitionsParams } from '../api';

interface TaskDefinitionSearchProps {
    searchParams: ListTaskDefinitionsParams;
    onSearchParamsChange: (params: ListTaskDefinitionsParams) => void;
    onSearch: () => void;
}

const TaskDefinitionSearch: React.FC<TaskDefinitionSearchProps> = ({
    searchParams,
    onSearchParamsChange,
    onSearch,
}) => {
    const handleSearch = () => {
        onSearchParamsChange({
            ...searchParams,
            page_number: 1, // 검색 시 첫 페이지로 이동
        });
        onSearch();
    };

    const handleKeyPress = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter') {
            handleSearch();
        }
    };

    return (
        <Box sx={{ display: 'flex', gap: 2, mb: 2 }}>
            <TextField
                label="검색어"
                value={searchParams.contains_name || ''}
                onChange={e =>
                    onSearchParamsChange({
                        ...searchParams,
                        contains_name: e.target.value,
                    })
                }
                onKeyPress={handleKeyPress}
                size="small"
            />
            <Button variant="contained" onClick={handleSearch}>
                검색
            </Button>
        </Box>
    );
};

export default TaskDefinitionSearch;
