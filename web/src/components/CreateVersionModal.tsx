import React from 'react';
import {
    Dialog,
    DialogTitle,
    DialogContent,
    DialogActions,
    Button,
} from '@mui/material';
import { TaskDefinition } from '../api';

interface CreateVersionModalProps {
    open: boolean;
    onClose: () => void;
    onSubmit: () => void;
    taskDefinition: TaskDefinition | null;
}

const CreateVersionModal: React.FC<CreateVersionModalProps> = ({
    open,
    onClose,
    onSubmit,
}) => {
    return (
        <Dialog open={open} onClose={onClose} maxWidth="md" fullWidth>
            <DialogTitle>새 버전 생성</DialogTitle>
            <DialogContent>{/* TODO: 새 버전 생성 폼 구현 */}</DialogContent>
            <DialogActions>
                <Button onClick={onClose}>취소</Button>
                <Button variant="contained" onClick={onSubmit}>
                    생성
                </Button>
            </DialogActions>
        </Dialog>
    );
};

export default CreateVersionModal;
