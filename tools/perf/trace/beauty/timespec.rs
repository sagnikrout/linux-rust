//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/timespec.c
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


// SPDX-License-Identifier: LGPL-2.1
// Copyright (C) 2022, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>

#[no_mangle]
unsafe extern "C" fn syscall_arg__scnprintf_augmented_timespec(arg: *mut syscall_arg, bf: *mut c_char, size: usize) -> usize {
    static size_t syscall_arg__scnprintf_augmented_timespec(struct syscall_arg *arg, char *bf, size_t size)
    {
    struct timespec *ts = (struct timespec *)arg.augmented.args.value;
    return scnprintf(bf, size, "{ .tv_sec: %" PRIu64 ", .tv_nsec: %" PRIu64 " }", ts.tv_sec, ts.tv_nsec);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_timespec(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_timespec(char *bf, size_t size, struct syscall_arg *arg)
    {
    if (arg.augmented.args)
    return syscall_arg__scnprintf_augmented_timespec(arg, bf, size);
    return scnprintf(bf, size, "%#lx", arg.val);
    }
