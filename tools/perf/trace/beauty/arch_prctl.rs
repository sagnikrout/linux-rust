//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/arch_prctl.c
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
// trace/beauty/arch_prctl.c
//
// Copyright (C) 2018, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
//

    static DEFINE_STRARRAY_OFFSET(x86_arch_prctl_codes_1, "ARCH_", x86_arch_prctl_codes_1_offset);
    static DEFINE_STRARRAY_OFFSET(x86_arch_prctl_codes_2, "ARCH_", x86_arch_prctl_codes_2_offset);
    static DEFINE_STRARRAY_OFFSET(x86_arch_prctl_codes_3, "ARCH_", x86_arch_prctl_codes_3_offset);
    static struct strarray *x86_arch_prctl_codes[] = {
    &strarray__x86_arch_prctl_codes_1,
    &strarray__x86_arch_prctl_codes_2,
    &strarray__x86_arch_prctl_codes_3,
    };
    static DEFINE_STRARRAYS(x86_arch_prctl_codes);
#[no_mangle]
unsafe extern "C" fn x86_arch_prctl__scnprintf_code(option: c_int, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t x86_arch_prctl__scnprintf_code(int option, char *bf, size_t size, bool show_prefix)
    {
    return strarrays__scnprintf(&strarrays__x86_arch_prctl_codes, bf, size, "%#x", show_prefix, option);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_x86_arch_prctl_code(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_x86_arch_prctl_code(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut code: c_ulong = arg.val;
    return x86_arch_prctl__scnprintf_code(code, bf, size, arg.show_string_prefix);
    }
