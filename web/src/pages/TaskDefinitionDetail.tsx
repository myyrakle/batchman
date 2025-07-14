import React, { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import {
  Box,
  Button,
  Card,
  CardContent,
  Typography,
  Chip,
  Divider,
  IconButton,
  Alert,
  CircularProgress,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableRow,
} from "@mui/material";
import ArrowBackIcon from "@mui/icons-material/ArrowBack";
import EditIcon from "@mui/icons-material/Edit";
import AddIcon from "@mui/icons-material/Add";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import StopIcon from "@mui/icons-material/Stop";
import {
  Job,
  JobStatus,
  TaskDefinition,
  listJobs,
  listTaskDefinitions,
  stopJob,
  patchTaskDefinition,
  createTaskDefinition,
  ErrorResponse,
} from "../api";
import { formatDate } from "../utils";
import CreateTaskDefinitionModal from "../components/CreateTaskDefinitionModal";
import JobSubmitModal from "../components/JobSubmitModal";
import { CreateTaskDefinitionFormData } from "../types/taskDefinition";

const TaskDefinitionDetail: React.FC = () => {
  const { taskDefinitionId } = useParams<{ taskDefinitionId: string }>();
  const navigate = useNavigate();
  const [taskDefinition, setTaskDefinition] = useState<TaskDefinition | null>(
    null,
  );
  const [allVersions, setAllVersions] = useState<TaskDefinition[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isStoppingJob, setIsStoppingJob] = useState(false);
  const [isEditModalOpen, setIsEditModalOpen] = useState(false);
  const [isVersionModalOpen, setIsVersionModalOpen] = useState(false);
  const [isJobSubmitModalOpen, setIsJobSubmitModalOpen] = useState(false);

  const fetchJobDetail = async () => {
    if (!taskDefinitionId) return;

    try {
      setIsLoading(true);
      setError(null);

      // 특정 작업 조회
      const taskDefResult = await listTaskDefinitions({
        page_number: 1,
        page_size: 1,
        task_definition_id: parseInt(taskDefinitionId),
      });

      if (taskDefResult.response instanceof ErrorResponse) {
        setError("작업 정보를 불러오는데 실패했습니다.");
        return;
      }

      if (taskDefResult.response.task_definitions.length === 0) {
        setError("작업을 찾을 수 없습니다.");
        return;
      }

      const taskDefinitionData = taskDefResult.response.task_definitions[0];
      setTaskDefinition(taskDefinitionData);

      // 같은 이름의 다른 버전들도 가져오기
      await fetchAllVersions(taskDefinitionData.name);
    } catch (err) {
      console.error("Failed to fetch job detail:", err);
      setError("작업 정보를 불러오는데 실패했습니다.");
    } finally {
      setIsLoading(false);
    }
  };

  const fetchAllVersions = async (taskDefinitionName: string) => {
    try {
      const versionsResult = await listTaskDefinitions({
        page_number: 1,
        page_size: 100,
        name: taskDefinitionName,
        is_latest_only: false,
      });

      if (versionsResult.response instanceof ErrorResponse) {
        console.error("Failed to fetch versions:", versionsResult.response);
        return;
      }

      // 버전 내림차순으로 정렬
      const sortedVersions = versionsResult.response.task_definitions.sort(
        (a, b) => b.version - a.version,
      );
      setAllVersions(sortedVersions);
    } catch (err) {
      console.error("Failed to fetch all versions:", err);
    }
  };

  useEffect(() => {
    fetchJobDetail();
  }, [taskDefinitionId]);

  const handleEditTask = () => {
    setIsEditModalOpen(true);
  };

  const handleEditModalClose = () => {
    setIsEditModalOpen(false);
  };

  const handleCreateVersion = () => {
    setIsVersionModalOpen(true);
  };

  const handleVersionModalClose = () => {
    setIsVersionModalOpen(false);
  };

  const handleJobSubmit = () => {
    setIsJobSubmitModalOpen(true);
  };

  const handleJobSubmitModalClose = () => {
    setIsJobSubmitModalOpen(false);
  };

  const handleCreateVersionSubmit = async (
    data: CreateTaskDefinitionFormData,
  ): Promise<number | void> => {
    if (!taskDefinitionId) return;

    try {
      setIsLoading(true);
      setError(null);

      // 새 버전 생성은 동일한 createTaskDefinition API를 사용하되 같은 이름으로 생성
      const result = await createTaskDefinition({
        name: data.name, // 동일한 이름으로 새 버전 생성
        description: data.description,
        image: data.image,
        command: data.command,
        env: JSON.stringify(data.env),
        memory_limit:
          data.resources.memory.unit === "g"
            ? data.resources.memory.value * 1024
            : data.resources.memory.value,
        cpu_limit: data.resources.cpu,
        args: undefined,
      });

      if (result.response instanceof ErrorResponse) {
        setError("새 버전 생성에 실패했습니다.");
        console.error(
          "Failed to create new version:",
          result.response.error_code,
          result.response.message,
        );
        return;
      }

      setIsVersionModalOpen(false);
      return result.response?.task_definition_id;
    } catch (error) {
      console.error("Failed to create new version:", error);
      setError("새 버전 생성 중 오류가 발생했습니다.");
    } finally {
      setIsLoading(false);
    }
  };

  const handleEditSubmit = async (
    data: CreateTaskDefinitionFormData,
  ): Promise<void> => {
    if (!taskDefinitionId) return;

    try {
      setIsLoading(true);
      setError(null);

      const result = await patchTaskDefinition(parseInt(taskDefinitionId), {
        image: data.image,
        command: data.command,
        env: JSON.stringify(data.env),
        memory_limit:
          data.resources.memory.unit === "g"
            ? data.resources.memory.value * 1024
            : data.resources.memory.value,
        cpu_limit: data.resources.cpu,
      });

      if (result.response instanceof ErrorResponse) {
        setError("작업정의 수정에 실패했습니다.");
        console.error(
          "Failed to update task definition:",
          result.response.error_code,
          result.response.message,
        );
      } else {
        setIsEditModalOpen(false);
        fetchJobDetail(); // 수정 후 데이터 새로고침
      }
    } catch (error) {
      console.error("Failed to edit task definition:", error);
      setError("작업정의 수정 중 오류가 발생했습니다.");
    } finally {
      setIsLoading(false);
    }
  };

  if (isLoading) {
    return (
      <Box
        sx={{
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          height: "400px",
        }}
      >
        <CircularProgress />
      </Box>
    );
  }

  if (error) {
    return (
      <Box sx={{ p: 3 }}>
        <Alert severity="error" sx={{ mb: 2 }}>
          {error}
        </Alert>
        <Button
          variant="outlined"
          startIcon={<ArrowBackIcon />}
          onClick={() => navigate("/task-definitions")}
        >
          작업 목록으로 돌아가기
        </Button>
      </Box>
    );
  }

  if (!taskDefinition) {
    return (
      <Box sx={{ p: 3 }}>
        <Alert severity="warning" sx={{ mb: 2 }}>
          작업 정의를 찾을 수 없습니다.
        </Alert>
        <Button
          variant="outlined"
          startIcon={<ArrowBackIcon />}
          onClick={() => navigate("/task-definitions")}
        >
          작업정의 목록으로 돌아가기
        </Button>
      </Box>
    );
  }

  return (
    <Box sx={{ p: 3 }}>
      {/* 헤더 */}
      <Box sx={{ display: "flex", alignItems: "center", mb: 3 }}>
        <IconButton
          onClick={() => navigate("/task-definitions")}
          sx={{ mr: 1 }}
        >
          <ArrowBackIcon />
        </IconButton>
        <Typography variant="h5" component="h1" sx={{ flexGrow: 1 }}>
          작업 정의 상세 - {taskDefinition.name}
        </Typography>
        <Box sx={{ display: "flex", gap: 1 }}>
          <Button
            variant="contained"
            startIcon={<PlayArrowIcon />}
            onClick={handleJobSubmit}
            disabled={isLoading || !taskDefinition.enabled}
          >
            작업 제출
          </Button>
          <Button
            variant="outlined"
            startIcon={<AddIcon />}
            onClick={handleCreateVersion}
            disabled={isLoading}
          >
            새 버전 생성
          </Button>
          <Button
            variant="outlined"
            startIcon={<EditIcon />}
            onClick={handleEditTask}
            disabled={isLoading}
          >
            수정
          </Button>
        </Box>
      </Box>

      <Box
        sx={{
          display: "grid",
          gridTemplateColumns: { xs: "1fr", md: "1fr 1fr" },
          gap: 3,
          alignItems: "start",
        }}
      >
        {/* 작업 정의 정보 */}
        <Card sx={{ height: "fit-content" }}>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              작업 정의 정보
            </Typography>
            <Divider sx={{ mb: 2 }} />
            {taskDefinition ? (
              <Table size="small">
                <TableBody>
                  <TableRow>
                    <TableCell
                      component="th"
                      scope="row"
                      sx={{
                        fontWeight: "bold",
                        width: "40%",
                      }}
                    >
                      이름
                    </TableCell>
                    <TableCell>{taskDefinition.name}</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell
                      component="th"
                      scope="row"
                      sx={{ fontWeight: "bold" }}
                    >
                      버전
                    </TableCell>
                    <TableCell>
                      {taskDefinition.version}{" "}
                      {taskDefinition.is_latest ? "[latest]" : ""}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell
                      component="th"
                      scope="row"
                      sx={{ fontWeight: "bold" }}
                    >
                      활성화
                    </TableCell>
                    <TableCell>
                      {taskDefinition.enabled ? "예" : "아니오"}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell
                      component="th"
                      scope="row"
                      sx={{ fontWeight: "bold" }}
                    >
                      설명
                    </TableCell>
                    <TableCell>{taskDefinition.description}</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell
                      component="th"
                      scope="row"
                      sx={{ fontWeight: "bold" }}
                    >
                      이미지
                    </TableCell>
                    <TableCell sx={{ wordBreak: "break-all" }}>
                      {taskDefinition.image}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell
                      component="th"
                      scope="row"
                      sx={{ fontWeight: "bold" }}
                    >
                      명령어
                    </TableCell>
                    <TableCell sx={{ wordBreak: "break-all" }}>
                      {taskDefinition.command || "-"}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell
                      component="th"
                      scope="row"
                      sx={{ fontWeight: "bold" }}
                    >
                      인자
                    </TableCell>
                    <TableCell sx={{ wordBreak: "break-all" }}>
                      {taskDefinition.args || "-"}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell
                      component="th"
                      scope="row"
                      sx={{ fontWeight: "bold" }}
                    >
                      환경 변수
                    </TableCell>
                    <TableCell sx={{ wordBreak: "break-all" }}>
                      {taskDefinition.env || "-"}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell
                      component="th"
                      scope="row"
                      sx={{ fontWeight: "bold" }}
                    >
                      메모리 제한
                    </TableCell>
                    <TableCell>
                      {taskDefinition.memory_limit
                        ? `${taskDefinition.memory_limit} MB`
                        : "-"}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell
                      component="th"
                      scope="row"
                      sx={{ fontWeight: "bold" }}
                    >
                      CPU 제한
                    </TableCell>
                    <TableCell>
                      {taskDefinition.cpu_limit
                        ? `${taskDefinition.cpu_limit}`
                        : "-"}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell
                      component="th"
                      scope="row"
                      sx={{ fontWeight: "bold" }}
                    >
                      생성 시간
                    </TableCell>
                    <TableCell>
                      {formatDate(taskDefinition.created_at)}
                    </TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            ) : (
              <Typography color="text.secondary">
                작업 정의 정보를 불러올 수 없습니다.
              </Typography>
            )}
          </CardContent>
        </Card>

        {/* 다른 버전 목록 */}
        {allVersions.length > 1 && (
          <Card sx={{ height: "fit-content" }}>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                버전 목록
              </Typography>
              <Box
                sx={{
                  maxHeight: 400,
                  overflowY: "auto",
                  border: "1px solid #e0e0e0",
                  borderRadius: 1,
                  p: 1,
                }}
              >
                {allVersions.map((version) => (
                  <Paper
                    key={version.id}
                    elevation={version.id === taskDefinition?.id ? 2 : 0}
                    sx={{
                      p: 1,
                      mb: 0.5,
                      cursor: "pointer",
                      bgcolor:
                        version.id === taskDefinition?.id
                          ? "primary.light"
                          : "background.paper",
                      color:
                        version.id === taskDefinition?.id
                          ? "primary.contrastText"
                          : "text.primary",
                      "&:hover": {
                        bgcolor:
                          version.id === taskDefinition?.id
                            ? "primary.main"
                            : "grey.100",
                      },
                      border:
                        version.id === taskDefinition?.id
                          ? "2px solid"
                          : "1px solid",
                      borderColor:
                        version.id === taskDefinition?.id
                          ? "primary.main"
                          : "grey.300",
                    }}
                    onClick={() => navigate(`/task-definitions/${version.id}`)}
                  >
                    <Box
                      sx={{
                        display: "flex",
                        justifyContent: "space-between",
                        alignItems: "center",
                      }}
                    >
                      <Box>
                        <Typography
                          variant="body1"
                          sx={{ fontWeight: "bold", lineHeight: 1.2 }}
                        >
                          v{version.version}
                          {version.is_latest && (
                            <Chip
                              label="최신"
                              size="small"
                              color="primary"
                              sx={{ ml: 1, height: 18 }}
                            />
                          )}
                          {version.id === taskDefinition?.id && (
                            <Chip
                              label="현재"
                              size="small"
                              color="secondary"
                              sx={{ ml: 1, height: 18 }}
                            />
                          )}
                        </Typography>
                        <Typography variant="caption" sx={{ lineHeight: 1.2 }}>
                          {formatDate(version.created_at)}
                        </Typography>
                      </Box>
                      <Box
                        sx={{ display: "flex", alignItems: "center", gap: 1 }}
                      >
                        <Chip
                          label={version.enabled ? "활성" : "비활성"}
                          size="small"
                          color={version.enabled ? "success" : "default"}
                          sx={{ height: 18 }}
                        />
                      </Box>
                    </Box>
                  </Paper>
                ))}
              </Box>
            </CardContent>
          </Card>
        )}
      </Box>

      {/* 수정 모달 */}
      <CreateTaskDefinitionModal
        open={isEditModalOpen}
        onClose={handleEditModalClose}
        onSubmit={handleEditSubmit}
        baseTaskDefinition={taskDefinition || undefined}
        isVersion={false}
      />

      {/* 새 버전 생성 모달 */}
      <CreateTaskDefinitionModal
        open={isVersionModalOpen}
        onClose={handleVersionModalClose}
        onSubmit={handleCreateVersionSubmit}
        baseTaskDefinition={taskDefinition || undefined}
        isVersion={true}
      />

      {/* 작업 제출 모달 */}
      <JobSubmitModal
        open={isJobSubmitModalOpen}
        onClose={handleJobSubmitModalClose}
        taskDefinition={taskDefinition}
      />
    </Box>
  );
};

export default TaskDefinitionDetail;
