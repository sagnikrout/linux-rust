//! Automatically rewritten from C to Rust
//! Source: samples/check-exec/set-exec.c
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Simple tool to set SECBIT_EXEC_RESTRICT_FILE, SECBIT_EXEC_DENY_INTERACTIVE,
// before executing a command.
//
// Copyright © 2024 Microsoft Corporation
//
// Macro flag: #define _GNU_SOURCE
// Macro flag: #define __SANE_USERSPACE_TYPES__

#[no_mangle]
unsafe extern "C" fn print_usage(argv0: *const c_char) {
    static void print_usage(const char *argv0)
    {
    fprintf(stderr, "usage: %s -f|-i -- <cmd> [args]...\n\n", argv0);
    fprintf(stderr, "Execute a command with\n");
    fprintf(stderr, "- SECBIT_EXEC_RESTRICT_FILE set: -f\n");
    fprintf(stderr, "- SECBIT_EXEC_DENY_INTERACTIVE set: -i\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *const *const c_char, envp: *const *const *const c_char) -> c_int {
    int main(const int argc, char *const argv[], char *const *const envp)
    {
    const char *cmd_path;
    char *const *cmd_argv;
    int opt, secbits_cur, secbits_new;
    let mut has_policy: bool = false;
    secbits_cur = prctl(PR_GET_SECUREBITS);
    if (secbits_cur == -1) {
//
// This should never happen, except with a buggy seccomp
// filter.
//
    perror("ERROR: Failed to get securebits");
    return 1;
    }
    secbits_new = secbits_cur;
    while ((opt = getopt(argc, argv, "fi")) != -1) {
    switch (opt) {
    case 'f':
    secbits_new |= SECBIT_EXEC_RESTRICT_FILE |
    SECBIT_EXEC_RESTRICT_FILE_LOCKED;
    has_policy = true;
    break;
    case 'i':
    secbits_new |= SECBIT_EXEC_DENY_INTERACTIVE |
    SECBIT_EXEC_DENY_INTERACTIVE_LOCKED;
    has_policy = true;
    break;
    default:
    print_usage(argv[0]);
    return 1;
    }
    }
    if (!argv[optind] || !has_policy) {
    print_usage(argv[0]);
    return 1;
    }
    if (secbits_cur != secbits_new &&
    prctl(PR_SET_SECUREBITS, secbits_new)) {
    perror("Failed to set secure bit(s).");
    fprintf(stderr,
    "Hint: The running kernel may not support this feature.\n");
    return 1;
    }
    cmd_path = argv[optind];
    cmd_argv = argv + optind;
    fprintf(stderr, "Executing command...\n");
    execvpe(cmd_path, cmd_argv, envp);
    fprintf(stderr, "Failed to execute \"%s\": %s\n", cmd_path,
    strerror(errno));
    return 1;
    }
