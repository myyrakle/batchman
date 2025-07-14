import React, { useState, useEffect } from "react";
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Snackbar,
  Alert,
  Box,
  Typography,
  CircularProgress,
  Autocomplete,
  Chip,
  Paper,
  RadioGroup,
  FormControlLabel,
  Radio,
  FormLabel,
} from "@mui/material";
import { PlayArrow as PlayArrowIcon } from "@mui/icons-material";
import {
  TaskDefinition,
  ErrorResponse,
  submitJob,
  listTaskDefinitions,
  ListTaskDefinitionsParams,
} from "../api";
import { useNavigate } from "react-router-dom";

interface JobCreateModalProps {
  open: boolean;
  onClose: () => void;
  onJobCreated?: () => void;
  preselectedTaskDefinition?: TaskDefinition;
}

const JobCreateModal: React.FC<JobCreateModalProps> = ({
  open,
  onClose,
  onJobCreated,
  preselectedTaskDefinition,
}) => {
  const navigate = useNavigate();
  const [taskDefinitions, setTaskDefinitions] = useState<TaskDefinition[]>([]);
  const [selectedTaskDefinitionName, setSelectedTaskDefinitionName] =
    useState<string>("");
  const [selectedTaskDefinitionId, setSelectedTaskDefinitionId] = useState<
    number | ""
  >("");
  const [availableVersions, setAvailableVersions] = useState<TaskDefinition[]>(
    [],
  );
  const [jobName, setJobName] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isLoadingTaskDefinitions, setIsLoadingTaskDefinitions] =
    useState(false);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  // 로그 만료일 관련 상태
  const [logExpirationMode, setLogExpirationMode] = useState<
    "none" | "days" | "date"
  >("none");
  const [logExpirationDays, setLogExpirationDays] = useState<number>(30);
  const [logExpirationDate, setLogExpirationDate] = useState<string>("");
  const [logExpirationTime, setLogExpirationTime] = useState<string>("23:59");

  const selectedTaskDefinition = taskDefinitions.find(
    (td) => td.id === selectedTaskDefinitionId,
  );

  // 고유한 작업정의 이름 목록 생성
  const uniqueTaskDefinitionNames = Array.from(
    new Set(taskDefinitions.map((td) => td.name)),
  ).sort();

  useEffect(() => {
    if (open) {
      fetchTaskDefinitions();
    }
  }, [open]);

  // preselectedTaskDefinition이 있을 때 초기화
  useEffect(() => {
    if (preselectedTaskDefinition && taskDefinitions.length > 0) {
      setSelectedTaskDefinitionName(preselectedTaskDefinition.name);
      setSelectedTaskDefinitionId(preselectedTaskDefinition.id);
    }
  }, [preselectedTaskDefinition, taskDefinitions]);

  // 선택된 작업정의 이름에 따라 버전 목록 업데이트
  useEffect(() => {
    if (selectedTaskDefinitionName) {
      const versions = taskDefinitions
        .filter((td) => td.name === selectedTaskDefinitionName && td.enabled)
        .sort((a, b) => b.version - a.version); // 버전 내림차순 정렬
      setAvailableVersions(versions);

      // preselectedTaskDefinition이 있으면 해당 버전을 선택, 없으면 최신 버전 선택
      if (
        preselectedTaskDefinition &&
        versions.find((v) => v.id === preselectedTaskDefinition.id)
      ) {
        setSelectedTaskDefinitionId(preselectedTaskDefinition.id);
      } else {
        const latestVersion = versions.find((v) => v.is_latest);
        if (latestVersion) {
          setSelectedTaskDefinitionId(latestVersion.id);
        } else if (versions.length > 0) {
          setSelectedTaskDefinitionId(versions[0].id);
        }
      }
    } else {
      setAvailableVersions([]);
      setSelectedTaskDefinitionId("");
    }
  }, [selectedTaskDefinitionName, taskDefinitions, preselectedTaskDefinition]);

  const fetchTaskDefinitions = async () => {
    try {
      setIsLoadingTaskDefinitions(true);
      const params: ListTaskDefinitionsParams = {
        page_number: 1,
        page_size: 1000, // 모든 버전을 가져오도록 크게 설정
        is_latest_only: false, // 모든 버전을 가져오도록 설정
      };

      const result = await listTaskDefinitions(params);

      if (result.response instanceof ErrorResponse) {
        setErrorMessage(
          `작업 정의 로드에 실패했습니다: ${
            result.response.message || "알 수 없는 오류"
          }`,
        );
        return;
      }

      const enabledTaskDefinitions = result.response.task_definitions.filter(
        (td) => td.enabled,
      );
      setTaskDefinitions(enabledTaskDefinitions);
    } catch (error) {
      setErrorMessage("작업 정의 로드 중 오류가 발생했습니다.");
      console.error("Failed to load task definitions:", error);
    } finally {
      setIsLoadingTaskDefinitions(false);
    }
  };

  const handleClose = () => {
    setJobName("");

    // preselectedTaskDefinition이 없을 때만 작업 정의 선택 상태 초기화
    if (!preselectedTaskDefinition) {
      setSelectedTaskDefinitionName("");
      setSelectedTaskDefinitionId("");
      setAvailableVersions([]);
    }

    setErrorMessage(null);

    // 로그 만료일 상태 초기화
    setLogExpirationMode("none");
    setLogExpirationDays(30);
    setLogExpirationDate("");
    setLogExpirationTime("23:59");

    onClose();
  };

  const handleSubmitConfirm = async () => {
    if (!selectedTaskDefinitionId || !jobName.trim()) {
      setErrorMessage("작업 정의와 작업 이름을 모두 입력해주세요.");
      return;
    }

    // 로그 만료일 계산
    let logExpireAfter: string | undefined;

    if (logExpirationMode === "days") {
      const expireDate = new Date();
      expireDate.setDate(expireDate.getDate() + logExpirationDays);
      logExpireAfter = expireDate.toISOString();
    } else if (logExpirationMode === "date") {
      if (!logExpirationDate) {
        setErrorMessage("로그 만료일을 선택해주세요.");
        return;
      }
      const expirationDateTime = new Date(
        `${logExpirationDate}T${logExpirationTime}`,
      );

      if (expirationDateTime.getTime() <= Date.now()) {
        setErrorMessage("로그 만료일은 현재 시간 이후로 설정해주세요.");
        return;
      }

      logExpireAfter = expirationDateTime.toISOString();
    }

    try {
      setIsSubmitting(true);
      setErrorMessage(null);

      const submitData: any = {
        task_definition_id: selectedTaskDefinitionId as number,
        job_name: jobName.trim(),
      };

      if (logExpireAfter) {
        submitData.log_expire_after = logExpireAfter;
      }

      const result = await submitJob(submitData);

      if (result.response instanceof ErrorResponse) {
        setErrorMessage(
          `작업 제출에 실패했습니다: ${
            result.response.message || "알 수 없는 오류"
          }`,
        );
        console.error(
          "Failed to submit job:",
          result.response.error_code,
          result.response.message,
        );
        return;
      }

      const jobID = result.response.job_id;

      setSuccessMessage(`작업 "${jobName}"이 성공적으로 제출되었습니다.`);
      handleClose();

      // 부모 컴포넌트에게 작업 생성 완료 알림
      if (onJobCreated) {
        onJobCreated();
      }

      // 작업 상세 페이지로 즉시 이동
      if (jobID) {
        navigate(`/jobs/${jobID}`);
      } else {
        navigate("/jobs");
      }
    } catch (error) {
      setErrorMessage("작업 제출 중 오류가 발생했습니다.");
      console.error("Failed to submit job:", error);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleCloseSuccessAlert = () => {
    setSuccessMessage(null);
  };

  const handleCloseErrorAlert = () => {
    setErrorMessage(null);
  };

  return (
    <>
      <Dialog open={open} onClose={handleClose} maxWidth="sm" fullWidth>
        <DialogTitle>새 작업 생성</DialogTitle>
        <DialogContent>
          <Box sx={{ display: "flex", flexDirection: "column", gap: 2, mt: 1 }}>
            <Autocomplete
              value={selectedTaskDefinitionName}
              onChange={(_, newValue) => {
                setSelectedTaskDefinitionName(newValue || "");
              }}
              options={uniqueTaskDefinitionNames}
              loading={isLoadingTaskDefinitions}
              renderInput={(params) => (
                <TextField
                  {...params}
                  label="작업 정의 검색"
                  placeholder="작업 정의 이름을 입력하세요"
                  required
                  InputProps={{
                    ...params.InputProps,
                    endAdornment: (
                      <>
                        {isLoadingTaskDefinitions ? (
                          <CircularProgress color="inherit" size={20} />
                        ) : null}
                        {params.InputProps.endAdornment}
                      </>
                    ),
                  }}
                />
              )}
              disabled={isLoadingTaskDefinitions || !!preselectedTaskDefinition}
            />

            {selectedTaskDefinitionName && availableVersions.length > 0 && (
              <FormControl fullWidth required>
                <InputLabel>버전</InputLabel>
                <Select
                  value={selectedTaskDefinitionId}
                  label="버전"
                  onChange={(e) =>
                    setSelectedTaskDefinitionId(e.target.value as number)
                  }
                  disabled={!!preselectedTaskDefinition}
                >
                  {availableVersions.map((taskDef) => (
                    <MenuItem key={taskDef.id} value={taskDef.id}>
                      <Box
                        sx={{ display: "flex", alignItems: "center", gap: 1 }}
                      >
                        v{taskDef.version}
                        {taskDef.is_latest && (
                          <Chip
                            label="최신"
                            size="small"
                            color="primary"
                            sx={{ height: 18 }}
                          />
                        )}
                      </Box>
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            )}

            {selectedTaskDefinition && (
              <Paper
                elevation={1}
                sx={{
                  p: 2,
                  mt: 1,
                }}
              >
                <Typography variant="body2">
                  <strong>이름:</strong> {selectedTaskDefinition.name}
                </Typography>
                <Typography variant="body2" sx={{ mt: 1 }}>
                  <strong>버전:</strong> v{selectedTaskDefinition.version}
                  {selectedTaskDefinition.is_latest && (
                    <Chip
                      label="최신"
                      size="small"
                      color="primary"
                      sx={{ ml: 1, height: 18 }}
                    />
                  )}
                </Typography>
                <Typography variant="body2" sx={{ mt: 1 }}>
                  <strong>설명:</strong> {selectedTaskDefinition.description}
                </Typography>
                <Typography variant="body2" sx={{ mt: 1 }}>
                  <strong>이미지:</strong> {selectedTaskDefinition.image}
                </Typography>
                {selectedTaskDefinition.command && (
                  <Typography variant="body2" sx={{ mt: 1 }}>
                    <strong>명령어:</strong> {selectedTaskDefinition.command}
                  </Typography>
                )}
              </Paper>
            )}

            <TextField
              autoFocus
              margin="dense"
              label="작업 이름"
              fullWidth
              value={jobName}
              onChange={(e) => setJobName(e.target.value)}
              required
              placeholder="작업에 대한 고유한 이름을 입력하세요"
            />

            {/* 로그 만료일 설정 */}
            <Box sx={{ mt: 2 }}>
              <FormLabel component="legend">로그 만료일</FormLabel>
              <RadioGroup
                row
                value={logExpirationMode}
                onChange={(e) =>
                  setLogExpirationMode(
                    e.target.value as "none" | "days" | "date",
                  )
                }
              >
                <FormControlLabel
                  value="none"
                  control={<Radio />}
                  label="만료 없음"
                />
                <FormControlLabel
                  value="days"
                  control={<Radio />}
                  label="일수 지정"
                />
                <FormControlLabel
                  value="date"
                  control={<Radio />}
                  label="날짜 지정"
                />
              </RadioGroup>

              {logExpirationMode === "days" && (
                <Box sx={{ mt: 1 }}>
                  <TextField
                    type="number"
                    label="만료 일수"
                    value={logExpirationDays}
                    onChange={(e) =>
                      setLogExpirationDays(Number(e.target.value))
                    }
                    inputProps={{ min: 1, max: 365 }}
                    sx={{ width: "200px" }}
                    helperText="1일 ~ 365일"
                  />
                </Box>
              )}

              {logExpirationMode === "date" && (
                <Box sx={{ mt: 1, display: "flex", gap: 2 }}>
                  <TextField
                    type="date"
                    label="만료 날짜"
                    value={logExpirationDate}
                    onChange={(e) => setLogExpirationDate(e.target.value)}
                    InputLabelProps={{
                      shrink: true,
                    }}
                    inputProps={{
                      min: new Date().toISOString().split("T")[0],
                    }}
                  />
                  <TextField
                    type="time"
                    label="만료 시간"
                    value={logExpirationTime}
                    onChange={(e) => setLogExpirationTime(e.target.value)}
                    InputLabelProps={{
                      shrink: true,
                    }}
                  />
                </Box>
              )}
            </Box>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleClose} disabled={isSubmitting}>
            취소
          </Button>
          <Button
            onClick={handleSubmitConfirm}
            variant="contained"
            startIcon={<PlayArrowIcon />}
            disabled={
              !selectedTaskDefinitionId || !jobName.trim() || isSubmitting
            }
          >
            {isSubmitting ? "제출 중..." : "작업 생성"}
          </Button>
        </DialogActions>
      </Dialog>

      {/* 성공 알림 */}
      <Snackbar
        open={!!successMessage}
        autoHideDuration={6000}
        onClose={handleCloseSuccessAlert}
        anchorOrigin={{ vertical: "top", horizontal: "center" }}
      >
        <Alert
          onClose={handleCloseSuccessAlert}
          severity="success"
          sx={{ width: "100%" }}
        >
          {successMessage}
        </Alert>
      </Snackbar>

      {/* 에러 알림 */}
      <Snackbar
        open={!!errorMessage}
        autoHideDuration={6000}
        onClose={handleCloseErrorAlert}
        anchorOrigin={{ vertical: "top", horizontal: "center" }}
      >
        <Alert
          onClose={handleCloseErrorAlert}
          severity="error"
          sx={{ width: "100%" }}
        >
          {errorMessage}
        </Alert>
      </Snackbar>
    </>
  );
};

export default JobCreateModal;
