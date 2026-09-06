//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/syscall_numbering.c
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
// syscall_numbering.c - test calling the x86-64 kernel with various
// valid and invalid system call numbers.
//
// Copyright (c) 2018 Andrew Lutomirski
//
// Macro flag: #define _GNU_SOURCE

// Common system call numbers
pub const SYS_READ: c_int = 0;
pub const SYS_WRITE: c_int = 1;
pub const SYS_GETPID: c_int = 39;
// x64-only system call numbers
pub const X64_IOCTL: c_int = 16;
pub const X64_READV: c_int = 19;
pub const X64_WRITEV: c_int = 20;
// x32-only system call numbers (without X32_BIT)
pub const X32_IOCTL: c_int = 514;
pub const X32_READV: c_int = 515;
pub const X32_WRITEV: c_int = 516;
pub const X32_BIT: c_uint = 0x40000000;
    static int nullfd = -1;		/* File descriptor for /dev/null */
    static bool with_x32;		/* x32 supported on this kernel? */
    enum ptrace_pass {
    PTP_NOTHING,
    PTP_GETREGS,
    PTP_WRITEBACK,
    PTP_FUZZRET,
    PTP_FUZZHIGH,
    PTP_INTNUM,
    PTP_DONE
    };
    static const char * const ptrace_pass_name[] =
    {
    [PTP_NOTHING]	= "just stop, no data read",
    [PTP_GETREGS]	= "only getregs",
    [PTP_WRITEBACK]	= "getregs, unmodified setregs",
    [PTP_FUZZRET]	= "modifying the default return",
    [PTP_FUZZHIGH]	= "clobbering the top 32 bits",
    [PTP_INTNUM]	= "sign-extending the syscall number",
    };
//
// Shared memory block between tracer and test
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shared {
    pub /: *mut *mut unsigned int nerr; / Total error count,
    pub /: *mut *mut unsigned int indent; / Message indentation level,
    pub ptrace_pass: enum ptrace_pass,
    pub /: *mut *mut bool probing_syscall; / In probe_syscall(),
}

    static volatile struct shared *sh;
