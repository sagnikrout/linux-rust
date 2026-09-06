//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_bd.c
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

pub const DPF_VERSION_REG: c_uint = 0x0000;
pub const DPF_MODE_REG: c_uint = 0x0004;

pub const DPF_STRENGTH_R_REG: c_uint = 0x0008;
pub const DPF_STRENGTH_G_REG: c_uint = 0x000c;
pub const DPF_STRENGTH_B_REG: c_uint = 0x0010;
pub const DPF_S_WEIGHT_G_1_4_REG: c_uint = 0x0014;
pub const DPF_S_WEIGHT_G_5_6_REG: c_uint = 0x0018;
pub const DPF_S_WEIGHT_RB_1_4_REG: c_uint = 0x001c;
pub const DPF_S_WEIGHT_RB_5_6_REG: c_uint = 0x0020;
pub const DPF_NLL_G_COEFF_REG_NUM: c_int = 17;

pub const DPF_NLL_RB_COEFF_REG_NUM: c_int = 17;

pub const DPF_NF_GAIN_R_REG: c_uint = 0x00ac;
pub const DPF_NF_GAIN_GR_REG: c_uint = 0x00b0;
pub const DPF_NF_GAIN_GB_REG: c_uint = 0x00b4;
pub const DPF_NF_GAIN_B_REG: c_uint = 0x00b8;
#[no_mangle]
unsafe extern "C" fn rppx1_bd_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_bd_probe(struct rpp_module *mod)
    {
// Version check.
    if (rpp_module_read(mod, DPF_VERSION_REG) != 5)
    return -EINVAL;
    return 0;
    }
    const struct rpp_module_ops rppx1_bd_ops = {
    .probe = rppx1_bd_probe,
    };
