//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/hyp/nvhe/clock.c
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
// Copyright (C) 2025 Google LLC
// Author: Vincent Donnefort <vdonnefort@google.com>
//

    static struct clock_data {
    struct {
    u32 mult;
    u32 shift;
    u64 epoch_ns;
    u64 epoch_cyc;
    u64 cyc_overflow64;
    } data[2];
    u64 cur;
    } trace_clock_data;
#[no_mangle]
unsafe extern "C" fn __clock_mult_uint128(cyc: u64, mult: u32, shift: u32) -> u64 {
    static u64 __clock_mult_uint128(u64 cyc, u32 mult, u32 shift)
    {
    let mut ns: __uint128_t = (__uint128_t)cyc * mult;
    ns >>= shift;
    return (u64)ns;
    }
// Does not guarantee no reader on the modified bank.
#[no_mangle]
pub unsafe extern "C" fn trace_hyp_clock_update(mult: u32, shift: u32, epoch_ns: u64, epoch_cyc: u64) {
    void trace_hyp_clock_update(u32 mult, u32 shift, u64 epoch_ns, u64 epoch_cyc)
    {
    struct clock_data *clock = &trace_clock_data;
    let mut bank: u64 = clock.cur ^ 1;
    if (!mult || shift >= 64)
    return;
    clock.data[bank].mult			= mult;
    clock.data[bank].shift			= shift;
    clock.data[bank].epoch_ns		= epoch_ns;
    clock.data[bank].epoch_cyc		= epoch_cyc;
    clock.data[bank].cyc_overflow64	= ULONG_MAX / mult;
    smp_store_release(&clock.cur, bank);
    }
// Use untrusted host data
#[no_mangle]
pub unsafe extern "C" fn trace_hyp_clock() -> u64 {
    u64 trace_hyp_clock(void)
    {
    struct clock_data *clock = &trace_clock_data;
    let mut bank: u64 = smp_load_acquire(&clock.cur);
    u64 cyc, ns;
    cyc = __arch_counter_get_cntvct() - clock.data[bank].epoch_cyc;
    if (likely(cyc < clock.data[bank].cyc_overflow64)) {
    ns = cyc * clock.data[bank].mult;
    ns >>= clock.data[bank].shift;
    } else {
    ns = __clock_mult_uint128(cyc, clock.data[bank].mult,
    clock.data[bank].shift);
    }
    return (u64)ns + clock.data[bank].epoch_ns;
    }
