//! Automatically rewritten from C to Rust
//! Source: samples/seccomp/bpf-direct.c
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
// Seccomp filter example for x86 (32-bit and 64-bit) with BPF macros
//
// Copyright (c) 2012 The Chromium OS Authors <chromium-os-dev@chromium.org>
// Author: Will Drewry <wad@chromium.org>
//
// The code may be used by anyone for any purpose,
// and can serve as a starting point for developing
// applications using prctl(PR_SET_SECCOMP, 2, ...).
//

pub const SUPPORTED_ARCH: c_int = 1;

pub const __USE_GNU: c_int = 1;
pub const _GNU_SOURCE: c_int = 1;

pub const PR_SET_NO_NEW_PRIVS: c_int = 38;

pub const SYS_SECCOMP: c_int = 1;

#[no_mangle]
unsafe extern "C" fn emulator(nr: c_int, info: *mut siginfo_t, void_context: *mut c_void) {
    static void emulator(int nr, siginfo_t *info, void *void_context)
    {
    ucontext_t *ctx = (ucontext_t *)(void_context);
    int syscall;
    char *buf;
    ssize_t bytes;
    size_t len;
    if (info.si_code != SYS_SECCOMP)
    return;
    if (!ctx)
    return;
    syscall = ctx.uc_mcontext.gregs[REG_SYSCALL];
    buf = (char *) ctx.uc_mcontext.gregs[REG_ARG1];
    len = (size_t) ctx.uc_mcontext.gregs[REG_ARG2];
    if (syscall != __NR_write)
    return;
    if (ctx.uc_mcontext.gregs[REG_ARG0] != STDERR_FILENO)
    return;
// Redirect stderr messages to stdout. Doesn't handle EINTR, etc
    ctx.uc_mcontext.gregs[REG_RESULT] = -1;
    if (write(STDOUT_FILENO, "[ERR] ", 6) > 0) {
    bytes = write(STDOUT_FILENO, buf, len);
    ctx.uc_mcontext.gregs[REG_RESULT] = bytes;
    }
    return;
    }
#[no_mangle]
unsafe extern "C" fn install_emulator() -> c_int {
    static int install_emulator(void)
    {
    struct sigaction act;
    sigset_t mask;
    memset(&act, 0, sizeof(act));
    sigemptyset(&mask);
    sigaddset(&mask, SIGSYS);
    act.sa_sigaction = &emulator;
    act.sa_flags = SA_SIGINFO;
    if (sigaction(SIGSYS, &act, core::ptr::null_mut()) < 0) {
    perror("sigaction");
    return -1;
    }
    if (sigprocmask(SIG_UNBLOCK, &mask, core::ptr::null_mut())) {
    perror("sigprocmask");
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn install_filter() -> c_int {
    static int install_filter(void)
    {
    struct sock_filter filter[] = {
// Grab the system call number
    BPF_STMT(BPF_LD+BPF_W+BPF_ABS, syscall_nr),
// Jump table for the allowed syscalls
    BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, __NR_rt_sigreturn, 0, 1),
    BPF_STMT(BPF_RET+BPF_K, SECCOMP_RET_ALLOW),

    BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, __NR_sigreturn, 0, 1),
    BPF_STMT(BPF_RET+BPF_K, SECCOMP_RET_ALLOW),

    BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, __NR_exit_group, 0, 1),
    BPF_STMT(BPF_RET+BPF_K, SECCOMP_RET_ALLOW),
    BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, __NR_exit, 0, 1),
    BPF_STMT(BPF_RET+BPF_K, SECCOMP_RET_ALLOW),
    BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, __NR_read, 1, 0),
    BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, __NR_write, 3, 2),
// Check that read is only using stdin.
    BPF_STMT(BPF_LD+BPF_W+BPF_ABS, syscall_arg(0)),
    BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, STDIN_FILENO, 4, 0),
    BPF_STMT(BPF_RET+BPF_K, SECCOMP_RET_KILL),
// Check that write is only using stdout
    BPF_STMT(BPF_LD+BPF_W+BPF_ABS, syscall_arg(0)),
    BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, STDOUT_FILENO, 1, 0),
// Trap attempts to write to stderr
    BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, STDERR_FILENO, 1, 2),
    BPF_STMT(BPF_RET+BPF_K, SECCOMP_RET_ALLOW),
    BPF_STMT(BPF_RET+BPF_K, SECCOMP_RET_TRAP),
    BPF_STMT(BPF_RET+BPF_K, SECCOMP_RET_KILL),
    };
    struct sock_fprog prog = {
    .len = (unsigned short)(sizeof(filter)/sizeof(filter[0])),
    .filter = filter,
    };
    if (prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0)) {
    perror("prctl(NO_NEW_PRIVS)");
    return 1;
    }
    if (prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &prog)) {
    perror("prctl");
    return 1;
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    char buf[4096];
    let mut bytes: isize = 0;
    if (install_emulator())
    return 1;
    if (install_filter())
    return 1;
    syscall(__NR_write, STDOUT_FILENO,
    payload("OHAI! WHAT IS YOUR NAME? "));
    bytes = syscall(__NR_read, STDIN_FILENO, buf, sizeof(buf));
    syscall(__NR_write, STDOUT_FILENO, payload("HELLO, "));
    syscall(__NR_write, STDOUT_FILENO, buf, bytes);
    syscall(__NR_write, STDERR_FILENO,
    payload("Error message going to STDERR\n"));
    return 0;
    }

//
// This sample is x86-only.  Since kernel samples are compiled with the
// host toolchain, a non-x86 host will result in using only the main()
// below.
//
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return 1;
    }
