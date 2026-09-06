//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/d_path.c
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

pub const MAX_PATH_LEN: c_int = 128;
pub const MAX_FILES: c_int = 7;

// sys_close_range is not around for long time, so let's
// make sure we can call it on systems with older glibc
//

pub const __NR_close_range: c_int = 546;

pub const __NR_close_range: c_int = 436;

    static int duration;
    static struct {
    __u32 cnt;
    char paths[MAX_FILES][MAX_PATH_LEN];
    } src;
#[no_mangle]
unsafe extern "C" fn set_pathname(fd: c_int, pid: pid_t) -> c_int {
    static int set_pathname(int fd, pid_t pid)
    {
    char buf[MAX_PATH_LEN];
    snprintf(buf, MAX_PATH_LEN, "/proc/%d/fd/%d", pid, fd);
    return readlink(buf, src.paths[src.cnt++], MAX_PATH_LEN);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_close(fd: c_int) -> c_long {
    static inline long syscall_close(int fd)
    {
    return syscall(__NR_close_range,
    (unsigned int)fd,
    (unsigned int)fd,
    0u);
    }
#[no_mangle]
unsafe extern "C" fn trigger_fstat_events(pid: pid_t) -> c_int {
    static int trigger_fstat_events(pid_t pid)
    {
    let mut sockfd: c_int = -1, procfd = -1, devfd = -1;
    let mut localfd: c_int = -1, indicatorfd = -1;
    int pipefd[2] = { -1, -1 };
    struct stat fileStat;
    let mut ret: c_int = -1;
// unmountable pseudo-filesystems
    if (CHECK(pipe(pipefd) < 0, "trigger", "pipe failed\n"))
    return ret;
// unmountable pseudo-filesystems
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (CHECK(sockfd < 0, "trigger", "socket failed\n"))
    goto out_close;
// mountable pseudo-filesystems
    procfd = open("/proc/self/comm", O_RDONLY);
    if (CHECK(procfd < 0, "trigger", "open /proc/self/comm failed\n"))
    goto out_close;
    devfd = open("/dev/urandom", O_RDONLY);
    if (CHECK(devfd < 0, "trigger", "open /dev/urandom failed\n"))
    goto out_close;
    localfd = open("/tmp/d_path_loadgen.txt", O_CREAT | O_RDONLY, 0644);
    if (CHECK(localfd < 0, "trigger", "open /tmp/d_path_loadgen.txt failed\n"))
    goto out_close;
// bpf_d_path will return path with (deleted)
    remove("/tmp/d_path_loadgen.txt");
    indicatorfd = open("/tmp/", O_PATH);
    if (CHECK(indicatorfd < 0, "trigger", "open /tmp/ failed\n"))
    goto out_close;
    ret = set_pathname(pipefd[0], pid);
    if (CHECK(ret < 0, "trigger", "set_pathname failed for pipe[0]\n"))
    goto out_close;
    ret = set_pathname(pipefd[1], pid);
    if (CHECK(ret < 0, "trigger", "set_pathname failed for pipe[1]\n"))
    goto out_close;
    ret = set_pathname(sockfd, pid);
    if (CHECK(ret < 0, "trigger", "set_pathname failed for socket\n"))
    goto out_close;
    ret = set_pathname(procfd, pid);
    if (CHECK(ret < 0, "trigger", "set_pathname failed for proc\n"))
    goto out_close;
    ret = set_pathname(devfd, pid);
    if (CHECK(ret < 0, "trigger", "set_pathname failed for dev\n"))
    goto out_close;
    ret = set_pathname(localfd, pid);
    if (CHECK(ret < 0, "trigger", "set_pathname failed for file\n"))
    goto out_close;
    ret = set_pathname(indicatorfd, pid);
    if (CHECK(ret < 0, "trigger", "set_pathname failed for dir\n"))
    goto out_close;
// triggers vfs_getattr
    fstat(pipefd[0], &fileStat);
    fstat(pipefd[1], &fileStat);
    fstat(sockfd, &fileStat);
    fstat(procfd, &fileStat);
    fstat(devfd, &fileStat);
    fstat(localfd, &fileStat);
    fstat(indicatorfd, &fileStat);
    out_close:
// sys_close no longer triggers filp_close, but we can
// call sys_close_range instead which still does
//
    syscall_close(pipefd[0]);
    syscall_close(pipefd[1]);
    syscall_close(sockfd);
    syscall_close(procfd);
    syscall_close(devfd);
    syscall_close(localfd);
    syscall_close(indicatorfd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn attach_and_load(skel: *mut test_d_path) {
    static void attach_and_load(struct test_d_path **skel)
    {
    int err;
// skel = test_d_path__open_and_load();
    if (CHECK(!*skel, "setup", "d_path skeleton failed\n"))
    goto cleanup;
    err = test_d_path__attach(*skel);
    if (CHECK(err, "setup", "attach failed: %d\n", err))
    goto cleanup;
    (*skel).bss.my_pid = getpid();
    return;
    cleanup:
    test_d_path__destroy(*skel);
// skel = NULL;
    }
#[no_mangle]
unsafe extern "C" fn test_d_path_basic() {
    static void test_d_path_basic(void)
    {
    struct test_d_path__bss *bss;
    struct test_d_path *skel;
    int err;
    attach_and_load(&skel);
    if (!skel)
    goto cleanup;
    bss = skel.bss;
    err = trigger_fstat_events(bss.my_pid);
    if (err < 0)
    goto cleanup;
    if (CHECK(!bss.called_stat,
    "stat",
    "trampoline for security_inode_getattr was not called\n"))
    goto cleanup;
    if (CHECK(!bss.called_close,
    "close",
    "trampoline for filp_close was not called\n"))
    goto cleanup;
    for (int i = 0; i < MAX_FILES; i++) {
    CHECK(strncmp(src.paths[i], bss.paths_stat[i], MAX_PATH_LEN),
    "check",
    "failed to get stat path[%d]: %s vs %s\n",
    i, src.paths[i], bss.paths_stat[i]);
    CHECK(strncmp(src.paths[i], bss.paths_close[i], MAX_PATH_LEN),
    "check",
    "failed to get close path[%d]: %s vs %s\n",
    i, src.paths[i], bss.paths_close[i]);
// The d_path helper returns size plus NUL char, hence + 1
    CHECK(bss.rets_stat[i] != strlen(bss.paths_stat[i]) + 1,
    "check",
    "failed to match stat return [%d]: %d vs %zd [%s]\n",
    i, bss.rets_stat[i], strlen(bss.paths_stat[i]) + 1,
    bss.paths_stat[i]);
    CHECK(bss.rets_close[i] != strlen(bss.paths_stat[i]) + 1,
    "check",
    "failed to match stat return [%d]: %d vs %zd [%s]\n",
    i, bss.rets_close[i], strlen(bss.paths_close[i]) + 1,
    bss.paths_stat[i]);
    }
    cleanup:
    test_d_path__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_d_path_check_rdonly_mem() {
    static void test_d_path_check_rdonly_mem(void)
    {
    struct test_d_path_check_rdonly_mem *skel;
    skel = test_d_path_check_rdonly_mem__open_and_load();
    ASSERT_ERR_PTR(skel, "unexpected_load_overwriting_rdonly_mem");
    test_d_path_check_rdonly_mem__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_d_path_check_types() {
    static void test_d_path_check_types(void)
    {
    struct test_d_path_check_types *skel;
    skel = test_d_path_check_types__open_and_load();
    ASSERT_ERR_PTR(skel, "unexpected_load_passing_wrong_type");
    test_d_path_check_types__destroy(skel);
    }
// Check if the verifier correctly generates code for
// accessing the memory modified by d_path helper.
//
#[no_mangle]
unsafe extern "C" fn test_d_path_mem_access() {
    static void test_d_path_mem_access(void)
    {
    let mut localfd: c_int = -1;
    char path_template[] = "/dev/shm/d_path_loadgen.XXXXXX";
    struct test_d_path__bss *bss;
    struct test_d_path *skel;
    attach_and_load(&skel);
    if (!skel)
    goto cleanup;
    bss = skel.bss;
    localfd = mkstemp(path_template);
    if (CHECK(localfd < 0, "trigger", "mkstemp failed\n"))
    goto cleanup;
    if (CHECK(fallocate(localfd, 0, 0, 1024) < 0, "trigger", "fallocate failed\n"))
    goto cleanup;
    remove(path_template);
    if (CHECK(!bss.path_match_fallocate, "check",
    "failed to read fallocate path"))
    goto cleanup;
    cleanup:
    syscall_close(localfd);
    test_d_path__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_d_path() {
    void test_d_path(void)
    {
    if (test__start_subtest("basic"))
    test_d_path_basic();
    if (test__start_subtest("check_rdonly_mem"))
    test_d_path_check_rdonly_mem();
    if (test__start_subtest("check_alloc_mem"))
    test_d_path_check_types();
    if (test__start_subtest("check_mem_access"))
    test_d_path_mem_access();
    }
