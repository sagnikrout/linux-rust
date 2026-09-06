//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_endian.c
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
// Copyright (c) 2020 Facebook

pub const IN16: c_uint = 0x1234;
pub const IN32: c_uint = 0x12345678U;
pub const IN64: c_uint = 0x123456789abcdef0ULL;
    let mut in16: __u16 = 0;
    let mut in32: __u32 = 0;
    let mut in64: __u64 = 0;
    let mut out16: __u16 = 0;
    let mut out32: __u32 = 0;
    let mut out64: __u64 = 0;
    let mut const16: __u16 = 0;
    let mut const32: __u32 = 0;
    let mut const64: __u64 = 0;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn sys_enter(ctx: *const c_void) -> c_int {
    int sys_enter(const void *ctx)
    {
    out16 = __builtin_bswap16(in16);
    out32 = __builtin_bswap32(in32);
    out64 = __builtin_bswap64(in64);
    const16 = ___bpf_swab16(IN16);
    const32 = ___bpf_swab32(IN32);
    const64 = ___bpf_swab64(IN64);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
