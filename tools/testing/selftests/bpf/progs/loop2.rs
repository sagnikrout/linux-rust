//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/loop2.c
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
// Copyright (c) 2019 Facebook

    char _license[] SEC("license") = "GPL";
    SEC("raw_tracepoint/consume_skb")
#[no_mangle]
pub unsafe extern "C" fn while_true(ctx: *mut *mut volatile struct pt_regs) -> c_int {
    int while_true(volatile struct pt_regs* ctx)
    {
    let mut i: c_int = 0;
    while (true) {
    if (PT_REGS_RC(ctx) & 1)
    i += 3;
    else
    i += 7;
    if (i > 40)
    break;
    }
    return i;
    }
