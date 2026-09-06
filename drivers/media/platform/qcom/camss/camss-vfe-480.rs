//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/camss/camss-vfe-480.c
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
// camss-vfe-480.c
//
// Qualcomm MSM Camera Subsystem - VFE (Video Front End) Module v480 (SM8250)
//
// Copyright (C) 2020-2021 Linaro Ltd.
// Copyright (C) 2021 Jonathan Marek
//

#[no_mangle]
pub unsafe extern "C" fn reg_update_rdi(vfe: *mut vfe_device, n: c_int) -> c_int {
    static inline int reg_update_rdi(struct vfe_device *vfe, int n)
    {
    return vfe_is_lite(vfe) ? BIT(n) : BIT(1 + (n));
    }

#[no_mangle]
pub unsafe extern "C" fn bus_irq_mask_0_rdi_rup(vfe: *mut vfe_device, n: c_int) -> c_int {
    static inline int bus_irq_mask_0_rdi_rup(struct vfe_device *vfe, int n)
    {
    return vfe_is_lite(vfe) ? BIT(n) : BIT(3 + (n));
    }

#[no_mangle]
pub unsafe extern "C" fn bus_irq_mask_0_comp_done(vfe: *mut vfe_device, n: c_int) -> c_int {
    static inline int bus_irq_mask_0_comp_done(struct vfe_device *vfe, int n)
    {
    return vfe_is_lite(vfe) ? BIT(4 + (n)) : BIT(6 + (n));
    }

// for titan 480, each bus client is hardcoded to a specific path
// and each bus client is part of a hardcoded "comp group"
//

pub const MAX_VFE_OUTPUT_LINES: c_int = 4;
#[no_mangle]
unsafe extern "C" fn vfe_global_reset(vfe: *mut vfe_device) {
    static void vfe_global_reset(struct vfe_device *vfe)
    {
    writel_relaxed(IRQ_MASK_0_RESET_ACK, vfe.base + VFE_IRQ_MASK(0));
    writel_relaxed(GLOBAL_RESET_HW_AND_REG, vfe.base + VFE_GLOBAL_RESET_CMD);
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_start(vfe: *mut vfe_device, wm: u8, line: *mut vfe_line) {
    static void vfe_wm_start(struct vfe_device *vfe, u8 wm, struct vfe_line *line)
    {
    struct v4l2_pix_format_mplane *pix =
    &line.video_out.active_fmt.fmt.pix_mp;
    wm = RDI_WM(wm); /* map to actual WM used (from wm=RDI index) */
// no clock gating at bus input
    writel_relaxed(WM_CGC_OVERRIDE_ALL, vfe.base + VFE_BUS_WM_CGC_OVERRIDE);
    writel_relaxed(0x0, vfe.base + VFE_BUS_WM_TEST_BUS_CTRL);
    writel_relaxed(pix.plane_fmt[0].bytesperline * pix.height,
    vfe.base + VFE_BUS_WM_FRAME_INCR(wm));
    writel_relaxed(0xf, vfe.base + VFE_BUS_WM_BURST_LIMIT(wm));
    writel_relaxed(WM_IMAGE_CFG_0_DEFAULT_WIDTH,
    vfe.base + VFE_BUS_WM_IMAGE_CFG_0(wm));
    writel_relaxed(pix.plane_fmt[0].bytesperline,
    vfe.base + VFE_BUS_WM_IMAGE_CFG_2(wm));
    writel_relaxed(0, vfe.base + VFE_BUS_WM_PACKER_CFG(wm));
// no dropped frames, one irq per frame
    writel_relaxed(0, vfe.base + VFE_BUS_WM_FRAMEDROP_PERIOD(wm));
    writel_relaxed(1, vfe.base + VFE_BUS_WM_FRAMEDROP_PATTERN(wm));
    writel_relaxed(0, vfe.base + VFE_BUS_WM_IRQ_SUBSAMPLE_PERIOD(wm));
    writel_relaxed(1, vfe.base + VFE_BUS_WM_IRQ_SUBSAMPLE_PATTERN(wm));
    writel_relaxed(1 << WM_CFG_EN | MODE_MIPI_RAW << WM_CFG_MODE,
    vfe.base + VFE_BUS_WM_CFG(wm));
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_stop(vfe: *mut vfe_device, wm: u8) {
    static void vfe_wm_stop(struct vfe_device *vfe, u8 wm)
    {
    wm = RDI_WM(wm); /* map to actual WM used (from wm=RDI index) */
    writel_relaxed(0, vfe.base + VFE_BUS_WM_CFG(wm));
    }
    static void vfe_wm_update(struct vfe_device *vfe, u8 wm, u32 addr,
    struct vfe_line *line)
    {
    wm = RDI_WM(wm); /* map to actual WM used (from wm=RDI index) */
    writel_relaxed(addr, vfe.base + VFE_BUS_WM_IMAGE_ADDR(wm));
    }
#[no_mangle]
unsafe extern "C" fn vfe_reg_update(vfe: *mut vfe_device, line_id: enum vfe_line_id) {
    static void vfe_reg_update(struct vfe_device *vfe, enum vfe_line_id line_id)
    {
    vfe.reg_update |= REG_UPDATE_RDI(vfe, line_id);
    writel_relaxed(vfe.reg_update, vfe.base + VFE_REG_UPDATE_CMD);
    }
    static inline void vfe_reg_update_clear(struct vfe_device *vfe,
    enum vfe_line_id line_id)
    {
    vfe.reg_update &= ~REG_UPDATE_RDI(vfe, line_id);
    }
#[no_mangle]
unsafe extern "C" fn vfe_enable_irq(vfe: *mut vfe_device) {
    static void vfe_enable_irq(struct vfe_device *vfe)
    {
    int i;
    let mut bus_irq_mask: u32 = 0;
    if (!vfe.stream_count)
// enable reset ack IRQ and top BUS status IRQ
    writel(IRQ_MASK_0_RESET_ACK | IRQ_MASK_0_BUS_TOP_IRQ,
    vfe.base + VFE_IRQ_MASK(0));
    for (i = 0; i < MAX_VFE_OUTPUT_LINES; i++) {
// Enable IRQ for newly added lines, but also keep already running lines's IRQ
    if (vfe.line[i].output.state == VFE_OUTPUT_RESERVED ||
    vfe.line[i].output.state == VFE_OUTPUT_ON) {
    bus_irq_mask |= BUS_IRQ_MASK_0_RDI_RUP(vfe, i)
    | BUS_IRQ_MASK_0_COMP_DONE(vfe, RDI_COMP_GROUP(i));
    }
    }
    writel(bus_irq_mask, vfe.base + VFE_BUS_IRQ_MASK(0));
    }
    static void vfe_isr_reg_update(struct vfe_device *vfe, enum vfe_line_id line_id);
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
    u32 status;
    int i;
    status = readl_relaxed(vfe.base + VFE_IRQ_STATUS(0));
    writel_relaxed(status, vfe.base + VFE_IRQ_CLEAR(0));
    writel_relaxed(IRQ_CMD_GLOBAL_CLEAR, vfe.base + VFE_IRQ_CMD);
    if (status & IRQ_MASK_0_RESET_ACK)
    vfe_isr_reset_ack(vfe);
    if (status & IRQ_MASK_0_BUS_TOP_IRQ) {
    let mut status: u32 = readl_relaxed(vfe.base + VFE_BUS_IRQ_STATUS(0));
    writel_relaxed(status, vfe.base + VFE_BUS_IRQ_CLEAR(0));
    writel_relaxed(1, vfe.base + VFE_BUS_IRQ_CLEAR_GLOBAL);
    for (i = 0; i < MAX_VFE_OUTPUT_LINES; i++) {
    if (status & BUS_IRQ_MASK_0_RDI_RUP(vfe, i))
    vfe_isr_reg_update(vfe, i);
    }
// Loop through all WMs IRQs
    for (i = 0; i < MSM_VFE_IMAGE_MASTERS_NUM; i++) {
    if (status & BUS_IRQ_MASK_0_COMP_DONE(vfe, RDI_COMP_GROUP(i)))
    vfe_buf_done(vfe, i);
    }
    }
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
//
// vfe_isr_reg_update - Process reg update interrupt
// @vfe: VFE Device
// @line_id: VFE line
//
#[no_mangle]
unsafe extern "C" fn vfe_isr_reg_update(vfe: *mut vfe_device, line_id: enum vfe_line_id) {
    static void vfe_isr_reg_update(struct vfe_device *vfe, enum vfe_line_id line_id)
    {
    struct vfe_output *output;
    unsigned long flags;
    spin_lock_irqsave(&vfe.output_lock, flags);
    vfe_reg_update_clear(vfe, line_id);
    output = &vfe.line[line_id].output;
    if (output.wait_reg_update) {
    output.wait_reg_update = 0;
    complete(&output.reg_update);
    }
    spin_unlock_irqrestore(&vfe.output_lock, flags);
    }
    static const struct camss_video_ops vfe_video_ops_480 = {
    .queue_buffer = vfe_queue_buffer_v2,
    .flush_buffers = vfe_flush_buffers,
    };
#[no_mangle]
unsafe extern "C" fn vfe_subdev_init(dev: *mut device, vfe: *mut vfe_device) {
    static void vfe_subdev_init(struct device *dev, struct vfe_device *vfe)
    {
    vfe.video_ops = vfe_video_ops_480;
    }
#[no_mangle]
unsafe extern "C" fn vfe_isr_read(vfe: *mut vfe_device, value0: *mut u32, value1: *mut u32) {
    static void vfe_isr_read(struct vfe_device *vfe, u32 *value0, u32 *value1)
    {
// nop
    }
#[no_mangle]
unsafe extern "C" fn vfe_violation_read(vfe: *mut vfe_device) {
    static void vfe_violation_read(struct vfe_device *vfe)
    {
// nop
    }
#[no_mangle]
unsafe extern "C" fn vfe_buf_done_480(vfe: *mut vfe_device, port_id: c_int) {
    static void vfe_buf_done_480(struct vfe_device *vfe, int port_id)
    {
// nop
    }
    const struct vfe_hw_ops vfe_ops_480 = {
    .enable_irq = vfe_enable_irq,
    .global_reset = vfe_global_reset,
    .hw_version = vfe_hw_version,
    .isr = vfe_isr,
    .isr_read = vfe_isr_read,
    .reg_update = vfe_reg_update,
    .reg_update_clear = vfe_reg_update_clear,
    .pm_domain_off = vfe_pm_domain_off,
    .pm_domain_on = vfe_pm_domain_on,
    .subdev_init = vfe_subdev_init,
    .vfe_disable = vfe_disable,
    .vfe_enable = vfe_enable_v2,
    .vfe_halt = vfe_halt,
    .violation_read = vfe_violation_read,
    .vfe_wm_start = vfe_wm_start,
    .vfe_wm_stop = vfe_wm_stop,
    .vfe_buf_done = vfe_buf_done_480,
    .vfe_wm_update = vfe_wm_update,
    };
