import React from 'react';
import { Box, Typography } from '@mui/material';
import { PageContainer, StatsCard } from '../components/common';

const Dashboard: React.FC = () => {
    return (
        <PageContainer>
            <Typography variant="h4" gutterBottom>
                대시보드
            </Typography>
            <Box
                sx={{
                    display: 'grid',
                    gridTemplateColumns: {
                        xs: '1fr',
                        sm: '1fr 1fr',
                        md: '1fr 1fr 1fr 1fr',
                    },
                    gap: 3,
                }}
            >
                <StatsCard title="전체 태스크" value={0} />
                <StatsCard title="실행 중인 태스크" value={0} />
                <StatsCard title="성공한 태스크" value={0} />
                <StatsCard title="실패한 태스크" value={0} />
            </Box>
        </PageContainer>
    );
};

export default Dashboard;
