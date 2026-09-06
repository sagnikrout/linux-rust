//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/ptrace/set_syscall_info.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2018-2025 Dmitry V. Levin <ldv@strace.io>
// All rights reserved.
//
// Check whether PTRACE_SET_SYSCALL_INFO semantics implemented in the kernel
// matches userspace expectations.
//

//
// MIPS N32 is the only architecture where __kernel_ulong_t
// does not match the bitness of syscall arguments.
//
    typedef unsigned long long kernel_ulong_t;

    typedef __kernel_ulong_t kernel_ulong_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_entry {
    pub nr: c_int,
    pub args: [kernel_ulong_t; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_exit {
    pub is_error: c_uint,
    pub rval: c_int,
}

    static unsigned int ptrace_stop;
    static pid_t tracee_pid;
    static int
    kill_tracee(pid_t pid)
    {
    if (!pid)
    return 0;
    let mut saved_errno: c_int = errno;
    let mut rc: c_int = kill(pid, SIGKILL);
    errno = saved_errno;
    return rc;
    }
    static long
    sys_ptrace(int request, pid_t pid, unsigned long addr, unsigned long data)
    {
    return syscall(__NR_ptrace, request, pid, addr, data);
    }

    do {							\
    kill_tracee(tracee_pid);			\
    TH_LOG("wait #%d: " fmt,			\
    ptrace_stop, ##__VA_ARGS__);		\
    } while (0)
    static void
    check_psi_entry(struct __test_metadata *_metadata,
    const struct ptrace_syscall_info *info,
    const struct si_entry *exp_entry,
    const char *text)
    {
    unsigned int i;
    let mut exp_nr: c_int = exp_entry.nr;

// s390 is the only architecture that has 16-bit syscall numbers
    exp_nr &= 0xffff;

    ASSERT_EQ(PTRACE_SYSCALL_INFO_ENTRY, info.op) {
    LOG_KILL_TRACEE("%s: entry stop mismatch", text);
    }
    ASSERT_TRUE(info.arch) {
    LOG_KILL_TRACEE("%s: entry stop mismatch", text);
    }
    ASSERT_TRUE(info.instruction_pointer) {
    LOG_KILL_TRACEE("%s: entry stop mismatch", text);
    }
    ASSERT_TRUE(info.stack_pointer) {
    LOG_KILL_TRACEE("%s: entry stop mismatch", text);
    }
    ASSERT_EQ(exp_nr, info.entry.nr) {
    LOG_KILL_TRACEE("%s: syscall nr mismatch", text);
    }
    for (i = 0; i < ARRAY_SIZE(exp_entry.args); ++i) {
    ASSERT_EQ(exp_entry.args[i], info.entry.args[i]) {
    LOG_KILL_TRACEE("%s: syscall arg #%u mismatch",
    text, i);
    }
    }
    }
    static void
    check_psi_exit(struct __test_metadata *_metadata,
    const struct ptrace_syscall_info *info,
    const struct si_exit *exp_exit,
    const char *text)
    {
    ASSERT_EQ(PTRACE_SYSCALL_INFO_EXIT, info.op) {
    LOG_KILL_TRACEE("%s: exit stop mismatch", text);
    }
    ASSERT_TRUE(info.arch) {
    LOG_KILL_TRACEE("%s: exit stop mismatch", text);
    }
    ASSERT_TRUE(info.instruction_pointer) {
    LOG_KILL_TRACEE("%s: exit stop mismatch", text);
    }
    ASSERT_TRUE(info.stack_pointer) {
    LOG_KILL_TRACEE("%s: exit stop mismatch", text);
    }
    ASSERT_EQ(exp_exit.is_error, info.exit.is_error) {
    LOG_KILL_TRACEE("%s: exit stop mismatch", text);
    }
    ASSERT_EQ(exp_exit.rval, info.exit.rval) {
    LOG_KILL_TRACEE("%s: exit stop mismatch", text);
    }
    }
    TEST(set_syscall_info)
    {
    let mut tracer_pid: pid_t = getpid();
    const kernel_ulong_t dummy[] = {
    (kernel_ulong_t) 0xdad0bef0bad0fed0ULL,
    (kernel_ulong_t) 0xdad1bef1bad1fed1ULL,
    (kernel_ulong_t) 0xdad2bef2bad2fed2ULL,
    (kernel_ulong_t) 0xdad3bef3bad3fed3ULL,
    (kernel_ulong_t) 0xdad4bef4bad4fed4ULL,
    (kernel_ulong_t) 0xdad5bef5bad5fed5ULL,
    };
    int splice_in[2], splice_out[2];
    ASSERT_EQ(0, pipe(splice_in));
    ASSERT_EQ(0, pipe(splice_out));
    ASSERT_EQ(sizeof(dummy), write(splice_in[1], dummy, sizeof(dummy)));
    const struct {
    struct si_entry entry[2];
    struct si_exit exit[2];
    } si[] = {
// change scno, keep non-error rval
    {
    {
    {
    __NR_gettid,
    {
    dummy[0], dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }, {
    __NR_getppid,
    {
    dummy[0], dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }
    }, {
    { 0, tracer_pid }, { 0, tracer_pid }
    }
    },
// set scno to -1, keep error rval
    {
    {
    {
    __NR_chdir,
    {
    (uintptr_t) ".",
    dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }, {
    -1,
    {
    (uintptr_t) ".",
    dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }
    }, {
    { 1, -ENOSYS }, { 1, -ENOSYS }
    }
    },
// keep scno, change non-error rval
    {
    {
    {
    __NR_getppid,
    {
    dummy[0], dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }, {
    __NR_getppid,
    {
    dummy[0], dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }
    }, {
    { 0, tracer_pid }, { 0, tracer_pid + 1 }
    }
    },
// change arg1, keep non-error rval
    {
    {
    {
    __NR_chdir,
    {
    (uintptr_t) "",
    dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }, {
    __NR_chdir,
    {
    (uintptr_t) ".",
    dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }
    }, {
    { 0, 0 }, { 0, 0 }
    }
    },
// set scno to -1, change error rval to non-error
    {
    {
    {
    __NR_gettid,
    {
    dummy[0], dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }, {
    -1,
    {
    dummy[0], dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }
    }, {
    { 1, -ENOSYS }, { 0, tracer_pid }
    }
    },
// change scno, change non-error rval to error
    {
    {
    {
    __NR_chdir,
    {
    dummy[0], dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }, {
    __NR_getppid,
    {
    dummy[0], dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }
    }, {
    { 0, tracer_pid }, { 1, -EISDIR }
    }
    },
// change scno and all args, change non-error rval
    {
    {
    {
    __NR_gettid,
    {
    dummy[0], dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }, {
    __NR_splice,
    {
    splice_in[0], 0, splice_out[1], 0,
    sizeof(dummy), SPLICE_F_NONBLOCK
    }
    }
    }, {
    { 0, sizeof(dummy) }, { 0, sizeof(dummy) + 1 }
    }
    },
// change arg1, no exit stop
    {
    {
    {
    __NR_exit_group,
    {
    dummy[0], dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }, {
    __NR_exit_group,
    {
    0, dummy[1], dummy[2],
    dummy[3], dummy[4], dummy[5]
    }
    }
    }, {
    { 0, 0 }, { 0, 0 }
    }
    },
    };
    long rc;
    unsigned int i;
    tracee_pid = fork();
    ASSERT_LE(0, tracee_pid) {
    TH_LOG("fork: %m");
    }
    if (tracee_pid == 0) {
// get the pid before PTRACE_TRACEME
    tracee_pid = getpid();
    ASSERT_EQ(0, sys_ptrace(PTRACE_TRACEME, 0, 0, 0)) {
    TH_LOG("PTRACE_TRACEME: %m");
    }
    ASSERT_EQ(0, kill(tracee_pid, SIGSTOP)) {
// cannot happen
    TH_LOG("kill SIGSTOP: %m");
    }
    for (i = 0; i < ARRAY_SIZE(si); ++i) {
    rc = syscall(si[i].entry[0].nr,
    si[i].entry[0].args[0],
    si[i].entry[0].args[1],
    si[i].entry[0].args[2],
    si[i].entry[0].args[3],
    si[i].entry[0].args[4],
    si[i].entry[0].args[5]);
    if (si[i].exit[1].is_error) {
    if (rc != -1 || errno != -si[i].exit[1].rval)
    break;
    } else {
    if (rc != si[i].exit[1].rval)
    break;
    }
    }
//
// Something went wrong, but in this state tracee
// cannot reliably issue syscalls, so just crash.
//
// (volatile unsigned char *) (uintptr_t) i = 42;
// unreachable
    _exit(i + 1);
    }
    for (ptrace_stop = 0; ; ++ptrace_stop) {
    struct ptrace_syscall_info info = {
    .op = 0xff	/* invalid PTRACE_SYSCALL_INFO_* op */
    };
    let mut size: usize = sizeof(info);
    const int expected_entry_size =
    (void *) &info.entry.args[6] - (void *) &info;
    const int expected_exit_size =
    (void *) (&info.exit.is_error + 1) -
    (void *) &info;
    int status;
    ASSERT_EQ(tracee_pid, wait(&status)) {
// cannot happen
    LOG_KILL_TRACEE("wait: %m");
    }
    if (WIFEXITED(status)) {
    tracee_pid = 0;	/* the tracee is no more */
    ASSERT_EQ(0, WEXITSTATUS(status)) {
    LOG_KILL_TRACEE("unexpected exit status %u",
    WEXITSTATUS(status));
    }
    break;
    }
    ASSERT_FALSE(WIFSIGNALED(status)) {
    tracee_pid = 0;	/* the tracee is no more */
    LOG_KILL_TRACEE("unexpected signal %u",
    WTERMSIG(status));
    }
    ASSERT_TRUE(WIFSTOPPED(status)) {
// cannot happen
    LOG_KILL_TRACEE("unexpected wait status %#x", status);
    }
    ASSERT_LT(ptrace_stop, ARRAY_SIZE(si) * 2) {
    LOG_KILL_TRACEE("ptrace stop overflow");
    }
    switch (WSTOPSIG(status)) {
    case SIGSTOP:
    ASSERT_EQ(0, ptrace_stop) {
    LOG_KILL_TRACEE("unexpected signal stop");
    }
    ASSERT_EQ(0, sys_ptrace(PTRACE_SETOPTIONS, tracee_pid,
    0, PTRACE_O_TRACESYSGOOD)) {
    LOG_KILL_TRACEE("PTRACE_SETOPTIONS: %m");
    }
    break;
    case SIGTRAP | 0x80:
    ASSERT_LT(0, ptrace_stop) {
    LOG_KILL_TRACEE("unexpected syscall stop");
    }
    ASSERT_LT(0, (rc = sys_ptrace(PTRACE_GET_SYSCALL_INFO,
    tracee_pid, size,
    (uintptr_t) &info))) {
    LOG_KILL_TRACEE("PTRACE_GET_SYSCALL_INFO #1: %m");
    }
    if (ptrace_stop & 1) {
// entering syscall
    const struct si_entry *exp_entry =
    &si[ptrace_stop / 2].entry[0];
    const struct si_entry *set_entry =
    &si[ptrace_stop / 2].entry[1];
// check ptrace_syscall_info before the changes
    ASSERT_EQ(expected_entry_size, rc) {
    LOG_KILL_TRACEE("PTRACE_GET_SYSCALL_INFO #1"
    ": entry stop mismatch");
    }
    check_psi_entry(_metadata, &info, exp_entry,
    "PTRACE_GET_SYSCALL_INFO #1");
// apply the changes
    info.entry.nr = set_entry.nr;
    for (i = 0; i < ARRAY_SIZE(set_entry.args); ++i)
    info.entry.args[i] = set_entry.args[i];
    ASSERT_EQ(0, sys_ptrace(PTRACE_SET_SYSCALL_INFO,
    tracee_pid, size,
    (uintptr_t) &info)) {
    LOG_KILL_TRACEE("PTRACE_SET_SYSCALL_INFO: %m");
    }
// check ptrace_syscall_info after the changes
    memset(&info, 0, sizeof(info));
    info.op = 0xff;
    ASSERT_LT(0, (rc = sys_ptrace(PTRACE_GET_SYSCALL_INFO,
    tracee_pid, size,
    (uintptr_t) &info))) {
    LOG_KILL_TRACEE("PTRACE_GET_SYSCALL_INFO: %m");
    }
    ASSERT_EQ(expected_entry_size, rc) {
    LOG_KILL_TRACEE("PTRACE_GET_SYSCALL_INFO #2"
    ": entry stop mismatch");
    }
    check_psi_entry(_metadata, &info, set_entry,
    "PTRACE_GET_SYSCALL_INFO #2");
    } else {
// exiting syscall
    const struct si_exit *exp_exit =
    &si[ptrace_stop / 2 - 1].exit[0];
    const struct si_exit *set_exit =
    &si[ptrace_stop / 2 - 1].exit[1];
// check ptrace_syscall_info before the changes
    ASSERT_EQ(expected_exit_size, rc) {
    LOG_KILL_TRACEE("PTRACE_GET_SYSCALL_INFO #1"
    ": exit stop mismatch");
    }
    check_psi_exit(_metadata, &info, exp_exit,
    "PTRACE_GET_SYSCALL_INFO #1");
// apply the changes
    info.exit.is_error = set_exit.is_error;
    info.exit.rval = set_exit.rval;
    ASSERT_EQ(0, sys_ptrace(PTRACE_SET_SYSCALL_INFO,
    tracee_pid, size,
    (uintptr_t) &info)) {
    LOG_KILL_TRACEE("PTRACE_SET_SYSCALL_INFO: %m");
    }
// check ptrace_syscall_info after the changes
    memset(&info, 0, sizeof(info));
    info.op = 0xff;
    ASSERT_LT(0, (rc = sys_ptrace(PTRACE_GET_SYSCALL_INFO,
    tracee_pid, size,
    (uintptr_t) &info))) {
    LOG_KILL_TRACEE("PTRACE_GET_SYSCALL_INFO #2: %m");
    }
    ASSERT_EQ(expected_exit_size, rc) {
    LOG_KILL_TRACEE("PTRACE_GET_SYSCALL_INFO #2"
    ": exit stop mismatch");
    }
    check_psi_exit(_metadata, &info, set_exit,
    "PTRACE_GET_SYSCALL_INFO #2");
    }
    break;
    default:
    LOG_KILL_TRACEE("unexpected stop signal %u",
    WSTOPSIG(status));
    abort();
    }
    ASSERT_EQ(0, sys_ptrace(PTRACE_SYSCALL, tracee_pid, 0, 0)) {
    LOG_KILL_TRACEE("PTRACE_SYSCALL: %m");
    }
    }
    ASSERT_EQ(ptrace_stop, ARRAY_SIZE(si) * 2);
    }
    TEST_HARNESS_MAIN
