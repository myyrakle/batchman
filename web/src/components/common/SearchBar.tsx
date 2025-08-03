import React from 'react';
import { Box, TextField, Button } from '@mui/material';
import SearchIcon from '@mui/icons-material/Search';

interface SearchField {
  label: string;
  value: string;
  onChange: (value: string) => void;
  width?: string;
  onKeyPress?: (event: React.KeyboardEvent) => void;
}

interface SearchFilter {
  component: React.ReactNode;
}

interface SearchBarProps {
  fields: SearchField[];
  filters?: SearchFilter[];
  onSearch: () => void;
  isLoading?: boolean;
}

const SearchBar: React.FC<SearchBarProps> = ({ 
  fields, 
  filters = [], 
  onSearch, 
  isLoading = false 
}) => {
  return (
    <Box sx={{ display: "flex", gap: 2, mb: 3, alignItems: "center" }}>
      {fields.map((field, index) => (
        <TextField
          key={index}
          label={field.label}
          value={field.value}
          onChange={(e) => field.onChange(e.target.value)}
          onKeyPress={field.onKeyPress}
          sx={{ width: field.width || "300px" }}
        />
      ))}
      {filters.map((filter, index) => (
        <React.Fragment key={index}>
          {filter.component}
        </React.Fragment>
      ))}
      <Button
        variant="contained"
        startIcon={<SearchIcon />}
        onClick={onSearch}
        disabled={isLoading}
      >
        검색
      </Button>
    </Box>
  );
};

export default SearchBar;