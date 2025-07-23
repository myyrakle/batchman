import React, { useState, useEffect } from "react";
import {
  Box,
  Button,
  Typography,
  Alert,
  Snackbar,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Card,
  CardContent,
  CardActions,
  Chip,
  IconButton,
  Tooltip,
  Stack,
} from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";
import SearchIcon from "@mui/icons-material/Search";
import AddIcon from "@mui/icons-material/Add";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/Delete";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import PauseIcon from "@mui/icons-material/Pause";
import AccessTimeIcon from "@mui/icons-material/AccessTime";
import {
  ErrorResponse,
  listSchedules,
  ListSchedulesResponse,
  Schedule,
} from "../api";
import { useSearchParams } from "react-router-dom";

const ScheduleList: React.FC = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [schedules, setSchedules] = useState<Schedule[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [searchText, setSearchText] = useState(
    searchParams.get("contains_name") || "",
  );
  const [enabledFilter, setEnabledFilter] = useState<string>(
    searchParams.get("enabled") || "all",
  );

  const fetchSchedules = async () => {
    try {
      setIsLoading(true);
      const result = await listSchedules();

      if (result.status_code === 200) {
        const response = result.response as ListSchedulesResponse;
        let filteredSchedules = response.schedules;

        // 검색 필터 적용
        if (searchText) {
          filteredSchedules = filteredSchedules.filter((schedule) =>
            schedule.name.toLowerCase().includes(searchText.toLowerCase()) ||
            schedule.job_name.toLowerCase().includes(searchText.toLowerCase())
          );
        }

        // 활성화 상태 필터 적용
        if (enabledFilter !== "all") {
          const isEnabled = enabledFilter === "enabled";
          filteredSchedules = filteredSchedules.filter(
            (schedule) => schedule.enabled === isEnabled
          );
        }

        setSchedules(filteredSchedules);
        setError(null);
      } else {
        const errorResponse = result.response as ErrorResponse;
        setError(errorResponse.message || "스케줄을 불러오는 중 오류가 발생했습니다.");
      }
    } catch (err) {
      setError("스케줄을 불러오는 중 오류가 발생했습니다.");
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchSchedules();
  }, []);

  const handleSearch = () => {
    const params = new URLSearchParams();
    if (searchText) {
      params.set("contains_name", searchText);
    }
    if (enabledFilter !== "all") {
      params.set("enabled", enabledFilter);
    }
    setSearchParams(params);
    fetchSchedules();
  };

  const handleRefresh = () => {
    fetchSchedules();
  };

  const formatDateTime = (dateString: string) => {
    return new Date(dateString).toLocaleString("ko-KR");
  };

  const formatCronExpression = (cron: string) => {
    // 간단한 cron 표현식 설명
    if (cron === "0 * * * *") return "매시 정각";
    if (cron === "0 0 * * *") return "매일 자정";
    if (cron === "0 0 * * 0") return "매주 일요일 자정";
    if (cron === "0 0 1 * *") return "매월 1일 자정";
    return cron;
  };

  return (
    <Box sx={{ p: 3 }}>
      <Box
        sx={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          mb: 3,
        }}
      >
        <Typography variant="h4" component="h1">
          스케줄 관리
        </Typography>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={() => {
            // TODO: 스케줄 생성 모달 열기
          }}
        >
          스케줄 생성
        </Button>
      </Box>

      {/* 검색 및 필터 */}
      <Box sx={{ mb: 3 }}>
        <Stack direction={{ xs: "column", sm: "row" }} spacing={2} alignItems="center">
          <TextField
            label="스케줄명 또는 작업명 검색"
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            onKeyPress={(e) => e.key === "Enter" && handleSearch()}
            sx={{ minWidth: 200 }}
          />
          <FormControl sx={{ minWidth: 150 }}>
            <InputLabel>활성화 상태</InputLabel>
            <Select
              value={enabledFilter}
              label="활성화 상태"
              onChange={(e) => setEnabledFilter(e.target.value)}
            >
              <MenuItem value="all">전체</MenuItem>
              <MenuItem value="enabled">활성화</MenuItem>
              <MenuItem value="disabled">비활성화</MenuItem>
            </Select>
          </FormControl>
          <Button
            variant="outlined"
            startIcon={<SearchIcon />}
            onClick={handleSearch}
          >
            검색
          </Button>
          <Button
            variant="outlined"
            startIcon={<RefreshIcon />}
            onClick={handleRefresh}
            disabled={isLoading}
          >
            새로고침
          </Button>
        </Stack>
      </Box>

      {/* 스케줄 카드 목록 */}
      {isLoading ? (
        <Typography>로딩 중...</Typography>
      ) : schedules.length === 0 ? (
        <Card sx={{ p: 4, textAlign: "center" }}>
          <Typography variant="h6" color="text.secondary">
            스케줄이 없습니다.
          </Typography>
          <Typography variant="body2" color="text.secondary" sx={{ mt: 1 }}>
            새 스케줄을 생성해보세요.
          </Typography>
        </Card>
      ) : (
        <Box
          sx={{
            display: "grid",
            gridTemplateColumns: "repeat(auto-fill, minmax(400px, 1fr))",
            gap: 3,
          }}
        >
          {schedules.map((schedule) => (
            <Card 
              key={schedule.id}
              sx={{ 
                display: "flex", 
                flexDirection: "column",
                border: schedule.enabled ? "1px solid #4caf50" : "1px solid #666"
              }}
            >
              <CardContent sx={{ flexGrow: 1 }}>
                <Box sx={{ display: "flex", justifyContent: "space-between", mb: 2 }}>
                  <Typography variant="h6" component="h2" noWrap>
                    {schedule.name}
                  </Typography>
                  <Chip
                    label={schedule.enabled ? "활성화" : "비활성화"}
                    color={schedule.enabled ? "success" : "default"}
                    size="small"
                  />
                </Box>

                <Typography color="text.secondary" gutterBottom>
                  작업명: {schedule.job_name}
                </Typography>

                <Box sx={{ display: "flex", alignItems: "center", mb: 1 }}>
                  <AccessTimeIcon sx={{ mr: 1, fontSize: 16 }} />
                  <Typography variant="body2" color="text.secondary">
                    {formatCronExpression(schedule.cron_expression)}
                  </Typography>
                </Box>

                <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
                  태스크 정의 ID: {schedule.task_definition_id}
                </Typography>

                {schedule.command && (
                  <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
                    명령어: {schedule.command}
                  </Typography>
                )}

                {schedule.timezone && (
                  <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
                    시간대: {schedule.timezone}
                  </Typography>
                )}

                <Typography variant="caption" color="text.secondary">
                  생성일: {formatDateTime(schedule.created_at)}
                </Typography>
              </CardContent>

              <CardActions sx={{ justifyContent: "space-between", px: 2, pb: 2 }}>
                <Box>
                  <Tooltip title={schedule.enabled ? "일시정지" : "활성화"}>
                    <IconButton
                      color={schedule.enabled ? "warning" : "success"}
                      onClick={() => {
                        // TODO: 스케줄 활성화/비활성화 토글
                      }}
                    >
                      {schedule.enabled ? <PauseIcon /> : <PlayArrowIcon />}
                    </IconButton>
                  </Tooltip>
                </Box>

                <Box>
                  <Tooltip title="수정">
                    <IconButton
                      color="primary"
                      onClick={() => {
                        // TODO: 스케줄 수정 모달 열기
                      }}
                    >
                      <EditIcon />
                    </IconButton>
                  </Tooltip>
                  <Tooltip title="삭제">
                    <IconButton
                      color="error"
                      onClick={() => {
                        // TODO: 스케줄 삭제 확인 대화상자
                      }}
                    >
                      <DeleteIcon />
                    </IconButton>
                  </Tooltip>
                </Box>
              </CardActions>
            </Card>
          ))}
        </Box>
      )}

      {/* 에러 스낵바 */}
      <Snackbar
        open={!!error}
        autoHideDuration={6000}
        onClose={() => setError(null)}
      >
        <Alert onClose={() => setError(null)} severity="error">
          {error}
        </Alert>
      </Snackbar>
    </Box>
  );
};

export default ScheduleList;
