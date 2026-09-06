//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_hist.c
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

pub const HIST_VERSION_REG: c_uint = 0x0000;
pub const HIST_CTRL_REG: c_uint = 0x0004;

pub const HIST_MODE_REG: c_uint = 0x0008;

pub const HIST_MODE_HIST_MODE_DISABLE: c_int = 0;
pub const HIST_MODE_HIST_MODE_YRGB: c_int = 1;
pub const HIST_MODE_HIST_MODE_R: c_int = 2;
pub const HIST_MODE_HIST_MODE_GR: c_int = 3;
pub const HIST_MODE_HIST_MODE_B: c_int = 4;
pub const HIST_MODE_HIST_MODE_GB: c_int = 5;
pub const HIST_CHANNEL_SEL_REG: c_uint = 0x000c;

pub const HIST_LAST_MEAS_LINE_REG: c_uint = 0x0010;
pub const HIST_SUBSAMPLING_REG: c_uint = 0x0014;

pub const HIST_COEFF_R_REG: c_uint = 0x0018;
pub const HIST_COEFF_G_REG: c_uint = 0x001c;
pub const HIST_COEFF_B_REG: c_uint = 0x0020;
pub const HIST_H_OFFS_REG: c_uint = 0x0024;
pub const HIST_V_OFFS_REG: c_uint = 0x0028;
pub const HIST_H_SIZE_REG: c_uint = 0x002c;
pub const HIST_V_SIZE_REG: c_uint = 0x0030;
pub const HIST_SAMPLE_RANGE_REG: c_uint = 0x0034;

pub const HIST_WEIGHT_00TO30_REG: c_uint = 0x0038;
pub const HIST_WEIGHT_40TO21_REG: c_uint = 0x003c;
pub const HIST_WEIGHT_31TO12_REG: c_uint = 0x0040;
pub const HIST_WEIGHT_22TO03_REG: c_uint = 0x0044;
pub const HIST_WEIGHT_13TO43_REG: c_uint = 0x0048;
pub const HIST_WEIGHT_04TO34_REG: c_uint = 0x004c;
pub const HIST_WEIGHT_44_REG: c_uint = 0x0050;
pub const HIST_FORCED_UPD_START_LINE_REG: c_uint = 0x0054;
pub const HIST_FORCED_UPDATE_REG: c_uint = 0x0058;
pub const HIST_VSTART_STATUS_REG: c_uint = 0x005c;

