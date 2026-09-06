//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/perf_events/remove_on_exec.c
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
//
// Test for remove_on_exec.
//
// Copyright (C) 2021, Google LLC.
//
// Macro flag: #define _GNU_SOURCE
// We need the latest siginfo from the kernel repo.

pub const __have_siginfo_t: c_int = 1;
pub const __have_sigval_t: c_int = 1;
pub const __have_sigevent_t: c_int = 1;
// Macro flag: #define __siginfo_t_defined
// Macro flag: #define __sigval_t_defined
// Macro flag: #define __sigevent_t_defined
pub const _BITS_SIGINFO_CONSTS_H: c_int = 1;
pub const _BITS_SIGEVENT_CONSTS_H: c_int = 1;

    static volatile int signal_count;
#[no_mangle]
unsafe extern "C" fn make_event_attr() -> perf_event_attr {
    static struct perf_event_attr make_event_attr(void)
    {
    struct perf_event_attr attr = {
    .type		= PERF_TYPE_HARDWARE,
    .size		= sizeof(attr),
    .config		= PERF_COUNT_HW_INSTRUCTIONS,
    .sample_period	= 1000,
    .exclude_kernel = 1,
    .exclude_hv	= 1,
    .disabled	= 1,
    .inherit	= 1,
//
// Children normally retain their inherited event on exec; with
// remove_on_exec, we'll remove their event, but the parent and
// any other non-exec'd children will keep their events.
//
    .remove_on_exec = 1,
    .sigtrap	= 1,
    };
    return attr;
    }
#[no_mangle]
unsafe extern "C" fn sigtrap_handler(signum: c_int, info: *mut siginfo_t, ucontext: *mut c_void) {
    static void sigtrap_handler(int signum, siginfo_t *info, void *ucontext)
    {
    if (info.si_code != TRAP_PERF) {
    fprintf(stderr, "%s: unexpected si_code %d\n", __func__, info.si_code);
    return;
    }
    signal_count++;
    }
    FIXTURE(remove_on_exec)
    {
    struct sigaction oldact;
    int fd;
    };
    FIXTURE_SETUP(remove_on_exec)
    {
    let mut attr: perf_event_attr = make_event_attr();
    let mut action: sigaction = {};
    signal_count = 0;
// Initialize sigtrap handler.
    action.sa_flags = SA_SIGINFO | SA_NODEFER;
    action.sa_sigaction = sigtrap_handler;
    sigemptyset(&action.sa_mask);
    ASSERT_EQ(sigaction(SIGTRAP, &action, &self.oldact), 0);
// Initialize perf event.
    self.fd = syscall(__NR_perf_event_open, &attr, 0, -1, -1, PERF_FLAG_FD_CLOEXEC);
    ASSERT_NE(self.fd, -1);
    }
    FIXTURE_TEARDOWN(remove_on_exec)
    {
    close(self.fd);
    sigaction(SIGTRAP, &self.oldact, core::ptr::null_mut());
    }
// Verify event propagates to fork'd child.
    TEST_F(remove_on_exec, fork_only)
    {
    int status;
    let mut pid: pid_t = fork();
    if (pid == 0) {
    ASSERT_EQ(signal_count, 0);
    ASSERT_EQ(ioctl(self.fd, PERF_EVENT_IOC_ENABLE, 0), 0);
    while (!signal_count);
    _exit(42);
    }
    while (!signal_count); /* Child enables event. */
    EXPECT_EQ(waitpid(pid, &status, 0), pid);
    EXPECT_EQ(WEXITSTATUS(status), 42);
    }
//
// Verify that event does _not_ propagate to fork+exec'd child; event enabled
// after fork+exec.
//
    TEST_F(remove_on_exec, fork_exec_then_enable)
    {
    pid_t pid_exec, pid_only_fork;
    int pipefd[2];
    int tmp;
//
// Non-exec child, to ensure exec does not affect inherited events of
// other children.
//
    pid_only_fork = fork();
    if (pid_only_fork == 0) {
// Block until parent enables event.
    while (!signal_count);
    _exit(42);
    }
    ASSERT_NE(pipe(pipefd), -1);
    pid_exec = fork();
    if (pid_exec == 0) {
    ASSERT_NE(dup2(pipefd[1], STDOUT_FILENO), -1);
    close(pipefd[0]);
    execl("/proc/self/exe", "exec_child", core::ptr::null_mut());
    _exit((perror("exec failed"), 1));
    }
    close(pipefd[1]);
    ASSERT_EQ(waitpid(pid_exec, &tmp, WNOHANG), 0); /* Child is running. */
// Wait for exec'd child to start spinning.
    EXPECT_EQ(read(pipefd[0], &tmp, sizeof(int)), sizeof(int));
    EXPECT_EQ(tmp, 42);
    close(pipefd[0]);
// Now we can enable the event, knowing the child is doing work.
    EXPECT_EQ(ioctl(self.fd, PERF_EVENT_IOC_ENABLE, 0), 0);
// If the event propagated to the exec'd child, it will exit normally...
    usleep(100000); /* ... give time for event to trigger (in case of bug). */
    EXPECT_EQ(waitpid(pid_exec, &tmp, WNOHANG), 0); /* Should still be running. */
    EXPECT_EQ(kill(pid_exec, SIGKILL), 0);
// Verify removal from child did not affect this task's event.
    tmp = signal_count;
    while (signal_count == tmp); /* Should not hang! */
// Nor should it have affected the first child.
    EXPECT_EQ(waitpid(pid_only_fork, &tmp, 0), pid_only_fork);
    EXPECT_EQ(WEXITSTATUS(tmp), 42);
    }
//
// Verify that event does _not_ propagate to fork+exec'd child; event enabled
// before fork+exec.
//
    TEST_F(remove_on_exec, enable_then_fork_exec)
    {
    pid_t pid_exec;
    int tmp;
    EXPECT_EQ(ioctl(self.fd, PERF_EVENT_IOC_ENABLE, 0), 0);
    pid_exec = fork();
    if (pid_exec == 0) {
    execl("/proc/self/exe", "exec_child", core::ptr::null_mut());
    _exit((perror("exec failed"), 1));
    }
//
// The child may exit abnormally at any time if the event propagated and
// a SIGTRAP is sent before the handler was set up.
//
    usleep(100000); /* ... give time for event to trigger (in case of bug). */
    EXPECT_EQ(waitpid(pid_exec, &tmp, WNOHANG), 0); /* Should still be running. */
    EXPECT_EQ(kill(pid_exec, SIGKILL), 0);
// Verify removal from child did not affect this task's event.
    tmp = signal_count;
    while (signal_count == tmp); /* Should not hang! */
    }
    TEST_F(remove_on_exec, exec_stress)
    {
    pid_t pids[30];
    int i, tmp;
    for (i = 0; i < sizeof(pids) / sizeof(pids[0]); i++) {
    pids[i] = fork();
    if (pids[i] == 0) {
    execl("/proc/self/exe", "exec_child", core::ptr::null_mut());
    _exit((perror("exec failed"), 1));
    }
// Some forked with event disabled, rest with enabled.
    if (i > 10)
    EXPECT_EQ(ioctl(self.fd, PERF_EVENT_IOC_ENABLE, 0), 0);
    }
    usleep(100000); /* ... give time for event to trigger (in case of bug). */
    for (i = 0; i < sizeof(pids) / sizeof(pids[0]); i++) {
// All children should still be running.
    EXPECT_EQ(waitpid(pids[i], &tmp, WNOHANG), 0);
    EXPECT_EQ(kill(pids[i], SIGKILL), 0);
    }
// Verify event is still alive.
    tmp = signal_count;
    while (signal_count == tmp);
    }
// For exec'd child.
#[no_mangle]
unsafe extern "C" fn exec_child() {
    static void exec_child(void)
    {
    let mut action: sigaction = {};
    let mut val: c_int = 42;
// Set up sigtrap handler in case we erroneously receive a trap.
    action.sa_flags = SA_SIGINFO | SA_NODEFER;
    action.sa_sigaction = sigtrap_handler;
    sigemptyset(&action.sa_mask);
    if (sigaction(SIGTRAP, &action, core::ptr::null_mut()))
    _exit((perror("sigaction failed"), 1));
// Signal parent that we're starting to spin.
    if (write(STDOUT_FILENO, &val, sizeof(int)) == -1)
    _exit((perror("write failed"), 1));
// Should hang here until killed.
    while (!signal_count);
    }

    TEST_HARNESS_MAIN

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    if (!strcmp(argv[0], "exec_child")) {
    exec_child();
    return 1;
    }
    return test_main(argc, argv);
    }
