//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_wbmeas.c
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

pub const AWB_MEAS_VERSION_REG: c_uint = 0x0000;
pub const AWB_MEAS_PROP_REG: c_uint = 0x0004;

pub const AWB_MEAS_H_OFFS_REG: c_uint = 0x0008;
pub const AWB_MEAS_V_OFFS_REG: c_uint = 0x000c;
pub const AWB_MEAS_H_SIZE_REG: c_uint = 0x0010;
pub const AWB_MEAS_V_SIZE_REG: c_uint = 0x0014;
pub const AWB_MEAS_FRAMES_REG: c_uint = 0x0018;
pub const AWB_MEAS_REF_CB_MAX_B_REG: c_uint = 0x001c;
pub const AWB_MEAS_REF_CR_MAX_R_REG: c_uint = 0x0020;
pub const AWB_MEAS_MAX_Y_REG: c_uint = 0x0024;
pub const AWB_MEAS_MIN_Y_MAX_G_REG: c_uint = 0x0028;
pub const AWB_MEAS_MAX_CSUM_REG: c_uint = 0x002c;
pub const AWB_MEAS_MIN_C_REG: c_uint = 0x0030;
pub const AWB_MEAS_WHITE_CNT_REG: c_uint = 0x0034;
pub const AWB_MEAS_MEAN_Y_G_REG: c_uint = 0x0038;
pub const AWB_MEAS_MEAN_CB_B_REG: c_uint = 0x003c;
pub const AWB_MEAS_MEAN_CR_R_REG: c_uint = 0x0040;
pub const AWB_MEAS_CCOR_COEFF_NUM: c_int = 9;

