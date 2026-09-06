//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_ga.c
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

pub const GAMMA_OUT_VERSION_REG: c_uint = 0x0000;
pub const GAMMA_OUT_ENABLE_REG: c_uint = 0x0004;

pub const GAMMA_OUT_MODE_REG: c_uint = 0x0008;

#[no_mangle]
unsafe extern "C" fn rppx1_ga_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_ga_probe(struct rpp_module *mod)
    {
// Version check.
    switch (rpp_module_read(mod, GAMMA_OUT_VERSION_REG)) {
    case 1:
// 12-bit.
    break;
    case 2:
// 24-bit.
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int rppx1_ga_start(struct rpp_module *mod,
    const struct v4l2_mbus_framefmt *fmt)
    {
// Disable stage.
    rpp_module_write(mod, GAMMA_OUT_ENABLE_REG, 0);
    return 0;
    }
    static int
    rppx1_ga_fill_params(struct rpp_module *mod,
    const union rppx1_params_block *block,
    rppx1_reg_write write, void *priv)
    {
    const struct rppx1_ga_params *cfg = &block.ga;
    u32 mask;
// If the modules is disabled, simply bypass it.
    if (cfg.header.flags & V4L2_ISP_PARAMS_FL_BLOCK_DISABLE) {
    write(priv, mod.base + GAMMA_OUT_ENABLE_REG, 0);
    return 0;
    }
    switch (cfg.header.type) {
    case RPPX1_PARAMS_BLOCK_TYPE_GA_HV:
    mask = GAMMA_OUT_HV_GAMMA_CURVE_MASK;
    break;
    case RPPX1_PARAMS_BLOCK_TYPE_GA_MV:
    mask = GAMMA_OUT_MV_GAMMA_CURVE_MASK;
    break;
    default:
    return -EINVAL;
    }
    write(priv, mod.base + GAMMA_OUT_MODE_REG, cfg.mode);
    for (unsigned int i = 0; i < RPPX1_GA_MAX_SAMPLES; i++)
    write(priv, mod.base + GAMMA_OUT_Y_REG(i),
    cfg.gamma_y[i] & mask);
// Enable module.
    write(priv, mod.base + GAMMA_OUT_ENABLE_REG,
    GAMMA_OUT_ENABLE_GAMMA_OUT_EN);
    return 0;
    }
    const struct rpp_module_ops rppx1_ga_ops = {
    .probe = rppx1_ga_probe,
    .start = rppx1_ga_start,
    .fill_params = rppx1_ga_fill_params,
    };
