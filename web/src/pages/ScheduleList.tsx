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
  Pagination,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogContentText,
  DialogActions,
} from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";
import SearchIcon from "@mui/icons-material/Search";
import AddIcon from "@mui/icons-material/Add";
import {
  ErrorResponse,
  listSchedules,
  ListSchedulesResponse,
  ListSchedulesRequest,
  Schedule,
  createSchedule,
  CreateScheduleRequest,
  deleteSchedule,
} from "../api";
import { useSearchParams } from "react-router-dom";
import ScheduleCreateModal from "../components/ScheduleCreateModal";
import ScheduleTable from "../components/ScheduleTable";

const ScheduleList: React.FC = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [schedules, setSchedules] = useState<Schedule[]>([]);
  const [total, setTotal] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [searchText, setSearchText] = useState(
    searchParams.get("contains_name") || "",
  );
  const [enabledFilter, setEnabledFilter] = useState<string>(
    searchParams.get("enabled") || "all",
  );
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
  const [selectedSchedule, setSelectedSchedule] = useState<Schedule | null>(
    null,
  );

  const currentPage = Number(searchParams.get("page_number")) || 1;
  const currentPageSize = Number(searchParams.get("page_size")) || 10;

  const fetchSchedules = async () => {
    try {
      setIsLoading(true);

      const request: ListSchedulesRequest = {
        page_number: currentPage,
        page_size: currentPageSize,
        contains_name: searchParams.get("contains_name") || undefined,
        enabled: searchParams.get("enabled")
          ? searchParams.get("enabled") === "enabled"
          : undefined,
      };

      const result = await listSchedules(request);

      if (result.status_code === 200) {
        const response = result.response as ListSchedulesResponse;
        setSchedules(response.schedules);
        setTotal(response.total_count);
        setError(null);
      } else {
        const errorResponse = result.response as ErrorResponse;
        setError(
          errorResponse.message || "스케줄을 불러오는 중 오류가 발생했습니다.",
        );
      }
    } catch (err) {
      setError("스케줄을 불러오는 중 오류가 발생했습니다.");
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchSchedules();
  }, [currentPage, currentPageSize, searchParams]);

  const handleSearch = () => {
    const newParams = new URLSearchParams(searchParams);
    newParams.set("page_number", "1");

    if (searchText) {
      newParams.set("contains_name", searchText);
    } else {
      newParams.delete("contains_name");
    }

    if (enabledFilter !== "all") {
      newParams.set("enabled", enabledFilter);
    } else {
      newParams.delete("enabled");
    }

    setSearchParams(newParams);
  };

  const handleRefresh = () => {
    fetchSchedules();
  };

  const handlePageChange = (_: React.ChangeEvent<unknown>, page: number) => {
    const newParams = new URLSearchParams(searchParams);
    newParams.set("page_number", page.toString());
    setSearchParams(newParams);
  };

  const handleCreateSchedule = async (scheduleData: CreateScheduleRequest) => {
    try {
      const result = await createSchedule(scheduleData);

      if (result.status_code === 200) {
        setIsCreateModalOpen(false);
        fetchSchedules(); // 목록 새로고침
      } else {
        const errorResponse = result.response as ErrorResponse;
        setError(
          errorResponse.message || "스케줄 생성 중 오류가 발생했습니다.",
        );
      }
    } catch (err) {
      setError("스케줄 생성 중 오류가 발생했습니다.");
    }
  };

  const handleEdit = (schedule: Schedule) => {
    // TODO: 스케줄 수정 모달 열기
    console.log("Edit schedule:", schedule);
  };

  const handleDelete = (schedule: Schedule) => {
    setSelectedSchedule(schedule);
    setIsDeleteModalOpen(true);
  };

  const handleDeleteConfirm = async () => {
    if (!selectedSchedule) return;

    try {
      setIsLoading(true);
      const result = await deleteSchedule(selectedSchedule.id);

      if (result.status_code === 200) {
        setIsDeleteModalOpen(false);
        setSelectedSchedule(null);
        fetchSchedules(); // 목록 새로고침
      } else {
        const errorResponse = result.response as ErrorResponse;
        setError(
          errorResponse.message || "스케줄 삭제 중 오류가 발생했습니다.",
        );
      }
    } catch (err) {
      setError("스케줄 삭제 중 오류가 발생했습니다.");
    } finally {
      setIsLoading(false);
    }
  };

  const handleDeleteCancel = () => {
    setIsDeleteModalOpen(false);
    setSelectedSchedule(null);
  };

  const handleToggleEnabled = (schedule: Schedule) => {
    // TODO: 스케줄 활성화/비활성화 토글
    console.log("Toggle enabled:", schedule);
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
        <Typography variant="h5" component="h1">
          스케줄러
        </Typography>
        <Box sx={{ display: "flex", gap: 1 }}>
          <Button
            variant="contained"
            startIcon={<AddIcon />}
            onClick={() => setIsCreateModalOpen(true)}
          >
            스케줄 생성
          </Button>
          <Button
            variant="outlined"
            startIcon={<RefreshIcon />}
            onClick={handleRefresh}
            disabled={isLoading}
          >
            새로고침
          </Button>
        </Box>
      </Box>

      <Box sx={{ display: "flex", gap: 2, mb: 3, alignItems: "center" }}>
        <TextField
          label="스케줄러 검색"
          value={searchText}
          onChange={(e) => setSearchText(e.target.value)}
          onKeyPress={(e) => e.key === "Enter" && handleSearch()}
          sx={{ width: "300px" }}
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
          variant="contained"
          startIcon={<SearchIcon />}
          onClick={handleSearch}
          disabled={isLoading}
        >
          검색
        </Button>
      </Box>

      {/* 스케줄 테이블 */}
      <ScheduleTable
        schedules={schedules}
        isLoading={isLoading}
        onEdit={handleEdit}
        onDelete={handleDelete}
        onToggleEnabled={handleToggleEnabled}
      />

      {/* 페이지네이션 */}
      <Box sx={{ display: "flex", justifyContent: "center", mt: 2 }}>
        <Pagination
          count={Math.ceil(total / currentPageSize)}
          page={currentPage}
          onChange={handlePageChange}
          color="primary"
        />
      </Box>

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

      {/* 스케줄 생성 모달 */}
      <ScheduleCreateModal
        open={isCreateModalOpen}
        onClose={() => setIsCreateModalOpen(false)}
        onSubmit={handleCreateSchedule}
      />

      {/* 스케줄 삭제 확인 모달 */}
      <Dialog open={isDeleteModalOpen} onClose={handleDeleteCancel}>
        <DialogTitle>스케줄 삭제 확인</DialogTitle>
        <DialogContent>
          <DialogContentText>
            "{selectedSchedule?.name}" 스케줄을 정말 삭제하시겠습니까?
            <br />
            삭제된 스케줄은 복구할 수 없습니다.
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleDeleteCancel} color="inherit">
            취소
          </Button>
          <Button
            onClick={handleDeleteConfirm}
            color="error"
            variant="contained"
            disabled={isLoading}
          >
            삭제
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default ScheduleList;