pub const AWB_MEAS_CCOR_OFFSET_R_REG: c_uint = 0x0068;
pub const AWB_MEAS_CCOR_OFFSET_G_REG: c_uint = 0x006c;
pub const AWB_MEAS_CCOR_OFFSET_B_REG: c_uint = 0x0070;
#[no_mangle]
unsafe extern "C" fn rppx1_wbmeas_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_wbmeas_probe(struct rpp_module *mod)
    {
// Version check.
    switch (rpp_module_read(mod, AWB_MEAS_VERSION_REG)) {
    case 1:
// 8-bit.
    break;
    case 2:
// 20-bit.
    break;
    case 3:
// 24-bit.
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int
    rppx1_wbmeas_fill_params(struct rpp_module *mod,
    const union rppx1_params_block *block,
    rppx1_reg_write write, void *priv)
    {
    const struct rppx1_wbmeas_params *cfg = &block.wbmeas;
    u32 awb_meas_props;
// If the modules is disabled, simply bypass it.
    if (cfg.header.flags & V4L2_ISP_PARAMS_FL_BLOCK_DISABLE) {
    write(priv, mod.base + AWB_MEAS_PROP_REG, 0);
    return 0;
    }
// Program measurement window.
    write(priv, mod.base + AWB_MEAS_H_OFFS_REG, cfg.wnd.h_offs);
    write(priv, mod.base + AWB_MEAS_V_OFFS_REG, cfg.wnd.v_offs);
    write(priv, mod.base + AWB_MEAS_H_SIZE_REG, cfg.wnd.h_size);
    write(priv, mod.base + AWB_MEAS_V_SIZE_REG, cfg.wnd.v_size);
// Set number of frames to sample.
    write(priv, mod.base + AWB_MEAS_FRAMES_REG, cfg.frames);
    if (cfg.mode == RPPX1_WBMEAS_MODE_YCBCR) {
    write(priv, mod.base + AWB_MEAS_REF_CB_MAX_B_REG,
    cfg.ref_cb_max_b);
    write(priv, mod.base + AWB_MEAS_REF_CR_MAX_R_REG,
    cfg.ref_cr_max_r);
    write(priv, mod.base + AWB_MEAS_MAX_Y_REG, cfg.max_y);
    write(priv, mod.base + AWB_MEAS_MIN_Y_MAX_G_REG,
    cfg.min_y_max_g);
    write(priv, mod.base + AWB_MEAS_MAX_CSUM_REG, cfg.max_csum);
    write(priv, mod.base + AWB_MEAS_MIN_C_REG, cfg.min_c);
//
// Program the color conversion matrix coefficients and the
// per-color channel offsets.
//
    for (unsigned int i = 0; i < 3; i++) {
    for (unsigned int j = 0; j < 3; j++) {
    let mut index: c_uint = i * 3 + j;
    write(priv,
    mod.base + AWB_MEAS_CCOR_COEFF_REG(index),
    cfg.ccor_coeff[i][j]);
    }
    }
    write(priv, mod.base + AWB_MEAS_CCOR_OFFSET_R_REG,
    cfg.ccor_offs[0]);
    write(priv, mod.base + AWB_MEAS_CCOR_OFFSET_G_REG,
    cfg.ccor_offs[1]);
    write(priv, mod.base + AWB_MEAS_CCOR_OFFSET_B_REG,
    cfg.ccor_offs[2]);
    awb_meas_props = cfg.ymax_cmp ? AWB_MEAS_PROP_YMAX : 0;
    } else {
    write(priv, mod.base + AWB_MEAS_REF_CB_MAX_B_REG,
    cfg.ref_cb_max_b);
    write(priv, mod.base + AWB_MEAS_REF_CR_MAX_R_REG,
    cfg.ref_cr_max_r);
    write(priv, mod.base + AWB_MEAS_MIN_Y_MAX_G_REG,
    cfg.min_y_max_g);
// Bypass color conversion matrix and color offsets.
    write(priv, mod.base + AWB_MEAS_CCOR_COEFF_REG(0), 0x1000);
    write(priv, mod.base + AWB_MEAS_CCOR_COEFF_REG(1), 0x0000);
    write(priv, mod.base + AWB_MEAS_CCOR_COEFF_REG(2), 0x0000);
    write(priv, mod.base + AWB_MEAS_CCOR_COEFF_REG(3), 0x0000);
    write(priv, mod.base + AWB_MEAS_CCOR_COEFF_REG(4), 0x1000);
    write(priv, mod.base + AWB_MEAS_CCOR_COEFF_REG(5), 0x0000);
    write(priv, mod.base + AWB_MEAS_CCOR_COEFF_REG(6), 0x0000);
    write(priv, mod.base + AWB_MEAS_CCOR_COEFF_REG(7), 0x0000);
    write(priv, mod.base + AWB_MEAS_CCOR_COEFF_REG(8), 0x1000);
    write(priv, mod.base + AWB_MEAS_CCOR_OFFSET_R_REG, 0x00000000);
    write(priv, mod.base + AWB_MEAS_CCOR_OFFSET_G_REG, 0x00000000);
    write(priv, mod.base + AWB_MEAS_CCOR_OFFSET_B_REG, 0x00000000);
    awb_meas_props = AWB_MEAS_PROP_MEAS_MODE_RGB;
    }
    write(priv, mod.base + AWB_MEAS_PROP_REG,
    awb_meas_props | AWB_MEAS_PROP_AWB_MODE_ON);
    return 0;
    }
    static int rppx1_wbmeas_fill_stats(struct rpp_module *mod,
    union rppx1_stats_block *block)
    {
    struct rppx1_wbmeas_stats *stats = &block.wbmeas;
// Return measurements at native hardware precision.
    stats.cnt = rpp_module_read(mod, AWB_MEAS_WHITE_CNT_REG);
    stats.mean_y_or_g = rpp_module_read(mod, AWB_MEAS_MEAN_Y_G_REG);
    stats.mean_cb_or_b = rpp_module_read(mod, AWB_MEAS_MEAN_CB_B_REG);
    stats.mean_cr_or_r = rpp_module_read(mod, AWB_MEAS_MEAN_CR_R_REG);
    return 0;
    }
    const struct rpp_module_ops rppx1_wbmeas_ops = {
    .probe = rppx1_wbmeas_probe,
    .fill_params = rppx1_wbmeas_fill_params,
    .fill_stats = rppx1_wbmeas_fill_stats
    };
