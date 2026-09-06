//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/os-Linux/tls.c
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

pub const PTRACE_GET_THREAD_AREA: c_int = 25;

pub const PTRACE_SET_THREAD_AREA: c_int = 26;

// Checks whether host supports TLS, and sets *tls_min according to the value
// valid on the host.
// i386 host have it == 6; x86_64 host have it == 12, for i386 emulation.
#[no_mangle]
pub unsafe extern "C" fn check_host_supports_tls(supports_tls: *mut c_int, tls_min: *mut c_int) {
    void check_host_supports_tls(int *supports_tls, int *tls_min)
    {
// Values for x86 and x86_64.
    int val[] = {GDT_ENTRY_TLS_MIN_I386, GDT_ENTRY_TLS_MIN_X86_64};
    int i;
    for (i = 0; i < ARRAY_SIZE(val); i++) {
    user_desc_t info;
    info.entry_number = val[i];
    if (syscall(__NR_get_thread_area, &info) == 0) {
// tls_min = val[i];
// supports_tls = 1;
    return;
    } else {
    if (errno == EINVAL)
    continue;
#[no_mangle]
pub unsafe extern "C" fn if(ENOSYS: errno ==) -> else {
    else if (errno == ENOSYS)
// supports_tls = 0;
    return;
    }
    }
// supports_tls = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_set_thread_area(info: *mut user_desc_t, pid: c_int) -> c_int {
    int os_set_thread_area(user_desc_t *info, int pid)
    {
    int ret;
    ret = ptrace(PTRACE_SET_THREAD_AREA, pid, info.entry_number,
    (unsigned long) info);
    if (ret < 0)
    ret = -errno;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn os_get_thread_area(info: *mut user_desc_t, pid: c_int) -> c_int {
    int os_get_thread_area(user_desc_t *info, int pid)
    {
    int ret;
    ret = ptrace(PTRACE_GET_THREAD_AREA, pid, info.entry_number,
    (unsigned long) info);
    if (ret < 0)
    ret = -errno;
    return ret;
    }
