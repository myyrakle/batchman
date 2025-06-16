import React from 'react';
import { Box, Typography, Paper } from '@mui/material';

const Dashboard: React.FC = () => {
  return (
    <Box sx={{ p: 3 }}>
      <Typography variant="h4" gutterBottom>
        대시보드
      </Typography>
      <Box sx={{ display: 'grid', gridTemplateColumns: { xs: '1fr', sm: '1fr 1fr', md: '1fr 1fr 1fr 1fr' }, gap: 3 }}>
        <Box>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              전체 태스크
            </Typography>
            <Typography variant="h4">0</Typography>
          </Paper>
        </Box>
        <Box>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              실행 중인 태스크
            </Typography>
            <Typography variant="h4">0</Typography>
          </Paper>
        </Box>
        <Box>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              성공한 태스크
            </Typography>
            <Typography variant="h4">0</Typography>
          </Paper>
        </Box>
        <Box>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              실패한 태스크
            </Typography>
            <Typography variant="h4">0</Typography>
          </Paper>
        </Box>
      </Box>
    </Box>
  );
};

export default Dashboard; 