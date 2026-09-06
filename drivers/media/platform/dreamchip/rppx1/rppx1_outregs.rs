//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_outregs.c
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

pub const OUTREGS_VERSION_REG: c_uint = 0x0000;
pub const OUT_MODE_REG: c_uint = 0x0004;

pub const OUT_MODE_IN_SEL_MAIN: c_int = 1;
pub const OUT_MODE_IN_SEL_PRE1: c_int = 2;
pub const OUT_MODE_IN_SEL_PRE2: c_int = 4;
pub const OUT_CONV_422_METHOD_REG: c_uint = 0x0008;

pub const OUT_CONV_422_METHOD_CONV_422_METHOD_CO_SITED1: c_int = 0;
pub const OUT_CONV_422_METHOD_CONV_422_METHOD_CO_SITED2: c_int = 1;
pub const OUT_CONV_422_METHOD_CONV_422_METHOD_NON_CO_SITED: c_int = 2;
pub const OUTREGS_FORMAT_REG: c_uint = 0x000c;

pub const OUTREGS_FORMAT_OUTPUT_FORMAT_RGB: c_int = 0;
pub const OUTREGS_FORMAT_OUTPUT_FORMAT_YUV422: c_int = 1;
pub const OUTREGS_FORMAT_OUTPUT_FORMAT_YUV420: c_int = 2;
#[no_mangle]
unsafe extern "C" fn rppx1_outregs_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_outregs_probe(struct rpp_module *mod)
    {
// Version check.
    if (rpp_module_read(mod, OUTREGS_VERSION_REG) != 2)
    return -EINVAL;
    return 0;
    }
    static int rppx1_outregs_start(struct rpp_module *mod,
    const struct v4l2_mbus_framefmt *fmt)
    {
    u32 format;
    switch (fmt.code) {
    case MEDIA_BUS_FMT_YUYV12_1X24:
    format = OUTREGS_FORMAT_OUTPUT_FORMAT_YUV422;
    break;
    case MEDIA_BUS_FMT_RGB888_1X24:
    format = OUTREGS_FORMAT_OUTPUT_FORMAT_RGB;
    break;
    default:
    return -EINVAL;
    }
    rpp_module_clrset(mod, OUT_MODE_REG,
    OUT_MODE_UNSELECTED_MODE_MASK | OUT_MODE_IN_SEL_MASK,
    OUT_MODE_UNSELECTED_MODE_MASK | OUT_MODE_IN_SEL_MAIN);
    rpp_module_clrset(mod, OUT_CONV_422_METHOD_REG,
    OUT_CONV_422_METHOD_CONV_422_METHOD_MASK,
    OUT_CONV_422_METHOD_CONV_422_METHOD_CO_SITED1);
    rpp_module_clrset(mod, OUTREGS_FORMAT_REG,
    OUTREGS_FORMAT_OUTPUT_FORMAT_MASK, format);
    return 0;
    }
    const struct rpp_module_ops rppx1_outregs_ops = {
    .probe = rppx1_outregs_probe,
    .start = rppx1_outregs_start,
    };
