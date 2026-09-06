//! Automatically rewritten from C to Rust
//! Source: samples/seccomp/dropper.c
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
// Naive system call dropper built on seccomp_filter.
//
// Copyright (c) 2012 The Chromium OS Authors <chromium-os-dev@chromium.org>
// Author: Will Drewry <wad@chromium.org>
//
// The code may be used by anyone for any purpose,
// and can serve as a starting point for developing
// applications using prctl(PR_SET_SECCOMP, 2, ...).
//
// When run, returns the specified errno for the specified
// system call number against the given architecture.
//

#[no_mangle]
unsafe extern "C" fn install_filter(arch: c_int, nr: c_int, error: c_int) -> c_int {
    static int install_filter(int arch, int nr, int error)
    {
    struct sock_filter filter[] = {
    BPF_STMT(BPF_LD+BPF_W+BPF_ABS,
    (offsetof(struct seccomp_data, arch))),
    BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, arch, 0, 3),
    BPF_STMT(BPF_LD+BPF_W+BPF_ABS,
    (offsetof(struct seccomp_data, nr))),
    BPF_JUMP(BPF_JMP+BPF_JEQ+BPF_K, nr, 0, 1),
    BPF_STMT(BPF_RET+BPF_K,
    SECCOMP_RET_ERRNO|(error & SECCOMP_RET_DATA)),
    BPF_STMT(BPF_RET+BPF_K, SECCOMP_RET_ALLOW),
    };
    struct sock_fprog prog = {
    .len = (unsigned short)(sizeof(filter)/sizeof(filter[0])),
    .filter = filter,
    };
    if (error == -1) {
    let mut kill: sock_filter = BPF_STMT(BPF_RET+BPF_K, SECCOMP_RET_KILL);
    filter[4] = kill;
    }
    if (prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0)) {
    perror("prctl(NO_NEW_PRIVS)");
    return 1;
    }
    if (prctl(PR_SET_SECCOMP, 2, &prog)) {
    perror("prctl(PR_SET_SECCOMP)");
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    if (argc < 5) {
    fprintf(stderr, "Usage:\n"
    "dropper <arch> <syscall_nr> <errno> <prog> [<args>]\n"
    "Hint:	AUDIT_ARCH_I386: 0x%X\n"
    "	AUDIT_ARCH_X86_64: 0x%X\n"
    "	errno == -1 means SECCOMP_RET_KILL\n"
    "\n", AUDIT_ARCH_I386, AUDIT_ARCH_X86_64);
    return 1;
    }
    if (install_filter(strtol(argv[1], core::ptr::null_mut(), 0), strtol(argv[2], core::ptr::null_mut(), 0),
    strtol(argv[3], core::ptr::null_mut(), 0)))
    return 1;
    execv(argv[4], &argv[4]);
    printf("Failed to execv\n");
    return 255;
    }
