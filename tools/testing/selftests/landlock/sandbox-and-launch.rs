//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/landlock/sandbox-and-launch.c
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
// Sandbox itself and execute another program (in a different mount point).
//
// Used by layout1.umount_sandboxer from fs_test.c
//
// Copyright © 2024-2025 Microsoft Corporation
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    struct landlock_ruleset_attr ruleset_attr = {
    .scoped = LANDLOCK_SCOPE_SIGNAL,
    };
    int pipe_child, pipe_parent, ruleset_fd;
    char buf;
//
// The first argument must be the file descriptor number of a pipe.
// The second argument must be the program to execute.
//
    if (argc != 4) {
    fprintf(stderr, "Wrong number of arguments (not three)\n");
    return 1;
    }
    pipe_child = atoi(argv[2]);
    pipe_parent = atoi(argv[3]);
    ruleset_fd =
    landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
    if (ruleset_fd < 0) {
    perror("Failed to create ruleset");
    return 1;
    }
    if (prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0)) {
    perror("Failed to call prctl()");
    return 1;
    }
    if (landlock_restrict_self(ruleset_fd, 0)) {
    perror("Failed to restrict self");
    return 1;
    }
    if (close(ruleset_fd)) {
    perror("Failed to close ruleset");
    return 1;
    }
// Signals that we are sandboxed.
    errno = 0;
    if (write(pipe_child, ".", 1) != 1) {
    perror("Failed to write to the second argument");
    return 1;
    }
// Waits for the parent to try to umount.
    if (read(pipe_parent, &buf, 1) != 1) {
    perror("Failed to write to the third argument");
    return 1;
    }
// Shifts arguments.
    argv[0] = argv[1];
    argv[1] = argv[2];
    argv[2] = argv[3];
    argv[3] = core::ptr::null_mut();
    execve(argv[0], argv, core::ptr::null_mut());
    perror("Failed to execute the provided binary");
    return 1;
    }
