//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_shrp.c
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

pub const SHRPCNR_VERSION_REG: c_uint = 0x0000;
pub const SHRPCNR_CTRL_REG: c_uint = 0x0004;

pub const SHRPCNR_PARAM_REG: c_uint = 0x0008;

pub const SHRPCNR_MAT_1_REG: c_uint = 0x000c;
pub const SHRPCNR_MAT_2_REG: c_uint = 0x0010;
pub const SHRPCNR_CLB_LINESIZE_REG: c_uint = 0x0014;
pub const SHRPCNR_YUV2RGB_CCOR_COEFF_0_REG: c_uint = 0x0018;
pub const SHRPCNR_YUV2RGB_CCOR_COEFF_1_REG: c_uint = 0x001c;
pub const SHRPCNR_YUV2RGB_CCOR_COEFF_2_REG: c_uint = 0x0020;
pub const SHRPCNR_YUV2RGB_CCOR_COEFF_3_REG: c_uint = 0x0024;
pub const SHRPCNR_YUV2RGB_CCOR_COEFF_4_REG: c_uint = 0x0028;
pub const SHRPCNR_YUV2RGB_CCOR_COEFF_5_REG: c_uint = 0x002c;
pub const SHRPCNR_YUV2RGB_CCOR_COEFF_6_REG: c_uint = 0x0030;
pub const SHRPCNR_YUV2RGB_CCOR_COEFF_7_REG: c_uint = 0x0034;
pub const SHRPCNR_YUV2RGB_CCOR_COEFF_8_REG: c_uint = 0x0038;
pub const SHRPCNR_YUV2RGB_CCOR_OFFSET_R_REG: c_uint = 0x003c;
pub const SHRPCNR_YUV2RGB_CCOR_OFFSET_G_REG: c_uint = 0x0040;
pub const SHRPCNR_YUV2RGB_CCOR_OFFSET_B_REG: c_uint = 0x0044;
pub const SHRPCNR_CNR_THRES_REG: c_uint = 0x0048;

pub const SHRPCNR_CRED_THRES_REG: c_uint = 0x004c;
pub const SHRPCNR_CRED_SLOPE_REG: c_uint = 0x0050;
pub const SHRPCNR_CAD_RESTORE_LVL_REG: c_uint = 0x0054;
pub const SHRPCNR_CAD_THRESH_V_UNEG_REG: c_uint = 0x0058;
pub const SHRPCNR_CAD_THRESH_V_UPOS_REG: c_uint = 0x005c;
pub const SHRPCNR_CAD_THRESH_U_REG: c_uint = 0x0060;
#[no_mangle]
unsafe extern "C" fn rppx1_shrp_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_shrp_probe(struct rpp_module *mod)
    {
// Version check.
    switch (rpp_module_read(mod, SHRPCNR_VERSION_REG)) {
    case 2:
// 12-bit.
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    const struct rpp_module_ops rppx1_shrp_ops = {
    .probe = rppx1_shrp_probe,
    };
