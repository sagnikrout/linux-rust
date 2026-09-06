//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/camss/camss-vfe-4-1.c
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
// camss-vfe-4-1.c
//
// Qualcomm MSM Camera Subsystem - VFE (Video Front End) Module v4.1
//
// Copyright (c) 2013-2015, The Linux Foundation. All rights reserved.
// Copyright (C) 2015-2018 Linaro Ltd.
//

pub const VFE_0_HW_VERSION: c_uint = 0x000;
pub const VFE_0_GLOBAL_RESET_CMD: c_uint = 0x00c;

pub const VFE_0_MODULE_CFG: c_uint = 0x018;

pub const VFE_0_CORE_CFG: c_uint = 0x01c;
pub const VFE_0_CORE_CFG_PIXEL_PATTERN_YCBYCR: c_uint = 0x4;
pub const VFE_0_CORE_CFG_PIXEL_PATTERN_YCRYCB: c_uint = 0x5;
pub const VFE_0_CORE_CFG_PIXEL_PATTERN_CBYCRY: c_uint = 0x6;
pub const VFE_0_CORE_CFG_PIXEL_PATTERN_CRYCBY: c_uint = 0x7;
pub const VFE_0_IRQ_CMD: c_uint = 0x024;

pub const VFE_0_IRQ_MASK_0: c_uint = 0x028;

    ((n) == VFE_LINE_PIX ? BIT(4) : VFE_0_IRQ_MASK_0_RDIn_REG_UPDATE(n))

pub const VFE_0_IRQ_MASK_1: c_uint = 0x02c;

pub const VFE_0_IRQ_CLEAR_0: c_uint = 0x030;
pub const VFE_0_IRQ_CLEAR_1: c_uint = 0x034;
pub const VFE_0_IRQ_STATUS_0: c_uint = 0x038;

    ((n) == VFE_LINE_PIX ? BIT(4) : VFE_0_IRQ_STATUS_0_RDIn_REG_UPDATE(n))

pub const VFE_0_IRQ_STATUS_1: c_uint = 0x03c;

pub const VFE_0_IRQ_COMPOSITE_MASK_0: c_uint = 0x40;
pub const VFE_0_VIOLATION_STATUS: c_uint = 0x48;
pub const VFE_0_BUS_CMD: c_uint = 0x4c;

pub const VFE_0_BUS_CFG: c_uint = 0x050;

pub const VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_SHIFT: c_int = 8;
pub const VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_LUMA: c_int = 0;
pub const VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_VAL_RDI0: c_int = 5;
pub const VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_VAL_RDI1: c_int = 6;
pub const VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_VAL_RDI2: c_int = 7;

pub const VFE_0_BUS_IMAGE_MASTER_n_WR_CFG_WR_PATH_SHIFT: c_int = 0;
pub const VFE_0_BUS_IMAGE_MASTER_n_WR_CFG_FRM_BASED_SHIFT: c_int = 1;

pub const VFE_0_BUS_IMAGE_MASTER_n_WR_ADDR_CFG_FRM_DROP_PER_SHIFT: c_int = 2;

pub const VFE_0_BUS_IMAGE_MASTER_n_WR_UB_CFG_OFFSET_SHIFT: c_int = 16;

    (0x088 + 0x24 * (n))

    (0x08c + 0x24 * (n))
pub const VFE_0_BUS_IMAGE_MASTER_n_WR_IRQ_SUBSAMPLE_PATTERN_DEF: c_uint = 0xffffffff;
pub const VFE_0_BUS_PING_PONG_STATUS: c_uint = 0x268;
pub const VFE_0_BUS_BDG_CMD: c_uint = 0x2c0;
pub const VFE_0_BUS_BDG_CMD_HALT_REQ: c_int = 1;
pub const VFE_0_BUS_BDG_QOS_CFG_0: c_uint = 0x2c4;
pub const VFE_0_BUS_BDG_QOS_CFG_0_CFG: c_uint = 0xaaa5aaa5;
pub const VFE_0_BUS_BDG_QOS_CFG_1: c_uint = 0x2c8;
pub const VFE_0_BUS_BDG_QOS_CFG_2: c_uint = 0x2cc;
pub const VFE_0_BUS_BDG_QOS_CFG_3: c_uint = 0x2d0;
pub const VFE_0_BUS_BDG_QOS_CFG_4: c_uint = 0x2d4;
pub const VFE_0_BUS_BDG_QOS_CFG_5: c_uint = 0x2d8;
pub const VFE_0_BUS_BDG_QOS_CFG_6: c_uint = 0x2dc;
pub const VFE_0_BUS_BDG_QOS_CFG_7: c_uint = 0x2e0;
pub const VFE_0_BUS_BDG_QOS_CFG_7_CFG: c_uint = 0x0001aaa5;

