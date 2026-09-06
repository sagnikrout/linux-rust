//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/pkey_alloc.c
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
// trace/beauty/pkey_alloc.c
//
// Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
//

#[no_mangle]
pub unsafe extern "C" fn strarray__scnprintf_flags(sa: *mut strarray, bf: *mut c_char, size: usize, show_prefix: bool, flags: c_ulong) -> usize {
    size_t strarray__scnprintf_flags(struct strarray *sa, char *bf, size_t size, bool show_prefix, unsigned long flags)
    {
    int i, printed = 0;
    if (flags == 0) {
    const char *s = sa.entries[0];
    if (s)
    return scnprintf(bf, size, "%s%s", show_prefix ? sa.prefix : "", s);
    return scnprintf(bf, size, "%d", 0);
    }
    for (i = 1; i < sa.nr_entries; ++i) {
    let mut bit: c_ulong = 1UL << (i - 1);
    if (!(flags & bit))
    continue;
    if (printed != 0)
    printed += scnprintf(bf + printed, size - printed, "|");
    if (sa.entries[i] != core::ptr::null_mut())
    printed += scnprintf(bf + printed, size - printed, "%s%s", show_prefix ? sa.prefix : "", sa.entries[i]);
    else
    printed += scnprintf(bf + printed, size - printed, "0x%#", bit);
    }
    return printed;
    }
#[no_mangle]
unsafe extern "C" fn pkey_alloc__scnprintf_access_rights(access_rights: c_int, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t pkey_alloc__scnprintf_access_rights(int access_rights, char *bf, size_t size, bool show_prefix)
    {

    static DEFINE_STRARRAY(pkey_alloc_access_rights, "PKEY_");
    return strarray__scnprintf_flags(&strarray__pkey_alloc_access_rights, bf, size, show_prefix, access_rights);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_pkey_alloc_access_rights(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_pkey_alloc_access_rights(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut cmd: c_ulong = arg.val;
    return pkey_alloc__scnprintf_access_rights(cmd, bf, size, arg.show_string_prefix);
    }