#[no_mangle]
unsafe extern "C" fn rppx1_hist_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_hist_probe(struct rpp_module *mod)
    {
// Version check.
    switch (rpp_module_read(mod, HIST_VERSION_REG)) {
    case 3:
// 12-bit.
    break;
    case 4:
// 20-bit.
    break;
    case 5:
// 24-bit.
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }

    (((v0) & 0x1f) | (((v1) & 0x1f) << 8)  | \
    (((v2) & 0x1f) << 16) | \
    (((v3) & 0x1f) << 24))
    static int rppx1_hist_fill_params(struct rpp_module *mod,
    const union rppx1_params_block *block,
    rppx1_reg_write write, void *priv)
    {
    const struct rppx1_hist_params *cfg = &block.hist;
    u32 h_offs, v_offs, h_size, v_size;
// If the modules is disabled, simply bypass it.
    if (cfg.header.flags & V4L2_ISP_PARAMS_FL_BLOCK_DISABLE) {
    write(priv, mod.base + HIST_MODE_REG,
    HIST_MODE_HIST_MODE_DISABLE);
    return 0;
    }
// Select sample point
    write(priv, mod.base + HIST_CHANNEL_SEL_REG,
    cfg.channel_sel & HIST_CHANNEL_SEL_CHANNEL_SELECT_MASK);
//
// Configure the input subsampling.
//
// In Bayer mode the vertical and horizontal subsampling counters are
// only incremented for color channels selected by hist_mode.
//
    write(priv, mod.base + HIST_SUBSAMPLING_REG,
    HIST_SUBSAMPLING_V_STEPSIZE(cfg.v_stepsize) |
    HIST_SUBSAMPLING_H_STEP_INC(cfg.h_step_inc));
//
// Adjust and set measurement window to hardware limitations,
// - Offsets must be even.
// - Width and height must be even and divisible in 5 windows.
//
    h_offs = cfg.wnd.h_offs & 0x1ffe;
    v_offs = cfg.wnd.v_offs & 0x1ffe;
    h_size = cfg.wnd.h_size - cfg.wnd.h_size % 10;
    v_size = cfg.wnd.v_size - cfg.wnd.v_size % 10;
    write(priv, mod.base + HIST_H_OFFS_REG, h_offs);
    write(priv, mod.base + HIST_V_OFFS_REG, v_offs);
    write(priv, mod.base + HIST_H_SIZE_REG, h_size / 5);
    write(priv, mod.base + HIST_V_SIZE_REG, v_size / 5);
//
// Set last measurement line for ready interrupt. Ignore the value
// from the parameters as it is only useful for fast-channel switching.
//
    write(priv, mod.base + HIST_LAST_MEAS_LINE_REG, v_offs + v_size + 1);
// Set measurement window weights.
    write(priv, mod.base + HIST_WEIGHT_00TO30_REG,
    RPPX1_HIST_WEIGHT(cfg.weights[0], cfg.weights[1],
    cfg.weights[2], cfg.weights[3]));
    write(priv, mod.base + HIST_WEIGHT_40TO21_REG,
    RPPX1_HIST_WEIGHT(cfg.weights[4], cfg.weights[5],
    cfg.weights[6], cfg.weights[7]));
    write(priv, mod.base + HIST_WEIGHT_31TO12_REG,
    RPPX1_HIST_WEIGHT(cfg.weights[8], cfg.weights[9],
    cfg.weights[10], cfg.weights[11]));
    write(priv, mod.base + HIST_WEIGHT_22TO03_REG,
    RPPX1_HIST_WEIGHT(cfg.weights[12], cfg.weights[13],
    cfg.weights[14], cfg.weights[15]));
    write(priv, mod.base + HIST_WEIGHT_13TO43_REG,
    RPPX1_HIST_WEIGHT(cfg.weights[16], cfg.weights[17],
    cfg.weights[18], cfg.weights[19]));
    write(priv, mod.base + HIST_WEIGHT_04TO34_REG,
    RPPX1_HIST_WEIGHT(cfg.weights[20], cfg.weights[21],
    cfg.weights[22], cfg.weights[23]));
    write(priv, mod.base + HIST_WEIGHT_44_REG,
    RPPX1_HIST_WEIGHT(cfg.weights[24], 0, 0, 0));
    write(priv, mod.base + HIST_MODE_REG, cfg.mode);
    write(priv, mod.base + HIST_COEFF_R_REG, cfg.coeff[0]);
    write(priv, mod.base + HIST_COEFF_G_REG, cfg.coeff[1]);
    write(priv, mod.base + HIST_COEFF_B_REG, cfg.coeff[2]);
    u32 sample_reg = FIELD_PREP(HIST_SAMPLE_RANGE_SAMPLE_SHIFT_MASK,
    cfg.sample_shift) |
    FIELD_PREP(HIST_SAMPLE_RANGE_SAMPLE_OFFSET_MASK,
    cfg.sample_offs);
    write(priv, mod.base + HIST_SAMPLE_RANGE_REG, sample_reg);
    write(priv, mod.base + HIST_FORCED_UPDATE_REG, 1);
    return 0;
    }
    static int rppx1_hist_fill_stats(struct rpp_module *mod,
    union rppx1_stats_block *block)
    {
    struct rppx1_hist_stats *stats = &block.hist;
    for (unsigned int i = 0; i < RPPX1_HIST_NUM_BINS; i++)
    stats.hist_bins[i] = rpp_module_read(mod, HIST_BIN_REG(i));
    return 0;
    }
    const struct rpp_module_ops rppx1_hist_ops = {
    .probe = rppx1_hist_probe,
    .fill_params = rppx1_hist_fill_params,
    .fill_stats = rppx1_hist_fill_stats,
    };
