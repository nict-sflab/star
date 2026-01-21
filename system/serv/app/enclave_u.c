#include "enclave_u.h"
#include <errno.h>

typedef struct ms_say_something_t {
	sgx_status_t ms_retval;
	const uint8_t* ms_some_string;
	size_t ms_len;
} ms_say_something_t;

typedef struct ms_u_read_ocall_t {
	size_t ms_retval;
	int* ms_error;
	int ms_fd;
	void* ms_buf;
	size_t ms_count;
} ms_u_read_ocall_t;

typedef struct ms_u_pread64_ocall_t {
	size_t ms_retval;
	int* ms_error;
	int ms_fd;
	void* ms_buf;
	size_t ms_count;
	int64_t ms_offset;
} ms_u_pread64_ocall_t;

typedef struct ms_u_write_ocall_t {
	size_t ms_retval;
	int* ms_error;
	int ms_fd;
	const void* ms_buf;
	size_t ms_count;
} ms_u_write_ocall_t;

typedef struct ms_u_pwrite64_ocall_t {
	size_t ms_retval;
	int* ms_error;
	int ms_fd;
	const void* ms_buf;
	size_t ms_count;
	int64_t ms_offset;
} ms_u_pwrite64_ocall_t;

typedef struct ms_u_sendfile_ocall_t {
	size_t ms_retval;
	int* ms_error;
	int ms_out_fd;
	int ms_in_fd;
	int64_t* ms_offset;
	size_t ms_count;
} ms_u_sendfile_ocall_t;

typedef struct ms_u_copy_file_range_ocall_t {
	size_t ms_retval;
	int* ms_error;
	int ms_fd_in;
	int64_t* ms_off_in;
	int ms_fd_out;
	int64_t* ms_off_out;
	size_t ms_len;
	unsigned int ms_flags;
} ms_u_copy_file_range_ocall_t;

typedef struct ms_u_splice_ocall_t {
	size_t ms_retval;
	int* ms_error;
	int ms_fd_in;
	int64_t* ms_off_in;
	int ms_fd_out;
	int64_t* ms_off_out;
	size_t ms_len;
	unsigned int ms_flags;
} ms_u_splice_ocall_t;

typedef struct ms_u_fcntl_arg0_ocall_t {
	int ms_retval;
	int* ms_error;
	int ms_fd;
	int ms_cmd;
} ms_u_fcntl_arg0_ocall_t;

typedef struct ms_u_fcntl_arg1_ocall_t {
	int ms_retval;
	int* ms_error;
	int ms_fd;
	int ms_cmd;
	int ms_arg;
} ms_u_fcntl_arg1_ocall_t;

typedef struct ms_u_ioctl_arg0_ocall_t {
	int ms_retval;
	int* ms_error;
	int ms_fd;
	unsigned long int ms_request;
} ms_u_ioctl_arg0_ocall_t;

typedef struct ms_u_ioctl_arg1_ocall_t {
	int ms_retval;
	int* ms_error;
	int ms_fd;
	unsigned long int ms_request;
	int* ms_arg;
} ms_u_ioctl_arg1_ocall_t;

typedef struct ms_u_close_ocall_t {
	int ms_retval;
	int* ms_error;
	int ms_fd;
} ms_u_close_ocall_t;

typedef struct ms_u_isatty_ocall_t {
	int ms_retval;
	int* ms_error;
	int ms_fd;
} ms_u_isatty_ocall_t;

typedef struct ms_u_dup_ocall_t {
	int ms_retval;
	int* ms_error;
	int ms_oldfd;
} ms_u_dup_ocall_t;

typedef struct ms_u_eventfd_ocall_t {
	int ms_retval;
	int* ms_error;
	unsigned int ms_initval;
	int ms_flags;
} ms_u_eventfd_ocall_t;

typedef struct ms_u_futimens_ocall_t {
	int ms_retval;
	int* ms_error;
	int ms_fd;
	const struct timespec* ms_times;
} ms_u_futimens_ocall_t;

typedef struct ms_u_malloc_ocall_t {
	void* ms_retval;
	int* ms_error;
	size_t ms_size;
	size_t ms_align;
	int ms_zeroed;
} ms_u_malloc_ocall_t;

typedef struct ms_u_free_ocall_t {
	void* ms_p;
} ms_u_free_ocall_t;

typedef struct ms_u_mmap_ocall_t {
	void* ms_retval;
	int* ms_error;
	void* ms_start;
	size_t ms_length;
	int ms_prot;
	int ms_flags;
	int ms_fd;
	int64_t ms_offset;
} ms_u_mmap_ocall_t;

