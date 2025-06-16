import React, { useState, useEffect } from 'react';
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
  TextField,
  Box,
  IconButton,
  Typography,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
} from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import DeleteIcon from '@mui/icons-material/Delete';
import { CreateTaskDefinitionFormData } from '../types/taskDefinition';
import { TaskDefinition } from '../api';

interface CreateTaskDefinitionModalProps {
  open: boolean;
  onClose: () => void;
  onSubmit: (data: CreateTaskDefinitionFormData) => void;
  baseTaskDefinition?: TaskDefinition;
  isVersion?: boolean;
}

const CreateTaskDefinitionModal: React.FC<CreateTaskDefinitionModalProps> = ({
  open,
  onClose,
  onSubmit,
  baseTaskDefinition,
  isVersion = false,
}) => {
  const initialFormData: CreateTaskDefinitionFormData = {
    name: '',
    image: '',
    command: '',
    env: [{ key: '', value: '' }],
    resources: {
      memory: {
        value: 1,
        unit: 'g',
      },
      cpu: 1,
    },
    description: '',
  };

  const [formData, setFormData] = useState<CreateTaskDefinitionFormData>(initialFormData);

  useEffect(() => {
    if (open && baseTaskDefinition) {
      const env = baseTaskDefinition.env ? JSON.parse(baseTaskDefinition.env) : [];
      setFormData({
        name:baseTaskDefinition.name,
        image: baseTaskDefinition.image,
        command: baseTaskDefinition.command || '',
        env: env.length > 0 ? env : [{ key: '', value: '' }],
        resources: {
          memory: {
            value: baseTaskDefinition.memory_limit || 1,
            unit: 'm',
          },
          cpu: baseTaskDefinition.cpu_limit || 1,
        },
        description: baseTaskDefinition.description,
      });
    } else if (open) {
      setFormData(initialFormData);
    }
  }, [open, baseTaskDefinition, isVersion]);

  const handleClose = () => {
    setFormData(initialFormData);
    onClose();
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(formData);
  };

  const handleEnvChange = (index: number, field: 'key' | 'value', value: string) => {
    const newEnv = [...formData.env];
    newEnv[index] = { ...newEnv[index], [field]: value };
    setFormData({ ...formData, env: newEnv });
  };

  const addEnvField = () => {
    setFormData({
      ...formData,
      env: [...formData.env, { key: '', value: '' }],
    });
  };

  const removeEnvField = (index: number) => {
    const newEnv = formData.env.filter((_, i) => i !== index);
    setFormData({ ...formData, env: newEnv });
  };

  return (
    <Dialog open={open} onClose={handleClose} maxWidth="md" fullWidth>
      <form onSubmit={handleSubmit}>
        <DialogTitle>{isVersion ? '새 버전 생성' : '새 작업정의 생성'}</DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, mt: 2 }}>
            <TextField
              label="이름"
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              required
              fullWidth
            />

            <TextField
              label="설명"
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              fullWidth
            />

            <TextField
              label="이미지"
              value={formData.image}
              onChange={(e) => setFormData({ ...formData, image: e.target.value })}
              required
              fullWidth
              placeholder="예: nginx:latest"
            />

            <TextField
              label="명령어"
              value={formData.command}
              onChange={(e) => setFormData({ ...formData, command: e.target.value })}
              required
              fullWidth
              placeholder="예: nginx -g 'daemon off;'"
            />

            <Box>
              <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 1 }}>
                <Typography variant="subtitle1">환경 변수</Typography>
                <IconButton onClick={addEnvField} size="small">
                  <AddIcon />
                </IconButton>
              </Box>
              {formData.env.map((env, index) => (
                <Box key={index} sx={{ display: 'flex', gap: 1, mb: 1 }}>
                  <TextField
                    label="키"
                    value={env.key}
                    onChange={(e) => handleEnvChange(index, 'key', e.target.value)}
                    size="small"
                    sx={{ flex: 1 }}
                  />
                  <TextField
                    label="값"
                    value={env.value}
                    onChange={(e) => handleEnvChange(index, 'value', e.target.value)}
                    size="small"
                    sx={{ flex: 1 }}
                  />
                  <IconButton
                    onClick={() => removeEnvField(index)}
                    size="small"
                    disabled={formData?.env?.length === 1}
                  >
                    <DeleteIcon />
                  </IconButton>
                </Box>
              ))}
            </Box>

            <Box sx={{ display: 'flex', gap: 2 }}>
              <Box sx={{ flex: 1 }}>
                <Box sx={{ display: 'flex', gap: 1 }}>
                  <TextField
                    label="메모리 제한"
                    type="number"
                    value={formData.resources.memory.value}
                    onChange={(e) => setFormData({
                      ...formData,
                      resources: {
                        ...formData.resources,
                        memory: {
                          ...formData.resources.memory,
                          value: Number(e.target.value),
                        },
                      },
                    })}
                    required
                    sx={{ flex: 1 }}
                  />
                  <FormControl sx={{ minWidth: 100 }}>
                    <InputLabel>단위</InputLabel>
                    <Select
                      value={formData.resources.memory.unit}
                      label="단위"
                      onChange={(e) => setFormData({
                        ...formData,
                        resources: {
                          ...formData.resources,
                          memory: {
                            ...formData.resources.memory,
                            unit: e.target.value as 'm' | 'g',
                          },
                        },
                      })}
                    >
                      <MenuItem value="m">MB</MenuItem>
                      <MenuItem value="g">GB</MenuItem>
                    </Select>
                  </FormControl>
                </Box>
              </Box>
              <Box sx={{ flex: 1 }}>
                <TextField
                  label="CPU 제한"
                  type="number"
                  value={formData.resources.cpu}
                  onChange={(e) => setFormData({
                    ...formData,
                    resources: {
                      ...formData.resources,
                      cpu: Number(e.target.value),
                    },
                  })}
                  required
                  fullWidth
                  inputProps={{ step: 0.1 }}
                  placeholder="예: 0.5, 1, 2"
                />
              </Box>
            </Box>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleClose}>취소</Button>
          <Button type="submit" variant="contained">
            {isVersion ? '버전 생성' : '생성'}
          </Button>
        </DialogActions>
      </form>
    </Dialog>
  );
};

export default CreateTaskDefinitionModal; 