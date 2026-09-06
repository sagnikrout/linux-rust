//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/camss/camss-vfe-4-8.c
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
// camss-vfe-4-8.c
//
// Qualcomm MSM Camera Subsystem - VFE (Video Front End) Module v4.8
//
// Copyright (c) 2013-2015, The Linux Foundation. All rights reserved.
// Copyright (C) 2015-2021 Linaro Ltd.
//

pub const VFE_0_GLOBAL_RESET_CMD: c_uint = 0x018;

pub const VFE_0_MODULE_LENS_EN: c_uint = 0x040;

pub const VFE_0_MODULE_ZOOM_EN: c_uint = 0x04c;

pub const VFE_0_CORE_CFG: c_uint = 0x050;
pub const VFE_0_CORE_CFG_PIXEL_PATTERN_YCBYCR: c_uint = 0x4;
pub const VFE_0_CORE_CFG_PIXEL_PATTERN_YCRYCB: c_uint = 0x5;
pub const VFE_0_CORE_CFG_PIXEL_PATTERN_CBYCRY: c_uint = 0x6;
pub const VFE_0_CORE_CFG_PIXEL_PATTERN_CRYCBY: c_uint = 0x7;

pub const VFE_0_IRQ_CMD: c_uint = 0x058;

pub const VFE_0_IRQ_MASK_0: c_uint = 0x05c;

    ((n) == VFE_LINE_PIX ? BIT(4) : VFE_0_IRQ_MASK_0_RDIn_REG_UPDATE(n))

pub const VFE_0_IRQ_MASK_1: c_uint = 0x060;

pub const VFE_0_IRQ_CLEAR_0: c_uint = 0x064;
pub const VFE_0_IRQ_CLEAR_1: c_uint = 0x068;
pub const VFE_0_IRQ_STATUS_0: c_uint = 0x06c;

    ((n) == VFE_LINE_PIX ? BIT(4) : VFE_0_IRQ_STATUS_0_RDIn_REG_UPDATE(n))

pub const VFE_0_IRQ_STATUS_1: c_uint = 0x070;

pub const VFE_0_IRQ_COMPOSITE_MASK_0: c_uint = 0x074;
pub const VFE_0_VIOLATION_STATUS: c_uint = 0x07c;
pub const VFE_0_BUS_CMD: c_uint = 0x80;

pub const VFE_0_BUS_CFG: c_uint = 0x084;

pub const VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_SHIFT: c_int = 8;
pub const VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_LUMA: c_uint = 0x0;
pub const VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_VAL_RDI0: c_uint = 0xc;
pub const VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_VAL_RDI1: c_uint = 0xd;
pub const VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_VAL_RDI2: c_uint = 0xe;

pub const VFE_0_BUS_IMAGE_MASTER_n_WR_CFG_WR_PATH_SHIFT: c_int = 0;

pub const VFE_0_BUS_IMAGE_MASTER_n_WR_ADDR_CFG_FRM_BASED_SHIFT: c_int = 1;
pub const VFE_0_BUS_IMAGE_MASTER_n_WR_ADDR_CFG_FRM_DROP_PER_SHIFT: c_int = 2;

pub const VFE_0_BUS_IMAGE_MASTER_n_WR_UB_CFG_OFFSET_SHIFT: c_int = 16;

    (0x0c4 + 0x2c * (n))

    (0x0c8 + 0x2c * (n))
pub const VFE_0_BUS_IMAGE_MASTER_n_WR_IRQ_SUBSAMPLE_PATTERN_DEF: c_uint = 0xffffffff;
pub const VFE_0_BUS_PING_PONG_STATUS: c_uint = 0x338;
pub const VFE_0_BUS_BDG_CMD: c_uint = 0x400;
pub const VFE_0_BUS_BDG_CMD_HALT_REQ: c_int = 1;
pub const VFE_0_BUS_BDG_QOS_CFG_0: c_uint = 0x404;
pub const VFE_0_BUS_BDG_QOS_CFG_0_CFG: c_uint = 0xaaa5aaa5;
pub const VFE_0_BUS_BDG_QOS_CFG_1: c_uint = 0x408;
pub const VFE_0_BUS_BDG_QOS_CFG_2: c_uint = 0x40c;
pub const VFE_0_BUS_BDG_QOS_CFG_3: c_uint = 0x410;
pub const VFE_0_BUS_BDG_QOS_CFG_3_CFG: c_uint = 0xaa55aaa5;
pub const VFE_0_BUS_BDG_QOS_CFG_4: c_uint = 0x414;
pub const VFE_0_BUS_BDG_QOS_CFG_4_CFG: c_uint = 0xaa55aa55;
pub const VFE_0_BUS_BDG_QOS_CFG_5: c_uint = 0x418;
pub const VFE_0_BUS_BDG_QOS_CFG_6: c_uint = 0x41c;
pub const VFE_0_BUS_BDG_QOS_CFG_7: c_uint = 0x420;
pub const VFE_0_BUS_BDG_QOS_CFG_7_CFG: c_uint = 0x0005aa55;
pub const VFE_0_BUS_BDG_DS_CFG_0: c_uint = 0x424;
pub const VFE_0_BUS_BDG_DS_CFG_0_CFG: c_uint = 0xcccc1111;
pub const VFE_0_BUS_BDG_DS_CFG_1: c_uint = 0x428;
pub const VFE_0_BUS_BDG_DS_CFG_2: c_uint = 0x42c;
pub const VFE_0_BUS_BDG_DS_CFG_3: c_uint = 0x430;
pub const VFE_0_BUS_BDG_DS_CFG_4: c_uint = 0x434;
pub const VFE_0_BUS_BDG_DS_CFG_5: c_uint = 0x438;
pub const VFE_0_BUS_BDG_DS_CFG_6: c_uint = 0x43c;
pub const VFE_0_BUS_BDG_DS_CFG_7: c_uint = 0x440;
pub const VFE_0_BUS_BDG_DS_CFG_8: c_uint = 0x444;
pub const VFE_0_BUS_BDG_DS_CFG_9: c_uint = 0x448;
pub const VFE_0_BUS_BDG_DS_CFG_10: c_uint = 0x44c;
pub const VFE_0_BUS_BDG_DS_CFG_11: c_uint = 0x450;
pub const VFE_0_BUS_BDG_DS_CFG_12: c_uint = 0x454;
pub const VFE_0_BUS_BDG_DS_CFG_13: c_uint = 0x458;
pub const VFE_0_BUS_BDG_DS_CFG_14: c_uint = 0x45c;
pub const VFE_0_BUS_BDG_DS_CFG_15: c_uint = 0x460;
pub const VFE_0_BUS_BDG_DS_CFG_16: c_uint = 0x464;
pub const VFE_0_BUS_BDG_DS_CFG_16_CFG: c_uint = 0x00000110;

pub const VFE_0_RDI_CFG_x_RDI_STREAM_SEL_SHIFT: c_int = 28;

pub const VFE_0_RDI_CFG_x_RDI_M0_SEL_SHIFT: c_int = 4;