pub const VFE_0_RDI_CFG_x_RDI_STREAM_SEL_SHIFT: c_int = 28;

pub const VFE_0_RDI_CFG_x_RDI_M0_SEL_SHIFT: c_int = 4;

pub const VFE_0_RDI_CFG_x_MIPI_EN_BITS: c_uint = 0x3;

pub const VFE_0_CAMIF_CMD: c_uint = 0x2f4;
pub const VFE_0_CAMIF_CMD_DISABLE_FRAME_BOUNDARY: c_int = 0;
pub const VFE_0_CAMIF_CMD_ENABLE_FRAME_BOUNDARY: c_int = 1;
pub const VFE_0_CAMIF_CMD_NO_CHANGE: c_int = 3;

pub const VFE_0_CAMIF_CFG: c_uint = 0x2f8;

pub const VFE_0_CAMIF_FRAME_CFG: c_uint = 0x300;
pub const VFE_0_CAMIF_WINDOW_WIDTH_CFG: c_uint = 0x304;
pub const VFE_0_CAMIF_WINDOW_HEIGHT_CFG: c_uint = 0x308;
pub const VFE_0_CAMIF_SUBSAMPLE_CFG_0: c_uint = 0x30c;
pub const VFE_0_CAMIF_IRQ_SUBSAMPLE_PATTERN: c_uint = 0x314;
pub const VFE_0_CAMIF_STATUS: c_uint = 0x31c;

pub const VFE_0_REG_UPDATE: c_uint = 0x378;

    ((n) == VFE_LINE_PIX ? 1 : VFE_0_REG_UPDATE_RDIn(n))
pub const VFE_0_DEMUX_CFG: c_uint = 0x424;
pub const VFE_0_DEMUX_CFG_PERIOD: c_uint = 0x3;
pub const VFE_0_DEMUX_GAIN_0: c_uint = 0x428;

pub const VFE_0_DEMUX_GAIN_1: c_uint = 0x42c;

pub const VFE_0_DEMUX_EVEN_CFG: c_uint = 0x438;
pub const VFE_0_DEMUX_EVEN_CFG_PATTERN_YUYV: c_uint = 0x9cac;
pub const VFE_0_DEMUX_EVEN_CFG_PATTERN_YVYU: c_uint = 0xac9c;
pub const VFE_0_DEMUX_EVEN_CFG_PATTERN_UYVY: c_uint = 0xc9ca;
pub const VFE_0_DEMUX_EVEN_CFG_PATTERN_VYUY: c_uint = 0xcac9;
pub const VFE_0_DEMUX_ODD_CFG: c_uint = 0x43c;
pub const VFE_0_DEMUX_ODD_CFG_PATTERN_YUYV: c_uint = 0x9cac;
pub const VFE_0_DEMUX_ODD_CFG_PATTERN_YVYU: c_uint = 0xac9c;
pub const VFE_0_DEMUX_ODD_CFG_PATTERN_UYVY: c_uint = 0xc9ca;
pub const VFE_0_DEMUX_ODD_CFG_PATTERN_VYUY: c_uint = 0xcac9;
pub const VFE_0_SCALE_ENC_Y_CFG: c_uint = 0x75c;
pub const VFE_0_SCALE_ENC_Y_H_IMAGE_SIZE: c_uint = 0x760;
pub const VFE_0_SCALE_ENC_Y_H_PHASE: c_uint = 0x764;
pub const VFE_0_SCALE_ENC_Y_V_IMAGE_SIZE: c_uint = 0x76c;
pub const VFE_0_SCALE_ENC_Y_V_PHASE: c_uint = 0x770;
pub const VFE_0_SCALE_ENC_CBCR_CFG: c_uint = 0x778;
pub const VFE_0_SCALE_ENC_CBCR_H_IMAGE_SIZE: c_uint = 0x77c;
pub const VFE_0_SCALE_ENC_CBCR_H_PHASE: c_uint = 0x780;
pub const VFE_0_SCALE_ENC_CBCR_V_IMAGE_SIZE: c_uint = 0x790;
pub const VFE_0_SCALE_ENC_CBCR_V_PHASE: c_uint = 0x794;
pub const VFE_0_CROP_ENC_Y_WIDTH: c_uint = 0x854;
pub const VFE_0_CROP_ENC_Y_HEIGHT: c_uint = 0x858;
pub const VFE_0_CROP_ENC_CBCR_WIDTH: c_uint = 0x85c;
pub const VFE_0_CROP_ENC_CBCR_HEIGHT: c_uint = 0x860;
pub const VFE_0_CLAMP_ENC_MAX_CFG: c_uint = 0x874;

