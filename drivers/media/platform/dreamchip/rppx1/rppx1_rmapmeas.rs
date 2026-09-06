//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_rmapmeas.c
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
// Copyright (C) 2026 Renesas Electronics Corp.
// Copyright (C) 2026 Ideas on Board Oy
// Copyright (C) 2026 Ragnatech AB
//

pub const RMAP_MEAS_VERSION_REG: c_uint = 0x0000;
pub const RMAP_MEAS_MODE_REG: c_uint = 0x0004;
pub const RMAP_MEAS_SUBSAMPLING_REG: c_uint = 0x0008;
pub const RMAP_MEAS_RESERVED_1_REG: c_uint = 0x000c;
pub const RMAP_MEAS_MIN_THRES_SHORT_REG: c_uint = 0x0010;
pub const RMAP_MEAS_MAX_THRES_SHORT_REG: c_uint = 0x0014;
pub const RMAP_MEAS_MAX_THRES_LONG_REG: c_uint = 0x0018;
pub const RMAP_MEAS_H_OFFS_REG: c_uint = 0x001c;
pub const RMAP_MEAS_V_OFFS_REG: c_uint = 0x0020;
pub const RMAP_MEAS_H_SIZE_REG: c_uint = 0x0024;
pub const RMAP_MEAS_V_SIZE_REG: c_uint = 0x0028;
pub const RMAP_MEAS_LAST_MEAS_LINE_REG: c_uint = 0x002c;
pub const RMAP_MEAS_LS_RESULTSHORT0_REG: c_uint = 0x0030;
pub const RMAP_MEAS_LS_RESULTLONG0_REG: c_uint = 0x0034;
pub const RMAP_MEAS_RESERVED_2_REG: c_uint = 0x0038;
pub const RMAP_MEAS_RESERVED_3_REG: c_uint = 0x003c;
pub const RMAP_MEAS_LS_RESULTSHORT1_REG: c_uint = 0x0040;
pub const RMAP_MEAS_LS_RESULTLONG1_REG: c_uint = 0x0044;
pub const RMAP_MEAS_RESERVED_4_REG: c_uint = 0x0048;
pub const RMAP_MEAS_RESERVED_5_REG: c_uint = 0x004c;
#[no_mangle]
unsafe extern "C" fn rppx1_rmapmeas_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_rmapmeas_probe(struct rpp_module *mod)
    {
// Version check.
    switch (rpp_module_read(mod, RMAP_MEAS_VERSION_REG)) {
    case 3:
// low: 12-bit, high: 24-bit.
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    const struct rpp_module_ops rppx1_rmapmeas_ops = {
    .probe = rppx1_rmapmeas_probe,
    };
