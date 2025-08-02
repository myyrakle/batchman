import React from 'react';
import { Box, Typography, Button } from '@mui/material';

interface PageHeaderProps {
  title: string;
  actions?: React.ReactNode[];
}

const PageHeader: React.FC<PageHeaderProps> = ({ title, actions = [] }) => {
  return (
    <Box
      sx={{
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
        mb: 3,
      }}
    >
      <Typography variant="h5" component="h1">
        {title}
      </Typography>
      {actions.length > 0 && (
        <Box sx={{ display: "flex", gap: 1 }}>
          {actions}
        </Box>
      )}
    </Box>
  );
};

export default PageHeader;