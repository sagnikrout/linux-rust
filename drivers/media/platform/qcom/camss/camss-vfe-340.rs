//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/camss/camss-vfe-340.c
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
// Qualcomm MSM Camera Subsystem - VFE (Video Front End) Module 340 (TFE)
//
// Copyright (c) 2025 Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const TFE_BUS_IMAGE_CFG_0_DEFAULT: c_uint = 0xffff;

pub const TFE_BUS_IMAGE_CFG_2_DEFAULT: c_uint = 0xffff;

pub const TFE_BUS_PACKER_CFG_FMT_PLAIN8: c_uint = 0x1;
pub const TFE_BUS_PACKER_CFG_FMT_PLAIN64: c_uint = 0xa;
pub const TFE_BUS_PACKER_CFG_FMT_MIPI10: c_uint = 0xc;
pub const TFE_BUS_PACKER_CFG_FMT_MIPI12: c_uint = 0xd;

    enum tfe_client {
    TFE_CLI_BAYER,
    TFE_CLI_IDEAL_RAW,
    TFE_CLI_STATS_TINTLESS_BG,
    TFE_CLI_STATS_BHIST,
    TFE_CLI_STATS_AWB_BG,
    TFE_CLI_STATS_AEC_BG,
    TFE_CLI_STATS_BAF,
    TFE_CLI_RDI0,
    TFE_CLI_RDI1,
    TFE_CLI_RDI2,
    TFE_CLI_NUM
    };
    enum tfe_iface {
    TFE_IFACE_PIX,
    TFE_IFACE_RDI0,
    TFE_IFACE_RDI1,
    TFE_IFACE_RDI2,
    TFE_IFACE_NUM
    };
    enum tfe_subgroups {
    TFE_SUBGROUP_BAYER,
    TFE_SUBGROUP_IDEAL_RAW,
    TFE_SUBGROUP_HDR,
    TFE_SUBGROUP_BG,
    TFE_SUBGROUP_BAF,
    TFE_SUBGROUP_RDI0,
    TFE_SUBGROUP_RDI1,
    TFE_SUBGROUP_RDI2,
    TFE_SUBGROUP_NUM
    };
    static enum tfe_client tfe_wm_client_map[VFE_LINE_NUM_MAX] = {
    [VFE_LINE_RDI0] = TFE_CLI_RDI0,
    [VFE_LINE_RDI1] = TFE_CLI_RDI1,
    [VFE_LINE_RDI2] = TFE_CLI_RDI2,
    [VFE_LINE_PIX] = TFE_CLI_BAYER,
    };
    static enum tfe_iface tfe_line_iface_map[VFE_LINE_NUM_MAX] = {
    [VFE_LINE_RDI0] = TFE_IFACE_RDI0,
    [VFE_LINE_RDI1] = TFE_IFACE_RDI1,
    [VFE_LINE_RDI2] = TFE_IFACE_RDI2,
    [VFE_LINE_PIX] = TFE_IFACE_PIX,
    };
    static enum vfe_line_id tfe_subgroup_line_map[TFE_SUBGROUP_NUM] = {
    [TFE_SUBGROUP_BAYER] = VFE_LINE_PIX,
    [TFE_SUBGROUP_IDEAL_RAW] = VFE_LINE_PIX,
    [TFE_SUBGROUP_HDR] = VFE_LINE_PIX,
    [TFE_SUBGROUP_BG] = VFE_LINE_PIX,
    [TFE_SUBGROUP_BAF] = VFE_LINE_PIX,
    [TFE_SUBGROUP_RDI0] = VFE_LINE_RDI0,
    [TFE_SUBGROUP_RDI1] = VFE_LINE_RDI1,
    [TFE_SUBGROUP_RDI2] = VFE_LINE_RDI2,
    };
