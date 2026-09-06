//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/pidfd/pidfd_wait.c
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

// Attempt to de-conflict with the selftests tree.

    TEST(wait_simple)
    {
    let mut pidfd: c_int = -1;
    let mut parent_tid: pid_t = -1;
    struct __clone_args args = {
    .parent_tid = ptr_to_u64(&parent_tid),
    .pidfd = ptr_to_u64(&pidfd),
    .flags = CLONE_PIDFD | CLONE_PARENT_SETTID,
    .exit_signal = SIGCHLD,
    };
    pid_t pid;
    siginfo_t info = {
    .si_signo = 0,
    };
    pidfd = open("/proc/self", O_DIRECTORY | O_RDONLY | O_CLOEXEC);
    ASSERT_GE(pidfd, 0);
    pid = sys_waitid(P_PIDFD, pidfd, &info, WEXITED);
    ASSERT_NE(pid, 0);
    EXPECT_EQ(close(pidfd), 0);
    pidfd = -1;
    pidfd = open("/dev/null", O_RDONLY | O_CLOEXEC);
    ASSERT_GE(pidfd, 0);
    pid = sys_waitid(P_PIDFD, pidfd, &info, WEXITED);
    ASSERT_NE(pid, 0);
    EXPECT_EQ(close(pidfd), 0);
    pidfd = -1;
    pid = sys_clone3(&args, sizeof(args));
    ASSERT_GE(pid, 0);
    if (pid == 0)
    exit(EXIT_SUCCESS);
    pid = sys_waitid(P_PIDFD, pidfd, &info, WEXITED);
    ASSERT_GE(pid, 0);
    ASSERT_EQ(WIFEXITED(info.si_status), true);
    ASSERT_EQ(WEXITSTATUS(info.si_status), 0);
    EXPECT_EQ(close(pidfd), 0);
    ASSERT_EQ(info.si_signo, SIGCHLD);
    ASSERT_EQ(info.si_code, CLD_EXITED);
    ASSERT_EQ(info.si_pid, parent_tid);
    }
    TEST(wait_states)
    {
    let mut pidfd: c_int = -1;
    let mut parent_tid: pid_t = -1;
    struct __clone_args args = {
    .parent_tid = ptr_to_u64(&parent_tid),
    .pidfd = ptr_to_u64(&pidfd),
    .flags = CLONE_PIDFD | CLONE_PARENT_SETTID,
    .exit_signal = SIGCHLD,
    };
    int pfd[2];
    pid_t pid;
    siginfo_t info = {
    .si_signo = 0,
    };
    ASSERT_EQ(pipe(pfd), 0);
    pid = sys_clone3(&args, sizeof(args));
    ASSERT_GE(pid, 0);
    if (pid == 0) {
    char buf[2];
    close(pfd[1]);
    kill(getpid(), SIGSTOP);
    ASSERT_EQ(read(pfd[0], buf, 1), 1);
    close(pfd[0]);
    kill(getpid(), SIGSTOP);
    exit(EXIT_SUCCESS);
    }
    close(pfd[0]);
    ASSERT_EQ(sys_waitid(P_PIDFD, pidfd, &info, WSTOPPED), 0);
    ASSERT_EQ(info.si_signo, SIGCHLD);
    ASSERT_EQ(info.si_code, CLD_STOPPED);
    ASSERT_EQ(info.si_pid, parent_tid);
    ASSERT_EQ(sys_pidfd_send_signal(pidfd, SIGCONT, core::ptr::null_mut(), 0), 0);
    ASSERT_EQ(sys_waitid(P_PIDFD, pidfd, &info, WCONTINUED), 0);
    ASSERT_EQ(write(pfd[1], "C", 1), 1);
    close(pfd[1]);
    ASSERT_EQ(info.si_signo, SIGCHLD);
    ASSERT_EQ(info.si_code, CLD_CONTINUED);
    ASSERT_EQ(info.si_pid, parent_tid);
    ASSERT_EQ(sys_waitid(P_PIDFD, pidfd, &info, WUNTRACED), 0);
    ASSERT_EQ(info.si_signo, SIGCHLD);
    ASSERT_EQ(info.si_code, CLD_STOPPED);
    ASSERT_EQ(info.si_pid, parent_tid);
    ASSERT_EQ(sys_pidfd_send_signal(pidfd, SIGKILL, core::ptr::null_mut(), 0), 0);
    ASSERT_EQ(sys_waitid(P_PIDFD, pidfd, &info, WEXITED), 0);
    ASSERT_EQ(info.si_signo, SIGCHLD);
    ASSERT_EQ(info.si_code, CLD_KILLED);
    ASSERT_EQ(info.si_pid, parent_tid);
    EXPECT_EQ(close(pidfd), 0);
    }
    TEST(wait_nonblock)
    {
    int pidfd;
    let mut flags: c_uint = 0;
    let mut parent_tid: pid_t = -1;
    struct __clone_args args = {
    .parent_tid = ptr_to_u64(&parent_tid),
    .flags = CLONE_PARENT_SETTID,
    .exit_signal = SIGCHLD,
    };
    int ret;
    pid_t pid;
    siginfo_t info = {
    .si_signo = 0,
    };
//
// Callers need to see ECHILD with non-blocking pidfds when no child
// processes exists.
//
    pidfd = sys_pidfd_open(getpid(), PIDFD_NONBLOCK);
    EXPECT_GE(pidfd, 0) {
// pidfd_open() doesn't support PIDFD_NONBLOCK.
    ASSERT_EQ(errno, EINVAL);
    SKIP(return, "Skipping PIDFD_NONBLOCK test");
    }
    ret = sys_waitid(P_PIDFD, pidfd, &info, WEXITED);
    ASSERT_LT(ret, 0);
    ASSERT_EQ(errno, ECHILD);
    EXPECT_EQ(close(pidfd), 0);
    pid = sys_clone3(&args, sizeof(args));
    ASSERT_GE(pid, 0);
    if (pid == 0) {
    kill(getpid(), SIGSTOP);
    exit(EXIT_SUCCESS);
    }
    pidfd = sys_pidfd_open(pid, PIDFD_NONBLOCK);
    EXPECT_GE(pidfd, 0) {
// pidfd_open() doesn't support PIDFD_NONBLOCK.
    ASSERT_EQ(errno, EINVAL);
    SKIP(return, "Skipping PIDFD_NONBLOCK test");
    }
    flags = fcntl(pidfd, F_GETFL, 0);
    ASSERT_GT(flags, 0);
    ASSERT_GT((flags & O_NONBLOCK), 0);
//
// Callers need to see EAGAIN/EWOULDBLOCK with non-blocking pidfd when
// child processes exist but none have exited.
//
    ret = sys_waitid(P_PIDFD, pidfd, &info, WEXITED);
    ASSERT_LT(ret, 0);
    ASSERT_EQ(errno, EAGAIN);
//
// Callers need to continue seeing 0 with non-blocking pidfd and
// WNOHANG raised explicitly when child processes exist but none have
// exited.
//
    ret = sys_waitid(P_PIDFD, pidfd, &info, WEXITED | WNOHANG);
    ASSERT_EQ(ret, 0);
    ASSERT_EQ(fcntl(pidfd, F_SETFL, (flags & ~O_NONBLOCK)), 0);
    ASSERT_EQ(sys_waitid(P_PIDFD, pidfd, &info, WSTOPPED), 0);
    ASSERT_EQ(info.si_signo, SIGCHLD);
    ASSERT_EQ(info.si_code, CLD_STOPPED);
    ASSERT_EQ(info.si_pid, parent_tid);
    ASSERT_EQ(sys_pidfd_send_signal(pidfd, SIGCONT, core::ptr::null_mut(), 0), 0);
    ASSERT_EQ(sys_waitid(P_PIDFD, pidfd, &info, WEXITED), 0);
    ASSERT_EQ(info.si_signo, SIGCHLD);
    ASSERT_EQ(info.si_code, CLD_EXITED);
    ASSERT_EQ(info.si_pid, parent_tid);
    EXPECT_EQ(close(pidfd), 0);
    }
    TEST_HARNESS_MAIN
