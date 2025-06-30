// YYYY-MM-DD HH:MM:SS
export const formatDate = (dateString: string | null) => {
    if (!dateString) return '-';

    return new Date(dateString).toLocaleString('ko-KR');
};
