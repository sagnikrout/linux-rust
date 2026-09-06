//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_hist256.c
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

pub const HIST256_VERSION_REG: c_uint = 0x0000;
pub const HIST256_MODE_REG: c_uint = 0x0004;

pub const HIST256_CHANNEL_SEL_REG: c_uint = 0x0008;

pub const HIST256_H_OFFS_REG: c_uint = 0x000c;
pub const HIST256_V_OFFS_REG: c_uint = 0x0010;
pub const HIST256_H_SIZE_REG: c_uint = 0x0014;
pub const HIST256_V_SIZE_REG: c_uint = 0x0018;
pub const HIST256_SAMPLE_OFFSET_REG: c_uint = 0x001c;
pub const HIST256_SAMPLE_SCALE_REG: c_uint = 0x0020;
pub const HIST256_MEAS_RESULT_ADDR_AUTOINCR_REG: c_uint = 0x0024;
pub const HIST256_MEAS_RESULT_ADDR_REG: c_uint = 0x0028;
pub const HIST256_MEAS_RESULT_DATA_REG: c_uint = 0x002c;
pub const HIST256_LOG_ENABLE_REG: c_uint = 0x0030;

pub const HIST256_LOG_DX_LO_REG: c_uint = 0x0034;
pub const HIST256_LOG_DX_HI_REG: c_uint = 0x0038;
pub const HIST256_Y_REG_NUM: c_int = 17;

#[no_mangle]
unsafe extern "C" fn rppx1_hist256_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_hist256_probe(struct rpp_module *mod)
    {
// Version check.
    if (rpp_module_read(mod, HIST256_VERSION_REG) != 2)
    return -EINVAL;
    return 0;
    }
    const struct rpp_module_ops rppx1_hist256_ops = {
    .probe = rppx1_hist256_probe,
    };
