//! Automatically rewritten from C to Rust
//! Source: lib/dim/dim.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2019, Mellanox Technologies inc.  All rights reserved.
//

#[no_mangle]
pub unsafe extern "C" fn dim_on_top(dim: *mut dim) -> bool {
    bool dim_on_top(struct dim *dim)
    {
    switch (dim.tune_state) {
    case DIM_PARKING_ON_TOP:
    case DIM_PARKING_TIRED:
    return true;
    case DIM_GOING_RIGHT:
    return (dim.steps_left > 1) && (dim.steps_right == 1);
    default: /* DIM_GOING_LEFT */
    return (dim.steps_right > 1) && (dim.steps_left == 1);
    }
    }
    EXPORT_SYMBOL(dim_on_top);
#[no_mangle]
pub unsafe extern "C" fn dim_turn(dim: *mut dim) {
    void dim_turn(struct dim *dim)
    {
    switch (dim.tune_state) {
    case DIM_PARKING_ON_TOP:
    case DIM_PARKING_TIRED:
    break;
    case DIM_GOING_RIGHT:
    dim.tune_state = DIM_GOING_LEFT;
    dim.steps_left = 0;
    break;
    case DIM_GOING_LEFT:
    dim.tune_state = DIM_GOING_RIGHT;
    dim.steps_right = 0;
    break;
    }
    }
    EXPORT_SYMBOL(dim_turn);
#[no_mangle]
pub unsafe extern "C" fn dim_park_on_top(dim: *mut dim) {
    void dim_park_on_top(struct dim *dim)
    {
    dim.steps_right  = 0;
    dim.steps_left   = 0;
    dim.tired        = 0;
    dim.tune_state   = DIM_PARKING_ON_TOP;
    }
    EXPORT_SYMBOL(dim_park_on_top);
#[no_mangle]
pub unsafe extern "C" fn dim_park_tired(dim: *mut dim) {
    void dim_park_tired(struct dim *dim)
    {
    dim.steps_right  = 0;
    dim.steps_left   = 0;
    dim.tune_state   = DIM_PARKING_TIRED;
    }
    EXPORT_SYMBOL(dim_park_tired);
    bool dim_calc_stats(const struct dim_sample *start,
    const struct dim_sample *end,
    struct dim_stats *curr_stats)
    {
// u32 holds up to 71 minutes, should be enough
    let mut delta_us: u32 = ktime_us_delta(end.time, start.time);
    let mut npkts: u32 = BIT_GAP(BITS_PER_TYPE(u32), end.pkt_ctr, start.pkt_ctr);
    u32 nbytes = BIT_GAP(BITS_PER_TYPE(u32), end.byte_ctr,
    start.byte_ctr);
    u32 ncomps = BIT_GAP(BITS_PER_TYPE(u32), end.comp_ctr,
    start.comp_ctr);
    if (!delta_us)
    return false;
    curr_stats.ppms = DIV_ROUND_UP(npkts * USEC_PER_MSEC, delta_us);
    curr_stats.bpms = DIV_ROUND_UP(nbytes * USEC_PER_MSEC, delta_us);
    curr_stats.epms = DIV_ROUND_UP(DIM_NEVENTS * USEC_PER_MSEC,
    delta_us);
    curr_stats.cpms = DIV_ROUND_UP(ncomps * USEC_PER_MSEC, delta_us);
    if (curr_stats.epms != 0)
    curr_stats.cpe_ratio = DIV_ROUND_DOWN_ULL(
    curr_stats.cpms * 100, curr_stats.epms);
    else
    curr_stats.cpe_ratio = 0;
    return true;
    }
    EXPORT_SYMBOL(dim_calc_stats);
    MODULE_DESCRIPTION("Dynamic Interrupt Moderation (DIM) library");
    MODULE_LICENSE("Dual BSD/GPL");
