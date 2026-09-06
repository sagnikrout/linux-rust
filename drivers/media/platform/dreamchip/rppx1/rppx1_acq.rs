//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_acq.c
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

pub const ACQ_VERSION_REG: c_uint = 0x0000;
pub const ACQ_CTRL_REG: c_uint = 0x0004;

pub const ACQ_PROP_REG: c_uint = 0x0008;

pub const ACQ_H_OFFS_REG: c_uint = 0x000c;
pub const ACQ_V_OFFS_REG: c_uint = 0x0010;
pub const ACQ_H_SIZE_REG: c_uint = 0x0014;
pub const ACQ_V_SIZE_REG: c_uint = 0x0018;
pub const ACQ_OUT_H_OFFS_REG: c_uint = 0x001c;
pub const ACQ_OUT_V_OFFS_REG: c_uint = 0x0020;
pub const ACQ_OUT_H_SIZE_REG: c_uint = 0x0024;
pub const ACQ_OUT_V_SIZE_REG: c_uint = 0x0028;
pub const FLAGS_SHD_REG: c_uint = 0x002c;
pub const ACQ_OUT_H_OFFS_SHD_REG: c_uint = 0x0030;
pub const ACQ_OUT_V_OFFS_SHD_REG: c_uint = 0x0034;
pub const ACQ_OUT_H_SIZE_SHD_REG: c_uint = 0x0038;
pub const ACQ_OUT_V_SIZE_SHD_REG: c_uint = 0x003c;
#[no_mangle]
unsafe extern "C" fn rppx1_acq_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_acq_probe(struct rpp_module *mod)
    {
// Version check.
    if (rpp_module_read(mod, ACQ_VERSION_REG) != 0x0b)
    return -EINVAL;
    return 0;
    }
    static int rppx1_acq_start(struct rpp_module *mod,
    const struct v4l2_mbus_framefmt *fmt)
    {
    u32 bayerpat, selection;
    rpp_module_clrset(mod, ACQ_CTRL_REG, ACQ_CTRL_RPP_MODE_MASK,
    ACQ_CTRL_RPP_MODE_BAYER);
    rpp_module_write(mod, ACQ_H_OFFS_REG, 0);
    rpp_module_write(mod, ACQ_V_OFFS_REG, 0);
    rpp_module_write(mod, ACQ_H_SIZE_REG, fmt.width);
    rpp_module_write(mod, ACQ_V_SIZE_REG, fmt.height);
    rpp_module_write(mod, ACQ_OUT_H_OFFS_REG, 0);
    rpp_module_write(mod, ACQ_OUT_V_OFFS_REG, 0);
    rpp_module_write(mod, ACQ_OUT_H_SIZE_REG, fmt.width);
    rpp_module_write(mod, ACQ_OUT_V_SIZE_REG, fmt.height);
    switch (fmt.code) {
    case MEDIA_BUS_FMT_SBGGR8_1X8:
    case MEDIA_BUS_FMT_SBGGR10_1X10:
    case MEDIA_BUS_FMT_SBGGR12_1X12:
    mod.info.acq.raw_pattern = RPP_BGGR;
    bayerpat = ACQ_PROP_BAYER_PAT_BGBG;
    break;
    case MEDIA_BUS_FMT_SGBRG8_1X8:
    case MEDIA_BUS_FMT_SGBRG10_1X10:
    case MEDIA_BUS_FMT_SGBRG12_1X12:
    mod.info.acq.raw_pattern = RPP_GBRG;
    bayerpat = ACQ_PROP_BAYER_PAT_GBGB;
    break;
    case MEDIA_BUS_FMT_SGRBG8_1X8:
    case MEDIA_BUS_FMT_SGRBG10_1X10:
    case MEDIA_BUS_FMT_SGRBG12_1X12:
    mod.info.acq.raw_pattern = RPP_GRBG;
    bayerpat = ACQ_PROP_BAYER_PAT_GRGR;
    break;
    case MEDIA_BUS_FMT_SRGGB8_1X8:
    case MEDIA_BUS_FMT_SRGGB10_1X10:
    case MEDIA_BUS_FMT_SRGGB12_1X12:
    mod.info.acq.raw_pattern = RPP_RGGB;
    bayerpat = ACQ_PROP_BAYER_PAT_RGRG;
    break;
    default:
    return -EINVAL;
    }
    switch (fmt.code) {
    case MEDIA_BUS_FMT_SBGGR8_1X8:
    case MEDIA_BUS_FMT_SGBRG8_1X8:
    case MEDIA_BUS_FMT_SGRBG8_1X8:
    case MEDIA_BUS_FMT_SRGGB8_1X8:
    selection = ACQ_PROP_INPUT_SELECTION_8BIT;
    break;
    case MEDIA_BUS_FMT_SBGGR10_1X10:
    case MEDIA_BUS_FMT_SGBRG10_1X10:
    case MEDIA_BUS_FMT_SGRBG10_1X10:
    case MEDIA_BUS_FMT_SRGGB10_1X10:
    selection = ACQ_PROP_INPUT_SELECTION_10BIT;
    break;
    case MEDIA_BUS_FMT_SBGGR12_1X12:
    case MEDIA_BUS_FMT_SGBRG12_1X12:
    case MEDIA_BUS_FMT_SGRBG12_1X12:
    case MEDIA_BUS_FMT_SRGGB12_1X12:
    selection = ACQ_PROP_INPUT_SELECTION_12BIT;
    break;
    default:
    return -EINVAL;
    }
    rpp_module_write(mod, ACQ_PROP_REG, bayerpat | selection |
    ACQ_PROP_SENSOR_IN_LSB_ALIGNED_IN_LSB);
    rpp_module_clrset(mod, ACQ_CTRL_REG, ACQ_CTRL_INFORM_EN_ENABLE,
    ACQ_CTRL_INFORM_EN_ENABLE);
    return 0;
    }
    const struct rpp_module_ops rppx1_acq_ops = {
    .probe = rppx1_acq_probe,
    .start = rppx1_acq_start,
    };
