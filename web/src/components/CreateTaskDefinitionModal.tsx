import React from 'react';
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
} from '@mui/material';

interface CreateTaskDefinitionModalProps {
  open: boolean;
  onClose: () => void;
  onSubmit: () => void;
}

const CreateTaskDefinitionModal: React.FC<CreateTaskDefinitionModalProps> = ({
  open,
  onClose,
  onSubmit,
}) => {
  return (
    <Dialog open={open} onClose={onClose} maxWidth="md" fullWidth>
      <DialogTitle>새 작업정의 생성</DialogTitle>
      <DialogContent>
        {/* TODO: 작업정의 생성 폼 구현 */}
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose}>취소</Button>
        <Button variant="contained" onClick={onSubmit}>생성</Button>
      </DialogActions>
    </Dialog>
  );
};

export default CreateTaskDefinitionModal; 