//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/landlock/wait-pipe.c
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
// Write in a pipe and wait.
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
    int pipe_child, pipe_parent;
    char buf;
// The first argument must be the file descriptor number of a pipe.
    if (argc != 3) {
    fprintf(stderr, "Wrong number of arguments (not two)\n");
    return 1;
    }
    pipe_child = atoi(argv[1]);
    pipe_parent = atoi(argv[2]);
// Signals that we are waiting.
    if (write(pipe_child, ".", 1) != 1) {
    perror("Failed to write to first argument");
    return 1;
    }
// Waits for the parent do its test.
    if (read(pipe_parent, &buf, 1) != 1) {
    perror("Failed to write to the second argument");
    return 1;
    }
    return 0;
    }
