//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/kcmp/kcmp_test.c
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

#[no_mangle]
unsafe extern "C" fn sys_kcmp(pid1: c_int, pid2: c_int, type: c_int, fd1: c_ulong, fd2: c_ulong) -> c_long {
    static long sys_kcmp(int pid1, int pid2, int type, unsigned long fd1, unsigned long fd2)
    {
    return syscall(__NR_kcmp, pid1, pid2, type, fd1, fd2);
    }
    let mut duped_num: static unsigned int = 64;
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char kpath[] = "kcmp-test-file";
    struct kcmp_epoll_slot epoll_slot;
    struct epoll_event ev;
    int pid1, pid2;
    int pipefd[2];
    int fd1, fd2;
    int epollfd;
    int status;
    int fddup;
    fd1 = open(kpath, O_RDWR | O_CREAT | O_TRUNC, 0644);
    pid1 = getpid();
    if (fd1 < 0) {
    perror("Can't create file");
    ksft_exit_fail();
    }
    if (pipe(pipefd)) {
    perror("Can't create pipe");
    ksft_exit_fail();
    }
    epollfd = epoll_create1(0);
    if (epollfd < 0) {
    perror("epoll_create1 failed");
    ksft_exit_fail();
    }
    memset(&ev, 0xff, sizeof(ev));
    ev.events = EPOLLIN | EPOLLOUT;
    if (epoll_ctl(epollfd, EPOLL_CTL_ADD, pipefd[0], &ev)) {
    perror("epoll_ctl failed");
    ksft_exit_fail();
    }
    fddup = dup2(pipefd[1], duped_num);
    if (fddup < 0) {
    perror("dup2 failed");
    ksft_exit_fail();
    }
    if (epoll_ctl(epollfd, EPOLL_CTL_ADD, fddup, &ev)) {
    perror("epoll_ctl failed");
    ksft_exit_fail();
    }
    close(fddup);
    pid2 = fork();
    if (pid2 < 0) {
    perror("fork failed");
    ksft_exit_fail();
    }
    if (!pid2) {
    let mut pid2: c_int = getpid();
    int ret;
    ksft_print_header();
    ksft_set_plan(3);
    fd2 = open(kpath, O_RDWR);
    if (fd2 < 0) {
    perror("Can't open file");
    ksft_exit_fail();
    }
// An example of output and arguments
    printf("pid1: %6d pid2: %6d FD: %2ld FILES: %2ld VM: %2ld "
    "FS: %2ld SIGHAND: %2ld IO: %2ld SYSVSEM: %2ld "
    "INV: %2ld\n",
    pid1, pid2,
    sys_kcmp(pid1, pid2, KCMP_FILE,		fd1, fd2),
    sys_kcmp(pid1, pid2, KCMP_FILES,		0, 0),
    sys_kcmp(pid1, pid2, KCMP_VM,		0, 0),
    sys_kcmp(pid1, pid2, KCMP_FS,		0, 0),
    sys_kcmp(pid1, pid2, KCMP_SIGHAND,	0, 0),
    sys_kcmp(pid1, pid2, KCMP_IO,		0, 0),
    sys_kcmp(pid1, pid2, KCMP_SYSVSEM,	0, 0),
// This one should fail
    sys_kcmp(pid1, pid2, KCMP_TYPES + 1,	0, 0));
// This one should return same fd
    ret = sys_kcmp(pid1, pid2, KCMP_FILE, fd1, fd1);
    if (ret) {
    printf("FAIL: 0 expected but %d returned (%s)\n",
    ret, strerror(errno));
    ksft_inc_fail_cnt();
    ret = -1;
    } else {
    printf("PASS: 0 returned as expected\n");
    ksft_inc_pass_cnt();
    }
// Compare with self
    ret = sys_kcmp(pid1, pid1, KCMP_VM, 0, 0);
    if (ret) {
    printf("FAIL: 0 expected but %d returned (%s)\n",
    ret, strerror(errno));
    ksft_inc_fail_cnt();
    ret = -1;
    } else {
    printf("PASS: 0 returned as expected\n");
    ksft_inc_pass_cnt();
    }
// Compare epoll target
    epoll_slot = (struct kcmp_epoll_slot) {
    .efd	= epollfd,
    .tfd	= duped_num,
    .toff	= 0,
    };
    ret = sys_kcmp(pid1, pid1, KCMP_EPOLL_TFD, pipefd[1],
    (unsigned long)(void *)&epoll_slot);
    if (ret) {
    printf("FAIL: 0 expected but %d returned (%s)\n",
    ret, strerror(errno));
    ksft_inc_fail_cnt();
    ret = -1;
    } else {
    printf("PASS: 0 returned as expected\n");
    ksft_inc_pass_cnt();
    }
    if (ret)
    ksft_exit_fail();
    else
    ksft_exit_pass();
    }
    waitpid(pid2, &status, P_ALL);
    return 0;
    }
