import React from 'react';
import { Box, Paper, Typography } from '@mui/material';

interface StatsCardProps {
  title: string;
  value: string | number;
  sx?: object;
}

const StatsCard: React.FC<StatsCardProps> = ({ title, value, sx = {} }) => {
  return (
    <Box sx={sx}>
      <Paper sx={{ p: 2 }}>
        <Typography variant="h6" gutterBottom>
          {title}
        </Typography>
        <Typography variant="h4">{value}</Typography>
      </Paper>
    </Box>
  );
};

export default StatsCard;