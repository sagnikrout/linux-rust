//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/kcmp.c
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
// trace/beauty/kcmp.c
//
// Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
//

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_kcmp_idx(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_kcmp_idx(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut fd: c_ulong = arg.val;
    let mut type: c_int = syscall_arg__val(arg, 2);
    pid_t pid;
    if (type != KCMP_FILE)
    return syscall_arg__scnprintf_long(bf, size, arg);
    pid = syscall_arg__val(arg, arg.idx == 3 ? 0 : 1); /* idx1 . pid1, idx2 . pid2 */
    return pid__scnprintf_fd(arg.trace, pid, fd, bf, size);
    }
#[no_mangle]
unsafe extern "C" fn kcmp__scnprintf_type(type: c_int, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t kcmp__scnprintf_type(int type, char *bf, size_t size, bool show_prefix)
    {
    static DEFINE_STRARRAY(kcmp_types, "KCMP_");
    return strarray__scnprintf(&strarray__kcmp_types, bf, size, "%d", show_prefix, type);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_kcmp_type(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_kcmp_type(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut type: c_ulong = arg.val;
    if (type != KCMP_FILE)
    arg.mask |= (1 << 3) | (1 << 4); /* Ignore idx1 and idx2 */
    return kcmp__scnprintf_type(type, bf, size, arg.show_string_prefix);
    }
