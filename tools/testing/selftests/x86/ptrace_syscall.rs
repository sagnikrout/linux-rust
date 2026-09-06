//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/ptrace_syscall.c
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

// Bitness-agnostic defines for user_regs_struct fields.

    let mut nerrs: static int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_args32 {
    pub arg5: uint32_t nr, arg0, arg1, arg2, arg3, arg4,,
}

    extern void sys32_helper(struct syscall_args32 *, void *);
    extern void int80_and_ret(void);

//
// Helper to invoke int80 with controlled regs and capture the final regs.
//
#[no_mangle]
unsafe extern "C" fn do_full_int80(args: *mut syscall_args32) {
    static void do_full_int80(struct syscall_args32 *args)
    {

    register unsigned long bp asm("bp") = args.arg5;
    asm volatile ("int $0x80"
    : "+a" (args.nr),
    "+b" (args.arg0), "+c" (args.arg1), "+d" (args.arg2),
    "+S" (args.arg3), "+D" (args.arg4), "+r" (bp)
    : : "r8", "r9", "r10", "r11");
    args.arg5 = bp;

    sys32_helper(args, int80_and_ret);

    }

    static void (*vsyscall32)(void);
//
// Nasty helper to invoke AT_SYSINFO (i.e. __kernel_vsyscall) with
// controlled regs and capture the final regs.  This is so nasty that it
// crashes my copy of gdb :)
//
#[no_mangle]
unsafe extern "C" fn do_full_vsyscall32(args: *mut syscall_args32) {
    static void do_full_vsyscall32(struct syscall_args32 *args)
    {
    sys32_helper(args, vsyscall32);
    }

#[no_mangle]
unsafe extern "C" fn wait_trap(chld: pid_t) -> siginfo_t {
    static siginfo_t wait_trap(pid_t chld)
    {
    siginfo_t si;
    if (waitid(P_PID, chld, &si, WEXITED|WSTOPPED) != 0)
    err(1, "waitid");
    if (si.si_pid != chld)
    errx(1, "got unexpected pid in event\n");
    if (si.si_code != CLD_TRAPPED)
    errx(1, "got unexpected event type %d\n", si.si_code);
    return si;
    }
#[no_mangle]
unsafe extern "C" fn setsigign(sig: c_int, flags: c_int) {
    static void setsigign(int sig, int flags)
    {
    struct sigaction sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_sigaction = (void *)SIG_IGN;
    sa.sa_flags = flags;
    sigemptyset(&sa.sa_mask);
    if (sigaction(sig, &sa, 0))
    err(1, "sigaction");
    }

#[no_mangle]
unsafe extern "C" fn empty_handler(sig: c_int, si: *mut siginfo_t, ctx_void: *mut c_void) {
    static void empty_handler(int sig, siginfo_t *si, void *ctx_void)
    {
    }
#[no_mangle]
unsafe extern "C" fn test_sys32_regs(): *mut *mut void (do_syscall)(struct syscall_args32) {
    static void test_sys32_regs(void (*do_syscall)(struct syscall_args32 *))
    {
    struct syscall_args32 args = {
    .nr = 224,	/* gettid */
    .arg0 = 10, .arg1 = 11, .arg2 = 12,
    .arg3 = 13, .arg4 = 14, .arg5 = 15,
    };
    do_syscall(&args);
    if (args.nr != getpid() ||
    args.arg0 != 10 || args.arg1 != 11 || args.arg2 != 12 ||
    args.arg3 != 13 || args.arg4 != 14 || args.arg5 != 15) {
    printf("[FAIL]\tgetpid() failed to preserve regs\n");
    nerrs++;
    } else {
    printf("[OK]\tgetpid() preserves regs\n");
    }
    sethandler(SIGUSR1, empty_handler, 0);
    args.nr = 37;	/* kill */
    args.arg0 = getpid();
    args.arg1 = SIGUSR1;
    do_syscall(&args);
    if (args.nr != 0 ||
    args.arg0 != getpid() || args.arg1 != SIGUSR1 || args.arg2 != 12 ||
    args.arg3 != 13 || args.arg4 != 14 || args.arg5 != 15) {
    printf("[FAIL]\tkill(getpid(), SIGUSR1) failed to preserve regs\n");
    nerrs++;
    } else {
    printf("[OK]\tkill(getpid(), SIGUSR1) preserves regs\n");
    }
    clearhandler(SIGUSR1);
    }
#[no_mangle]
unsafe extern "C" fn test_ptrace_syscall_restart() {
    static void test_ptrace_syscall_restart(void)
    {
    printf("[RUN]\tptrace-induced syscall restart\n");
    let mut chld: pid_t = fork();
    if (chld < 0)
    err(1, "fork");
    if (chld == 0) {
    if (ptrace(PTRACE_TRACEME, 0, 0, 0) != 0)
    err(1, "PTRACE_TRACEME");
    let mut pid: pid_t = getpid(), tid = syscall(SYS_gettid);
    printf("\tChild will make one syscall\n");
    syscall(SYS_tgkill, pid, tid, SIGSTOP);
    syscall(SYS_gettid, 10, 11, 12, 13, 14, 15);
    _exit(0);
    }
    int status;
// Wait for SIGSTOP.
    if (waitpid(chld, &status, 0) != chld || !WIFSTOPPED(status))
    err(1, "waitpid");
    struct user_regs_struct regs;
    printf("[RUN]\tSYSEMU\n");
    if (ptrace(PTRACE_SYSEMU, chld, 0, 0) != 0)
    err(1, "PTRACE_SYSEMU");
    wait_trap(chld);
    if (ptrace(PTRACE_GETREGS, chld, 0, &regs) != 0)
    err(1, "PTRACE_GETREGS");
    if (regs.user_syscall_nr != SYS_gettid ||
    regs.user_arg0 != 10 || regs.user_arg1 != 11 ||
    regs.user_arg2 != 12 || regs.user_arg3 != 13 ||
    regs.user_arg4 != 14 || regs.user_arg5 != 15) {
    printf("[FAIL]\tInitial args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n", (unsigned long)regs.user_syscall_nr, (unsigned long)regs.user_arg0, (unsigned long)regs.user_arg1, (unsigned long)regs.user_arg2, (unsigned long)regs.user_arg3, (unsigned long)regs.user_arg4, (unsigned long)regs.user_arg5);
    nerrs++;
    } else {
    printf("[OK]\tInitial nr and args are correct\n");
    }
    printf("[RUN]\tRestart the syscall (ip = 0x%lx)\n",
    (unsigned long)regs.user_ip);
//
// This does exactly what it appears to do if syscall is int80 or
// SYSCALL64.  For SYSCALL32 or SYSENTER, though, this is highly
// magical.  It needs to work so that ptrace and syscall restart
// work as expected.
//
    regs.user_ax = regs.user_syscall_nr;
    regs.user_ip -= 2;
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
    printf("[FAIL]\tRestart nr or args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n", (unsigned long)regs.user_syscall_nr, (unsigned long)regs.user_arg0, (unsigned long)regs.user_arg1, (unsigned long)regs.user_arg2, (unsigned long)regs.user_arg3, (unsigned long)regs.user_arg4, (unsigned long)regs.user_arg5);
    nerrs++;
    } else {
    printf("[OK]\tRestarted nr and args are correct\n");
    }
    printf("[RUN]\tChange nr and args and restart the syscall (ip = 0x%lx)\n",
    (unsigned long)regs.user_ip);
    regs.user_ax = SYS_getpid;
    regs.user_arg0 = 20;
    regs.user_arg1 = 21;
    regs.user_arg2 = 22;
    regs.user_arg3 = 23;
    regs.user_arg4 = 24;
    regs.user_arg5 = 25;
    regs.user_ip -= 2;
    if (ptrace(PTRACE_SETREGS, chld, 0, &regs) != 0)
    err(1, "PTRACE_SETREGS");
    if (ptrace(PTRACE_SYSEMU, chld, 0, 0) != 0)
    err(1, "PTRACE_SYSEMU");
    wait_trap(chld);
    if (ptrace(PTRACE_GETREGS, chld, 0, &regs) != 0)
    err(1, "PTRACE_GETREGS");
    if (regs.user_syscall_nr != SYS_getpid ||
    regs.user_arg0 != 20 || regs.user_arg1 != 21 || regs.user_arg2 != 22 ||
    regs.user_arg3 != 23 || regs.user_arg4 != 24 || regs.user_arg5 != 25) {
    printf("[FAIL]\tRestart nr or args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n", (unsigned long)regs.user_syscall_nr, (unsigned long)regs.user_arg0, (unsigned long)regs.user_arg1, (unsigned long)regs.user_arg2, (unsigned long)regs.user_arg3, (unsigned long)regs.user_arg4, (unsigned long)regs.user_arg5);
    nerrs++;
    } else {
    printf("[OK]\tReplacement nr and args are correct\n");
    }
    if (ptrace(PTRACE_CONT, chld, 0, 0) != 0)
    err(1, "PTRACE_CONT");
    if (waitpid(chld, &status, 0) != chld)
    err(1, "waitpid");
    if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) {
    printf("[FAIL]\tChild failed\n");
    nerrs++;
    } else {
    printf("[OK]\tChild exited cleanly\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn test_restart_under_ptrace() {
    static void test_restart_under_ptrace(void)
    {
    printf("[RUN]\tkernel syscall restart under ptrace\n");
    let mut chld: pid_t = fork();
    if (chld < 0)
    err(1, "fork");
    if (chld == 0) {
    if (ptrace(PTRACE_TRACEME, 0, 0, 0) != 0)
    err(1, "PTRACE_TRACEME");
    let mut pid: pid_t = getpid(), tid = syscall(SYS_gettid);
    printf("\tChild will take a nap until signaled\n");
    setsigign(SIGUSR1, SA_RESTART);
    syscall(SYS_tgkill, pid, tid, SIGSTOP);
    syscall(SYS_pause, 0, 0, 0, 0, 0, 0);
    _exit(0);
    }
    int status;
// Wait for SIGSTOP.
    if (waitpid(chld, &status, 0) != chld || !WIFSTOPPED(status))
    err(1, "waitpid");
    struct user_regs_struct regs;
    printf("[RUN]\tSYSCALL\n");
    if (ptrace(PTRACE_SYSCALL, chld, 0, 0) != 0)
    err(1, "PTRACE_SYSCALL");
    wait_trap(chld);
// We should be stopped at pause(2) entry.
    if (ptrace(PTRACE_GETREGS, chld, 0, &regs) != 0)
    err(1, "PTRACE_GETREGS");
    if (regs.user_syscall_nr != SYS_pause ||
    regs.user_arg0 != 0 || regs.user_arg1 != 0 ||
    regs.user_arg2 != 0 || regs.user_arg3 != 0 ||
    regs.user_arg4 != 0 || regs.user_arg5 != 0) {
    printf("[FAIL]\tInitial args are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n", (unsigned long)regs.user_syscall_nr, (unsigned long)regs.user_arg0, (unsigned long)regs.user_arg1, (unsigned long)regs.user_arg2, (unsigned long)regs.user_arg3, (unsigned long)regs.user_arg4, (unsigned long)regs.user_arg5);
    nerrs++;
    } else {
    printf("[OK]\tInitial nr and args are correct\n");
    }
// Interrupt it.
    kill(chld, SIGUSR1);
// Advance.  We should be stopped at exit.
    printf("[RUN]\tSYSCALL\n");
    if (ptrace(PTRACE_SYSCALL, chld, 0, 0) != 0)
    err(1, "PTRACE_SYSCALL");
    wait_trap(chld);
    if (ptrace(PTRACE_GETREGS, chld, 0, &regs) != 0)
    err(1, "PTRACE_GETREGS");
    if (regs.user_syscall_nr != SYS_pause ||
    regs.user_arg0 != 0 || regs.user_arg1 != 0 ||
    regs.user_arg2 != 0 || regs.user_arg3 != 0 ||
    regs.user_arg4 != 0 || regs.user_arg5 != 0) {
    printf("[FAIL]\tArgs after SIGUSR1 are wrong (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n", (unsigned long)regs.user_syscall_nr, (unsigned long)regs.user_arg0, (unsigned long)regs.user_arg1, (unsigned long)regs.user_arg2, (unsigned long)regs.user_arg3, (unsigned long)regs.user_arg4, (unsigned long)regs.user_arg5);
    nerrs++;
    } else {
    printf("[OK]\tArgs after SIGUSR1 are correct (ax = %ld)\n",
    (long)regs.user_ax);
    }
// Poke the regs back in.  This must not break anything.
    if (ptrace(PTRACE_SETREGS, chld, 0, &regs) != 0)
    err(1, "PTRACE_SETREGS");
// Catch the (ignored) SIGUSR1.
    if (ptrace(PTRACE_CONT, chld, 0, 0) != 0)
    err(1, "PTRACE_CONT");
    if (waitpid(chld, &status, 0) != chld)
    err(1, "waitpid");
    if (!WIFSTOPPED(status)) {
    printf("[FAIL]\tChild was stopped for SIGUSR1 (status = 0x%x)\n", status);
    nerrs++;
    } else {
    printf("[OK]\tChild got SIGUSR1\n");
    }
// The next event should be pause(2) again.
    printf("[RUN]\tStep again\n");
    if (ptrace(PTRACE_SYSCALL, chld, 0, 0) != 0)
    err(1, "PTRACE_SYSCALL");
    wait_trap(chld);
// We should be stopped at pause(2) entry.
    if (ptrace(PTRACE_GETREGS, chld, 0, &regs) != 0)
    err(1, "PTRACE_GETREGS");
    if (regs.user_syscall_nr != SYS_pause ||
    regs.user_arg0 != 0 || regs.user_arg1 != 0 ||
    regs.user_arg2 != 0 || regs.user_arg3 != 0 ||
    regs.user_arg4 != 0 || regs.user_arg5 != 0) {
    printf("[FAIL]\tpause did not restart (nr=%lu, args=%lu %lu %lu %lu %lu %lu)\n", (unsigned long)regs.user_syscall_nr, (unsigned long)regs.user_arg0, (unsigned long)regs.user_arg1, (unsigned long)regs.user_arg2, (unsigned long)regs.user_arg3, (unsigned long)regs.user_arg4, (unsigned long)regs.user_arg5);
    nerrs++;
    } else {
    printf("[OK]\tpause(2) restarted correctly\n");
    }
// Kill it.
    kill(chld, SIGKILL);
    if (waitpid(chld, &status, 0) != chld)
    err(1, "waitpid");
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main()
    {
    printf("[RUN]\tCheck int80 return regs\n");
    test_sys32_regs(do_full_int80);

    vsyscall32 = (void *)getauxval(AT_SYSINFO);
    if (vsyscall32) {
    printf("[RUN]\tCheck AT_SYSINFO return regs\n");
    test_sys32_regs(do_full_vsyscall32);
    } else {
    printf("[SKIP]\tAT_SYSINFO is not available\n");
    }

    test_ptrace_syscall_restart();
    test_restart_under_ptrace();
    return 0;
    }
