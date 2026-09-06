//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_lin.c
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

// NOTE: The module is called LIN the registers GAMMA_IN.
pub const LIN_VERSION_REG: c_uint = 0x0000;
pub const LIN_ENABLE_REG: c_uint = 0x0004;

pub const LIN_DX_LO_REG: c_uint = 0x0008;
pub const LIN_DX_HI_REG: c_uint = 0x000c;
pub const LIN_R_Y_REG_NUM: c_int = 17;

pub const LIN_G_Y_REG_NUM: c_int = 17;

pub const LIN_B_Y_REG_NUM: c_int = 17;

#[no_mangle]
unsafe extern "C" fn rppx1_lin_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_lin_probe(struct rpp_module *mod)
    {
// Version check.
    switch (rpp_module_read(mod, LIN_VERSION_REG)) {
    case 7:
// 12-bit.
    break;
    case 8:
// 20-bit.
    break;
    case 9:
// 24-bit.
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int rppx1_lin_start(struct rpp_module *mod,
    const struct v4l2_mbus_framefmt *fmt)
    {
    rpp_module_clrset(mod, LIN_ENABLE_REG, LIN_ENABLE_GAMMA_IN_EN, 0);
    return 0;
    }
    static int rppx1_lin_fill_params(struct rpp_module *mod,
    const union rppx1_params_block *block,
    rppx1_reg_write write, void *priv)
    {
    const struct rppx1_lin_params *cfg = &block.lin;
    u8 sample_mask;
    let mut dx_lo: u32 = 0;
    let mut dx_hi: u32 = 0;
    u32 mask;
    if (cfg.header.flags & V4L2_ISP_PARAMS_FL_BLOCK_DISABLE) {
    write(priv, mod.base + LIN_ENABLE_REG, 0);
    return 0;
    }
    switch (cfg.header.type) {
    case RPPX1_PARAMS_BLOCK_TYPE_LIN_PRE1:
    mask = LIN_PRE1_DEGAMMA_CURVE_MASK;
    sample_mask = LIN_PRE1_SAMPLE_POINTS_MASK;
    break;
    case RPPX1_PARAMS_BLOCK_TYPE_LIN_PRE2:
    mask = LIN_PRE2_DEGAMMA_CURVE_MASK;
    sample_mask = LIN_PRE2_SAMPLE_POINTS_MASK;
    break;
    default:
    return -EINVAL;
    }
    for (unsigned int i = 0; i < 8; ++i) {
    dx_lo |= (cfg.dx[i] & sample_mask) << 4 * i;
    dx_hi |= (cfg.dx[i + 8] & sample_mask) << 4 * i;
    }
    write(priv, mod.base + LIN_DX_LO_REG, dx_lo);
    write(priv, mod.base + LIN_DX_HI_REG, dx_hi);
    for (unsigned int i = 0; i < RPPX1_LIN_DEGAMMA_CURVE_NUM; i++) {
    write(priv, mod.base + LIN_R_Y_REG(i), cfg.curve_r[i] & mask);
    write(priv, mod.base + LIN_G_Y_REG(i), cfg.curve_g[i] & mask);
    write(priv, mod.base + LIN_B_Y_REG(i), cfg.curve_b[i] & mask);
    }
    write(priv, mod.base + LIN_ENABLE_REG, LIN_ENABLE_GAMMA_IN_EN);
    return 0;
    }
    const struct rpp_module_ops rppx1_lin_ops = {
    .probe = rppx1_lin_probe,
    .start = rppx1_lin_start,
    .fill_params = rppx1_lin_fill_params,
    };
