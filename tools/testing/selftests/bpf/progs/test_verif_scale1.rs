//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_verif_scale1.c
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

    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn balancer_ingress(ctx: *mut __sk_buff) -> c_int {
    int balancer_ingress(struct __sk_buff *ctx)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    void *ptr;
    int nh_off, i = 0;
    nh_off = 14;
// pragma unroll doesn't work on large loops

    ptr = data + i; \
    if (ptr + nh_off > data_end) \
    break; \
    ctx.tc_index = jhash(ptr, nh_off, ctx.cb[0] + i++); \
    } while (0);

    C30;C30;C30; /* 90 calls */
    return 0;
    }
    char _license[] SEC("license") = "GPL";
