//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/util.c
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
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[no_mangle]
pub unsafe extern "C" fn stack_protections(address: c_ulong) {
    void stack_protections(unsigned long address)
    {
    if (mprotect((void *) address, UM_THREAD_SIZE, PROT_READ | PROT_WRITE) < 0)
    panic("protecting stack failed, errno = %d", errno);
    }
#[no_mangle]
pub unsafe extern "C" fn raw(fd: c_int) -> c_int {
    int raw(int fd)
    {
    struct termios tt;
    int err;
    CATCH_EINTR(err = tcgetattr(fd, &tt));
    if (err < 0)
    return -errno;
    cfmakeraw(&tt);
    CATCH_EINTR(err = tcsetattr(fd, TCSADRAIN, &tt));
    if (err < 0)
    return -errno;
//
// XXX tcsetattr could have applied only some changes
// (and cfmakeraw() is a set of changes)
//
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn setup_machinename(machine_out: *mut c_char) {
    void setup_machinename(char *machine_out)
    {
    struct utsname host;
    uname(&host);

    if (!strcmp(host.machine, "x86_64")) {
    strcpy(machine_out, "i686");
    return;
    }

    if (!strcmp(host.machine, "i686")) {
    strcpy(machine_out, "x86_64");
    return;
    }

    strcpy(machine_out, host.machine);
    }
#[no_mangle]
pub unsafe extern "C" fn setup_hostinfo(buf: *mut c_char, len: c_int) {
    void setup_hostinfo(char *buf, int len)
    {
    struct utsname host;
    uname(&host);
    snprintf(buf, len, "%s %s %s %s %s", host.sysname, host.nodename,
    host.release, host.version, host.machine);
    }
//
// We cannot use glibc's abort(). It makes use of tgkill() which
// has no effect within UML's kernel threads.
// After that glibc would execute an invalid instruction to kill
// the calling process and UML crashes with SIGSEGV.
//
#[no_mangle]
pub unsafe extern "C" fn __attribute__(uml_abort(void: (noreturn))) {
    static inline void __attribute__ ((noreturn)) uml_abort(void)
    {
    sigset_t sig;
    fflush(core::ptr::null_mut());
    if (!sigemptyset(&sig) && !sigaddset(&sig, SIGABRT))
    sigprocmask(SIG_UNBLOCK, &sig, 0);
    for (;;)
    if (kill(getpid(), SIGABRT) < 0)
    exit(127);
    }
#[no_mangle]
pub unsafe extern "C" fn os_getrandom(buf: *mut c_void, len: usize, flags: c_uint) -> isize {
    ssize_t os_getrandom(void *buf, size_t len, unsigned int flags)
    {
    return getrandom(buf, len, flags);
    }
//
// UML helper threads must not handle SIGWINCH/INT/TERM
//
#[no_mangle]
pub unsafe extern "C" fn os_fix_helper_signals() {
    void os_fix_helper_signals(void)
    {
    signal(SIGWINCH, SIG_IGN);
    signal(SIGINT, SIG_DFL);
    signal(SIGTERM, SIG_DFL);
    }
#[no_mangle]
pub unsafe extern "C" fn os_dump_core() {
    void os_dump_core(void)
    {
    int pid;
    signal(SIGSEGV, SIG_DFL);
//
// We are about to SIGTERM this entire process group to ensure that
// nothing is around to run after the kernel exits.  The
// kernel wants to abort, not die through SIGTERM, so we
// ignore it here.
//
    signal(SIGTERM, SIG_IGN);
    kill(0, SIGTERM);
//
// Most of the other processes associated with this UML are
// likely sTopped, so give them a SIGCONT so they see the
// SIGTERM.
//
    kill(0, SIGCONT);
//
// Now, having sent signals to everyone but us, make sure they
// die by ptrace.  Processes can survive what's been done to
// them so far - the mechanism I understand is receiving a
// SIGSEGV and segfaulting immediately upon return.  There is
// always a SIGSEGV pending, and (I'm guessing) signals are
// processed in numeric order so the SIGTERM (signal 15 vs
// SIGSEGV being signal 11) is never handled.
//
// Run a waitpid loop until we get some kind of error.
// Hopefully, it's ECHILD, but there's not a lot we can do if
// it's something else.  Tell os_kill_ptraced_process not to
// wait for the child to report its death because there's
// nothing reasonable to do if that fails.
//
    while ((pid = waitpid(-1, core::ptr::null_mut(), WNOHANG | __WALL)) > 0)
    os_kill_ptraced_process(pid, 0);
    uml_abort();
    }
#[no_mangle]
pub unsafe extern "C" fn um_early_printk(s: *const c_char, n: c_uint) {
    void um_early_printk(const char *s, unsigned int n)
    {
    printf("%.*s", n, s);
    }
    static int quiet_info;
#[no_mangle]
unsafe extern "C" fn quiet_cmd_param(str: *mut c_char, add: *mut c_int) -> int __init {
    static int __init quiet_cmd_param(char *str, int *add)
    {
    quiet_info = 1;
    return 0;
    }
    __uml_setup("quiet", quiet_cmd_param,
    "quiet\n"
    "    Turns off information messages during boot.\n\n");
//
// The os_info/os_warn functions will be called by helper threads. These
// have a very limited stack size and using the libc formatting functions
// may overflow the stack.
// So pull in the kernel vscnprintf and use that instead with a fixed
// on-stack buffer.
//
    int vscnprintf(char *buf, size_t size, const char *fmt, va_list args);
#[no_mangle]
pub unsafe extern "C" fn os_info(fmt: *const c_char, ...) {
    void os_info(const char *fmt, ...)
    {
    char buf[256];
    va_list list;
    int len;
    if (quiet_info)
    return;
    va_start(list, fmt);
    len = vscnprintf(buf, sizeof(buf), fmt, list);
    fwrite(buf, len, 1, stderr);
    va_end(list);
    }
#[no_mangle]
pub unsafe extern "C" fn os_warn(fmt: *const c_char, ...) {
    void os_warn(const char *fmt, ...)
    {
    char buf[256];
    va_list list;
    int len;
    va_start(list, fmt);
    len = vscnprintf(buf, sizeof(buf), fmt, list);
    fwrite(buf, len, 1, stderr);
    va_end(list);
    }