typedef struct ms_u_munmap_ocall_t {
	int ms_retval;
	int* ms_error;
	void* ms_start;
	size_t ms_length;
} ms_u_munmap_ocall_t;

typedef struct ms_u_msync_ocall_t {
	int ms_retval;
	int* ms_error;
	void* ms_addr;
	size_t ms_length;
	int ms_flags;
} ms_u_msync_ocall_t;

typedef struct ms_u_mprotect_ocall_t {
	int ms_retval;
	int* ms_error;
	void* ms_addr;
	size_t ms_length;
	int ms_prot;
} ms_u_mprotect_ocall_t;

typedef struct ms_u_read_hostbuf_ocall_t {
	size_t ms_retval;
	int* ms_error;
	const void* ms_host_buf;
	void* ms_encl_buf;
	size_t ms_count;
} ms_u_read_hostbuf_ocall_t;

typedef struct ms_u_write_hostbuf_ocall_t {
	size_t ms_retval;
	int* ms_error;
	void* ms_host_buf;
	const void* ms_encl_buf;
	size_t ms_count;
} ms_u_write_hostbuf_ocall_t;

typedef struct ms_u_thread_set_event_ocall_t {
	int ms_retval;
	int* ms_error;
	size_t ms_tcs;
} ms_u_thread_set_event_ocall_t;

typedef struct ms_u_thread_wait_event_ocall_t {
	int ms_retval;
	int* ms_error;
	size_t ms_tcs;
	const struct timespec* ms_timeout;
	int ms_clockid;
	int ms_absolute_time;
} ms_u_thread_wait_event_ocall_t;

typedef struct ms_u_thread_set_multiple_events_ocall_t {
	int ms_retval;
	int* ms_error;
	size_t* ms_tcss;
	size_t ms_total;
} ms_u_thread_set_multiple_events_ocall_t;

typedef struct ms_u_thread_setwait_events_ocall_t {
	int ms_retval;
	int* ms_error;
	size_t ms_waiter_tcs;
	size_t ms_self_tcs;
	const struct timespec* ms_timeout;
	int ms_clockid;
	int ms_absolute_time;
} ms_u_thread_setwait_events_ocall_t;

typedef struct ms_u_clock_gettime_ocall_t {
	int ms_retval;
	int* ms_error;
	int ms_clk_id;
	struct timespec* ms_tp;
} ms_u_clock_gettime_ocall_t;