#[no_mangle]
pub unsafe extern "C" fn __line_to_iface(line_id: enum vfe_line_id) -> enum tfe_iface {
    static inline enum tfe_iface  __line_to_iface(enum vfe_line_id line_id)
    {
    if (line_id <= VFE_LINE_NONE || line_id >= VFE_LINE_NUM_MAX) {
    pr_warn("VFE: Invalid line %d\n", line_id);
    return TFE_IFACE_RDI0;
    }
    return tfe_line_iface_map[line_id];
    }
#[no_mangle]
pub unsafe extern "C" fn __iface_to_line(iface: c_uint) -> enum vfe_line_id {
    static inline enum vfe_line_id __iface_to_line(unsigned int iface)
    {
    int i;
    for (i = 0; i < VFE_LINE_NUM_MAX; i++) {
    if (tfe_line_iface_map[i] == iface)
    return i;
    }
    return VFE_LINE_NONE;
    }
#[no_mangle]
pub unsafe extern "C" fn __subgroup_to_line(sg: enum tfe_subgroups) -> enum vfe_line_id {
    static inline enum vfe_line_id __subgroup_to_line(enum tfe_subgroups sg)
    {
    if (sg >= TFE_SUBGROUP_NUM)
    return VFE_LINE_NONE;
    return tfe_subgroup_line_map[sg];
    }
#[no_mangle]
unsafe extern "C" fn vfe_global_reset(vfe: *mut vfe_device) {
    static void vfe_global_reset(struct vfe_device *vfe)
    {
    writel(TFE_IRQ_MASK_0_RST_DONE, vfe.base + TFE_IRQ_MASK_0);
    writel(TFE_GLOBAL_RESET_CMD_CORE, vfe.base + TFE_GLOBAL_RESET_CMD);
    }
#[no_mangle]
unsafe extern "C" fn vfe_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t vfe_isr(int irq, void *dev)
    {
    struct vfe_device *vfe = dev;
    u32 status;
    int i;
    status = readl_relaxed(vfe.base + TFE_IRQ_STATUS_0);
    writel_relaxed(status, vfe.base + TFE_IRQ_CLEAR_0);
    writel_relaxed(TFE_IRQ_CMD_CLEAR, vfe.base + TFE_IRQ_CMD);
    if (status & TFE_IRQ_MASK_0_RST_DONE) {
    dev_dbg(vfe.camss.dev, "VFE%u: Reset done!", vfe.id);
    vfe_isr_reset_ack(vfe);
    }
    if (status & TFE_IRQ_MASK_0_BUS_WR) {
    let mut bus_status: u32 = readl_relaxed(vfe.base + TFE_BUS_IRQ_STATUS_0);
    writel_relaxed(bus_status, vfe.base + TFE_BUS_IRQ_CLEAR_0);
    writel_relaxed(TFE_BUS_IRQ_CMD_CLEAR, vfe.base + TFE_BUS_IRQ_CMD);
    for (i = 0; i < TFE_IFACE_NUM; i++) {
    if (bus_status & TFE_BUS_IRQ_MASK_RUP_DONE(i))
    vfe.res.hw_ops.reg_update_clear(vfe, __iface_to_line(i));
    }
    for (i = 0; i < TFE_SUBGROUP_NUM; i++) {
    if (bus_status & TFE_BUS_IRQ_MASK_BUF_DONE(i))
    vfe_buf_done(vfe, __subgroup_to_line(i));
    }
    if (bus_status & TFE_BUS_IRQ_MASK_0_CONS_VIOL)
    dev_err_ratelimited(vfe.camss.dev, "VFE%u: Bad config violation",
    vfe.id);
    if (bus_status & TFE_BUS_IRQ_MASK_0_VIOL)
    dev_err_ratelimited(vfe.camss.dev, "VFE%u: Input data violation",
    vfe.id);
    if (bus_status & TFE_BUS_IRQ_MASK_0_IMG_VIOL)
    dev_err_ratelimited(vfe.camss.dev, "VFE%u: Image size violation",
    vfe.id);
    }
    status = readl_relaxed(vfe.base + TFE_BUS_OVERFLOW_STATUS);
    if (status) {
    writel_relaxed(status, vfe.base + TFE_BUS_STATUS_CLEAR);
    for (i = 0; i < TFE_CLI_NUM; i++) {
    if (status & BIT(i))
    dev_err_ratelimited(vfe.camss.dev,
    "VFE%u: bus overflow for client %u\n",
    vfe.id, i);
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn vfe_halt(vfe: *mut vfe_device) -> c_int {
    static int vfe_halt(struct vfe_device *vfe)
    {
// rely on vfe_disable_output() to stop the VFE
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vfe_enable_irq(vfe: *mut vfe_device) {
    static void vfe_enable_irq(struct vfe_device *vfe)
    {
    writel(TFE_IRQ_MASK_0_RST_DONE | TFE_IRQ_MASK_0_BUS_WR,
    vfe.base + TFE_IRQ_MASK_0);
    writel(TFE_BUS_IRQ_MASK_RUP_DONE_MASK | TFE_BUS_IRQ_MASK_BUF_DONE_MASK |
    TFE_BUS_IRQ_MASK_0_CONS_VIOL | TFE_BUS_IRQ_MASK_0_VIOL |
    TFE_BUS_IRQ_MASK_0_IMG_VIOL, vfe.base + TFE_BUS_IRQ_MASK_0);
    }
    static void vfe_wm_update(struct vfe_device *vfe, u8 wm, u32 addr,
    struct vfe_line *line)
    {
    let mut client: u8 = tfe_wm_client_map[wm];
    writel_relaxed(addr, vfe.base + TFE_BUS_IMAGE_ADDR(client));
    }
#[no_mangle]
unsafe extern "C" fn vfe_packer_format(vfe: *mut vfe_device, pixelformat: u32) -> u32 {
    static u32 vfe_packer_format(struct vfe_device *vfe, u32 pixelformat)
    {
    const struct camss_formats *fmt = vfe.res.formats_rdi;
    let mut bpp: c_uint = 0;
    int i;
    for (i = 0; i < fmt.nformats; i++) {
    if (fmt.formats[i].pixelformat == pixelformat) {
    bpp = fmt.formats[i].mbus_bpp;
    break;
    }
    }
    switch (bpp) {
    case 10:
    return TFE_BUS_PACKER_CFG_FMT_MIPI10;
    case 12:
    return TFE_BUS_PACKER_CFG_FMT_MIPI12;
    default:
    return TFE_BUS_PACKER_CFG_FMT_PLAIN8;
    }
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_start(vfe: *mut vfe_device, wm: u8, line: *mut vfe_line) {
    static void vfe_wm_start(struct vfe_device *vfe, u8 wm, struct vfe_line *line)
    {
    struct v4l2_pix_format_mplane *pix = &line.video_out.active_fmt.fmt.pix_mp;
    let mut stride: u32 = pix.plane_fmt[0].bytesperline;
    let mut client: u8 = tfe_wm_client_map[wm];
    let mut cfg: u32 = TFE_BUS_CLIENT_CFG_EN;
    if (client == TFE_CLI_BAYER) { /* PIX - Line based */
    struct v4l2_rect *crop = &line.crop;
// Cropping
    writel_relaxed(TFE_PP_CROP_CFG_EN, vfe.base + TFE_PP_CROP_CFG);
    writel_relaxed(FIELD_PREP(TFE_PP_CROP_FIRST, crop.top) |
    FIELD_PREP(TFE_PP_CROP_LAST, crop.top + crop.height - 1),
    vfe.base + TFE_PP_CROP_LINE_CFG);
    writel_relaxed(FIELD_PREP(TFE_PP_CROP_FIRST, crop.left) |
    FIELD_PREP(TFE_PP_CROP_LAST, crop.left + crop.width - 1),
    vfe.base + TFE_PP_CROP_PIX_CFG);
// Write Engine
    writel_relaxed(pix.width | (pix.height << 16),
    vfe.base + TFE_BUS_IMAGE_CFG_0(client));
    writel_relaxed(0u, vfe.base + TFE_BUS_IMAGE_CFG_1(client));
    writel_relaxed(stride, vfe.base + TFE_BUS_IMAGE_CFG_2(client));
    writel_relaxed(stride * pix.height, vfe.base + TFE_BUS_FRAME_INCR(client));
    writel_relaxed(vfe_packer_format(vfe, pix.pixelformat),
    vfe.base + TFE_BUS_PACKER_CFG(client));
    cfg |= TFE_BUS_CLIENT_CFG_AUTORECOVER;
    } else { /* RDI - Frame based */
    writel_relaxed(TFE_BUS_IMAGE_CFG_0_DEFAULT,
    vfe.base + TFE_BUS_IMAGE_CFG_0(client));
    writel_relaxed(0u, vfe.base + TFE_BUS_IMAGE_CFG_1(client));
    writel_relaxed(TFE_BUS_IMAGE_CFG_2_DEFAULT,
    vfe.base + TFE_BUS_IMAGE_CFG_2(client));
    writel_relaxed(stride * pix.height, vfe.base + TFE_BUS_FRAME_INCR(client));
    writel_relaxed(TFE_BUS_PACKER_CFG_FMT_PLAIN64,
    vfe.base + TFE_BUS_PACKER_CFG(client));
    cfg |= TFE_BUS_CLIENT_CFG_MODE_FRAME;
    }
// No dropped frames, one irq per frame
    writel_relaxed(0, vfe.base + TFE_BUS_FRAMEDROP_CFG_0(client));
    writel_relaxed(1, vfe.base + TFE_BUS_FRAMEDROP_CFG_1(client));
    writel_relaxed(0, vfe.base + TFE_BUS_IRQ_SUBSAMPLE_CFG_0(client));
    writel_relaxed(1, vfe.base + TFE_BUS_IRQ_SUBSAMPLE_CFG_1(client));
    vfe_enable_irq(vfe);
    writel(cfg, vfe.base + TFE_BUS_CLIENT_CFG(client));
    dev_dbg(vfe.camss.dev, "VFE%u: Started client %u width %u height %u stride %u\n",
    vfe.id, client, pix.width, pix.height, stride);
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_stop(vfe: *mut vfe_device, wm: u8) {
    static void vfe_wm_stop(struct vfe_device *vfe, u8 wm)
    {
    let mut client: u8 = tfe_wm_client_map[wm];
    writel(0, vfe.base + TFE_BUS_CLIENT_CFG(client));
    dev_dbg(vfe.camss.dev, "VFE%u: Stopped client %u\n", vfe.id, client);
    }
    static const struct camss_video_ops vfe_video_ops_520 = {
    .queue_buffer = vfe_queue_buffer_v2,
    .flush_buffers = vfe_flush_buffers,
    };
#[no_mangle]
unsafe extern "C" fn vfe_subdev_init(dev: *mut device, vfe: *mut vfe_device) {
    static void vfe_subdev_init(struct device *dev, struct vfe_device *vfe)
    {
    vfe.video_ops = vfe_video_ops_520;
    }
#[no_mangle]
unsafe extern "C" fn vfe_reg_update(vfe: *mut vfe_device, line_id: enum vfe_line_id) {
    static void vfe_reg_update(struct vfe_device *vfe, enum vfe_line_id line_id)
    {
    vfe.reg_update |= BIT(__line_to_iface(line_id));
    writel_relaxed(vfe.reg_update, vfe.base + TFE_REG_UPDATE_CMD);
    }
#[no_mangle]
unsafe extern "C" fn vfe_reg_update_clear(vfe: *mut vfe_device, line_id: enum vfe_line_id) {
    static void vfe_reg_update_clear(struct vfe_device *vfe, enum vfe_line_id line_id)
    {
    vfe.reg_update &= ~BIT(__line_to_iface(line_id));
    }
    const struct vfe_hw_ops vfe_ops_340 = {
    .global_reset = vfe_global_reset,
    .hw_version = vfe_hw_version,
    .isr = vfe_isr,
    .pm_domain_off = vfe_pm_domain_off,
    .pm_domain_on = vfe_pm_domain_on,
    .subdev_init = vfe_subdev_init,
    .vfe_disable = vfe_disable,
    .vfe_enable = vfe_enable_v2,
    .vfe_halt = vfe_halt,
    .vfe_wm_start = vfe_wm_start,
    .vfe_wm_stop = vfe_wm_stop,
    .vfe_buf_done = vfe_buf_done,
    .vfe_wm_update = vfe_wm_update,
    .reg_update = vfe_reg_update,
    .reg_update_clear = vfe_reg_update_clear,
    };
