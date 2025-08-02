import React, { useState, useEffect } from "react";
import {
  Box,
  Button,
  Alert,
  Snackbar,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
} from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";
import SearchIcon from "@mui/icons-material/Search";
import AddIcon from "@mui/icons-material/Add";
import JobTable from "../components/JobTable";
import JobCreateModal from "../components/JobCreateModal";
import { PageContainer, PageHeader, SearchBar, ErrorSnackbar, PagePagination } from "../components/common";
import {
  ErrorResponse,
  listJobs,
  ListJobsRequest,
  Job,
  JobStatus,
} from "../api";
import { useSearchParams, useNavigate } from "react-router-dom";

const JobList: React.FC = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const navigate = useNavigate();
  const [jobs, setJobs] = useState<Job[]>([]);
  const [total, setTotal] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [isJobCreateModalOpen, setIsJobCreateModalOpen] = useState(false);
  const [searchText, setSearchText] = useState(
    searchParams.get("contains_name") || "",
  );
  const [statusFilter, setStatusFilter] = useState<JobStatus | "">(
    (searchParams.get("status") as JobStatus) || "",
  );

  const currentPage = Number(searchParams.get("page_number")) || 1;
  const currentPageSize = Number(searchParams.get("page_size")) || 10;

  const fetchJobs = async () => {
    try {
      setIsLoading(true);
      const request: ListJobsRequest = {
        page_number: currentPage,
        page_size: currentPageSize,
        job_id: searchParams.get("job_id")
          ? Number(searchParams.get("job_id"))
          : undefined,
        status: (searchParams.get("status") as JobStatus) || undefined,
        contains_name: searchParams.get("contains_name") || undefined,
      };
      console.log("API request:", request); // 디버깅용
      const result = await listJobs(request);

      if (result.response instanceof ErrorResponse) {
        setError("작업 목록을 불러오는데 실패했습니다.");
        console.error(
          "Failed to fetch jobs:",
          result.response.error_code,
          result.response.message,
        );
      } else {
        setJobs(result.response.jobs);
        setTotal(result.response.total_count);
      }
    } catch (err) {
      setError("작업 목록을 불러오는데 실패했습니다.");
      console.error("Failed to fetch jobs:", err);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchJobs();
  }, [currentPage, currentPageSize, searchParams]);

  const handleSearch = () => {
    const newParams = new URLSearchParams(searchParams);
    newParams.set("page_number", "1");

    if (searchText) {
      newParams.set("contains_name", searchText);
    } else {
      newParams.delete("contains_name");
    }

    if (statusFilter) {
      newParams.set("status", statusFilter);
    } else {
      newParams.delete("status");
    }

    setSearchParams(newParams);
  };

  const handleCloseError = () => {
    setError(null);
  };

  const handlePageChange = (
    _event: React.ChangeEvent<unknown>,
    value: number,
  ) => {
    const newParams = new URLSearchParams(searchParams);
    newParams.set("page_number", String(value));
    setSearchParams(newParams);
  };

  const handleJobClick = (job: Job) => {
    navigate(`/jobs/${job.id}`);
  };

  const handleOpenJobCreateModal = () => {
    setIsJobCreateModalOpen(true);
  };

  const handleCloseJobCreateModal = () => {
    setIsJobCreateModalOpen(false);
  };

  const handleJobCreated = () => {
    // 새 작업이 생성되면 목록을 새로고침
    fetchJobs();
  };

  return (
    <PageContainer>
      <PageHeader
        title="작업 목록"
        actions={[
          <Button
            key="create"
            variant="contained"
            startIcon={<AddIcon />}
            onClick={handleOpenJobCreateModal}
            color="primary"
          >
            새 작업 생성
          </Button>,
          <Button
            key="refresh"
            variant="outlined"
            startIcon={<RefreshIcon />}
            onClick={fetchJobs}
            disabled={isLoading}
          >
            새로고침
          </Button>
        ]}
      />

      <SearchBar
        fields={[
          {
            label: "검색",
            value: searchText,
            onChange: setSearchText,
            width: "300px"
          }
        ]}
        filters={[
          {
            component: (
              <FormControl sx={{ minWidth: 120 }}>
                <InputLabel>상태</InputLabel>
                <Select
                  value={statusFilter}
                  label="상태"
                  onChange={(e) => setStatusFilter(e.target.value as JobStatus | "")}
                >
                  <MenuItem value="">전체</MenuItem>
                  <MenuItem value="Pending">대기중</MenuItem>
                  <MenuItem value="Starting">시작중</MenuItem>
                  <MenuItem value="Running">실행중</MenuItem>
                  <MenuItem value="Finished">완료</MenuItem>
                  <MenuItem value="Failed">실패</MenuItem>
                </Select>
              </FormControl>
            )
          }
        ]}
        onSearch={handleSearch}
        isLoading={isLoading}
      />

      <Box sx={{ position: "relative", minHeight: "400px" }}>
        <JobTable
          jobs={jobs}
          onRowClick={handleJobClick}
          isLoading={isLoading}
        />
      </Box>

      <PagePagination
        total={total}
        pageSize={currentPageSize}
        currentPage={currentPage}
        onPageChange={handlePageChange}
      />

      <ErrorSnackbar
        error={error}
        onClose={handleCloseError}
      />

      <JobCreateModal
        open={isJobCreateModalOpen}
        onClose={handleCloseJobCreateModal}
        onJobCreated={handleJobCreated}
      />
    </PageContainer>
  );
};

export default JobList;
