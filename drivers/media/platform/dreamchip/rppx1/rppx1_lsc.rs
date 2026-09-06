//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_lsc.c
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

pub const LSC_VERSION_REG: c_uint = 0x0000;
pub const LSC_CTRL_REG: c_uint = 0x0004;

pub const LSC_R_TABLE_ADDR_REG: c_uint = 0x0008;
pub const LSC_GR_TABLE_ADDR_REG: c_uint = 0x000c;
pub const LSC_B_TABLE_ADDR_REG: c_uint = 0x0010;
pub const LSC_GB_TABLE_ADDR_REG: c_uint = 0x0014;
pub const LSC_R_TABLE_DATA_REG: c_uint = 0x0018;
pub const LSC_GR_TABLE_DATA_REG: c_uint = 0x001c;
pub const LSC_B_TABLE_DATA_REG: c_uint = 0x0020;
pub const LSC_GB_TABLE_DATA_REG: c_uint = 0x0024;
pub const LSC_XGRAD_01_REG: c_uint = 0x0028;
pub const LSC_XGRAD_23_REG: c_uint = 0x002c;
pub const LSC_XGRAD_45_REG: c_uint = 0x0030;
pub const LSC_XGRAD_67_REG: c_uint = 0x0034;
pub const LSC_XGRAD_89_REG: c_uint = 0x0038;
pub const LSC_XGRAD_1011_REG: c_uint = 0x003c;
pub const LSC_XGRAD_1213_REG: c_uint = 0x0040;
pub const LSC_XGRAD_1415_REG: c_uint = 0x0044;
pub const LSC_YGRAD_01_REG: c_uint = 0x0048;
pub const LSC_YGRAD_23_REG: c_uint = 0x004c;
pub const LSC_YGRAD_45_REG: c_uint = 0x0050;
pub const LSC_YGRAD_67_REG: c_uint = 0x0054;
pub const LSC_YGRAD_89_REG: c_uint = 0x0058;
pub const LSC_YGRAD_1011_REG: c_uint = 0x005c;
pub const LSC_YGRAD_1213_REG: c_uint = 0x0060;
pub const LSC_YGRAD_1415_REG: c_uint = 0x0064;
pub const LSC_XSIZE_01_REG: c_uint = 0x0068;
pub const LSC_XSIZE_23_REG: c_uint = 0x006c;
pub const LSC_XSIZE_45_REG: c_uint = 0x0070;
pub const LSC_XSIZE_67_REG: c_uint = 0x0074;
pub const LSC_XSIZE_89_REG: c_uint = 0x0078;
pub const LSC_XSIZE_1011_REG: c_uint = 0x007c;
pub const LSC_XSIZE_1213_REG: c_uint = 0x0080;
pub const LSC_XSIZE_1415_REG: c_uint = 0x0084;
pub const LSC_YSIZE_01_REG: c_uint = 0x0088;
pub const LSC_YSIZE_23_REG: c_uint = 0x008c;
pub const LSC_YSIZE_45_REG: c_uint = 0x0090;
pub const LSC_YSIZE_67_REG: c_uint = 0x0094;
pub const LSC_YSIZE_89_REG: c_uint = 0x0098;
pub const LSC_YSIZE_1011_REG: c_uint = 0x009c;
pub const LSC_YSIZE_1213_REG: c_uint = 0x00a0;
pub const LSC_YSIZE_1415_REG: c_uint = 0x00a4;
pub const LSC_TABLE_SEL_REG: c_uint = 0x00a8;
pub const LSC_STATUS_REG: c_uint = 0x00ac;