pub const VFE_0_RDI_CFG_x_MIPI_EN_BITS: c_uint = 0x3;
pub const VFE_0_CAMIF_CMD: c_uint = 0x478;
pub const VFE_0_CAMIF_CMD_DISABLE_FRAME_BOUNDARY: c_int = 0;
pub const VFE_0_CAMIF_CMD_ENABLE_FRAME_BOUNDARY: c_int = 1;
pub const VFE_0_CAMIF_CMD_NO_CHANGE: c_int = 3;

pub const VFE_0_CAMIF_CFG: c_uint = 0x47c;

pub const VFE_0_CAMIF_FRAME_CFG: c_uint = 0x484;
pub const VFE_0_CAMIF_WINDOW_WIDTH_CFG: c_uint = 0x488;
pub const VFE_0_CAMIF_WINDOW_HEIGHT_CFG: c_uint = 0x48c;
pub const VFE_0_CAMIF_SUBSAMPLE_CFG: c_uint = 0x490;
pub const VFE_0_CAMIF_IRQ_FRAMEDROP_PATTERN: c_uint = 0x498;
pub const VFE_0_CAMIF_IRQ_SUBSAMPLE_PATTERN: c_uint = 0x49c;
pub const VFE_0_CAMIF_STATUS: c_uint = 0x4a4;

pub const VFE_0_REG_UPDATE: c_uint = 0x4ac;

    ((n) == VFE_LINE_PIX ? 1 : VFE_0_REG_UPDATE_RDIn(n))
pub const VFE_0_DEMUX_CFG: c_uint = 0x560;
pub const VFE_0_DEMUX_CFG_PERIOD: c_uint = 0x3;
pub const VFE_0_DEMUX_GAIN_0: c_uint = 0x564;

pub const VFE_0_DEMUX_GAIN_1: c_uint = 0x568;

pub const VFE_0_DEMUX_EVEN_CFG: c_uint = 0x574;
pub const VFE_0_DEMUX_EVEN_CFG_PATTERN_YUYV: c_uint = 0x9cac;
pub const VFE_0_DEMUX_EVEN_CFG_PATTERN_YVYU: c_uint = 0xac9c;
pub const VFE_0_DEMUX_EVEN_CFG_PATTERN_UYVY: c_uint = 0xc9ca;
pub const VFE_0_DEMUX_EVEN_CFG_PATTERN_VYUY: c_uint = 0xcac9;
pub const VFE_0_DEMUX_ODD_CFG: c_uint = 0x578;
pub const VFE_0_DEMUX_ODD_CFG_PATTERN_YUYV: c_uint = 0x9cac;
pub const VFE_0_DEMUX_ODD_CFG_PATTERN_YVYU: c_uint = 0xac9c;
pub const VFE_0_DEMUX_ODD_CFG_PATTERN_UYVY: c_uint = 0xc9ca;
pub const VFE_0_DEMUX_ODD_CFG_PATTERN_VYUY: c_uint = 0xcac9;
pub const VFE_0_SCALE_ENC_Y_CFG: c_uint = 0x91c;
pub const VFE_0_SCALE_ENC_Y_H_IMAGE_SIZE: c_uint = 0x920;
pub const VFE_0_SCALE_ENC_Y_H_PHASE: c_uint = 0x924;
pub const VFE_0_SCALE_ENC_Y_V_IMAGE_SIZE: c_uint = 0x934;
pub const VFE_0_SCALE_ENC_Y_V_PHASE: c_uint = 0x938;
pub const VFE_0_SCALE_ENC_CBCR_CFG: c_uint = 0x948;
pub const VFE_0_SCALE_ENC_CBCR_H_IMAGE_SIZE: c_uint = 0x94c;
pub const VFE_0_SCALE_ENC_CBCR_H_PHASE: c_uint = 0x950;
pub const VFE_0_SCALE_ENC_CBCR_V_IMAGE_SIZE: c_uint = 0x960;
pub const VFE_0_SCALE_ENC_CBCR_V_PHASE: c_uint = 0x964;
pub const VFE_0_CROP_ENC_Y_WIDTH: c_uint = 0x974;
pub const VFE_0_CROP_ENC_Y_HEIGHT: c_uint = 0x978;
pub const VFE_0_CROP_ENC_CBCR_WIDTH: c_uint = 0x97c;
pub const VFE_0_CROP_ENC_CBCR_HEIGHT: c_uint = 0x980;
pub const VFE_0_CLAMP_ENC_MAX_CFG: c_uint = 0x984;

pub const VFE_0_CLAMP_ENC_MIN_CFG: c_uint = 0x988;

pub const VFE_0_REALIGN_BUF_CFG: c_uint = 0xaac;

pub const VFE_0_BUS_IMAGE_MASTER_CMD: c_uint = 0xcec;

pub const CAMIF_TIMEOUT_SLEEP_US: c_int = 1000;
pub const CAMIF_TIMEOUT_ALL_US: c_int = 1000000;
pub const MSM_VFE_VFE0_UB_SIZE: c_int = 2047;

pub const MSM_VFE_VFE1_UB_SIZE: c_int = 1535;