#[no_mangle]
pub unsafe extern "C" fn offset() -> c_uint {
    static inline unsigned int offset(void)
    {
    let mut level: c_uint = sh ? sh.indent : 0;
    return 8 + level * 4;
    }

    do {						\
    msg(FAIL, fmt, ## __VA_ARGS__);		\
    sh.nerr++;				\
    } while (0)

    do {						\
    sh.indent = 0;				\
    msg(FAIL, fmt, ## __VA_ARGS__);		\
    msg(SKIP, "Unable to run test\n");	\
    exit(EX_OSERR);				\
    } while (0)
// Sentinel for ptrace-modified return value

//
// Directly invokes the given syscall with nullfd as the first argument
// and the rest zero. Avoids involving glibc wrappers in case they ever
// end up intercepting some system calls for some reason, or modify
// the system call number itself.
//
#[no_mangle]
unsafe extern "C" fn probe_syscall(msb: c_int, lsb: c_int) -> c_longlong {
    static long long probe_syscall(int msb, int lsb)
    {
    register long long arg1 asm("rdi") = nullfd;
    register long long arg2 asm("rsi") = 0;
    register long long arg3 asm("rdx") = 0;
    register long long arg4 asm("r10") = 0;
    register long long arg5 asm("r8")  = 0;
    register long long arg6 asm("r9")  = 0;
    let mut nr: c_longlong = ((long long)msb << 32) | (unsigned int)lsb;
    long long ret;
//
// We pass in an extra copy of the extended system call number
// in %rbx, so we can examine it from the ptrace handler without
// worrying about it being possibly modified. This is to test
// the validity of struct user regs.orig_rax a.k.a.
// struct pt_regs.orig_ax.
//
    sh.probing_syscall = true;
    asm volatile("syscall"
    : "=a" (ret)
    : "a" (nr), "b" (nr),
    "r" (arg1), "r" (arg2), "r" (arg3),
    "r" (arg4), "r" (arg5), "r" (arg6)
    : "rcx", "r11", "memory", "cc");
    sh.probing_syscall = false;
    return ret;
    }
    static const char *syscall_str(int msb, int start, int end)
    {
    static char buf[64];
    let mut type: *const char  const = (start & X32_BIT) ? "x32" : "x64";
    let mut lsb: c_int = start;
//
// Improve readability by stripping the x32 bit, but round
// toward zero so we don't display -1 as -1073741825.
//
    if (lsb < 0)
    lsb |= X32_BIT;
    else
    lsb &= ~X32_BIT;
    if (start == end)
    snprintf(buf, sizeof buf, "%s syscall %d:%d",
    type, msb, lsb);
    else
    snprintf(buf, sizeof buf, "%s syscalls %d:%d..%d",
    type, msb, lsb, lsb + (end-start));
    return buf;
    }
    static unsigned int _check_for(int msb, int start, int end, long long expect,
    const char *expect_str)
    {
    let mut err: c_uint = 0;
    sh.indent++;
    if (start != end)
    sh.indent++;
    for (int nr = start; nr <= end; nr++) {
    let mut ret: c_longlong = probe_syscall(msb, nr);
    if (ret != expect) {
    fail("%s returned %lld, but it should have returned %s\n",
    syscall_str(msb, nr, nr),
    ret, expect_str);
    err++;
    }
    }
    if (start != end)
    sh.indent--;
    if (err) {
    if (start != end)
    fail("%s had %u failure%s\n",
    syscall_str(msb, start, end),
    err, err == 1 ? "s" : "");
    } else {
    ok("%s returned %s as expected\n",
    syscall_str(msb, start, end), expect_str);
    }
    sh.indent--;
    return err;
    }

    _check_for(msb,start,end,expect,#expect)
#[no_mangle]
unsafe extern "C" fn check_zero(msb: c_int, nr: c_int) -> bool {
    static bool check_zero(int msb, int nr)
    {
    return check_for(msb, nr, nr, 0);
    }
#[no_mangle]
unsafe extern "C" fn check_enosys(msb: c_int, nr: c_int) -> bool {
    static bool check_enosys(int msb, int nr)
    {
    return check_for(msb, nr, nr, -ENOSYS);
    }
//
// Anyone diagnosing a failure will want to know whether the kernel
// supports x32. Tell them. This can also be used to conditionalize
// tests based on existence or nonexistence of x32.
//
#[no_mangle]
unsafe extern "C" fn test_x32() -> bool {
    static bool test_x32(void)
    {
    long long ret;
    let mut mypid: pid_t = getpid();
    run("Checking for x32 by calling x32 getpid()\n");
    ret = probe_syscall(0, SYS_GETPID | X32_BIT);
    sh.indent++;
    if (ret == mypid) {
    info("x32 is supported\n");
    with_x32 = true;
    } else if (ret == -ENOSYS) {
    info("x32 is not supported\n");
    with_x32 = false;
    } else {
    fail("x32 getpid() returned %lld, but it should have returned either %lld or -ENOSYS\n", ret, (long long)mypid);
    with_x32 = false;
    }
    sh.indent--;
    return with_x32;
    }
#[no_mangle]
unsafe extern "C" fn test_syscalls_common(msb: c_int) {
    static void test_syscalls_common(int msb)
    {
    let mut pass: enum ptrace_pass = sh.ptrace_pass;
    run("Checking some common syscalls as 64 bit\n");
    check_zero(msb, SYS_READ);
    check_zero(msb, SYS_WRITE);
    run("Checking some 64-bit only syscalls as 64 bit\n");
    check_zero(msb, X64_READV);
    check_zero(msb, X64_WRITEV);
    run("Checking out of range system calls\n");
    check_for(msb, -64, -2, -ENOSYS);
    if (pass >= PTP_FUZZRET)
    check_for(msb, -1, -1, MODIFIED_BY_PTRACE);
    else
    check_for(msb, -1, -1, -ENOSYS);
    check_for(msb, X32_BIT-64, X32_BIT-1, -ENOSYS);
    check_for(msb, -64-X32_BIT, -1-X32_BIT, -ENOSYS);
    check_for(msb, INT_MAX-64, INT_MAX-1, -ENOSYS);
    }
#[no_mangle]
unsafe extern "C" fn test_syscalls_with_x32(msb: c_int) {
    static void test_syscalls_with_x32(int msb)
    {
//
// Syscalls 512-547 are "x32" syscalls.  They are
// intended to be called with the x32 (0x40000000) bit
// set.  Calling them without the x32 bit set is
// nonsense and should not work.
//
    run("Checking x32 syscalls as 64 bit\n");
    check_for(msb, 512, 547, -ENOSYS);
    run("Checking some common syscalls as x32\n");
    check_zero(msb, SYS_READ   | X32_BIT);
    check_zero(msb, SYS_WRITE  | X32_BIT);
    run("Checking some x32 syscalls as x32\n");
    check_zero(msb, X32_READV  | X32_BIT);
    check_zero(msb, X32_WRITEV | X32_BIT);
    run("Checking some 64-bit syscalls as x32\n");
    check_enosys(msb, X64_IOCTL  | X32_BIT);
    check_enosys(msb, X64_READV  | X32_BIT);
    check_enosys(msb, X64_WRITEV | X32_BIT);
    }
#[no_mangle]
unsafe extern "C" fn test_syscalls_without_x32(msb: c_int) {
    static void test_syscalls_without_x32(int msb)
    {
    run("Checking for absence of x32 system calls\n");
    check_for(msb, 0 | X32_BIT, 999 | X32_BIT, -ENOSYS);
    }
#[no_mangle]
unsafe extern "C" fn test_syscall_numbering() {
    static void test_syscall_numbering(void)
    {
    static const int msbs[] = {
    0, 1, -1, X32_BIT-1, X32_BIT, X32_BIT-1, -X32_BIT, INT_MAX,
    INT_MIN, INT_MIN+1
    };
    sh.indent++;
//
// The MSB is supposed to be ignored, so we loop over a few
// to test that out.
//
    for (size_t i = 0; i < ARRAY_SIZE(msbs); i++) {
    let mut msb: c_int = msbs[i];
    run("Checking system calls with msb = %d (0x%x)\n",
    msb, msb);
    sh.indent++;
    test_syscalls_common(msb);
    if (with_x32)
    test_syscalls_with_x32(msb);
    else
    test_syscalls_without_x32(msb);
    sh.indent--;
    }
    sh.indent--;
    }
#[no_mangle]
unsafe extern "C" fn syscall_numbering_tracee() {
    static void syscall_numbering_tracee(void)
    {
    enum ptrace_pass pass;
    if (ptrace(PTRACE_TRACEME, 0, 0, 0)) {
    crit("Failed to request tracing\n");
    return;
    }
    raise(SIGSTOP);
    for (sh.ptrace_pass = pass = PTP_NOTHING; pass < PTP_DONE;
    sh.ptrace_pass = ++pass) {
    run("Running tests under ptrace: %s\n", ptrace_pass_name[pass]);
    test_syscall_numbering();
    }
    }
#[no_mangle]
unsafe extern "C" fn mess_with_syscall(testpid: pid_t, pass: enum ptrace_pass) {
    static void mess_with_syscall(pid_t testpid, enum ptrace_pass pass)
    {
    struct user_regs_struct regs;
    sh.probing_syscall = false; /* Do this on entry only */
// For these, don't even getregs
    if (pass == PTP_NOTHING || pass == PTP_DONE)
    return;
    ptrace(PTRACE_GETREGS, testpid, core::ptr::null_mut(), &regs);
    if (regs.orig_rax != regs.rbx) {
    fail("orig_rax %#llx doesn't match syscall number %#llx\n",
    (unsigned long long)regs.orig_rax,
    (unsigned long long)regs.rbx);
    }
    switch (pass) {
    case PTP_GETREGS:
// Just read, no writeback
    return;
    case PTP_WRITEBACK:
// Write back the same register state verbatim
    break;
    case PTP_FUZZRET:
    regs.rax = MODIFIED_BY_PTRACE;
    break;
    case PTP_FUZZHIGH:
    regs.rax = MODIFIED_BY_PTRACE;
    regs.orig_rax = regs.orig_rax | 0xffffffff00000000ULL;
    break;
    case PTP_INTNUM:
    regs.rax = MODIFIED_BY_PTRACE;
    regs.orig_rax = (int)regs.orig_rax;
    break;
    default:
    crit("invalid ptrace_pass\n");
    break;
    }
    ptrace(PTRACE_SETREGS, testpid, core::ptr::null_mut(), &regs);
    }
#[no_mangle]
unsafe extern "C" fn syscall_numbering_tracer(testpid: pid_t) {
    static void syscall_numbering_tracer(pid_t testpid)
    {
    int wstatus;
    do {
    let mut wpid: pid_t = waitpid(testpid, &wstatus, 0);
    if (wpid < 0 && errno != EINTR)
    break;
    if (wpid != testpid)
    continue;
    if (!WIFSTOPPED(wstatus))
    break;	/* Thread exited? */
    if (sh.probing_syscall && WSTOPSIG(wstatus) == SIGTRAP)
    mess_with_syscall(testpid, sh.ptrace_pass);
    } while (sh.ptrace_pass != PTP_DONE &&
    !ptrace(PTRACE_SYSCALL, testpid, core::ptr::null_mut(), core::ptr::null_mut()));
    ptrace(PTRACE_DETACH, testpid, core::ptr::null_mut(), core::ptr::null_mut());
// Wait for the child process to terminate
    while (waitpid(testpid, &wstatus, 0) != testpid || !WIFEXITED(wstatus))
// wait some more */;
    }
#[no_mangle]
unsafe extern "C" fn test_traced_syscall_numbering() {
    static void test_traced_syscall_numbering(void)
    {
    pid_t testpid;
// Launch the test thread; this thread continues as the tracer thread
    testpid = fork();
    if (testpid < 0) {
    crit("Unable to launch tracer process\n");
    } else if (testpid == 0) {
    syscall_numbering_tracee();
    _exit(0);
    } else {
    syscall_numbering_tracer(testpid);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    unsigned int nerr;
//
// It is quite likely to get a segfault on a failure, so make
// sure the message gets out by setting stdout to nonbuffered.
//
    setvbuf(stdout, core::ptr::null_mut(), _IONBF, 0);
//
// Harmless file descriptor to work on...
//
    nullfd = open("/dev/null", O_RDWR);
    if (nullfd < 0) {
    crit("Unable to open /dev/null: %s\n", strerror(errno));
    }
//
// Set up a block of shared memory...
//
    sh = mmap(core::ptr::null_mut(), sysconf(_SC_PAGE_SIZE), PROT_READ|PROT_WRITE,
    MAP_ANONYMOUS|MAP_SHARED, 0, 0);
    if (sh == MAP_FAILED) {
    crit("Unable to allocated shared memory block: %s\n",
    strerror(errno));
    }
    with_x32 = test_x32();
    run("Running tests without ptrace...\n");
    test_syscall_numbering();
    test_traced_syscall_numbering();
    nerr = sh.nerr;
    if (!nerr) {
    ok("All system calls succeeded or failed as expected\n");
    return 0;
    } else {
    fail("A total of %u system call%s had incorrect behavior\n",
    nerr, nerr != 1 ? "s" : "");
    return 1;
    }
    }
