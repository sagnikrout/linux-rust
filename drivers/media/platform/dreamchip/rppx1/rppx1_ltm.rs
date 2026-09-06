//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_ltm.c
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

pub const LTM_VERSION_REG: c_uint = 0x0000;
pub const LTM_CTRL_REG: c_uint = 0x0004;

pub const LTM_RGB_WEIGHTS_REG: c_uint = 0x0008;
pub const LTM_CLB_LINESIZE_REG: c_uint = 0x000c;
pub const LTM_TONECURVE_1_REG: c_uint = 0x0010;
pub const LTM_TONECURVE_2_REG: c_uint = 0x0014;
pub const LTM_TONECURVE_3_REG: c_uint = 0x0018;
pub const LTM_TONECURVE_4_REG: c_uint = 0x001c;
pub const LTM_TONECURVE_5_REG: c_uint = 0x0020;
pub const LTM_TONECURVE_6_REG: c_uint = 0x0024;

pub const LTM_L0W_REG: c_uint = 0x00ec;
pub const LTM_L0W_R_REG: c_uint = 0x00f0;
pub const LTM_L0D_REG: c_uint = 0x00f4;
pub const LTM_L0D_R_REG: c_uint = 0x00f8;
pub const LTM_KMIND_REG: c_uint = 0x00fc;
pub const LTM_KMAXD_REG: c_uint = 0x0100;
pub const LTM_KDIFFD_REG: c_uint = 0x0104;
pub const LTM_KDIFFD_R_REG: c_uint = 0x0108;
pub const LTM_KW_REG: c_uint = 0x010c;
pub const LTM_KW_R_REG: c_uint = 0x0110;
pub const LTM_CGAIN_REG: c_uint = 0x0114;
pub const LTM_LPRCH_R_HIGH_REG: c_uint = 0x0118;
pub const LTM_LPRCH_R_LOW_REG: c_uint = 0x011c;
#[no_mangle]
unsafe extern "C" fn rppx1_ltm_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_ltm_probe(struct rpp_module *mod)
    {
// Version check.
    if (rpp_module_read(mod, LTM_VERSION_REG) != 8)
    return -EINVAL;
    return 0;
    }
    const struct rpp_module_ops rppx1_ltm_ops = {
    .probe = rppx1_ltm_probe,
    };
