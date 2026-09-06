//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/cachestat/test_cachestat.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
// Macro flag: #define _GNU_SOURCE

pub const NR_TESTS: c_int = 9;
    static const char * const dev_files[] = {
    "/dev/zero", "/dev/null", "/dev/urandom",
    "/proc/version", "/proc"
    };
#[no_mangle]
pub unsafe extern "C" fn print_cachestat(cs: *mut cachestat) {
    void print_cachestat(struct cachestat *cs)
    {
    ksft_print_msg(
    "Using cachestat: Cached: %llu, Dirty: %llu, Writeback: %llu, Evicted: %llu, Recently Evicted: %llu\n",
    cs.nr_cache, cs.nr_dirty, cs.nr_writeback,
    cs.nr_evicted, cs.nr_recently_evicted);
    }
    enum file_type {
    FILE_MMAP,
    FILE_SHMEM
    };
#[no_mangle]
pub unsafe extern "C" fn write_exactly(fd: c_int, filesize: usize) -> bool {
    bool write_exactly(int fd, size_t filesize)
    {
    let mut random_fd: c_int = open("/dev/urandom", O_RDONLY);
    char *cursor, *data;
    int remained;
    bool ret;
    if (random_fd < 0) {
    ksft_print_msg("Unable to access urandom.\n");
    ret = false;
    goto out;
    }
    data = malloc(filesize);
    if (!data) {
    ksft_print_msg("Unable to allocate data.\n");
    ret = false;
    goto close_random_fd;
    }
    remained = filesize;
    cursor = data;
    while (remained) {
    let mut read_len: isize = read(random_fd, cursor, remained);
    if (read_len <= 0) {
    ksft_print_msg("Unable to read from urandom.\n");
    ret = false;
    goto out_free_data;
    }
    remained -= read_len;
    cursor += read_len;
    }
// write random data to fd
    remained = filesize;
    cursor = data;
    while (remained) {
    let mut write_len: isize = write(fd, cursor, remained);
    if (write_len <= 0) {
    ksft_print_msg("Unable write random data to file.\n");
    ret = false;
    goto out_free_data;
    }
    remained -= write_len;
    cursor += write_len;
    }
    ret = true;
    out_free_data:
    free(data);
    close_random_fd:
    close(random_fd);
    out:
    return ret;
    }
//
// fsync() is implemented via noop_fsync() on tmpfs. This makes the fsync()
// test fail below, so we need to check for test file living on a tmpfs.
//
#[no_mangle]
unsafe extern "C" fn is_on_tmpfs(fd: c_int) -> bool {
    static bool is_on_tmpfs(int fd)
    {
    struct statfs statfs_buf;
    if (fstatfs(fd, &statfs_buf))
    return false;
    return statfs_buf.f_type == TMPFS_MAGIC;
    }
//
// Open/create the file at filename, (optionally) write random data to it
// (exactly num_pages), then test the cachestat syscall on this file.
//
// If test_fsync == true, fsync the file, then check the number of dirty
// pages.
//
    static int test_cachestat(const char *filename, bool write_random, bool create,
    bool test_fsync, unsigned long num_pages,
    int open_flags, mode_t open_mode)
    {
    let mut PS: usize = sysconf(_SC_PAGESIZE);
    let mut filesize: c_int = num_pages * PS;
    let mut ret: c_int = KSFT_PASS;
    long syscall_ret;
    struct cachestat cs;
    let mut cs_range: cachestat_range = { 0, filesize };
    let mut fd: c_int = open(filename, open_flags, open_mode);
    if (fd == -1) {
    ksft_print_msg("Unable to create/open file.\n");
    ret = KSFT_FAIL;
    goto out;
    } else {
    ksft_print_msg("Create/open %s\n", filename);
    }
    if (write_random) {
    if (!write_exactly(fd, filesize)) {
    ksft_print_msg("Unable to access urandom.\n");
    ret = KSFT_FAIL;
    goto out1;
    }
    }
    syscall_ret = syscall(__NR_cachestat, fd, &cs_range, &cs, 0);
    ksft_print_msg("Cachestat call returned %ld\n", syscall_ret);
    if (syscall_ret) {
    ksft_print_msg("Cachestat returned non-zero.\n");
    ret = KSFT_FAIL;
    goto out1;
    } else {
    print_cachestat(&cs);
    if (write_random) {
    if (cs.nr_cache + cs.nr_evicted != num_pages) {
    ksft_print_msg(
    "Total number of cached and evicted pages is off.\n");
    ret = KSFT_FAIL;
    }
    }
    }
    if (test_fsync) {
    if (is_on_tmpfs(fd)) {
    ret = KSFT_SKIP;
    } else if (fsync(fd)) {
    ksft_print_msg("fsync fails.\n");
    ret = KSFT_FAIL;
    } else {
    syscall_ret = syscall(__NR_cachestat, fd, &cs_range, &cs, 0);
    ksft_print_msg("Cachestat call (after fsync) returned %ld\n",
    syscall_ret);
    if (!syscall_ret) {
    print_cachestat(&cs);
    if (cs.nr_dirty) {
    ret = KSFT_FAIL;
    ksft_print_msg(
    "Number of dirty should be zero after fsync.\n");
    }
    } else {
    ksft_print_msg("Cachestat (after fsync) returned non-zero.\n");
    ret = KSFT_FAIL;
    goto out1;
    }
    }
    }
    out1:
    close(fd);
    if (create)
    remove(filename);
    out:
    return ret;
    }
    const char *file_type_str(enum file_type type)
    {
    switch (type) {
    case FILE_SHMEM:
    return "shmem";
    case FILE_MMAP:
    return "mmap";
    default:
    return "unknown";
    }
    }
#[no_mangle]
pub unsafe extern "C" fn run_cachestat_test(type: enum file_type) -> bool {
    bool run_cachestat_test(enum file_type type)
    {
    let mut PS: usize = sysconf(_SC_PAGESIZE);
    size_t filesize = PS * 512 * 2; /* 2 2MB huge pages */
    int syscall_ret;
    let mut compute_len: usize = PS * 512;
    let mut cs_range: cachestat_range = { PS, compute_len };
    char *filename = "tmpshmcstat", *map;
    struct cachestat cs;
    let mut ret: bool = true;
    int fd;
    let mut num_pages: c_ulong = compute_len / PS;
    if (type == FILE_SHMEM)
    fd = shm_open(filename, O_CREAT | O_RDWR, 0600);
    else
    fd = open(filename, O_RDWR | O_CREAT | O_TRUNC, 0666);
    if (fd < 0) {
    ksft_print_msg("Unable to create %s file.\n",
    file_type_str(type));
    ret = false;
    goto out;
    }
    if (ftruncate(fd, filesize)) {
    ksft_print_msg("Unable to truncate %s file.\n",file_type_str(type));
    ret = false;
    goto close_fd;
    }
    switch (type) {
    case FILE_SHMEM:
    if (!write_exactly(fd, filesize)) {
    ksft_print_msg("Unable to write to file.\n");
    ret = false;
    goto close_fd;
    }
    break;
    case FILE_MMAP:
    map = mmap(core::ptr::null_mut(), filesize, PROT_READ | PROT_WRITE,
    MAP_SHARED, fd, 0);
    if (map == MAP_FAILED) {
    ksft_print_msg("mmap failed.\n");
    ret = false;
    goto close_fd;
    }
    for (int i = 0; i < filesize; i++)
    map[i] = 'A';
    break;
    default:
    ksft_print_msg("Unsupported file type.\n");
    ret = false;
    goto close_fd;
    }
    syscall_ret = syscall(__NR_cachestat, fd, &cs_range, &cs, 0);
    if (syscall_ret) {
    ksft_print_msg("Cachestat returned non-zero.\n");
    ret = false;
    goto close_fd;
    } else {
    print_cachestat(&cs);
    if (cs.nr_cache + cs.nr_evicted != num_pages) {
    ksft_print_msg(
    "Total number of cached and evicted pages is off.\n");
    ret = false;
    }
    }
    close_fd:
    shm_unlink(filename);
    out:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int ret;
    ksft_print_header();
    ret = syscall(__NR_cachestat, -1, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    if (ret == -1 && errno == ENOSYS)
    ksft_exit_skip("cachestat syscall not available\n");
    ksft_set_plan(NR_TESTS);
    if (ret == -1 && errno == EBADF) {
    ksft_test_result_pass("bad file descriptor recognized\n");
    ret = 0;
    } else {
    ksft_test_result_fail("bad file descriptor ignored\n");
    ret = 1;
    }
    for (int i = 0; i < 5; i++) {
    const char *dev_filename = dev_files[i];
    if (test_cachestat(dev_filename, false, false, false,
    4, O_RDONLY, 0400) == KSFT_PASS)
    ksft_test_result_pass("cachestat works with %s\n", dev_filename);
    else {
    ksft_test_result_fail("cachestat fails with %s\n", dev_filename);
    ret = 1;
    }
    }
    if (test_cachestat("tmpfilecachestat", true, true,
    false, 4, O_CREAT | O_RDWR, 0600) == KSFT_PASS)
    ksft_test_result_pass("cachestat works with a normal file\n");
    else {
    ksft_test_result_fail("cachestat fails with normal file\n");
    ret = 1;
    }
    switch (test_cachestat("tmpfilecachestat", true, true,
    true, 4, O_CREAT | O_RDWR, 0600)) {
    case KSFT_FAIL:
    ksft_test_result_fail("cachestat fsync fails with normal file\n");
    ret = KSFT_FAIL;
    break;
    case KSFT_PASS:
    ksft_test_result_pass("cachestat fsync works with a normal file\n");
    break;
    case KSFT_SKIP:
    ksft_test_result_skip("tmpfilecachestat is on tmpfs\n");
    break;
    }
    if (run_cachestat_test(FILE_SHMEM))
    ksft_test_result_pass("cachestat works with a shmem file\n");
    else {
    ksft_test_result_fail("cachestat fails with a shmem file\n");
    ret = 1;
    }
    if (run_cachestat_test(FILE_MMAP))
    ksft_test_result_pass("cachestat works with a mmap file\n");
    else {
    ksft_test_result_fail("cachestat fails with a mmap file\n");
    ret = 1;
    }
    return ret;
    }
