//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/signal/sig_sc_double_restart.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Test that a syscall does not get restarted twice, handled by trap_norestart()
//
// Based on Al's description, and a test for the bug fixed in this commit:
//
// commit 9a81c16b527528ad307843be5571111aa8d35a80
// Author: Al Viro <viro@zeniv.linux.org.uk>
// Date:   Mon Sep 20 21:48:57 2010 +0100
//
// powerpc: fix double syscall restarts
//
// Make sigreturn zero regs->trap, make do_signal() do the same on all
// paths.  As it is, signal interrupting e.g. read() from fd 512 (==
// ERESTARTSYS) with another signal getting unblocked when the first
// handler finishes will lead to restart one insn earlier than it ought
// to.  Same for multiple signals with in-kernel handlers interrupting
// that sucker at the same time.  Same for multiple signals of any kind
// interrupting that sucker on 64bit...
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn SIGUSR1_handler(sig: c_int) {
    static void SIGUSR1_handler(int sig)
    {
    kill(getpid(), SIGUSR2);
//
// SIGUSR2 is blocked until the handler exits, at which point it will
// be raised again and think there is a restart to be done because the
// pending restarted syscall has 512 (ERESTARTSYS) in r3. The second
// restart will retreat NIP another 4 bytes to fail case branch.
//
    }
#[no_mangle]
unsafe extern "C" fn SIGUSR2_handler(sig: c_int) {
    static void SIGUSR2_handler(int sig)
    {
    }
#[no_mangle]
unsafe extern "C" fn raw_read(fd: c_int, buf: *mut c_void, count: usize) -> isize {
    static ssize_t raw_read(int fd, void *buf, size_t count)
    {
    register long nr asm("r0") = __NR_read;
    register long _fd asm("r3") = fd;
    register void *_buf asm("r4") = buf;
    register size_t _count asm("r5") = count;
    asm volatile(
    "		b	0f		\n"
    "		b	1f		\n"
    "	0:	sc	0		\n"
    "		bns	2f		\n"
    "		neg	%0,%0		\n"
    "		b	2f		\n"
    "	1:				\n"
    "		li	%0,%4		\n"
    "	2:				\n"
    : "+r"(_fd), "+r"(nr), "+r"(_buf), "+r"(_count)
    : "i"(-ENOANO)
    : "memory", "r6", "r7", "r8", "r9", "r10", "r11", "r12", "ctr", "cr0");
    if (_fd < 0) {
    errno = -_fd;
    _fd = -1;
    }
    return _fd;
    }

#[no_mangle]
pub unsafe extern "C" fn test_restart() -> c_int {
    int test_restart(void)
    {
    int pipefd[2];
    pid_t pid;
    char buf[512];
    if (pipe(pipefd) == -1) {
    perror("pipe");
    exit(EXIT_FAILURE);
    }
    pid = fork();
    if (pid == -1) {
    perror("fork");
    exit(EXIT_FAILURE);
    }
    if (pid == 0) { /* Child reads from pipe */
    struct sigaction act;
    int fd;
    memset(&act, 0, sizeof(act));
    sigaddset(&act.sa_mask, SIGUSR2);
    act.sa_handler = SIGUSR1_handler;
    act.sa_flags = SA_RESTART;
    if (sigaction(SIGUSR1, &act, core::ptr::null_mut()) == -1) {
    perror("sigaction");
    exit(EXIT_FAILURE);
    }
    memset(&act, 0, sizeof(act));
    act.sa_handler = SIGUSR2_handler;
    act.sa_flags = SA_RESTART;
    if (sigaction(SIGUSR2, &act, core::ptr::null_mut()) == -1) {
    perror("sigaction");
    exit(EXIT_FAILURE);
    }
// Let's get ERESTARTSYS into r3
    while ((fd = dup(pipefd[0])) != 512) {
    if (fd == -1) {
    perror("dup");
    exit(EXIT_FAILURE);
    }
    }
    if (raw_read(fd, buf, 512) == -1) {
    if (errno == ENOANO) {
    fprintf(stderr, "Double restart moved restart before sc instruction.\n");
    _exit(EXIT_FAILURE);
    }
    perror("read");
    exit(EXIT_FAILURE);
    }
    if (strncmp(buf, DATA, DLEN)) {
    fprintf(stderr, "bad test string %s\n", buf);
    exit(EXIT_FAILURE);
    }
    return 0;
    } else {
    int wstatus;
    usleep(100000);		/* Hack to get reader waiting */
    kill(pid, SIGUSR1);
    usleep(100000);
    if (write(pipefd[1], DATA, DLEN) != DLEN) {
    perror("write");
    exit(EXIT_FAILURE);
    }
    close(pipefd[0]);
    close(pipefd[1]);
    if (wait(&wstatus) == -1) {
    perror("wait");
    exit(EXIT_FAILURE);
    }
    if (!WIFEXITED(wstatus)) {
    fprintf(stderr, "child exited abnormally\n");
    exit(EXIT_FAILURE);
    }
    FAIL_IF(WEXITSTATUS(wstatus) != EXIT_SUCCESS);
    return 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    test_harness_set_timeout(10);
    return test_harness(test_restart, "sig sys restart");
    }
