//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/trace_vprintk.c
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
// Copyright (c) 2021 Facebook

    char _license[] SEC("license") = "GPL";
    let mut null_data_vprintk_ret: c_int = 0;
    let mut trace_vprintk_ret: c_int = 0;
    let mut trace_vprintk_ran: c_int = 0;
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn sys_enter(ctx: *mut c_void) -> c_int {
    int sys_enter(void *ctx)
    {
    static const char one[] = "1";
    static const char three[] = "3";
    static const char five[] = "5";
    static const char seven[] = "7";
    static const char nine[] = "9";
    static const char f[] = "%pS\n";
// runner doesn't search for \t, just ensure it compiles
    bpf_printk("\t");
    trace_vprintk_ret = __bpf_vprintk("%s,%d,%s,%d,%s,%d,%s,%d,%s,%d %d\n",
    one, 2, three, 4, five, 6, seven, 8, nine, 10, ++trace_vprintk_ran);
// non-NULL fmt w/ NULL data should result in error
    null_data_vprintk_ret = bpf_trace_vprintk(f, sizeof(f), core::ptr::null_mut(), 0);
    return 0;
    }