pub const VFE_0_CLAMP_ENC_MIN_CFG: c_uint = 0x878;

pub const VFE_0_CGC_OVERRIDE_1: c_uint = 0x974;

pub const CAMIF_TIMEOUT_SLEEP_US: c_int = 1000;
pub const CAMIF_TIMEOUT_ALL_US: c_int = 1000000;
pub const MSM_VFE_VFE0_UB_SIZE: c_int = 1023;

#[no_mangle]
unsafe extern "C" fn vfe_get_ub_size(vfe_id: u8) -> u16 {
    static u16 vfe_get_ub_size(u8 vfe_id)
    {
    if (vfe_id == 0)
    return MSM_VFE_VFE0_UB_SIZE_RDI;
    return 0;
    }
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
    u32 reset_bits = VFE_0_GLOBAL_RESET_CMD_TESTGEN		|
    VFE_0_GLOBAL_RESET_CMD_BUS_MISR	|
    VFE_0_GLOBAL_RESET_CMD_PM		|
    VFE_0_GLOBAL_RESET_CMD_TIMER		|
    VFE_0_GLOBAL_RESET_CMD_REGISTER	|
    VFE_0_GLOBAL_RESET_CMD_BUS_BDG		|
    VFE_0_GLOBAL_RESET_CMD_BUS		|
    VFE_0_GLOBAL_RESET_CMD_CAMIF		|
    VFE_0_GLOBAL_RESET_CMD_CORE;
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
unsafe extern "C" fn vfe_wm_enable(vfe: *mut vfe_device, wm: u8, enable: u8) {
    static void vfe_wm_enable(struct vfe_device *vfe, u8 wm, u8 enable)
    {
    if (enable)
    vfe_reg_set(vfe, VFE_0_BUS_IMAGE_MASTER_n_WR_CFG(wm),
    1 << VFE_0_BUS_IMAGE_MASTER_n_WR_CFG_WR_PATH_SHIFT);
    else
    vfe_reg_clr(vfe, VFE_0_BUS_IMAGE_MASTER_n_WR_CFG(wm),
    1 << VFE_0_BUS_IMAGE_MASTER_n_WR_CFG_WR_PATH_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_frame_based(vfe: *mut vfe_device, wm: u8, enable: u8) {
    static void vfe_wm_frame_based(struct vfe_device *vfe, u8 wm, u8 enable)
    {
    if (enable)
    vfe_reg_set(vfe, VFE_0_BUS_IMAGE_MASTER_n_WR_CFG(wm),
    1 << VFE_0_BUS_IMAGE_MASTER_n_WR_CFG_FRM_BASED_SHIFT);
    else
    vfe_reg_clr(vfe, VFE_0_BUS_IMAGE_MASTER_n_WR_CFG(wm),
    1 << VFE_0_BUS_IMAGE_MASTER_n_WR_CFG_FRM_BASED_SHIFT);
    }
    static void vfe_get_wm_sizes(struct v4l2_pix_format_mplane *pix, u8 plane,
    u16 *width, u16 *height, u16 *bytesperline)
    {
// width = pix->width;
// height = pix->height;
// bytesperline = pix->plane_fmt[0].bytesperline;
    if (pix.pixelformat == V4L2_PIX_FMT_NV12 ||
    pix.pixelformat == V4L2_PIX_FMT_NV21)
    if (plane == 1)
// height /= 2;
    }
    static void vfe_wm_line_based(struct vfe_device *vfe, u32 wm,
    struct v4l2_pix_format_mplane *pix,
    u8 plane, u32 enable)
    {
    u32 reg;
    if (enable) {
    let mut width: u16 = 0, height = 0, bytesperline = 0, wpl;
    vfe_get_wm_sizes(pix, plane, &width, &height, &bytesperline);
    wpl = vfe_word_per_line(pix.pixelformat, width);
    reg = height - 1;
    reg |= ((wpl + 1) / 2 - 1) << 16;
    writel_relaxed(reg, vfe.base +
    VFE_0_BUS_IMAGE_MASTER_n_WR_IMAGE_SIZE(wm));
    wpl = vfe_word_per_line(pix.pixelformat, bytesperline);
    reg = 0x3;
    reg |= (height - 1) << 4;
    reg |= wpl << 16;
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
    writel_relaxed(pattern,
    vfe.base + VFE_0_BUS_IMAGE_MASTER_n_WR_FRAMEDROP_PATTERN(wm));
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
    wmb();
    writel_relaxed(VFE_0_BUS_CMD_Mx_RLD_CMD(wm), vfe.base + VFE_0_BUS_CMD);
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
    writel_relaxed(0x10000009, vfe.base + VFE_0_BUS_CFG);
    else
    writel_relaxed(0, vfe.base + VFE_0_BUS_CFG);
    }
    static void vfe_bus_connect_wm_to_rdi(struct vfe_device *vfe, u8 wm,
    enum vfe_line_id id)
    {
    u32 reg;
    reg = VFE_0_RDI_CFG_x_MIPI_EN_BITS;
    reg |= VFE_0_RDI_CFG_x_RDI_Mr_FRAME_BASED_EN(id);
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
    vfe.base +
    VFE_0_BUS_IMAGE_MASTER_n_WR_IRQ_SUBSAMPLE_PATTERN(wm));
    }
    static void vfe_bus_disconnect_wm_from_rdi(struct vfe_device *vfe, u8 wm,
    enum vfe_line_id id)
    {
    u32 reg;
    reg = VFE_0_RDI_CFG_x_RDI_Mr_FRAME_BASED_EN(id);
    vfe_reg_clr(vfe, VFE_0_RDI_CFG_x(0), reg);
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
    unsigned int i;
    for (i = 0; i < output.wm_num; i++) {
    if (i == 0) {
    reg = VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_LUMA <<
    VFE_0_BUS_XBAR_CFG_x_M_SINGLE_STREAM_SEL_SHIFT;
    } else if (i == 1) {
    reg = VFE_0_BUS_XBAR_CFG_x_M_PAIR_STREAM_EN;
    if (p == V4L2_PIX_FMT_NV12 || p == V4L2_PIX_FMT_NV16)
    reg |= VFE_0_BUS_XBAR_CFG_x_M_PAIR_STREAM_SWAP_INTER_INTRA;
    } else {
// On current devices output->wm_num is always <= 2
    break;
    }
    if (output.wm_idx[i] % 2 == 1)
    reg <<= 16;
    if (enable)
    vfe_reg_set(vfe,
    VFE_0_BUS_XBAR_CFG_x(output.wm_idx[i]),
    reg);
    else
    vfe_reg_clr(vfe,
    VFE_0_BUS_XBAR_CFG_x(output.wm_idx[i]),
    reg);
    }
    }
    static void vfe_set_realign_cfg(struct vfe_device *vfe, struct vfe_line *line,
    u8 enable)
    {
// empty
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
    wmb();
    writel_relaxed(vfe.reg_update, vfe.base + VFE_0_REG_UPDATE);
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
    irq_en1 |= VFE_0_IRQ_MASK_1_IMAGE_MASTER_n_BUS_OVERFLOW(
    output.wm_idx[i]);
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
    input = line.fmt[MSM_VFE_PAD_SINK].width;
    output = line.compose.width;
    reg = (output << 16) | input;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_Y_H_IMAGE_SIZE);
    interp_reso = vfe_calc_interp_reso(input, output);
    phase_mult = input * (1 << (13 + interp_reso)) / output;
    reg = (interp_reso << 20) | phase_mult;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_Y_H_PHASE);
    input = line.fmt[MSM_VFE_PAD_SINK].height;
    output = line.compose.height;
    reg = (output << 16) | input;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_Y_V_IMAGE_SIZE);
    interp_reso = vfe_calc_interp_reso(input, output);
    phase_mult = input * (1 << (13 + interp_reso)) / output;
    reg = (interp_reso << 20) | phase_mult;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_Y_V_PHASE);
    writel_relaxed(0x3, vfe.base + VFE_0_SCALE_ENC_CBCR_CFG);
    input = line.fmt[MSM_VFE_PAD_SINK].width;
    output = line.compose.width / 2;
    reg = (output << 16) | input;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_CBCR_H_IMAGE_SIZE);
    interp_reso = vfe_calc_interp_reso(input, output);
    phase_mult = input * (1 << (13 + interp_reso)) / output;
    reg = (interp_reso << 20) | phase_mult;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_CBCR_H_PHASE);
    input = line.fmt[MSM_VFE_PAD_SINK].height;
    output = line.compose.height;
    if (p == V4L2_PIX_FMT_NV12 || p == V4L2_PIX_FMT_NV21)
    output = line.compose.height / 2;
    reg = (output << 16) | input;
    writel_relaxed(reg, vfe.base + VFE_0_SCALE_ENC_CBCR_V_IMAGE_SIZE);
    interp_reso = vfe_calc_interp_reso(input, output);
    phase_mult = input * (1 << (13 + interp_reso)) / output;
    reg = (interp_reso << 20) | phase_mult;
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
unsafe extern "C" fn vfe_set_qos(vfe: *mut vfe_device) {
    static void vfe_set_qos(struct vfe_device *vfe)
    {
    let mut val: u32 = VFE_0_BUS_BDG_QOS_CFG_0_CFG;
    let mut val7: u32 = VFE_0_BUS_BDG_QOS_CFG_7_CFG;
    int ret;
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_QOS_CFG_0);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_QOS_CFG_1);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_QOS_CFG_2);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_QOS_CFG_3);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_QOS_CFG_4);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_QOS_CFG_5);
    writel_relaxed(val, vfe.base + VFE_0_BUS_BDG_QOS_CFG_6);
    writel_relaxed(val7, vfe.base + VFE_0_BUS_BDG_QOS_CFG_7);
