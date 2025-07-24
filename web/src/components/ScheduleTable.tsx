import React from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Box,
  Skeleton,
  Chip,
  IconButton,
  Tooltip,
} from "@mui/material";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/Delete";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import PauseIcon from "@mui/icons-material/Pause";
import AccessTimeIcon from "@mui/icons-material/AccessTime";
import { Schedule } from "../api";

interface ScheduleTableProps {
  schedules: Schedule[];
  isLoading?: boolean;
  onEdit?: (schedule: Schedule) => void;
  onDelete?: (schedule: Schedule) => void;
  onToggleEnabled?: (schedule: Schedule) => void;
}

const ScheduleTable: React.FC<ScheduleTableProps> = ({
  schedules,
  isLoading = false,
  onEdit,
  onDelete,
  onToggleEnabled,
}) => {
  const formatDateTime = (dateString: string) => {
    return new Date(dateString).toLocaleString("ko-KR");
  };

  const formatCronExpression = (cron: string) => {
    // 간단한 cron 표현식 설명
    if (cron === "0 * * * *") return "(매시 정각)";
    if (cron === "0 0 * * *") return "(매일 자정)";
    if (cron === "0 0 * * 0") return "(매주 일요일 자정)";
    if (cron === "0 0 1 * *") return "(매월 1일 자정)";
    return "";
  };

  if (isLoading) {
    return (
      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>ID</TableCell>
              <TableCell>스케줄명</TableCell>
              <TableCell>Cron 표현식</TableCell>
              <TableCell>작업 정의</TableCell>
              <TableCell>상태</TableCell>
              <TableCell>생성일</TableCell>
              <TableCell align="center">작업</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {Array.from({ length: 5 }).map((_, index) => (
              <TableRow key={index}>
                <TableCell>
                  <Skeleton />
                </TableCell>
                <TableCell>
                  <Skeleton />
                </TableCell>
                <TableCell>
                  <Skeleton />
                </TableCell>
                <TableCell>
                  <Skeleton />
                </TableCell>
                <TableCell>
                  <Skeleton />
                </TableCell>
                <TableCell>
                  <Skeleton />
                </TableCell>
                <TableCell>
                  <Skeleton />
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    );
  }

  return (
    <TableContainer component={Paper}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>ID</TableCell>
            <TableCell>스케줄명</TableCell>
            <TableCell>Cron 표현식</TableCell>
            <TableCell>작업 정의</TableCell>
            <TableCell>상태</TableCell>
            <TableCell>생성일</TableCell>
            <TableCell align="center">작업</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {schedules.length === 0 ? (
            <TableRow>
              <TableCell colSpan={8} align="center" sx={{ py: 4 }}>
                스케줄이 없습니다.
              </TableCell>
            </TableRow>
          ) : (
            schedules.map((schedule) => (
              <TableRow
                key={schedule.id}
                sx={{
                  "&:hover": { bgcolor: "action.hover" },
                  borderLeft: schedule.enabled
                    ? "4px solid #4caf50"
                    : "4px solid #666",
                }}
              >
                <TableCell>{schedule.id}</TableCell>
                <TableCell>
                  <Box sx={{ fontWeight: "medium" }}>{schedule.name}</Box>
                </TableCell>
                <TableCell>
                  <Box sx={{ display: "flex", alignItems: "center" }}>
                    <AccessTimeIcon sx={{ mr: 1, fontSize: 16 }} />
                    <Box>
                      <div
                        style={{ fontSize: "0.75rem", color: "text.secondary" }}
                      >
                        {schedule.cron_expression}
                      </div>
                      <div>
                        {formatCronExpression(schedule.cron_expression)}
                      </div>
                    </Box>
                  </Box>
                </TableCell>
                <TableCell>{schedule.task_definition_id}</TableCell>
                <TableCell>
                  <Chip
                    label={schedule.enabled ? "활성화" : "비활성화"}
                    color={schedule.enabled ? "success" : "default"}
                    size="small"
                  />
                </TableCell>
                <TableCell>{formatDateTime(schedule.created_at)}</TableCell>
                <TableCell align="center">
                  <Box
                    sx={{ display: "flex", gap: 0.5, justifyContent: "center" }}
                  >
                    <Tooltip title={schedule.enabled ? "일시정지" : "활성화"}>
                      <IconButton
                        size="small"
                        color={schedule.enabled ? "warning" : "success"}
                        onClick={() => onToggleEnabled?.(schedule)}
                      >
                        {schedule.enabled ? <PauseIcon /> : <PlayArrowIcon />}
                      </IconButton>
                    </Tooltip>
                    <Tooltip title="수정">
                      <IconButton
                        size="small"
                        color="primary"
                        onClick={() => onEdit?.(schedule)}
                      >
                        <EditIcon />
                      </IconButton>
                    </Tooltip>
                    <Tooltip title="삭제">
                      <IconButton
                        size="small"
                        color="error"
                        onClick={() => onDelete?.(schedule)}
                      >
                        <DeleteIcon />
                      </IconButton>
                    </Tooltip>
                  </Box>
                </TableCell>
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

export default ScheduleTable;
