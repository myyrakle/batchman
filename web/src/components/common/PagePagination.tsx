import React from 'react';
import { Box, Pagination } from '@mui/material';

interface PagePaginationProps {
  total: number;
  pageSize: number;
  currentPage: number;
  onPageChange: (event: React.ChangeEvent<unknown>, value: number) => void;
  color?: 'primary' | 'secondary' | 'standard';
}

const PagePagination: React.FC<PagePaginationProps> = ({
  total,
  pageSize,
  currentPage,
  onPageChange,
  color = 'primary'
}) => {
  const pageCount = Math.ceil(total / pageSize);
  
  if (pageCount <= 1) {
    return null;
  }

  return (
    <Box sx={{ display: "flex", justifyContent: "center", mt: 2 }}>
      <Pagination
        count={pageCount}
        page={currentPage}
        onChange={onPageChange}
        color={color}
      />
    </Box>
  );
};

export default PagePagination;