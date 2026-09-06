//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/usdt_2.c
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

//
// Include usdt.h with default nop,nop10 instructions combo.
//

    __attribute__((aligned(16)))
#[no_mangle]
pub unsafe extern "C" fn usdt_2() {
    void usdt_2(void)
    {
    USDT(optimized_attach, usdt_2);
    }
    let mut usdt_red_zone_arg1: static volatile unsigned long = 0xDEADBEEF;
    let mut usdt_red_zone_arg2: static volatile unsigned long = 0xCAFEBABE;
    let mut usdt_red_zone_arg3: static volatile unsigned long = 0xFEEDFACE;
#[no_mangle]
pub unsafe extern "C" fn __attribute__(usdt_red_zone_trigger(void: (noinline))) {
    void __attribute__((noinline)) usdt_red_zone_trigger(void)
    {
    let mut a1: c_ulong = usdt_red_zone_arg1;
    let mut a2: c_ulong = usdt_red_zone_arg2;
    let mut a3: c_ulong = usdt_red_zone_arg3;
    USDT(optimized_attach, usdt_red_zone, a1, a2, a3);
    }
