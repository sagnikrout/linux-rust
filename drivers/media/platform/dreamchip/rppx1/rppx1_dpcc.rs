//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_dpcc.c
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

pub const DPCC_VERSION_REG: c_uint = 0x0000;
pub const DPCC_MODE_REG: c_uint = 0x0004;

pub const DPCC_OUTPUT_MODE_REG: c_uint = 0x0008;
pub const DPCC_SET_USE_REG: c_uint = 0x000c;
pub const DPCC_METHODS_SET_1_REG: c_uint = 0x0010;
pub const DPCC_METHODS_SET_2_REG: c_uint = 0x0014;
pub const DPCC_METHODS_SET_3_REG: c_uint = 0x0018;
pub const DPCC_LINE_THRESH_1_REG: c_uint = 0x001c;
pub const DPCC_LINE_MAD_FAC_1_REG: c_uint = 0x0020;
pub const DPCC_PG_FAC_1_REG: c_uint = 0x0024;
pub const DPCC_RND_THRESH_1_REG: c_uint = 0x0028;
pub const DPCC_RG_FAC_1_REG: c_uint = 0x002c;
pub const DPCC_LINE_THRESH_2_REG: c_uint = 0x0030;
pub const DPCC_LINE_MAD_FAC_2_REG: c_uint = 0x0034;
pub const DPCC_PG_FAC_2_REG: c_uint = 0x0038;
pub const DPCC_RND_THRESH_2_REG: c_uint = 0x003c;
pub const DPCC_RG_FAC_2_REG: c_uint = 0x0040;
pub const DPCC_LINE_THRESH_3_REG: c_uint = 0x0044;
pub const DPCC_LINE_MAD_FAC_3_REG: c_uint = 0x0048;
pub const DPCC_PG_FAC_3_REG: c_uint = 0x004c;
pub const DPCC_RND_THRESH_3_REG: c_uint = 0x0050;
pub const DPCC_RG_FAC_3_REG: c_uint = 0x0054;
pub const DPCC_RO_LIMITS_REG: c_uint = 0x0058;
pub const DPCC_RND_OFFS_REG: c_uint = 0x005c;
pub const DPCC_BPT_CTRL_REG: c_uint = 0x0060;
pub const DPCC_BP_NUMBER_REG: c_uint = 0x0064;
pub const DPCC_BP_TADDR_REG: c_uint = 0x0068;
pub const DPCC_BP_POSITION_REG: c_uint = 0x006c;
#[no_mangle]
unsafe extern "C" fn rppx1_dpcc_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_dpcc_probe(struct rpp_module *mod)
    {
// Version check.
    switch (rpp_module_read(mod, DPCC_VERSION_REG)) {
    case 2:
    case 4:
    case 6:
// 12-bit.
    break;
    case 3:
    case 5:
    case 7:
// 24-bit.
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int rppx1_dpcc_start(struct rpp_module *mod,
    const struct v4l2_mbus_framefmt *fmt)
    {
// Bypass stage1 and DPCC.
    rpp_module_write(mod, DPCC_MODE_REG, 0);
    return 0;
    }
    const struct rpp_module_ops rppx1_dpcc_ops = {
    .probe = rppx1_dpcc_probe,
    .start = rppx1_dpcc_start,
    };
