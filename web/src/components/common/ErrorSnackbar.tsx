import React from 'react';
import { Alert, Snackbar } from '@mui/material';

interface ErrorSnackbarProps {
  error: string | null;
  onClose: () => void;
  autoHideDuration?: number;
}

const ErrorSnackbar: React.FC<ErrorSnackbarProps> = ({ 
  error, 
  onClose, 
  autoHideDuration = 6000 
}) => {
  return (
    <Snackbar
      open={!!error}
      autoHideDuration={autoHideDuration}
      onClose={onClose}
      anchorOrigin={{ vertical: "top", horizontal: "center" }}
    >
      <Alert
        onClose={onClose}
        severity="error"
        sx={{ width: "100%" }}
      >
        {error}
      </Alert>
    </Snackbar>
  );
};

export default ErrorSnackbar;