#[no_mangle]
pub unsafe extern "C" fn vfe_reg_clr(vfe: *mut vfe_device, reg: u32, clr_bits: u32) {
    static inline void vfe_reg_clr(struct vfe_device *vfe, u32 reg, u32 clr_bits)
    {
    let mut bits: u32 = readl_relaxed(vfe.base + reg);
    writel_relaxed(bits & ~clr_bits, vfe.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn vfe_reg_set(vfe: *mut vfe_device, reg: u32, set_bits: u32) {
    static inline void vfe_reg_set(struct vfe_device *vfe, u32 reg, u32 set_bits)
    {
    let mut bits: u32 = readl_relaxed(vfe.base + reg);
    writel_relaxed(bits | set_bits, vfe.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn vfe_global_reset(vfe: *mut vfe_device) {
    static void vfe_global_reset(struct vfe_device *vfe)
    {
    u32 reset_bits = VFE_0_GLOBAL_RESET_CMD_IDLE_CGC	|
    VFE_0_GLOBAL_RESET_CMD_DSP		|
    VFE_0_GLOBAL_RESET_CMD_TESTGEN		|
    VFE_0_GLOBAL_RESET_CMD_BUS_MISR	|
    VFE_0_GLOBAL_RESET_CMD_PM		|
    VFE_0_GLOBAL_RESET_CMD_REGISTER	|
    VFE_0_GLOBAL_RESET_CMD_BUS_BDG		|
    VFE_0_GLOBAL_RESET_CMD_BUS		|
    VFE_0_GLOBAL_RESET_CMD_CAMIF		|
    VFE_0_GLOBAL_RESET_CMD_CORE;
    writel_relaxed(BIT(31), vfe.base + VFE_0_IRQ_MASK_0);
// Enforce barrier between IRQ mask setup and global reset
    wmb();
    writel_relaxed(reset_bits, vfe.base + VFE_0_GLOBAL_RESET_CMD);
    }
#[no_mangle]
unsafe extern "C" fn vfe_halt_request(vfe: *mut vfe_device) {
    static void vfe_halt_request(struct vfe_device *vfe)
    {
    writel_relaxed(VFE_0_BUS_BDG_CMD_HALT_REQ,
    vfe.base + VFE_0_BUS_BDG_CMD);
    }
#[no_mangle]
unsafe extern "C" fn vfe_halt_clear(vfe: *mut vfe_device) {
    static void vfe_halt_clear(struct vfe_device *vfe)
    {
    writel_relaxed(0x0, vfe.base + VFE_0_BUS_BDG_CMD);
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_frame_based(vfe: *mut vfe_device, wm: u8, enable: u8) {
    static void vfe_wm_frame_based(struct vfe_device *vfe, u8 wm, u8 enable)
    {
    if (enable)
    vfe_reg_set(vfe, VFE_0_BUS_IMAGE_MASTER_n_WR_ADDR_CFG(wm),
    1 << VFE_0_BUS_IMAGE_MASTER_n_WR_ADDR_CFG_FRM_BASED_SHIFT);
    else
    vfe_reg_clr(vfe, VFE_0_BUS_IMAGE_MASTER_n_WR_ADDR_CFG(wm),
    1 << VFE_0_BUS_IMAGE_MASTER_n_WR_ADDR_CFG_FRM_BASED_SHIFT);
    }

#[no_mangle]
unsafe extern "C" fn vfe_word_per_line_by_pixel(format: u32, pixel_per_line: u32) -> c_int {
    static int vfe_word_per_line_by_pixel(u32 format, u32 pixel_per_line)
    {
    let mut val: c_int = 0;
    switch (format) {
    case V4L2_PIX_FMT_NV12:
    case V4L2_PIX_FMT_NV21:
    case V4L2_PIX_FMT_NV16:
    case V4L2_PIX_FMT_NV61:
    val = CALC_WORD(pixel_per_line, 1, 8);
    break;
    case V4L2_PIX_FMT_YUYV:
    case V4L2_PIX_FMT_YVYU:
    case V4L2_PIX_FMT_UYVY:
    case V4L2_PIX_FMT_VYUY:
    val = CALC_WORD(pixel_per_line, 2, 8);
    break;
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn vfe_word_per_line_by_bytes(bytes_per_line: u32) -> c_int {
    static int vfe_word_per_line_by_bytes(u32 bytes_per_line)
    {
    return CALC_WORD(bytes_per_line, 1, 8);
    }
    static void vfe_get_wm_sizes(struct v4l2_pix_format_mplane *pix, u8 plane,
    u16 *width, u16 *height, u16 *bytesperline)
    {
// width = pix->width;
// height = pix->height;
    switch (pix.pixelformat) {
    case V4L2_PIX_FMT_NV12:
    case V4L2_PIX_FMT_NV21:
// bytesperline = pix->plane_fmt[0].bytesperline;
    if (plane == 1)
// height /= 2;
    break;
    case V4L2_PIX_FMT_NV16:
    case V4L2_PIX_FMT_NV61:
// bytesperline = pix->plane_fmt[0].bytesperline;
    break;
    case V4L2_PIX_FMT_YUYV:
    case V4L2_PIX_FMT_YVYU:
    case V4L2_PIX_FMT_VYUY:
    case V4L2_PIX_FMT_UYVY:
// bytesperline = pix->plane_fmt[plane].bytesperline;
    break;
    }
    }
    static void vfe_wm_line_based(struct vfe_device *vfe, u32 wm,
    struct v4l2_pix_format_mplane *pix,
    u8 plane, u32 enable)
    {
    u32 reg;
    if (enable) {
    let mut width: u16 = 0, height = 0, bytesperline = 0, wpl;
    vfe_get_wm_sizes(pix, plane, &width, &height, &bytesperline);
    wpl = vfe_word_per_line_by_pixel(pix.pixelformat, width);
    reg = height - 1;
    reg |= ((wpl + 3) / 4 - 1) << 16;
    writel_relaxed(reg, vfe.base +
    VFE_0_BUS_IMAGE_MASTER_n_WR_IMAGE_SIZE(wm));
    wpl = vfe_word_per_line_by_bytes(bytesperline);
    reg = 0x3;
    reg |= (height - 1) << 2;
    reg |= ((wpl + 1) / 2) << 16;
    writel_relaxed(reg, vfe.base +
    VFE_0_BUS_IMAGE_MASTER_n_WR_BUFFER_CFG(wm));
    } else {
    writel_relaxed(0, vfe.base +
    VFE_0_BUS_IMAGE_MASTER_n_WR_IMAGE_SIZE(wm));
    writel_relaxed(0, vfe.base +
    VFE_0_BUS_IMAGE_MASTER_n_WR_BUFFER_CFG(wm));
    }
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_set_framedrop_period(vfe: *mut vfe_device, wm: u8, per: u8) {
    static void vfe_wm_set_framedrop_period(struct vfe_device *vfe, u8 wm, u8 per)
    {
    u32 reg;
    reg = readl_relaxed(vfe.base +
    VFE_0_BUS_IMAGE_MASTER_n_WR_ADDR_CFG(wm));
    reg &= ~(VFE_0_BUS_IMAGE_MASTER_n_WR_ADDR_CFG_FRM_DROP_PER_MASK);
    reg |= (per << VFE_0_BUS_IMAGE_MASTER_n_WR_ADDR_CFG_FRM_DROP_PER_SHIFT)
    & VFE_0_BUS_IMAGE_MASTER_n_WR_ADDR_CFG_FRM_DROP_PER_MASK;
    writel_relaxed(reg,
    vfe.base + VFE_0_BUS_IMAGE_MASTER_n_WR_ADDR_CFG(wm));
    }
    static void vfe_wm_set_framedrop_pattern(struct vfe_device *vfe, u8 wm,
    u32 pattern)
    {
    writel_relaxed(pattern, vfe.base + VFE_0_BUS_IMAGE_MASTER_n_WR_FRAMEDROP_PATTERN(wm));
    }
    static void vfe_wm_set_ub_cfg(struct vfe_device *vfe, u8 wm,
    u16 offset, u16 depth)
    {
    u32 reg;
    reg = (offset << VFE_0_BUS_IMAGE_MASTER_n_WR_UB_CFG_OFFSET_SHIFT) |
    depth;
    writel_relaxed(reg, vfe.base + VFE_0_BUS_IMAGE_MASTER_n_WR_UB_CFG(wm));
    }
#[no_mangle]
unsafe extern "C" fn vfe_bus_reload_wm(vfe: *mut vfe_device, wm: u8) {
    static void vfe_bus_reload_wm(struct vfe_device *vfe, u8 wm)
    {
// Enforce barrier between any outstanding register write
    wmb();
    writel_relaxed(VFE_0_BUS_CMD_Mx_RLD_CMD(wm), vfe.base + VFE_0_BUS_CMD);
// Use barrier to make sure bus reload is issued before anything else
    wmb();
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_set_ping_addr(vfe: *mut vfe_device, wm: u8, addr: u32) {
    static void vfe_wm_set_ping_addr(struct vfe_device *vfe, u8 wm, u32 addr)
    {
    writel_relaxed(addr,
    vfe.base + VFE_0_BUS_IMAGE_MASTER_n_WR_PING_ADDR(wm));
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_set_pong_addr(vfe: *mut vfe_device, wm: u8, addr: u32) {
    static void vfe_wm_set_pong_addr(struct vfe_device *vfe, u8 wm, u32 addr)
    {
    writel_relaxed(addr,
    vfe.base + VFE_0_BUS_IMAGE_MASTER_n_WR_PONG_ADDR(wm));
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_get_ping_pong_status(vfe: *mut vfe_device, wm: u8) -> c_int {
    static int vfe_wm_get_ping_pong_status(struct vfe_device *vfe, u8 wm)
    {
    u32 reg;
    reg = readl_relaxed(vfe.base + VFE_0_BUS_PING_PONG_STATUS);
    return (reg >> wm) & 0x1;
    }
#[no_mangle]
unsafe extern "C" fn vfe_bus_enable_wr_if(vfe: *mut vfe_device, enable: u8) {
    static void vfe_bus_enable_wr_if(struct vfe_device *vfe, u8 enable)
    {
    if (enable)
    writel_relaxed(0x101, vfe.base + VFE_0_BUS_CFG);
    else
    writel_relaxed(0, vfe.base + VFE_0_BUS_CFG);
    }
    static void vfe_bus_connect_wm_to_rdi(struct vfe_device *vfe, u8 wm,
    enum vfe_line_id id)
    {
    u32 reg;
    reg = VFE_0_RDI_CFG_x_MIPI_EN_BITS;
    vfe_reg_set(vfe, VFE_0_RDI_CFG_x(0), reg);
    reg = VFE_0_RDI_CFG_x_RDI_EN_BIT;
    reg |= ((3 * id) << VFE_0_RDI_CFG_x_RDI_STREAM_SEL_SHIFT) &
    VFE_0_RDI_CFG_x_RDI_STREAM_SEL_MASK;
    vfe_reg_set(vfe, VFE_0_RDI_CFG_x(id), reg);
    switch (id) {
    case VFE_LINE_RDI0:
    default:
    reg = VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_VAL_RDI0 <<
    VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_SHIFT;
    break;
    case VFE_LINE_RDI1:
    reg = VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_VAL_RDI1 <<
    VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_SHIFT;
    break;
    case VFE_LINE_RDI2:
    reg = VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_VAL_RDI2 <<
    VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_SHIFT;
    break;
    }
    if (wm % 2 == 1)
    reg <<= 16;
    vfe_reg_set(vfe, VFE_0_BUS_XBAR_CFG_x(wm), reg);
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_set_subsample(vfe: *mut vfe_device, wm: u8) {
    static void vfe_wm_set_subsample(struct vfe_device *vfe, u8 wm)
    {
    writel_relaxed(VFE_0_BUS_IMAGE_MASTER_n_WR_IRQ_SUBSAMPLE_PATTERN_DEF,
    vfe.base + VFE_0_BUS_IMAGE_MASTER_n_WR_IRQ_SUBSAMPLE_PATTERN(wm));
    }
    static void vfe_bus_disconnect_wm_from_rdi(struct vfe_device *vfe, u8 wm,
    enum vfe_line_id id)
    {
    u32 reg;
    reg = VFE_0_RDI_CFG_x_RDI_EN_BIT;
    vfe_reg_clr(vfe, VFE_0_RDI_CFG_x(id), reg);
    switch (id) {
    case VFE_LINE_RDI0:
    default:
    reg = VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_VAL_RDI0 <<
    VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_SHIFT;
    break;
    case VFE_LINE_RDI1:
    reg = VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_VAL_RDI1 <<
    VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_SHIFT;
    break;
    case VFE_LINE_RDI2:
    reg = VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_VAL_RDI2 <<
    VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_SHIFT;
    break;
    }
    if (wm % 2 == 1)
    reg <<= 16;
    vfe_reg_clr(vfe, VFE_0_BUS_XBAR_CFG_x(wm), reg);
    }
    static void vfe_set_xbar_cfg(struct vfe_device *vfe, struct vfe_output *output,
    u8 enable)
    {
    struct vfe_line *line = container_of(output, struct vfe_line, output);
    let mut p: u32 = line.video_out.active_fmt.fmt.pix_mp.pixelformat;
    u32 reg;
    switch (p) {
    case V4L2_PIX_FMT_NV12:
    case V4L2_PIX_FMT_NV21:
    case V4L2_PIX_FMT_NV16:
    case V4L2_PIX_FMT_NV61:
    reg = VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_LUMA <<
    VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_SHIFT;
    if (output.wm_idx[0] % 2 == 1)
    reg <<= 16;
    if (enable)
    vfe_reg_set(vfe,
    VFE_0_BUS_XBAR_CFG_x(output.wm_idx[0]),
    reg);
    else
    vfe_reg_clr(vfe,
    VFE_0_BUS_XBAR_CFG_x(output.wm_idx[0]),
    reg);
    reg = VFE_0_BUS_XBAR_CFG_x_M_PAIR_STREAM_EN;
    if (p == V4L2_PIX_FMT_NV12 || p == V4L2_PIX_FMT_NV16)
    reg |= VFE_0_BUS_XBAR_CFG_x_M_PAIR_STREAM_SWAP_INTER_INTRA;
    if (output.wm_idx[1] % 2 == 1)
    reg <<= 16;
    if (enable)
    vfe_reg_set(vfe,
    VFE_0_BUS_XBAR_CFG_x(output.wm_idx[1]),
    reg);
    else
    vfe_reg_clr(vfe,
    VFE_0_BUS_XBAR_CFG_x(output.wm_idx[1]),
    reg);
    break;
    case V4L2_PIX_FMT_YUYV:
    case V4L2_PIX_FMT_YVYU:
    case V4L2_PIX_FMT_VYUY:
    case V4L2_PIX_FMT_UYVY:
    reg = VFE_0_BUS_XBAR_CFG_x_M_REALIGN_BUF_EN;
    reg |= VFE_0_BUS_XBAR_CFG_x_M_PAIR_STREAM_EN;
    if (p == V4L2_PIX_FMT_YUYV || p == V4L2_PIX_FMT_YVYU)
    reg |= VFE_0_BUS_XBAR_CFG_x_M_PAIR_STREAM_SWAP_INTER_INTRA;
    if (output.wm_idx[0] % 2 == 1)
    reg <<= 16;
    if (enable)
    vfe_reg_set(vfe,
    VFE_0_BUS_XBAR_CFG_x(output.wm_idx[0]),
    reg);
    else
    vfe_reg_clr(vfe,
    VFE_0_BUS_XBAR_CFG_x(output.wm_idx[0]),
    reg);
    break;
    default:
    break;
    }
    }
    static void vfe_set_realign_cfg(struct vfe_device *vfe, struct vfe_line *line,
    u8 enable)
    {
    let mut p: u32 = line.video_out.active_fmt.fmt.pix_mp.pixelformat;
    let mut val: u32 = VFE_0_MODULE_ZOOM_EN_REALIGN_BUF;
    if (p != V4L2_PIX_FMT_YUYV && p != V4L2_PIX_FMT_YVYU &&
    p != V4L2_PIX_FMT_VYUY && p != V4L2_PIX_FMT_UYVY)
    return;
    if (enable) {
    vfe_reg_set(vfe, VFE_0_MODULE_ZOOM_EN, val);
    } else {
    vfe_reg_clr(vfe, VFE_0_MODULE_ZOOM_EN, val);
    return;
    }
    val = VFE_0_REALIGN_BUF_CFG_HSUB_ENABLE;
    if (p == V4L2_PIX_FMT_UYVY || p == V4L2_PIX_FMT_YUYV)
    val |= VFE_0_REALIGN_BUF_CFG_CR_ODD_PIXEL;
    else
    val |= VFE_0_REALIGN_BUF_CFG_CB_ODD_PIXEL;
    writel_relaxed(val, vfe.base + VFE_0_REALIGN_BUF_CFG);
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_rdi_cid(vfe: *mut vfe_device, id: enum vfe_line_id, cid: u8) {
    static void vfe_set_rdi_cid(struct vfe_device *vfe, enum vfe_line_id id, u8 cid)
    {
    vfe_reg_clr(vfe, VFE_0_RDI_CFG_x(id),
    VFE_0_RDI_CFG_x_RDI_M0_SEL_MASK);
    vfe_reg_set(vfe, VFE_0_RDI_CFG_x(id),
    cid << VFE_0_RDI_CFG_x_RDI_M0_SEL_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn vfe_reg_update(vfe: *mut vfe_device, line_id: enum vfe_line_id) {
    static void vfe_reg_update(struct vfe_device *vfe, enum vfe_line_id line_id)
    {
    vfe.reg_update |= VFE_0_REG_UPDATE_line_n(line_id);
// Enforce barrier between line update and commit
    wmb();
    writel_relaxed(vfe.reg_update, vfe.base + VFE_0_REG_UPDATE);
// Make sure register update is issued before further reg writes
    wmb();
    }
    static inline void vfe_reg_update_clear(struct vfe_device *vfe,
    enum vfe_line_id line_id)
    {
    vfe.reg_update &= ~VFE_0_REG_UPDATE_line_n(line_id);
    }
    static void vfe_enable_irq_wm_line(struct vfe_device *vfe, u8 wm,
    enum vfe_line_id line_id, u8 enable)
    {
    u32 irq_en0 = VFE_0_IRQ_MASK_0_IMAGE_MASTER_n_PING_PONG(wm) |
    VFE_0_IRQ_MASK_0_line_n_REG_UPDATE(line_id);
    u32 irq_en1 = VFE_0_IRQ_MASK_1_IMAGE_MASTER_n_BUS_OVERFLOW(wm) |
    VFE_0_IRQ_MASK_1_RDIn_SOF(line_id);
    if (enable) {
    vfe_reg_set(vfe, VFE_0_IRQ_MASK_0, irq_en0);
    vfe_reg_set(vfe, VFE_0_IRQ_MASK_1, irq_en1);
    } else {
    vfe_reg_clr(vfe, VFE_0_IRQ_MASK_0, irq_en0);
    vfe_reg_clr(vfe, VFE_0_IRQ_MASK_1, irq_en1);
    }
    }
    static void vfe_enable_irq_pix_line(struct vfe_device *vfe, u8 comp,
    enum vfe_line_id line_id, u8 enable)
    {
    struct vfe_output *output = &vfe.line[line_id].output;
    unsigned int i;
    u32 irq_en0;
    u32 irq_en1;
    let mut comp_mask: u32 = 0;
    irq_en0 = VFE_0_IRQ_MASK_0_CAMIF_SOF;
    irq_en0 |= VFE_0_IRQ_MASK_0_CAMIF_EOF;
    irq_en0 |= VFE_0_IRQ_MASK_0_IMAGE_COMPOSITE_DONE_n(comp);
    irq_en0 |= VFE_0_IRQ_MASK_0_line_n_REG_UPDATE(line_id);
    irq_en1 = VFE_0_IRQ_MASK_1_CAMIF_ERROR;
    for (i = 0; i < output.wm_num; i++) {
    irq_en1 |= VFE_0_IRQ_MASK_1_IMAGE_MASTER_n_BUS_OVERFLOW(output.wm_idx[i]);
    comp_mask |= (1 << output.wm_idx[i]) << comp * 8;
    }
    if (enable) {
    vfe_reg_set(vfe, VFE_0_IRQ_MASK_0, irq_en0);
    vfe_reg_set(vfe, VFE_0_IRQ_MASK_1, irq_en1);
    vfe_reg_set(vfe, VFE_0_IRQ_COMPOSITE_MASK_0, comp_mask);
    } else {
    vfe_reg_clr(vfe, VFE_0_IRQ_MASK_0, irq_en0);
    vfe_reg_clr(vfe, VFE_0_IRQ_MASK_1, irq_en1);
    vfe_reg_clr(vfe, VFE_0_IRQ_COMPOSITE_MASK_0, comp_mask);
    }
    }
#[no_mangle]
unsafe extern "C" fn vfe_enable_irq_common(vfe: *mut vfe_device) {
    static void vfe_enable_irq_common(struct vfe_device *vfe)
    {
    let mut irq_en0: u32 = VFE_0_IRQ_MASK_0_RESET_ACK;
    u32 irq_en1 = VFE_0_IRQ_MASK_1_VIOLATION |
    VFE_0_IRQ_MASK_1_BUS_BDG_HALT_ACK;
    vfe_reg_set(vfe, VFE_0_IRQ_MASK_0, irq_en0);
    vfe_reg_set(vfe, VFE_0_IRQ_MASK_1, irq_en1);
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_demux_cfg(vfe: *mut vfe_device, line: *mut vfe_line) {
    static void vfe_set_demux_cfg(struct vfe_device *vfe, struct vfe_line *line)
    {
    u32 val, even_cfg, odd_cfg;
    writel_relaxed(VFE_0_DEMUX_CFG_PERIOD, vfe.base + VFE_0_DEMUX_CFG);
    val = VFE_0_DEMUX_GAIN_0_CH0_EVEN | VFE_0_DEMUX_GAIN_0_CH0_ODD;
    writel_relaxed(val, vfe.base + VFE_0_DEMUX_GAIN_0);
    val = VFE_0_DEMUX_GAIN_1_CH1 | VFE_0_DEMUX_GAIN_1_CH2;
    writel_relaxed(val, vfe.base + VFE_0_DEMUX_GAIN_1);
    switch (line.fmt[MSM_VFE_PAD_SINK].code) {
    case MEDIA_BUS_FMT_YUYV8_1X16:
    even_cfg = VFE_0_DEMUX_EVEN_CFG_PATTERN_YUYV;
    odd_cfg = VFE_0_DEMUX_ODD_CFG_PATTERN_YUYV;
    break;
    case MEDIA_BUS_FMT_YVYU8_1X16:
    even_cfg = VFE_0_DEMUX_EVEN_CFG_PATTERN_YVYU;
    odd_cfg = VFE_0_DEMUX_ODD_CFG_PATTERN_YVYU;
    break;
    case MEDIA_BUS_FMT_UYVY8_1X16:
    default:
    even_cfg = VFE_0_DEMUX_EVEN_CFG_PATTERN_UYVY;
    odd_cfg = VFE_0_DEMUX_ODD_CFG_PATTERN_UYVY;
    break;
    case MEDIA_BUS_FMT_VYUY8_1X16:
    even_cfg = VFE_0_DEMUX_EVEN_CFG_PATTERN_VYUY;
    odd_cfg = VFE_0_DEMUX_ODD_CFG_PATTERN_VYUY;
    break;
    }
    writel_relaxed(even_cfg, vfe.base + VFE_0_DEMUX_EVEN_CFG);
    writel_relaxed(odd_cfg, vfe.base + VFE_0_DEMUX_ODD_CFG);
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_scale_cfg(vfe: *mut vfe_device, line: *mut vfe_line) {
    static void vfe_set_scale_cfg(struct vfe_device *vfe, struct vfe_line *line)
    {
    let mut p: u32 = line.video_out.active_fmt.fmt.pix_mp.pixelformat;
    u32 reg;
    u16 input, output;
    u8 interp_reso;
    u32 phase_mult;
    writel_relaxed(0x3, vfe.base + VFE_0_SCALE_ENC_Y_CFG);
    input = line.fmt[MSM_VFE_PAD_SINK].width - 1;
    output = line.compose.width - 1;
    reg = (output << 16) | input;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_Y_H_IMAGE_SIZE);
    interp_reso = vfe_calc_interp_reso(input, output);
    phase_mult = input * (1 << (14 + interp_reso)) / output;
    reg = (interp_reso << 28) | phase_mult;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_Y_H_PHASE);
    input = line.fmt[MSM_VFE_PAD_SINK].height - 1;
    output = line.compose.height - 1;
    reg = (output << 16) | input;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_Y_V_IMAGE_SIZE);
    interp_reso = vfe_calc_interp_reso(input, output);
    phase_mult = input * (1 << (14 + interp_reso)) / output;
    reg = (interp_reso << 28) | phase_mult;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_Y_V_PHASE);
    writel_relaxed(0x3, vfe.base + VFE_0_SCALE_ENC_CBCR_CFG);
    input = line.fmt[MSM_VFE_PAD_SINK].width - 1;
    output = line.compose.width / 2 - 1;
    reg = (output << 16) | input;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_CBCR_H_IMAGE_SIZE);
    interp_reso = vfe_calc_interp_reso(input, output);
    phase_mult = input * (1 << (14 + interp_reso)) / output;
    reg = (interp_reso << 28) | phase_mult;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_CBCR_H_PHASE);
    input = line.fmt[MSM_VFE_PAD_SINK].height - 1;
    output = line.compose.height - 1;
    if (p == V4L2_PIX_FMT_NV12 || p == V4L2_PIX_FMT_NV21)
    output = line.compose.height / 2 - 1;
    reg = (output << 16) | input;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_CBCR_V_IMAGE_SIZE);
    interp_reso = vfe_calc_interp_reso(input, output);
    phase_mult = input * (1 << (14 + interp_reso)) / output;
    reg = (interp_reso << 28) | phase_mult;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_CBCR_V_PHASE);
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_crop_cfg(vfe: *mut vfe_device, line: *mut vfe_line) {
    static void vfe_set_crop_cfg(struct vfe_device *vfe, struct vfe_line *line)
    {
    let mut p: u32 = line.video_out.active_fmt.fmt.pix_mp.pixelformat;
    u32 reg;
    u16 first, last;
    first = line.crop.left;
    last = line.crop.left + line.crop.width - 1;
    reg = (first << 16) | last;
    writel_relaxed(reg, vfe.base + VFE_0_CROP_ENC_Y_WIDTH);
    first = line.crop.top;
    last = line.crop.top + line.crop.height - 1;
    reg = (first << 16) | last;
    writel_relaxed(reg, vfe.base + VFE_0_CROP_ENC_Y_HEIGHT);
    first = line.crop.left / 2;
    last = line.crop.left / 2 + line.crop.width / 2 - 1;
    reg = (first << 16) | last;
    writel_relaxed(reg, vfe.base + VFE_0_CROP_ENC_CBCR_WIDTH);
    first = line.crop.top;
    last = line.crop.top + line.crop.height - 1;
    if (p == V4L2_PIX_FMT_NV12 || p == V4L2_PIX_FMT_NV21) {
    first = line.crop.top / 2;
    last = line.crop.top / 2 + line.crop.height / 2 - 1;
    }
    reg = (first << 16) | last;
    writel_relaxed(reg, vfe.base + VFE_0_CROP_ENC_CBCR_HEIGHT);
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_clamp_cfg(vfe: *mut vfe_device) {
    static void vfe_set_clamp_cfg(struct vfe_device *vfe)
    {
    u32 val = VFE_0_CLAMP_ENC_MAX_CFG_CH0 |
    VFE_0_CLAMP_ENC_MAX_CFG_CH1 |
    VFE_0_CLAMP_ENC_MAX_CFG_CH2;
    writel_relaxed(val, vfe.base + VFE_0_CLAMP_ENC_MAX_CFG);
    val = VFE_0_CLAMP_ENC_MIN_CFG_CH0 |
    VFE_0_CLAMP_ENC_MIN_CFG_CH1 |
    VFE_0_CLAMP_ENC_MIN_CFG_CH2;
    writel_relaxed(val, vfe.base + VFE_0_CLAMP_ENC_MIN_CFG);
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_cgc_override(vfe: *mut vfe_device, wm: u8, enable: u8) {
    static void vfe_set_cgc_override(struct vfe_device *vfe, u8 wm, u8 enable)
    {
// empty
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_camif_cfg(vfe: *mut vfe_device, line: *mut vfe_line) {
    static void vfe_set_camif_cfg(struct vfe_device *vfe, struct vfe_line *line)
    {
    u32 val;
    switch (line.fmt[MSM_VFE_PAD_SINK].code) {
    case MEDIA_BUS_FMT_YUYV8_1X16:
    val = VFE_0_CORE_CFG_PIXEL_PATTERN_YCBYCR;
    break;
    case MEDIA_BUS_FMT_YVYU8_1X16:
    val = VFE_0_CORE_CFG_PIXEL_PATTERN_YCRYCB;
    break;
    case MEDIA_BUS_FMT_UYVY8_1X16:
    default:
    val = VFE_0_CORE_CFG_PIXEL_PATTERN_CBYCRY;
    break;
    case MEDIA_BUS_FMT_VYUY8_1X16:
    val = VFE_0_CORE_CFG_PIXEL_PATTERN_CRYCBY;
    break;
    }
    val |= VFE_0_CORE_CFG_COMPOSITE_REG_UPDATE_EN;
    writel_relaxed(val, vfe.base + VFE_0_CORE_CFG);
    val = line.fmt[MSM_VFE_PAD_SINK].width * 2 - 1;
    val |= (line.fmt[MSM_VFE_PAD_SINK].height - 1) << 16;
    writel_relaxed(val, vfe.base + VFE_0_CAMIF_FRAME_CFG);
    val = line.fmt[MSM_VFE_PAD_SINK].width * 2 - 1;
    writel_relaxed(val, vfe.base + VFE_0_CAMIF_WINDOW_WIDTH_CFG);
    val = line.fmt[MSM_VFE_PAD_SINK].height - 1;
    writel_relaxed(val, vfe.base + VFE_0_CAMIF_WINDOW_HEIGHT_CFG);
    val = 0xffffffff;
    writel_relaxed(val, vfe.base + VFE_0_CAMIF_SUBSAMPLE_CFG);
    val = 0xffffffff;
    writel_relaxed(val, vfe.base + VFE_0_CAMIF_IRQ_FRAMEDROP_PATTERN);
    val = 0xffffffff;
    writel_relaxed(val, vfe.base + VFE_0_CAMIF_IRQ_SUBSAMPLE_PATTERN);
    val = VFE_0_RDI_CFG_x_MIPI_EN_BITS;
    vfe_reg_set(vfe, VFE_0_RDI_CFG_x(0), val);
    val = VFE_0_CAMIF_CFG_VFE_OUTPUT_EN;
    writel_relaxed(val, vfe.base + VFE_0_CAMIF_CFG);
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_camif_cmd(vfe: *mut vfe_device, enable: u8) {
    static void vfe_set_camif_cmd(struct vfe_device *vfe, u8 enable)
    {
    u32 cmd;
    cmd = VFE_0_CAMIF_CMD_CLEAR_CAMIF_STATUS | VFE_0_CAMIF_CMD_NO_CHANGE;
    writel_relaxed(cmd, vfe.base + VFE_0_CAMIF_CMD);
// Make sure camif command is issued written before it is changed again
    wmb();
    if (enable)
    cmd = VFE_0_CAMIF_CMD_ENABLE_FRAME_BOUNDARY;
    else
    cmd = VFE_0_CAMIF_CMD_DISABLE_FRAME_BOUNDARY;
    writel_relaxed(cmd, vfe.base + VFE_0_CAMIF_CMD);
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_module_cfg(vfe: *mut vfe_device, enable: u8) {
    static void vfe_set_module_cfg(struct vfe_device *vfe, u8 enable)
    {
    u32 val_lens = VFE_0_MODULE_LENS_EN_DEMUX |
    VFE_0_MODULE_LENS_EN_CHROMA_UPSAMPLE;
    u32 val_zoom = VFE_0_MODULE_ZOOM_EN_SCALE_ENC |
    VFE_0_MODULE_ZOOM_EN_CROP_ENC;
    if (enable) {
    vfe_reg_set(vfe, VFE_0_MODULE_LENS_EN, val_lens);
    vfe_reg_set(vfe, VFE_0_MODULE_ZOOM_EN, val_zoom);
    } else {
    vfe_reg_clr(vfe, VFE_0_MODULE_LENS_EN, val_lens);
    vfe_reg_clr(vfe, VFE_0_MODULE_ZOOM_EN, val_zoom);
    }
    }
#[no_mangle]
unsafe extern "C" fn vfe_camif_wait_for_stop(vfe: *mut vfe_device, dev: *mut device) -> c_int {
    static int vfe_camif_wait_for_stop(struct vfe_device *vfe, struct device *dev)
    {
    u32 val;
    int ret;
    ret = readl_poll_timeout(vfe.base + VFE_0_CAMIF_STATUS,
    val,
    (val & VFE_0_CAMIF_STATUS_HALT),
    CAMIF_TIMEOUT_SLEEP_US,
    CAMIF_TIMEOUT_ALL_US);
    if (ret < 0)
    dev_err(dev, "%s: camif stop timeout\n", __func__);
    return ret;
    }
//
// vfe_isr - VFE module interrupt handler
// @irq: Interrupt line
// @dev: VFE device
//
// Return IRQ_HANDLED on success
//
#[no_mangle]
unsafe extern "C" fn vfe_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t vfe_isr(int irq, void *dev)
    {
    struct vfe_device *vfe = dev;
    u32 value0, value1;
    int i, j;
    vfe.res.hw_ops.isr_read(vfe, &value0, &value1);
    dev_dbg(vfe.camss.dev, "VFE: status0 = 0x%08x, status1 = 0x%08x\n",
    value0, value1);
    if (value0 & VFE_0_IRQ_STATUS_0_RESET_ACK)
    vfe.isr_ops.reset_ack(vfe);
    if (value1 & VFE_0_IRQ_STATUS_1_VIOLATION)
    vfe.res.hw_ops.violation_read(vfe);
    if (value1 & VFE_0_IRQ_STATUS_1_BUS_BDG_HALT_ACK)
    vfe.isr_ops.halt_ack(vfe);
    for (i = VFE_LINE_RDI0; i < vfe.res.line_num; i++)
    if (value0 & VFE_0_IRQ_STATUS_0_line_n_REG_UPDATE(i))
    vfe.isr_ops.reg_update(vfe, i);
    if (value0 & VFE_0_IRQ_STATUS_0_CAMIF_SOF)
    vfe.isr_ops.sof(vfe, VFE_LINE_PIX);
    for (i = VFE_LINE_RDI0; i <= VFE_LINE_RDI2; i++)
    if (value1 & VFE_0_IRQ_STATUS_1_RDIn_SOF(i))
    vfe.isr_ops.sof(vfe, i);
    for (i = 0; i < MSM_VFE_COMPOSITE_IRQ_NUM; i++)
    if (value0 & VFE_0_IRQ_STATUS_0_IMAGE_COMPOSITE_DONE_n(i)) {
    vfe.isr_ops.comp_done(vfe, i);
    for (j = 0; j < ARRAY_SIZE(vfe.wm_output_map); j++)
    if (vfe.wm_output_map[j] == VFE_LINE_PIX)
    value0 &= ~VFE_0_IRQ_MASK_0_IMAGE_MASTER_n_PING_PONG(j);
    }
    for (i = 0; i < MSM_VFE_IMAGE_MASTERS_NUM; i++)
    if (value0 & VFE_0_IRQ_STATUS_0_IMAGE_MASTER_n_PING_PONG(i))
    vfe.isr_ops.wm_done(vfe, i);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn vfe_get_ub_size(vfe_id: u8) -> u16 {
    static u16 vfe_get_ub_size(u8 vfe_id)
    {
// On VFE4.8 the ub-size is the same on both instances
    return MSM_VFE_VFE0_UB_SIZE_RDI;
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_enable(vfe: *mut vfe_device, wm: u8, enable: u8) {
    static void vfe_wm_enable(struct vfe_device *vfe, u8 wm, u8 enable)
    {
    if (enable)
    writel_relaxed(2 << VFE_0_BUS_IMAGE_MASTER_n_SHIFT(wm),
    vfe.base + VFE_0_BUS_IMAGE_MASTER_CMD);
    else
    writel_relaxed(1 << VFE_0_BUS_IMAGE_MASTER_n_SHIFT(wm),
    vfe.base + VFE_0_BUS_IMAGE_MASTER_CMD);
// The WM must be enabled before sending other commands
    wmb();
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_qos(vfe: *mut vfe_device) {
    static void vfe_set_qos(struct vfe_device *vfe)
    {
    let mut val: u32 = VFE_0_BUS_BDG_QOS_CFG_0_CFG;
    let mut val3: u32 = VFE_0_BUS_BDG_QOS_CFG_3_CFG;
    let mut val4: u32 = VFE_0_BUS_BDG_QOS_CFG_4_CFG;
    let mut val7: u32 = VFE_0_BUS_BDG_QOS_CFG_7_CFG;
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_QOS_CFG_0);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_QOS_CFG_1);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_QOS_CFG_2);
    writel_relaxed(val3, vfe.base + VFE_0_BUS_BDG_QOS_CFG_3);
    writel_relaxed(val4, vfe.base + VFE_0_BUS_BDG_QOS_CFG_4);
    writel_relaxed(val4, vfe.base + VFE_0_BUS_BDG_QOS_CFG_5);
    writel_relaxed(val4, vfe.base + VFE_0_BUS_BDG_QOS_CFG_6);
    writel_relaxed(val7, vfe.base + VFE_0_BUS_BDG_QOS_CFG_7);
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_ds(vfe: *mut vfe_device) {
    static void vfe_set_ds(struct vfe_device *vfe)
    {
    let mut val: u32 = VFE_0_BUS_BDG_DS_CFG_0_CFG;
    let mut val16: u32 = VFE_0_BUS_BDG_DS_CFG_16_CFG;
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_0);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_1);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_2);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_3);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_4);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_5);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_6);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_7);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_8);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_9);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_10);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_11);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_12);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_13);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_14);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_DS_CFG_15);
    writel_relaxed(val16, vfe.base + VFE_0_BUS_BDG_DS_CFG_16);
    }
#[no_mangle]
unsafe extern "C" fn vfe_isr_read(vfe: *mut vfe_device, value0: *mut u32, value1: *mut u32) {
    static void vfe_isr_read(struct vfe_device *vfe, u32 *value0, u32 *value1)
    {
// value0 = readl_relaxed(vfe->base + VFE_0_IRQ_STATUS_0);
// value1 = readl_relaxed(vfe->base + VFE_0_IRQ_STATUS_1);
    writel_relaxed(*value0, vfe.base + VFE_0_IRQ_CLEAR_0);
    writel_relaxed(*value1, vfe.base + VFE_0_IRQ_CLEAR_1);
// Enforce barrier between local & global IRQ clear
    wmb();
    writel_relaxed(VFE_0_IRQ_CMD_GLOBAL_CLEAR, vfe.base + VFE_0_IRQ_CMD);
    }
#[no_mangle]
unsafe extern "C" fn vfe_violation_read(vfe: *mut vfe_device) {
    static void vfe_violation_read(struct vfe_device *vfe)
    {
    let mut violation: u32 = readl_relaxed(vfe.base + VFE_0_VIOLATION_STATUS);
    pr_err_ratelimited("VFE: violation = 0x%08x\n", violation);
    }
    static const struct vfe_hw_ops_gen1 vfe_ops_gen1_4_8 = {
    .bus_connect_wm_to_rdi = vfe_bus_connect_wm_to_rdi,
    .bus_disconnect_wm_from_rdi = vfe_bus_disconnect_wm_from_rdi,
    .bus_enable_wr_if = vfe_bus_enable_wr_if,
    .bus_reload_wm = vfe_bus_reload_wm,
    .camif_wait_for_stop = vfe_camif_wait_for_stop,
    .enable_irq_common = vfe_enable_irq_common,
    .enable_irq_pix_line = vfe_enable_irq_pix_line,
    .enable_irq_wm_line = vfe_enable_irq_wm_line,
    .get_ub_size = vfe_get_ub_size,
    .halt_clear = vfe_halt_clear,
    .halt_request = vfe_halt_request,
    .set_camif_cfg = vfe_set_camif_cfg,
    .set_camif_cmd = vfe_set_camif_cmd,
    .set_cgc_override = vfe_set_cgc_override,
    .set_clamp_cfg = vfe_set_clamp_cfg,
    .set_crop_cfg = vfe_set_crop_cfg,
    .set_demux_cfg = vfe_set_demux_cfg,
    .set_ds = vfe_set_ds,
    .set_module_cfg = vfe_set_module_cfg,
    .set_qos = vfe_set_qos,
    .set_rdi_cid = vfe_set_rdi_cid,
    .set_realign_cfg = vfe_set_realign_cfg,
    .set_scale_cfg = vfe_set_scale_cfg,
    .set_xbar_cfg = vfe_set_xbar_cfg,
    .wm_enable = vfe_wm_enable,
    .wm_frame_based = vfe_wm_frame_based,
    .wm_get_ping_pong_status = vfe_wm_get_ping_pong_status,
    .wm_line_based = vfe_wm_line_based,
    .wm_set_framedrop_pattern = vfe_wm_set_framedrop_pattern,
    .wm_set_framedrop_period = vfe_wm_set_framedrop_period,
    .wm_set_ping_addr = vfe_wm_set_ping_addr,
    .wm_set_pong_addr = vfe_wm_set_pong_addr,
    .wm_set_subsample = vfe_wm_set_subsample,
    .wm_set_ub_cfg = vfe_wm_set_ub_cfg,
    };
#[no_mangle]
unsafe extern "C" fn vfe_subdev_init(dev: *mut device, vfe: *mut vfe_device) {
    static void vfe_subdev_init(struct device *dev, struct vfe_device *vfe)
    {
    vfe.isr_ops = vfe_isr_ops_gen1;
    vfe.ops_gen1 = &vfe_ops_gen1_4_8;
    vfe.video_ops = vfe_video_ops_gen1;
    }
    const struct vfe_hw_ops vfe_ops_4_8 = {
    .global_reset = vfe_global_reset,
    .hw_version = vfe_hw_version,
    .isr_read = vfe_isr_read,
    .isr = vfe_isr,
    .pm_domain_off = vfe_pm_domain_off,
    .pm_domain_on = vfe_pm_domain_on,
    .reg_update_clear = vfe_reg_update_clear,
    .reg_update = vfe_reg_update,
    .subdev_init = vfe_subdev_init,
    .vfe_disable = vfe_gen1_disable,
    .vfe_enable = vfe_gen1_enable,
    .vfe_halt = vfe_gen1_halt,
    .violation_read = vfe_violation_read,
    };
