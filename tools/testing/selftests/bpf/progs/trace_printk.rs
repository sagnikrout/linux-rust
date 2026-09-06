//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/trace_printk.c
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
// Copyright (c) 2020, Oracle and/or its affiliates.

    char _license[] SEC("license") = "GPL";
    let mut trace_printk_ret: c_int = 0;
    let mut trace_printk_ran: c_int = 0;
    let mut trace_printk_invalid_spec_ret: c_int = 0;
    let mut trace_printk_utf8_ret: c_int = 0;
    let mut trace_printk_utf8_ran: c_int = 0;
    const char fmt[] = "Testing,testing %d\n";
    static const char utf8_fmt[] = "中文,测试 %d\n";
// Non-ASCII bytes after '%' must still be rejected.
    static const char invalid_spec_fmt[] = "%\x80\n";
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn sys_enter(ctx: *mut c_void) -> c_int {
    int sys_enter(void *ctx)
    {
    trace_printk_ret = bpf_trace_printk(fmt, sizeof(fmt),
    ++trace_printk_ran);
    trace_printk_utf8_ret = bpf_trace_printk(utf8_fmt, sizeof(utf8_fmt),
    ++trace_printk_utf8_ran);
    trace_printk_invalid_spec_ret = bpf_trace_printk(invalid_spec_fmt,
    sizeof(invalid_spec_fmt));
    return 0;
    }
