import React from 'react';
import { Box } from '@mui/material';

interface PageContainerProps {
  children: React.ReactNode;
  padding?: number;
}

const PageContainer: React.FC<PageContainerProps> = ({ children, padding = 3 }) => {
  return (
    <Box sx={{ p: padding }}>
      {children}
    </Box>
  );
};

export default PageContainer;