//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/camss/camss-vfe-17x.c
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
// camss-vfe-170.c
//
// Qualcomm MSM Camera Subsystem - VFE (Video Front End) Module v170
//
// Copyright (C) 2020-2021 Linaro Ltd.
//

// WM_CLIENT_BUF_DONE defined for buffers 0:19

    ((1 << BUS_VER2_MAX_CLIENTS) - 1)

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
    u32 reset_bits = GLOBAL_RESET_CMD_CORE		|
    GLOBAL_RESET_CMD_CAMIF		|
    GLOBAL_RESET_CMD_BUS		|
    GLOBAL_RESET_CMD_BUS_BDG	|
    GLOBAL_RESET_CMD_REGISTER	|
    GLOBAL_RESET_CMD_TESTGEN	|
    GLOBAL_RESET_CMD_DSP		|
    GLOBAL_RESET_CMD_IDLE_CGC	|
    GLOBAL_RESET_CMD_RDI0		|
    GLOBAL_RESET_CMD_RDI1		|
    GLOBAL_RESET_CMD_RDI2		|
    GLOBAL_RESET_CMD_RDI3;
    writel_relaxed(BIT(31), vfe.base + VFE_IRQ_MASK_0);
// Make sure IRQ mask has been written before resetting
    wmb();
    writel_relaxed(reset_bits, vfe.base + VFE_GLOBAL_RESET_CMD);
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_start(vfe: *mut vfe_device, wm: u8, line: *mut vfe_line) {
    static void vfe_wm_start(struct vfe_device *vfe, u8 wm, struct vfe_line *line)
    {
    u32 val;
// Set Debug Registers
    val = DEBUG_STATUS_CFG_STATUS0(1) |
    DEBUG_STATUS_CFG_STATUS0(7);
    writel_relaxed(val, vfe.base + VFE_BUS_WM_DEBUG_STATUS_CFG);
// BUS_WM_INPUT_IF_ADDR_SYNC_FRAME_HEADER
    writel_relaxed(0, vfe.base + VFE_BUS_WM_ADDR_SYNC_FRAME_HEADER);
// no clock gating at bus input
    val = WM_CGC_OVERRIDE_ALL;
    writel_relaxed(val, vfe.base + VFE_BUS_WM_CGC_OVERRIDE);
    writel_relaxed(0x0, vfe.base + VFE_BUS_WM_TEST_BUS_CTRL);
// if addr_no_sync has default value then config the addr no sync reg
    val = WM_ADDR_NO_SYNC_DEFAULT_VAL;
    writel_relaxed(val, vfe.base + VFE_BUS_WM_ADDR_SYNC_NO_SYNC);
    writel_relaxed(0xf, vfe.base + VFE_BUS_WM_BURST_LIMIT(wm));
    val = WM_BUFFER_DEFAULT_WIDTH;
    writel_relaxed(val, vfe.base + VFE_BUS_WM_BUFFER_WIDTH_CFG(wm));
    val = 0;
    writel_relaxed(val, vfe.base + VFE_BUS_WM_BUFFER_HEIGHT_CFG(wm));
    val = 0;
    writel_relaxed(val, vfe.base + VFE_BUS_WM_PACKER_CFG(wm)); // XXX 1 for PLAIN8?
// Configure stride for RDIs
    val = WM_STRIDE_DEFAULT_STRIDE;
    writel_relaxed(val, vfe.base + VFE_BUS_WM_STRIDE(wm));
// Enable WM
    val = 1 << WM_CFG_EN |
    MODE_MIPI_RAW << WM_CFG_MODE;
    writel_relaxed(val, vfe.base + VFE_BUS_WM_CFG(wm));
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_stop(vfe: *mut vfe_device, wm: u8) {
    static void vfe_wm_stop(struct vfe_device *vfe, u8 wm)
    {
// Disable WM
    writel_relaxed(0, vfe.base + VFE_BUS_WM_CFG(wm));
    }
    static void vfe_wm_update(struct vfe_device *vfe, u8 wm, u32 addr,
    struct vfe_line *line)
    {
    struct v4l2_pix_format_mplane *pix =
    &line.video_out.active_fmt.fmt.pix_mp;
    let mut stride: u32 = pix.plane_fmt[0].bytesperline;
    writel_relaxed(addr, vfe.base + VFE_BUS_WM_IMAGE_ADDR(wm));
    writel_relaxed(stride * pix.height, vfe.base + VFE_BUS_WM_FRAME_INC(wm));
    }
#[no_mangle]
unsafe extern "C" fn vfe_reg_update(vfe: *mut vfe_device, line_id: enum vfe_line_id) {
    static void vfe_reg_update(struct vfe_device *vfe, enum vfe_line_id line_id)
    {
    vfe.reg_update |= REG_UPDATE_RDI(line_id);
// Enforce ordering between previous reg writes and reg update
    wmb();
    writel_relaxed(vfe.reg_update, vfe.base + VFE_REG_UPDATE_CMD);
// Enforce ordering between reg update and subsequent reg writes
    wmb();
    }
    static inline void vfe_reg_update_clear(struct vfe_device *vfe,
    enum vfe_line_id line_id)
    {
    vfe.reg_update &= ~REG_UPDATE_RDI(line_id);
    }
#[no_mangle]
unsafe extern "C" fn vfe_enable_irq_common(vfe: *mut vfe_device) {
    static void vfe_enable_irq_common(struct vfe_device *vfe)
    {
    vfe_reg_set(vfe, VFE_IRQ_MASK_0, ~0u);
    vfe_reg_set(vfe, VFE_IRQ_MASK_1, ~0u);
    writel_relaxed(~0u, vfe.base + VFE_BUS_IRQ_MASK(0));
    writel_relaxed(~0u, vfe.base + VFE_BUS_IRQ_MASK(1));
    writel_relaxed(~0u, vfe.base + VFE_BUS_IRQ_MASK(2));
    }
#[no_mangle]
unsafe extern "C" fn vfe_isr_halt_ack(vfe: *mut vfe_device) {
    static void vfe_isr_halt_ack(struct vfe_device *vfe)
    {
    complete(&vfe.halt_complete);
    }
#[no_mangle]
unsafe extern "C" fn vfe_isr_read(vfe: *mut vfe_device, status0: *mut u32, status1: *mut u32) {
    static void vfe_isr_read(struct vfe_device *vfe, u32 *status0, u32 *status1)
    {
// status0 = readl_relaxed(vfe->base + VFE_IRQ_STATUS_0);
// status1 = readl_relaxed(vfe->base + VFE_IRQ_STATUS_1);
    writel_relaxed(*status0, vfe.base + VFE_IRQ_CLEAR_0);
    writel_relaxed(*status1, vfe.base + VFE_IRQ_CLEAR_1);
// Enforce ordering between IRQ Clear and Global IRQ Clear
    wmb();
    writel_relaxed(CMD_GLOBAL_CLEAR, vfe.base + VFE_IRQ_CMD);
    }
#[no_mangle]
unsafe extern "C" fn vfe_violation_read(vfe: *mut vfe_device) {
    static void vfe_violation_read(struct vfe_device *vfe)
    {
    let mut violation: u32 = readl_relaxed(vfe.base + VFE_VIOLATION_STATUS);
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
    u32 status0, status1, vfe_bus_status[VFE_LINE_NUM_MAX];
    int i, wm;
    status0 = readl_relaxed(vfe.base + VFE_IRQ_STATUS_0);
    status1 = readl_relaxed(vfe.base + VFE_IRQ_STATUS_1);
    writel_relaxed(status0, vfe.base + VFE_IRQ_CLEAR_0);
    writel_relaxed(status1, vfe.base + VFE_IRQ_CLEAR_1);
    for (i = VFE_LINE_RDI0; i < vfe.res.line_num; i++) {
    vfe_bus_status[i] = readl_relaxed(vfe.base + VFE_BUS_IRQ_STATUS(i));
    writel_relaxed(vfe_bus_status[i], vfe.base + VFE_BUS_IRQ_CLEAR(i));
    }
// Enforce ordering between IRQ reading and interpretation
    wmb();
    writel_relaxed(CMD_GLOBAL_CLEAR, vfe.base + VFE_IRQ_CMD);
    writel_relaxed(1, vfe.base + VFE_BUS_IRQ_CLEAR_GLOBAL);
    if (status0 & STATUS_0_RESET_ACK)
    vfe.isr_ops.reset_ack(vfe);
    for (i = VFE_LINE_RDI0; i < vfe.res.line_num; i++)
    if (status0 & STATUS_0_RDI_REG_UPDATE(i))
    vfe.isr_ops.reg_update(vfe, i);
    for (i = VFE_LINE_RDI0; i < vfe.res.line_num; i++)
    if (status0 & STATUS_1_RDI_SOF(i))
    vfe.isr_ops.sof(vfe, i);
    for (i = 0; i < MSM_VFE_COMPOSITE_IRQ_NUM; i++)
    if (vfe_bus_status[0] & STATUS0_COMP_BUF_DONE(i))
    vfe.isr_ops.comp_done(vfe, i);
    for (wm = 0; wm < MSM_VFE_IMAGE_MASTERS_NUM; wm++)
    if (status0 & BIT(9))
    if (vfe_bus_status[1] & STATUS1_WM_CLIENT_BUF_DONE(wm))
    vfe.isr_ops.wm_done(vfe, wm);
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
unsafe extern "C" fn vfe_get_output(line: *mut vfe_line) -> c_int {
    static int vfe_get_output(struct vfe_line *line)
    {
    struct vfe_device *vfe = to_vfe(line);
    struct vfe_output *output;
    unsigned long flags;
    int wm_idx;
    spin_lock_irqsave(&vfe.output_lock, flags);
    output = &line.output;
    if (output.state > VFE_OUTPUT_RESERVED) {
    dev_err(vfe.camss.dev, "Output is running\n");
    goto error;
    }
    output.wm_num = 1;
    wm_idx = vfe_reserve_wm(vfe, line.id);
    if (wm_idx < 0) {
    dev_err(vfe.camss.dev, "Can not reserve wm\n");
    goto error_get_wm;
    }
    output.wm_idx[0] = wm_idx;
    output.drop_update_idx = 0;
    spin_unlock_irqrestore(&vfe.output_lock, flags);
    return 0;
    error_get_wm:
    vfe_release_wm(vfe, output.wm_idx[0]);
    output.state = VFE_OUTPUT_OFF;
    error:
    spin_unlock_irqrestore(&vfe.output_lock, flags);
    return -EINVAL;
    }
//
// vfe_enable - Enable streaming on VFE line
// @line: VFE line
//
// Return 0 on success or a negative error code otherwise
//
#[no_mangle]
unsafe extern "C" fn vfe_enable(line: *mut vfe_line) -> c_int {
    static int vfe_enable(struct vfe_line *line)
    {
    struct vfe_device *vfe = to_vfe(line);
    int ret;
    mutex_lock(&vfe.stream_lock);
    if (!vfe.stream_count)
    vfe_enable_irq_common(vfe);
    vfe.stream_count++;
    mutex_unlock(&vfe.stream_lock);
    ret = vfe_get_output(line);
    if (ret < 0)
    goto error_get_output;
    ret = vfe_enable_output_v2(line);
    if (ret < 0)
    goto error_enable_output;
    vfe.was_streaming = 1;
    return 0;
    error_enable_output:
    vfe_put_output(line);
    error_get_output:
    mutex_lock(&vfe.stream_lock);
    vfe.stream_count--;
    mutex_unlock(&vfe.stream_lock);
    return ret;
    }
//
// vfe_isr_sof - Process start of frame interrupt
// @vfe: VFE Device
// @line_id: VFE line
//
#[no_mangle]
unsafe extern "C" fn vfe_isr_sof(vfe: *mut vfe_device, line_id: enum vfe_line_id) {
    static void vfe_isr_sof(struct vfe_device *vfe, enum vfe_line_id line_id)
    {
// nop
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
    vfe.res.hw_ops.reg_update_clear(vfe, line_id);
    output = &vfe.line[line_id].output;
    if (output.wait_reg_update) {
    output.wait_reg_update = 0;
    complete(&output.reg_update);
    }
    spin_unlock_irqrestore(&vfe.output_lock, flags);
    }
//
// vfe_isr_wm_done - Process write master done interrupt
// @vfe: VFE Device
// @wm: Write master id
//
#[no_mangle]
unsafe extern "C" fn vfe_isr_wm_done(vfe: *mut vfe_device, wm: u8) {
    static void vfe_isr_wm_done(struct vfe_device *vfe, u8 wm)
    {
    struct vfe_line *line = &vfe.line[vfe.wm_output_map[wm]];
    struct camss_buffer *ready_buf;
    struct vfe_output *output;
    unsigned long flags;
    u32 index;
    let mut ts: u64 = ktime_get_ns();
    spin_lock_irqsave(&vfe.output_lock, flags);
    if (vfe.wm_output_map[wm] == VFE_LINE_NONE) {
    dev_err_ratelimited(vfe.camss.dev,
    "Received wm done for unmapped index\n");
    goto out_unlock;
    }
    output = &vfe.line[vfe.wm_output_map[wm]].output;
    ready_buf = output.buf[0];
    if (!ready_buf) {
    dev_err_ratelimited(vfe.camss.dev,
    "Missing ready buf %d!\n", output.state);
    goto out_unlock;
    }
    ready_buf.vb.vb2_buf.timestamp = ts;
    ready_buf.vb.sequence = output.sequence++;
    index = 0;
    output.buf[0] = output.buf[1];
    if (output.buf[0])
    index = 1;
    output.buf[index] = vfe_buf_get_pending(output);
    if (output.buf[index])
    vfe_wm_update(vfe, output.wm_idx[0], output.buf[index].addr[0], line);
    else
    output.gen2.active_num--;
    spin_unlock_irqrestore(&vfe.output_lock, flags);
    vb2_buffer_done(&ready_buf.vb.vb2_buf, VB2_BUF_STATE_DONE);
    return;
    out_unlock:
    spin_unlock_irqrestore(&vfe.output_lock, flags);
    }
    static const struct vfe_isr_ops vfe_isr_ops_170 = {
    .reset_ack = vfe_isr_reset_ack,
    .halt_ack = vfe_isr_halt_ack,
    .reg_update = vfe_isr_reg_update,
    .sof = vfe_isr_sof,
    .comp_done = vfe_isr_comp_done,
    .wm_done = vfe_isr_wm_done,
    };
    static const struct camss_video_ops vfe_video_ops_170 = {
    .queue_buffer = vfe_queue_buffer_v2,
    .flush_buffers = vfe_flush_buffers,
    };
#[no_mangle]
unsafe extern "C" fn vfe_subdev_init(dev: *mut device, vfe: *mut vfe_device) {
    static void vfe_subdev_init(struct device *dev, struct vfe_device *vfe)
    {
    vfe.isr_ops = vfe_isr_ops_170;
    vfe.video_ops = vfe_video_ops_170;
    }
    const struct vfe_hw_ops vfe_ops_170 = {
    .global_reset = vfe_global_reset,
    .hw_version = vfe_hw_version,
    .isr_read = vfe_isr_read,
    .isr = vfe_isr,
    .pm_domain_off = vfe_pm_domain_off,
    .pm_domain_on = vfe_pm_domain_on,
    .reg_update_clear = vfe_reg_update_clear,
    .reg_update = vfe_reg_update,
    .subdev_init = vfe_subdev_init,
    .vfe_disable = vfe_disable,
    .vfe_enable = vfe_enable,
    .vfe_halt = vfe_halt,
    .violation_read = vfe_violation_read,
    .vfe_wm_start = vfe_wm_start,
    .vfe_wm_stop = vfe_wm_stop,
    .vfe_wm_update = vfe_wm_update,
    };
