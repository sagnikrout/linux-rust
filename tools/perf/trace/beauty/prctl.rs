//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/prctl.c
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
// trace/beauty/prctl.c
//
// Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
//

    DEFINE_STRARRAY(prctl_options, "PR_");
#[no_mangle]
unsafe extern "C" fn prctl__scnprintf_option(option: c_int, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t prctl__scnprintf_option(int option, char *bf, size_t size, bool show_prefix)
    {
    return strarray__scnprintf(&strarray__prctl_options, bf, size, "%d", show_prefix, option);
    }
#[no_mangle]
unsafe extern "C" fn prctl__scnprintf_set_mm(option: c_int, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t prctl__scnprintf_set_mm(int option, char *bf, size_t size, bool show_prefix)
    {
    static DEFINE_STRARRAY(prctl_set_mm_options, "PR_SET_MM_");
    return strarray__scnprintf(&strarray__prctl_set_mm_options, bf, size, "%d", show_prefix, option);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_prctl_arg2(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_prctl_arg2(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut option: c_int = syscall_arg__val(arg, 0);
    if (option == PR_SET_MM)
    return prctl__scnprintf_set_mm(arg.val, bf, size, arg.show_string_prefix);
//
// We still don't grab the contents of pointers on entry or exit,
// so just print them as hex numbers
//
    if (option == PR_SET_NAME)
    return syscall_arg__scnprintf_hex(bf, size, arg);
    return syscall_arg__scnprintf_long(bf, size, arg);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_prctl_arg3(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_prctl_arg3(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut option: c_int = syscall_arg__val(arg, 0);
    if (option == PR_SET_MM)
    return syscall_arg__scnprintf_hex(bf, size, arg);
    return syscall_arg__scnprintf_long(bf, size, arg);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_prctl_option(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_prctl_option(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut option: c_ulong = arg.val;
    enum {
    SPO_ARG2 = (1 << 1),
    SPO_ARG3 = (1 << 2),
    SPO_ARG4 = (1 << 3),
    SPO_ARG5 = (1 << 4),
    SPO_ARG6 = (1 << 5),
    };
    let mut all_but2: u8 = SPO_ARG3 | SPO_ARG4 | SPO_ARG5 | SPO_ARG6;
    let mut all: u8 = SPO_ARG2 | all_but2;
    const u8 masks[] = {
    [PR_GET_DUMPABLE]	 = all,
    [PR_SET_DUMPABLE]	 = all_but2,
    [PR_SET_NAME]		 = all_but2,
    [PR_GET_CHILD_SUBREAPER] = all_but2,
    [PR_SET_CHILD_SUBREAPER] = all_but2,
    [PR_GET_SECUREBITS]	 = all,
    [PR_SET_SECUREBITS]	 = all_but2,
    [PR_SET_MM]		 = SPO_ARG4 | SPO_ARG5 | SPO_ARG6,
    [PR_GET_PDEATHSIG]	 = all,
    [PR_SET_PDEATHSIG]	 = all_but2,
    };
    if (option < ARRAY_SIZE(masks))
    arg.mask |= masks[option];
    return prctl__scnprintf_option(option, bf, size, arg.show_string_prefix);
    }
