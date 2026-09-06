//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/fs_at_flags.c
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
// trace/beauty/fs_at_flags.c
//
// Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
//

//
// uapi/linux/fcntl.h does not keep a copy in tools headers directory,
// for system with kernel versions before v5.8, need to sync AT_EACCESS macro.
//

pub const AT_EACCESS: c_uint = 0x200;

    static DEFINE_STRARRAY(fs_at_flags, "AT_");
#[no_mangle]
unsafe extern "C" fn fs_at__scnprintf_flags(flags: c_ulong, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t fs_at__scnprintf_flags(unsigned long flags, char *bf, size_t size, bool show_prefix)
    {
    return strarray__scnprintf_flags(&strarray__fs_at_flags, bf, size, show_prefix, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_fs_at_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_fs_at_flags(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    let mut flags: c_int = arg.val;
    return fs_at__scnprintf_flags(flags, bf, size, show_prefix);
    }
#[no_mangle]
unsafe extern "C" fn faccessat2__scnprintf_flags(flags: c_ulong, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t faccessat2__scnprintf_flags(unsigned long flags, char *bf, size_t size, bool show_prefix)
    {
    let mut printed: c_int = 0;
// AT_EACCESS is the same as AT_REMOVEDIR, that is in fs_at_flags_array,
// special case it here.
    if (flags & AT_EACCESS) {
    flags &= ~AT_EACCESS;
    printed += scnprintf(bf + printed, size - printed, "%sEACCESS%s",
    show_prefix ? strarray__fs_at_flags.prefix : "", flags ? "|" : "");
    }
    return strarray__scnprintf_flags(&strarray__fs_at_flags, bf + printed, size - printed, show_prefix, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_faccessat2_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_faccessat2_flags(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    let mut flags: c_int = arg.val;
    return faccessat2__scnprintf_flags(flags, bf, size, show_prefix);
    }
