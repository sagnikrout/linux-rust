//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/fat/rename_exchange.c
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
// Program that atomically exchanges two paths using
// the renameat2() system call RENAME_EXCHANGE flag.
//
// Copyright 2022 Red Hat Inc.
// Author: Javier Martinez Canillas <javierm@redhat.com>
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn print_usage(program: *const c_char) {
    void print_usage(const char *program)
    {
    printf("Usage: %s [oldpath] [newpath]\n", program);
    printf("Atomically exchange oldpath and newpath\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int ret;
    if (argc != 3) {
    print_usage(argv[0]);
    exit(EXIT_FAILURE);
    }
    ret = renameat2(AT_FDCWD, argv[1], AT_FDCWD, argv[2], RENAME_EXCHANGE);
    if (ret) {
    perror("rename exchange failed");
    exit(EXIT_FAILURE);
    }
    exit(EXIT_SUCCESS);
    }
