//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_awbg.c
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

pub const AWB_GAIN_VERSION_REG: c_uint = 0x0000;
pub const AWB_ENABLE_REG: c_uint = 0x0004;

pub const AWB_GAIN_GR_REG: c_uint = 0x0008;
pub const AWB_GAIN_GB_REG: c_uint = 0x000c;
pub const AWB_GAIN_R_REG: c_uint = 0x0010;
pub const AWB_GAIN_B_REG: c_uint = 0x0014;
#[no_mangle]
unsafe extern "C" fn rppx1_awbg_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_awbg_probe(struct rpp_module *mod)
    {
// Version check.
    if (rpp_module_read(mod, AWB_GAIN_VERSION_REG) != 3)
    return -EINVAL;
    return 0;
    }
    static int
    rppx1_awbg_fill_params(struct rpp_module *mod,
    const union rppx1_params_block *block,
    rppx1_reg_write write, void *priv)
    {
    const struct rppx1_awbg_params *cfg = &block.awbg;
// If the modules is disabled, simply bypass it.
    if (cfg.header.flags & V4L2_ISP_PARAMS_FL_BLOCK_DISABLE) {
    write(priv, mod.base + AWB_ENABLE_REG, 0);
    return 0;
    }
//
// RPP gains are 18-bit with 12 bit fractional part and 0x1000 = 1.0,
// giving a possible range of 0.0 to 64.0. NOTE: RPP documentation is
// contradictory this is the register definition, the function
// description states 0x400 = 1.0 AND 18-bit with 12 fractional bits,
// which is not possible...
//
    write(priv, mod.base + AWB_GAIN_GR_REG, cfg.gain_green_r);
    write(priv, mod.base + AWB_GAIN_GB_REG, cfg.gain_green_b);
    write(priv, mod.base + AWB_GAIN_R_REG, cfg.gain_red);
    write(priv, mod.base + AWB_GAIN_B_REG, cfg.gain_blue);
    write(priv, mod.base + AWB_ENABLE_REG, AWB_ENABLE_AWB_GAIN_EN);
    return 0;
    }
    const struct rpp_module_ops rppx1_awbg_ops = {
    .probe = rppx1_awbg_probe,
    .fill_params = rppx1_awbg_fill_params,
    };
