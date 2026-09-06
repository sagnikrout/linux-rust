//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/camss/camss-vfe-680.c
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
// camss-vfe-680.c
//
// Qualcomm MSM Camera Subsystem - VFE (Video Front End) Module v680
//
// Copyright (C) 2025 Linaro Ltd.
//

//
// TODO: differentiate the port id based on requested type of RDI, BHIST etc
//
// IFE write master IDs
//
// VIDEO_FULL_Y		0
// VIDEO_FULL_C		1
// VIDEO_DS_4:1		2
// VIDEO_DS_16:1	3
// DISPLAY_FULL_Y	4
// DISPLAY_FULL_C	5
// DISPLAY_DS_4:1	6
// DISPLAY_DS_16:1	7
// FD_Y			8
// FD_C			9
// PIXEL_RAW		10
// STATS_BE0		11
// STATS_BHIST0		12
// STATS_TINTLESS_BG	13
// STATS_AWB_BG		14
// STATS_AWB_BFW	15
// STATS_BAF		16
// STATS_BHIST		17
// STATS_RS		18
// STATS_IHIST		19
// SPARSE_PD		20
// PDAF_V2.0_PD_DATA	21
// PDAF_V2.0_SAD	22
// LCR			23
// RDI0			24
// RDI1			25
// RDI2			26
// LTM_STATS		27
//
// IFE Lite write master IDs
//
// RDI0			0
// RDI1			1
// RDI2			2
// RDI3			3
// GAMMA		4
// BE			5
//
// TODO: assign an ENUM in resources and use the provided master
// id directly for RDI, STATS, AWB_BG, BHIST.
// This macro only works because RDI is all we support right now.
//

#[no_mangle]
unsafe extern "C" fn vfe_global_reset(vfe: *mut vfe_device) {
    static void vfe_global_reset(struct vfe_device *vfe)
    {
// VFE680 has no global reset, simply report a completion
    complete(&vfe.reset_complete);
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
    return IRQ_HANDLED;
    }
//
// vfe_halt - Trigger halt on VFE module and wait to complete
// @vfe: VFE device
//
// Return 0 on success or a negative error code otherwise
//
#[no_mangle]
unsafe extern "C" fn vfe_halt(vfe: *mut vfe_device) -> c_int {
    static int vfe_halt(struct vfe_device *vfe)
    {
// rely on vfe_disable_output() to stop the VFE
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vfe_disable_irq(vfe: *mut vfe_device) {
    static void vfe_disable_irq(struct vfe_device *vfe)
    {
    writel(0u, vfe.base + VFE_TOP_IRQn_MASK(vfe, 0));
    writel(0u, vfe.base + VFE_TOP_IRQn_MASK(vfe, 1));
    writel(0u, vfe.base + VFE_BUS_IRQn_MASK(vfe, 0));
    writel(0u, vfe.base + VFE_BUS_IRQn_MASK(vfe, 1));
    }
    static void vfe_wm_update(struct vfe_device *vfe, u8 rdi, u32 addr,
    struct vfe_line *line)
    {
    let mut wm: u8 = RDI_WM(rdi);
    writel(addr, vfe.base + VFE_BUS_IMAGE_ADDR(vfe, wm));
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_start(vfe: *mut vfe_device, rdi: u8, line: *mut vfe_line) {
    static void vfe_wm_start(struct vfe_device *vfe, u8 rdi, struct vfe_line *line)
    {
    struct v4l2_pix_format_mplane *pix =
    &line.video_out.active_fmt.fmt.pix_mp;
    let mut stride: u32 = pix.plane_fmt[0].bytesperline;
    u32 cfg;
    u8 wm;
    cfg = VFE_BUS_IMAGE_CFG0_DATA(pix.height, stride);
    wm = RDI_WM(rdi);
    writel(cfg, vfe.base + VFE_BUS_IMAGE_CFG0(vfe, wm));
    writel(0, vfe.base + VFE_BUS_IMAGE_CFG1(vfe, wm));
    writel(stride, vfe.base + VFE_BUS_IMAGE_CFG2(vfe, wm));
    writel(0, vfe.base + VFE_BUS_PACKER_CFG(vfe, wm));
// Set total frame increment value
    writel(pix.plane_fmt[0].bytesperline * pix.height,
    vfe.base + VFE_BUS_FRAME_INCR(vfe, wm));
// MMU
    writel(VFE_BUS_MMU_PREFETCH_CFG_EN, vfe.base + VFE_BUS_MMU_PREFETCH_CFG(vfe, wm));
    writel(~0u, vfe.base + VFE_BUS_MMU_PREFETCH_MAX_OFFSET(vfe, wm));
// no dropped frames, one irq per frame
    writel(1, vfe.base + VFE_BUS_FRAMEDROP_PATTERN(vfe, wm));
    writel(0, vfe.base + VFE_BUS_FRAMEDROP_PERIOD(vfe, wm));
    writel(1, vfe.base + VFE_BUS_IRQ_SUBSAMPLE_PATTERN(vfe, wm));
    writel(0, vfe.base + VFE_BUS_IRQ_SUBSAMPLE_PERIOD(vfe, wm));
// We don't process IRQs for VFE in RDI mode at the moment
    vfe_disable_irq(vfe);
// Enable WM
    writel(VFE_BUS_WRITE_CLIENT_CFG_EN,
    vfe.base + VFE_BUS_WRITE_CLIENT_CFG(vfe, wm));
    dev_dbg(vfe.camss.dev, "RDI%d WM:%d width %d height %d stride %d\n",
    rdi, wm, pix.width, pix.height, stride);
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_stop(vfe: *mut vfe_device, rdi: u8) {
    static void vfe_wm_stop(struct vfe_device *vfe, u8 rdi)
    {
    let mut wm: u8 = RDI_WM(rdi);
    writel(0, vfe.base + VFE_BUS_WRITE_CLIENT_CFG(vfe, wm));
    }
    static const struct camss_video_ops vfe_video_ops_680 = {
    .queue_buffer = vfe_queue_buffer_v2,
    .flush_buffers = vfe_flush_buffers,
    };
#[no_mangle]
unsafe extern "C" fn vfe_subdev_init(dev: *mut device, vfe: *mut vfe_device) {
    static void vfe_subdev_init(struct device *dev, struct vfe_device *vfe)
    {
    vfe.video_ops = vfe_video_ops_680;
    }
#[no_mangle]
unsafe extern "C" fn vfe_reg_update(vfe: *mut vfe_device, line_id: enum vfe_line_id) {
    static void vfe_reg_update(struct vfe_device *vfe, enum vfe_line_id line_id)
    {
    let mut port_id: c_int = line_id;
    camss_reg_update(vfe.camss, vfe.id, port_id, false);
    }
    static inline void vfe_reg_update_clear(struct vfe_device *vfe,
    enum vfe_line_id line_id)
    {
    let mut port_id: c_int = line_id;
    camss_reg_update(vfe.camss, vfe.id, port_id, true);
    }
    const struct vfe_hw_ops vfe_ops_680 = {
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
