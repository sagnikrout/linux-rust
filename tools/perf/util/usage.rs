//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/usage.c
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
// usage.c
//
// Various reporting routines.
// Originally copied from GIT source.
//
// Copyright (C) Linus Torvalds, 2005
//

    const char perf_usage_string[] =
    "perf [--version] [--help] [OPTIONS] COMMAND [ARGS]";
    const char perf_more_info_string[] =
    "See 'perf help COMMAND' for more information on a specific command.";
#[no_mangle]
unsafe extern "C" fn usage_builtin(err: *const c_char) -> __noreturn void {
    static __noreturn void usage_builtin(const char *err)
    {
    fprintf(stderr, "\n Usage: %s\n", err);
    exit(129);
    }
// If we are in a dlopen()ed .so write to a global variable would segfault
// (ugh), so keep things static.
    static void (*usage_routine)(const char *err) __noreturn = usage_builtin;
#[no_mangle]
pub unsafe extern "C" fn usage(err: *const c_char) {
    void usage(const char *err)
    {
    usage_routine(err);
    }