static sgx_status_t SGX_CDECL enclave_u_read_ocall(void* pms)
{
	ms_u_read_ocall_t* ms = SGX_CAST(ms_u_read_ocall_t*, pms);
	ms->ms_retval = u_read_ocall(ms->ms_error, ms->ms_fd, ms->ms_buf, ms->ms_count);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_pread64_ocall(void* pms)
{
	ms_u_pread64_ocall_t* ms = SGX_CAST(ms_u_pread64_ocall_t*, pms);
	ms->ms_retval = u_pread64_ocall(ms->ms_error, ms->ms_fd, ms->ms_buf, ms->ms_count, ms->ms_offset);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_write_ocall(void* pms)
{
	ms_u_write_ocall_t* ms = SGX_CAST(ms_u_write_ocall_t*, pms);
	ms->ms_retval = u_write_ocall(ms->ms_error, ms->ms_fd, ms->ms_buf, ms->ms_count);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_pwrite64_ocall(void* pms)
{
	ms_u_pwrite64_ocall_t* ms = SGX_CAST(ms_u_pwrite64_ocall_t*, pms);
	ms->ms_retval = u_pwrite64_ocall(ms->ms_error, ms->ms_fd, ms->ms_buf, ms->ms_count, ms->ms_offset);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_sendfile_ocall(void* pms)
{
	ms_u_sendfile_ocall_t* ms = SGX_CAST(ms_u_sendfile_ocall_t*, pms);
	ms->ms_retval = u_sendfile_ocall(ms->ms_error, ms->ms_out_fd, ms->ms_in_fd, ms->ms_offset, ms->ms_count);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_copy_file_range_ocall(void* pms)
{
	ms_u_copy_file_range_ocall_t* ms = SGX_CAST(ms_u_copy_file_range_ocall_t*, pms);
	ms->ms_retval = u_copy_file_range_ocall(ms->ms_error, ms->ms_fd_in, ms->ms_off_in, ms->ms_fd_out, ms->ms_off_out, ms->ms_len, ms->ms_flags);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_splice_ocall(void* pms)
{
	ms_u_splice_ocall_t* ms = SGX_CAST(ms_u_splice_ocall_t*, pms);
	ms->ms_retval = u_splice_ocall(ms->ms_error, ms->ms_fd_in, ms->ms_off_in, ms->ms_fd_out, ms->ms_off_out, ms->ms_len, ms->ms_flags);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_fcntl_arg0_ocall(void* pms)
{
	ms_u_fcntl_arg0_ocall_t* ms = SGX_CAST(ms_u_fcntl_arg0_ocall_t*, pms);
	ms->ms_retval = u_fcntl_arg0_ocall(ms->ms_error, ms->ms_fd, ms->ms_cmd);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_fcntl_arg1_ocall(void* pms)
{
	ms_u_fcntl_arg1_ocall_t* ms = SGX_CAST(ms_u_fcntl_arg1_ocall_t*, pms);
	ms->ms_retval = u_fcntl_arg1_ocall(ms->ms_error, ms->ms_fd, ms->ms_cmd, ms->ms_arg);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_ioctl_arg0_ocall(void* pms)
{
	ms_u_ioctl_arg0_ocall_t* ms = SGX_CAST(ms_u_ioctl_arg0_ocall_t*, pms);
	ms->ms_retval = u_ioctl_arg0_ocall(ms->ms_error, ms->ms_fd, ms->ms_request);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_ioctl_arg1_ocall(void* pms)
{
	ms_u_ioctl_arg1_ocall_t* ms = SGX_CAST(ms_u_ioctl_arg1_ocall_t*, pms);
	ms->ms_retval = u_ioctl_arg1_ocall(ms->ms_error, ms->ms_fd, ms->ms_request, ms->ms_arg);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_close_ocall(void* pms)
{
	ms_u_close_ocall_t* ms = SGX_CAST(ms_u_close_ocall_t*, pms);
	ms->ms_retval = u_close_ocall(ms->ms_error, ms->ms_fd);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_isatty_ocall(void* pms)
{
	ms_u_isatty_ocall_t* ms = SGX_CAST(ms_u_isatty_ocall_t*, pms);
	ms->ms_retval = u_isatty_ocall(ms->ms_error, ms->ms_fd);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_dup_ocall(void* pms)
{
	ms_u_dup_ocall_t* ms = SGX_CAST(ms_u_dup_ocall_t*, pms);
	ms->ms_retval = u_dup_ocall(ms->ms_error, ms->ms_oldfd);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_eventfd_ocall(void* pms)
{
	ms_u_eventfd_ocall_t* ms = SGX_CAST(ms_u_eventfd_ocall_t*, pms);
	ms->ms_retval = u_eventfd_ocall(ms->ms_error, ms->ms_initval, ms->ms_flags);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_futimens_ocall(void* pms)
{
	ms_u_futimens_ocall_t* ms = SGX_CAST(ms_u_futimens_ocall_t*, pms);
	ms->ms_retval = u_futimens_ocall(ms->ms_error, ms->ms_fd, ms->ms_times);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_malloc_ocall(void* pms)
{
	ms_u_malloc_ocall_t* ms = SGX_CAST(ms_u_malloc_ocall_t*, pms);
	ms->ms_retval = u_malloc_ocall(ms->ms_error, ms->ms_size, ms->ms_align, ms->ms_zeroed);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_free_ocall(void* pms)
{
	ms_u_free_ocall_t* ms = SGX_CAST(ms_u_free_ocall_t*, pms);
	u_free_ocall(ms->ms_p);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_mmap_ocall(void* pms)
{
	ms_u_mmap_ocall_t* ms = SGX_CAST(ms_u_mmap_ocall_t*, pms);
	ms->ms_retval = u_mmap_ocall(ms->ms_error, ms->ms_start, ms->ms_length, ms->ms_prot, ms->ms_flags, ms->ms_fd, ms->ms_offset);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_munmap_ocall(void* pms)
{
	ms_u_munmap_ocall_t* ms = SGX_CAST(ms_u_munmap_ocall_t*, pms);
	ms->ms_retval = u_munmap_ocall(ms->ms_error, ms->ms_start, ms->ms_length);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_msync_ocall(void* pms)
{
	ms_u_msync_ocall_t* ms = SGX_CAST(ms_u_msync_ocall_t*, pms);
	ms->ms_retval = u_msync_ocall(ms->ms_error, ms->ms_addr, ms->ms_length, ms->ms_flags);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_mprotect_ocall(void* pms)
{
	ms_u_mprotect_ocall_t* ms = SGX_CAST(ms_u_mprotect_ocall_t*, pms);
	ms->ms_retval = u_mprotect_ocall(ms->ms_error, ms->ms_addr, ms->ms_length, ms->ms_prot);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_read_hostbuf_ocall(void* pms)
{
	ms_u_read_hostbuf_ocall_t* ms = SGX_CAST(ms_u_read_hostbuf_ocall_t*, pms);
	ms->ms_retval = u_read_hostbuf_ocall(ms->ms_error, ms->ms_host_buf, ms->ms_encl_buf, ms->ms_count);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_write_hostbuf_ocall(void* pms)
{
	ms_u_write_hostbuf_ocall_t* ms = SGX_CAST(ms_u_write_hostbuf_ocall_t*, pms);
	ms->ms_retval = u_write_hostbuf_ocall(ms->ms_error, ms->ms_host_buf, ms->ms_encl_buf, ms->ms_count);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_thread_set_event_ocall(void* pms)
{
	ms_u_thread_set_event_ocall_t* ms = SGX_CAST(ms_u_thread_set_event_ocall_t*, pms);
	ms->ms_retval = u_thread_set_event_ocall(ms->ms_error, ms->ms_tcs);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_thread_wait_event_ocall(void* pms)
{
	ms_u_thread_wait_event_ocall_t* ms = SGX_CAST(ms_u_thread_wait_event_ocall_t*, pms);
	ms->ms_retval = u_thread_wait_event_ocall(ms->ms_error, ms->ms_tcs, ms->ms_timeout, ms->ms_clockid, ms->ms_absolute_time);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_thread_set_multiple_events_ocall(void* pms)
{
	ms_u_thread_set_multiple_events_ocall_t* ms = SGX_CAST(ms_u_thread_set_multiple_events_ocall_t*, pms);
	ms->ms_retval = u_thread_set_multiple_events_ocall(ms->ms_error, ms->ms_tcss, ms->ms_total);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_thread_setwait_events_ocall(void* pms)
{
	ms_u_thread_setwait_events_ocall_t* ms = SGX_CAST(ms_u_thread_setwait_events_ocall_t*, pms);
	ms->ms_retval = u_thread_setwait_events_ocall(ms->ms_error, ms->ms_waiter_tcs, ms->ms_self_tcs, ms->ms_timeout, ms->ms_clockid, ms->ms_absolute_time);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL enclave_u_clock_gettime_ocall(void* pms)
{
	ms_u_clock_gettime_ocall_t* ms = SGX_CAST(ms_u_clock_gettime_ocall_t*, pms);
	ms->ms_retval = u_clock_gettime_ocall(ms->ms_error, ms->ms_clk_id, ms->ms_tp);

	return SGX_SUCCESS;
}

static const struct {
	size_t nr_ocall;
	void * table[29];
} ocall_table_enclave = {
	29,
	{
		(void*)enclave_u_read_ocall,
		(void*)enclave_u_pread64_ocall,
		(void*)enclave_u_write_ocall,
		(void*)enclave_u_pwrite64_ocall,
		(void*)enclave_u_sendfile_ocall,
		(void*)enclave_u_copy_file_range_ocall,
		(void*)enclave_u_splice_ocall,
		(void*)enclave_u_fcntl_arg0_ocall,
		(void*)enclave_u_fcntl_arg1_ocall,
		(void*)enclave_u_ioctl_arg0_ocall,
		(void*)enclave_u_ioctl_arg1_ocall,
		(void*)enclave_u_close_ocall,
		(void*)enclave_u_isatty_ocall,
		(void*)enclave_u_dup_ocall,
		(void*)enclave_u_eventfd_ocall,
		(void*)enclave_u_futimens_ocall,
		(void*)enclave_u_malloc_ocall,
		(void*)enclave_u_free_ocall,
		(void*)enclave_u_mmap_ocall,
		(void*)enclave_u_munmap_ocall,
		(void*)enclave_u_msync_ocall,
		(void*)enclave_u_mprotect_ocall,
		(void*)enclave_u_read_hostbuf_ocall,
		(void*)enclave_u_write_hostbuf_ocall,
		(void*)enclave_u_thread_set_event_ocall,
		(void*)enclave_u_thread_wait_event_ocall,
		(void*)enclave_u_thread_set_multiple_events_ocall,
		(void*)enclave_u_thread_setwait_events_ocall,
		(void*)enclave_u_clock_gettime_ocall,
	}
};
sgx_status_t say_something(sgx_enclave_id_t eid, sgx_status_t* retval, const uint8_t* some_string, size_t len)
{
	sgx_status_t status;
	ms_say_something_t ms;
	ms.ms_some_string = some_string;
	ms.ms_len = len;
	status = sgx_ecall(eid, 0, &ocall_table_enclave, &ms);
	if (status == SGX_SUCCESS && retval) *retval = ms.ms_retval;
	return status;
}