#[no_mangle]
unsafe extern "C" fn rppx1_lsc_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_lsc_probe(struct rpp_module *mod)
    {
// Version check.
    if (rpp_module_read(mod, LSC_VERSION_REG) != 0x04)
    return -EINVAL;
    return 0;
    }
    static int
    rppx1_lsc_fill_params(struct rpp_module *mod,
    const union rppx1_params_block *block,
    rppx1_reg_write write, void *priv)
    {
    const struct rppx1_lsc_params *cfg = &block.lsc;
    const __u16 *v;
// Always disable module as it needs be disabled before configuring.
    write(priv, mod.base + LSC_CTRL_REG, 0);
    if (cfg.header.flags & V4L2_ISP_PARAMS_FL_BLOCK_DISABLE)
    return 0;
//
// Program the color correction sectors.
//
// There are two tables to one can program and switch between. As the
// RPPX1 supports preparing a buffer of commands to be applied later
// only use table 0. This works as long as the ISP is not used in
// inline-mode.
//
// For inline-mode support using DMA for configuration is not possible
// so this is not an issue, but needs to be address if inline-mode
// support is added to the driver.
//
// Start writing at beginning of table 0.
    write(priv, mod.base + LSC_R_TABLE_ADDR_REG, 0);
    write(priv, mod.base + LSC_GR_TABLE_ADDR_REG, 0);
    write(priv, mod.base + LSC_B_TABLE_ADDR_REG, 0);
    write(priv, mod.base + LSC_GB_TABLE_ADDR_REG, 0);
// Program data tables.
    for (unsigned int i = 0; i < RPPX1_LSC_SAMPLES_MAX; i++) {
    const __u16 *r = cfg.r_data[i];
    const __u16 *gr = cfg.gr_data[i];
    const __u16 *b = cfg.b_data[i];
    const __u16 *gb = cfg.gb_data[i];
    unsigned int j;
    for (j = 0; j < RPPX1_LSC_SAMPLES_MAX - 1; j += 2) {
    write(priv, mod.base + LSC_R_TABLE_DATA_REG,
    LSC_R_TABLE_DATA_VALUE(r[j], r[j + 1]));
    write(priv, mod.base + LSC_GR_TABLE_DATA_REG,
    LSC_R_TABLE_DATA_VALUE(gr[j], gr[j + 1]));
    write(priv, mod.base + LSC_B_TABLE_DATA_REG,
    LSC_R_TABLE_DATA_VALUE(b[j], b[j + 1]));
    write(priv, mod.base + LSC_GB_TABLE_DATA_REG,
    LSC_R_TABLE_DATA_VALUE(gb[j], gb[j + 1]));
    }
    write(priv, mod.base + LSC_R_TABLE_DATA_REG,
    LSC_R_TABLE_DATA_VALUE(r[j], 0));
    write(priv, mod.base + LSC_GR_TABLE_DATA_REG,
    LSC_R_TABLE_DATA_VALUE(gr[j], 0));
    write(priv, mod.base + LSC_B_TABLE_DATA_REG,
    LSC_R_TABLE_DATA_VALUE(b[j], 0));
    write(priv, mod.base + LSC_GB_TABLE_DATA_REG,
    LSC_R_TABLE_DATA_VALUE(gb[j], 0));
    }
// Activate table 0.
    write(priv, mod.base + LSC_TABLE_SEL_REG, 0);
//
// Program X- and Y- sizes, and gradients.
//
    v = cfg.x_grad;
    write(priv, mod.base + LSC_XGRAD_01_REG, LSC_GRAD_VALUE(v[0], v[1]));
    write(priv, mod.base + LSC_XGRAD_23_REG, LSC_GRAD_VALUE(v[2], v[3]));
    write(priv, mod.base + LSC_XGRAD_45_REG, LSC_GRAD_VALUE(v[4], v[5]));
    write(priv, mod.base + LSC_XGRAD_67_REG, LSC_GRAD_VALUE(v[6], v[7]));
    write(priv, mod.base + LSC_XGRAD_89_REG, LSC_GRAD_VALUE(v[8], v[9]));
    write(priv, mod.base + LSC_XGRAD_1011_REG, LSC_GRAD_VALUE(v[10], v[11]));
    write(priv, mod.base + LSC_XGRAD_1213_REG, LSC_GRAD_VALUE(v[12], v[13]));
    write(priv, mod.base + LSC_XGRAD_1415_REG, LSC_GRAD_VALUE(v[14], v[15]));
    v = cfg.y_grad;
    write(priv, mod.base + LSC_YGRAD_01_REG, LSC_GRAD_VALUE(v[0], v[1]));
    write(priv, mod.base + LSC_YGRAD_23_REG, LSC_GRAD_VALUE(v[2], v[3]));
    write(priv, mod.base + LSC_YGRAD_45_REG, LSC_GRAD_VALUE(v[4], v[5]));
    write(priv, mod.base + LSC_YGRAD_67_REG, LSC_GRAD_VALUE(v[6], v[7]));
    write(priv, mod.base + LSC_YGRAD_89_REG, LSC_GRAD_VALUE(v[8], v[9]));
    write(priv, mod.base + LSC_YGRAD_1011_REG, LSC_GRAD_VALUE(v[10], v[11]));
    write(priv, mod.base + LSC_YGRAD_1213_REG, LSC_GRAD_VALUE(v[12], v[13]));
    write(priv, mod.base + LSC_YGRAD_1415_REG, LSC_GRAD_VALUE(v[14], v[15]));
    v = cfg.x_sect_size;
    write(priv, mod.base + LSC_XSIZE_01_REG, LSC_GRAD_VALUE(v[0], v[1]));
    write(priv, mod.base + LSC_XSIZE_23_REG, LSC_GRAD_VALUE(v[2], v[3]));
    write(priv, mod.base + LSC_XSIZE_45_REG, LSC_GRAD_VALUE(v[4], v[5]));
    write(priv, mod.base + LSC_XSIZE_67_REG, LSC_GRAD_VALUE(v[6], v[7]));
    write(priv, mod.base + LSC_XSIZE_89_REG, LSC_GRAD_VALUE(v[8], v[9]));
    write(priv, mod.base + LSC_XSIZE_1011_REG, LSC_GRAD_VALUE(v[10], v[11]));
    write(priv, mod.base + LSC_XSIZE_1213_REG, LSC_GRAD_VALUE(v[12], v[13]));
    write(priv, mod.base + LSC_XSIZE_1415_REG, LSC_GRAD_VALUE(v[14], v[15]));
    v = cfg.y_sect_size;
    write(priv, mod.base + LSC_YSIZE_01_REG, LSC_GRAD_VALUE(v[0], v[1]));
    write(priv, mod.base + LSC_YSIZE_23_REG, LSC_GRAD_VALUE(v[2], v[3]));
    write(priv, mod.base + LSC_YSIZE_45_REG, LSC_GRAD_VALUE(v[4], v[5]));
    write(priv, mod.base + LSC_YSIZE_67_REG, LSC_GRAD_VALUE(v[6], v[7]));
    write(priv, mod.base + LSC_YSIZE_89_REG, LSC_GRAD_VALUE(v[8], v[9]));
    write(priv, mod.base + LSC_YSIZE_1011_REG, LSC_GRAD_VALUE(v[10], v[11]));
    write(priv, mod.base + LSC_YSIZE_1213_REG, LSC_GRAD_VALUE(v[12], v[13]));
    write(priv, mod.base + LSC_YSIZE_1415_REG, LSC_GRAD_VALUE(v[14], v[15]));
// Enable module.
    write(priv, mod.base + LSC_CTRL_REG, LSC_CTRL_LSC_EN);
    return 0;
    }
    const struct rpp_module_ops rppx1_lsc_ops = {
    .probe = rppx1_lsc_probe,
    .fill_params = rppx1_lsc_fill_params,
    };
