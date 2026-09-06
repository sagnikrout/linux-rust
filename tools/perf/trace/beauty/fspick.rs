//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/fspick.c
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
//
// trace/beauty/fspick.c
//
// Copyright (C) 2019, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
//

#[no_mangle]
unsafe extern "C" fn fspick__scnprintf_flags(flags: c_ulong, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t fspick__scnprintf_flags(unsigned long flags, char *bf, size_t size, bool show_prefix)
    {

    static DEFINE_STRARRAY(fspick_flags, "FSPICK_");
    return strarray__scnprintf_flags(&strarray__fspick_flags, bf, size, show_prefix, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_fspick_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_fspick_flags(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut flags: c_ulong = arg.val;
    return fspick__scnprintf_flags(flags, bf, size, arg.show_string_prefix);
    }
