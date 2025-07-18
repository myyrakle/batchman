import React from 'react';
import { Box, CircularProgress, Typography } from '@mui/material';

interface LoadingOverlayProps {
    message?: string;
}

const LoadingOverlay: React.FC<LoadingOverlayProps> = ({
    message = '로딩 중...',
}) => {
    return (
        <Box
            sx={{
                position: 'fixed',
                top: 0,
                left: 0,
                right: 0,
                bottom: 0,
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                justifyContent: 'center',
                backgroundColor: 'rgba(0, 0, 0, 0.95)',
                backdropFilter: 'blur(8px)',
                zIndex: 9999,
            }}
        >
            <CircularProgress sx={{ color: 'primary.main' }} />
            <Typography variant="h6" sx={{ mt: 2, color: 'primary.main' }}>
                {message}
            </Typography>
        </Box>
    );
};

export default LoadingOverlay;