// SoC-specific VBIF settings
    if (vfe.res.has_vbif) {
    ret = vfe_vbif_apply_settings(vfe);
    if (ret < 0) {
    dev_err_ratelimited(vfe.camss.dev,
    "VFE: VBIF error %d\n",
    ret);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_ds(vfe: *mut vfe_device) {
    static void vfe_set_ds(struct vfe_device *vfe)
    {
// empty
    }
#[no_mangle]
unsafe extern "C" fn vfe_set_cgc_override(vfe: *mut vfe_device, wm: u8, enable: u8) {
    static void vfe_set_cgc_override(struct vfe_device *vfe, u8 wm, u8 enable)
    {
    let mut val: u32 = VFE_0_CGC_OVERRIDE_1_IMAGE_Mx_CGC_OVERRIDE(wm);
    if (enable)
    vfe_reg_set(vfe, VFE_0_CGC_OVERRIDE_1, val);
    else
    vfe_reg_clr(vfe, VFE_0_CGC_OVERRIDE_1, val);
    wmb();
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
    writel_relaxed(val, vfe.base + VFE_0_CORE_CFG);
    val = line.fmt[MSM_VFE_PAD_SINK].width * 2;
    val |= line.fmt[MSM_VFE_PAD_SINK].height << 16;
    writel_relaxed(val, vfe.base + VFE_0_CAMIF_FRAME_CFG);
    val = line.fmt[MSM_VFE_PAD_SINK].width * 2 - 1;
    writel_relaxed(val, vfe.base + VFE_0_CAMIF_WINDOW_WIDTH_CFG);
    val = line.fmt[MSM_VFE_PAD_SINK].height - 1;
    writel_relaxed(val, vfe.base + VFE_0_CAMIF_WINDOW_HEIGHT_CFG);
    val = 0xffffffff;
    writel_relaxed(val, vfe.base + VFE_0_CAMIF_SUBSAMPLE_CFG_0);
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
    u32 val = VFE_0_MODULE_CFG_DEMUX |
    VFE_0_MODULE_CFG_CHROMA_UPSAMPLE |
    VFE_0_MODULE_CFG_SCALE_ENC |
    VFE_0_MODULE_CFG_CROP_ENC;
    if (enable)
    writel_relaxed(val, vfe.base + VFE_0_MODULE_CFG);
    else
    writel_relaxed(0x0, vfe.base + VFE_0_MODULE_CFG);
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
#[no_mangle]
unsafe extern "C" fn vfe_isr_read(vfe: *mut vfe_device, value0: *mut u32, value1: *mut u32) {
    static void vfe_isr_read(struct vfe_device *vfe, u32 *value0, u32 *value1)
    {
// value0 = readl_relaxed(vfe->base + VFE_0_IRQ_STATUS_0);
// value1 = readl_relaxed(vfe->base + VFE_0_IRQ_STATUS_1);
    writel_relaxed(*value0, vfe.base + VFE_0_IRQ_CLEAR_0);
    writel_relaxed(*value1, vfe.base + VFE_0_IRQ_CLEAR_1);
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
    for (i = VFE_LINE_RDI0; i <= VFE_LINE_PIX; i++)
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
//
// vfe_pm_domain_off - Disable power domains specific to this VFE.
// @vfe: VFE Device
//
#[no_mangle]
unsafe extern "C" fn vfe_4_1_pm_domain_off(vfe: *mut vfe_device) {
    static void vfe_4_1_pm_domain_off(struct vfe_device *vfe)
    {
    if (!vfe.res.has_pd)
    return;
    vfe_pm_domain_off(vfe);
    }
//
// vfe_pm_domain_on - Enable power domains specific to this VFE.
// @vfe: VFE Device
//
#[no_mangle]
unsafe extern "C" fn vfe_4_1_pm_domain_on(vfe: *mut vfe_device) -> c_int {
    static int vfe_4_1_pm_domain_on(struct vfe_device *vfe)
    {
    if (!vfe.res.has_pd)
    return 0;
    return vfe_pm_domain_on(vfe);
    }
    static const struct vfe_hw_ops_gen1 vfe_ops_gen1_4_1 = {
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
    vfe.ops_gen1 = &vfe_ops_gen1_4_1;
    vfe.video_ops = vfe_video_ops_gen1;
    }
    const struct vfe_hw_ops vfe_ops_4_1 = {
    .global_reset = vfe_global_reset,
    .hw_version = vfe_hw_version,
    .isr_read = vfe_isr_read,
    .isr = vfe_isr,
    .pm_domain_off = vfe_4_1_pm_domain_off,
    .pm_domain_on = vfe_4_1_pm_domain_on,
    .reg_update_clear = vfe_reg_update_clear,
    .reg_update = vfe_reg_update,
    .subdev_init = vfe_subdev_init,
    .vfe_disable = vfe_gen1_disable,
    .vfe_enable = vfe_gen1_enable,
    .vfe_halt = vfe_gen1_halt,
    .violation_read = vfe_violation_read,
    };
