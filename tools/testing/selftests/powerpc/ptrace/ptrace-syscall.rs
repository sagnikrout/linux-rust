//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/ptrace/ptrace-syscall.c
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
// A ptrace test for testing PTRACE_SYSEMU, PTRACE_SETREGS and
// PTRACE_GETREG.  This test basically create a child process that executes
// syscalls and the parent process check if it is being traced appropriated.
//
// This test is heavily based on tools/testing/selftests/x86/ptrace_syscall.c
// test, and it was adapted to run on Powerpc by
// Breno Leitao <leitao@debian.org>
//
// Macro flag: #define _GNU_SOURCE

// Bitness-agnostic defines for user_regs_struct fields.

pub const PTRACE_SYSEMU: c_uint = 0x1d;
    static int nerrs;
#[no_mangle]
unsafe extern "C" fn wait_trap(chld: pid_t) {
    static void wait_trap(pid_t chld)
    {
    siginfo_t si;
    if (waitid(P_PID, chld, &si, WEXITED|WSTOPPED) != 0)
    err(1, "waitid");
    if (si.si_pid != chld)
    errx(1, "got unexpected pid in event\n");
    if (si.si_code != CLD_TRAPPED)
    errx(1, "got unexpected event type %d\n", si.si_code);
    }
#[no_mangle]
unsafe extern "C" fn test_ptrace_syscall_restart() {
    static void test_ptrace_syscall_restart(void)
    {
    int status;
    struct pt_regs regs;
    pid_t chld;
    printf("[RUN]\tptrace-induced syscall restart\n");
    chld = fork();
    if (chld < 0)
    err(1, "fork");
//
// Child process is running 4 syscalls after ptrace.
//
// 1) getpid()
// 2) gettid()
// 3) tgkill() -> Send SIGSTOP
// 4) gettid() -> Where the tests will happen essentially
//
    if (chld == 0) {
    if (ptrace(PTRACE_TRACEME, 0, 0, 0) != 0)
    err(1, "PTRACE_TRACEME");
    let mut pid: pid_t = getpid(), tid = syscall(SYS_gettid);
    printf("\tChild will make one syscall\n");
    syscall(SYS_tgkill, pid, tid, SIGSTOP);
    syscall(SYS_gettid, 10, 11, 12, 13, 14, 15);
    _exit(0);
    }
// Parent process below
// Wait for SIGSTOP sent by tgkill above.
    if (waitpid(chld, &status, 0) != chld || !WIFSTOPPED(status))
    err(1, "waitpid");
    printf("[RUN]\tSYSEMU\n");
    if (ptrace(PTRACE_SYSEMU, chld, 0, 0) != 0)
    err(1, "PTRACE_SYSEMU");
    wait_trap(chld);
    if (ptrace(PTRACE_GETREGS, chld, 0, &regs) != 0)
    err(1, "PTRACE_GETREGS");
//
// Ptrace trapped prior to executing the syscall, thus r3 still has
// the syscall number instead of the sys_gettid() result
//
    if (regs.user_syscall_nr != SYS_gettid ||
    regs.user_arg0 != 10 || regs.user_arg1 != 11 ||
    regs.user_arg2 != 12 || regs.user_arg3 != 13 ||
    regs.user_arg4 != 14 || regs.user_arg5 != 15) {
    printf("[FAIL]\tInitial args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n",
    (unsigned long)regs.user_syscall_nr,
    (unsigned long)regs.user_arg0,
    (unsigned long)regs.user_arg1,
    (unsigned long)regs.user_arg2,
    (unsigned long)regs.user_arg3,
    (unsigned long)regs.user_arg4,
    (unsigned long)regs.user_arg5);
    nerrs++;
    } else {
    printf("[OK]\tInitial nr and args are correct\n"); }
    printf("[RUN]\tRestart the syscall (ip = 0x%lx)\n",
    (unsigned long)regs.user_ip);
//
// Rewind to retry the same syscall again. This will basically test
// the rewind process together with PTRACE_SETREGS and PTRACE_GETREGS.
//
    regs.user_ip -= 4;
    if (ptrace(PTRACE_SETREGS, chld, 0, &regs) != 0)
    err(1, "PTRACE_SETREGS");
    if (ptrace(PTRACE_SYSEMU, chld, 0, 0) != 0)
    err(1, "PTRACE_SYSEMU");
    wait_trap(chld);
    if (ptrace(PTRACE_GETREGS, chld, 0, &regs) != 0)
    err(1, "PTRACE_GETREGS");
    if (regs.user_syscall_nr != SYS_gettid ||
    regs.user_arg0 != 10 || regs.user_arg1 != 11 ||
    regs.user_arg2 != 12 || regs.user_arg3 != 13 ||
    regs.user_arg4 != 14 || regs.user_arg5 != 15) {
    printf("[FAIL]\tRestart nr or args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n",
    (unsigned long)regs.user_syscall_nr,
    (unsigned long)regs.user_arg0,
    (unsigned long)regs.user_arg1,
    (unsigned long)regs.user_arg2,
    (unsigned long)regs.user_arg3,
    (unsigned long)regs.user_arg4,
    (unsigned long)regs.user_arg5);
    nerrs++;
    } else {
    printf("[OK]\tRestarted nr and args are correct\n");
    }
    printf("[RUN]\tChange nr and args and restart the syscall (ip = 0x%lx)\n",
    (unsigned long)regs.user_ip);
//
// Inject a new syscall (getpid) in the same place the previous
// syscall (gettid), rewind and re-execute.
//
    regs.user_syscall_nr = SYS_getpid;
    regs.user_arg0 = 20;
    regs.user_arg1 = 21;
    regs.user_arg2 = 22;
    regs.user_arg3 = 23;
    regs.user_arg4 = 24;
    regs.user_arg5 = 25;
    regs.user_ip -= 4;
    if (ptrace(PTRACE_SETREGS, chld, 0, &regs) != 0)
    err(1, "PTRACE_SETREGS");
    if (ptrace(PTRACE_SYSEMU, chld, 0, 0) != 0)
    err(1, "PTRACE_SYSEMU");
    wait_trap(chld);
    if (ptrace(PTRACE_GETREGS, chld, 0, &regs) != 0)
    err(1, "PTRACE_GETREGS");
// Check that ptrace stopped at the new syscall that was
// injected, and guarantee that it haven't executed, i.e, user_args
// contain the arguments and not the syscall return value, for
// instance.
//
    if (regs.user_syscall_nr != SYS_getpid
    || regs.user_arg0 != 20 || regs.user_arg1 != 21
    || regs.user_arg2 != 22 || regs.user_arg3 != 23
    || regs.user_arg4 != 24 || regs.user_arg5 != 25) {
    printf("[FAIL]\tRestart nr or args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n",
    (unsigned long)regs.user_syscall_nr,
    (unsigned long)regs.user_arg0,
    (unsigned long)regs.user_arg1,
    (unsigned long)regs.user_arg2,
    (unsigned long)regs.user_arg3,
    (unsigned long)regs.user_arg4,
    (unsigned long)regs.user_arg5);
    nerrs++;
    } else {
    printf("[OK]\tReplacement nr and args are correct\n");
    }
    if (ptrace(PTRACE_CONT, chld, 0, 0) != 0)
    err(1, "PTRACE_CONT");
    if (waitpid(chld, &status, 0) != chld)
    err(1, "waitpid");
// Guarantee that the process executed properly, returning 0
    if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) {
    printf("[FAIL]\tChild failed\n");
    nerrs++;
    } else {
    printf("[OK]\tChild exited cleanly\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_syscall() -> c_int {
    int ptrace_syscall(void)
    {
    test_ptrace_syscall_restart();
    return nerrs;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(ptrace_syscall, "ptrace_syscall");
    }
