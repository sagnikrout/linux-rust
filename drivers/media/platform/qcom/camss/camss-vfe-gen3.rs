//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/camss/camss-vfe-gen3.c
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
// Qualcomm MSM Camera Subsystem - VFE (Video Front End) Module gen3
//
// Copyright (c) 2024 Qualcomm Technologies, Inc.
//

    ((vfe.camss.res.version == CAMSS_8775P) \
    || (vfe.camss.res.version == CAMSS_8300))

    (vfe_is_lite(vfe) ? 0x480 : 0x400)

    (vfe_is_lite(vfe) ? 0x200 : 0xC00)

    (IS_VFE_690(vfe) ? BUS_REG_BASE_690 : BUS_REG_BASE_780)

    (IS_VFE_690(vfe) ? VFE_BUS_WM_TEST_BUS_CTRL_690 \
    : VFE_BUS_WM_TEST_BUS_CTRL_780)
//
// Bus client mapping:
//
// Full VFE:
// VFE_690: 16 = RDI0, 17 = RDI1, 18 = RDI2
// VFE_780: 23 = RDI0, 24 = RDI1, 25 = RDI2
//
// VFE LITE:
// VFE_690 : 0 = RDI0, 1 = RDI1, 2 = RDI2, 3 = RDI3, 4 = RDI4, 5 = RDI5
// VFE_780 : 0 = RDI0, 1 = RDI1, 2 = RDI2, 3 = RDI3, 4 = RDI4
//

#[no_mangle]
unsafe extern "C" fn vfe_wm_start(vfe: *mut vfe_device, wm: u8, line: *mut vfe_line) {
    static void vfe_wm_start(struct vfe_device *vfe, u8 wm, struct vfe_line *line)
    {
    struct v4l2_pix_format_mplane *pix =
    &line.video_out.active_fmt.fmt.pix_mp;
    wm = RDI_WM(wm);
// no clock gating at bus input
    writel(WM_CGC_OVERRIDE_ALL, vfe.base + VFE_BUS_WM_CGC_OVERRIDE);
    writel(0x0, vfe.base + VFE_BUS_WM_TEST_BUS_CTRL);
    if (IS_VFE_690(vfe))
    writel(ALIGN(pix.plane_fmt[0].bytesperline, 16) * pix.height,
    vfe.base + VFE_BUS_WM_FRAME_INCR(wm));
    else
    writel(ALIGN(pix.plane_fmt[0].bytesperline, 16) * pix.height >> 8,
    vfe.base + VFE_BUS_WM_FRAME_INCR(wm));
    writel((WM_IMAGE_CFG_0_DEFAULT_WIDTH & 0xFFFF),
    vfe.base + VFE_BUS_WM_IMAGE_CFG_0(wm));
    writel(WM_IMAGE_CFG_2_DEFAULT_STRIDE,
    vfe.base + VFE_BUS_WM_IMAGE_CFG_2(wm));
    writel(0, vfe.base + VFE_BUS_WM_PACKER_CFG(wm));
// TOP CORE CFG
    if (IS_VFE_690(vfe))
    writel(VFE_DISABLE_DSCALING_DS4 | VFE_DISABLE_DSCALING_DS16,
    vfe.base + VFE_TOP_CORE_CFG);
// no dropped frames, one irq per frame
    writel(0, vfe.base + VFE_BUS_WM_FRAMEDROP_PERIOD(wm));
    writel(1, vfe.base + VFE_BUS_WM_FRAMEDROP_PATTERN(wm));
    writel(0, vfe.base + VFE_BUS_WM_IRQ_SUBSAMPLE_PERIOD(wm));
    writel(1, vfe.base + VFE_BUS_WM_IRQ_SUBSAMPLE_PATTERN(wm));
    writel(1, vfe.base + VFE_BUS_WM_MMU_PREFETCH_CFG(wm));
    writel(0xFFFFFFFF, vfe.base + VFE_BUS_WM_MMU_PREFETCH_MAX_OFFSET(wm));
    writel(WM_CFG_EN | WM_CFG_MODE, vfe.base + VFE_BUS_WM_CFG(wm));
    }
#[no_mangle]
unsafe extern "C" fn vfe_wm_stop(vfe: *mut vfe_device, wm: u8) {
    static void vfe_wm_stop(struct vfe_device *vfe, u8 wm)
    {
    wm = RDI_WM(wm);
    writel(0, vfe.base + VFE_BUS_WM_CFG(wm));
    }
    static void vfe_wm_update(struct vfe_device *vfe, u8 wm, u32 addr,
    struct vfe_line *line)
    {
    wm = RDI_WM(wm);
    if (IS_VFE_690(vfe))
    writel(addr, vfe.base + VFE_BUS_WM_IMAGE_ADDR(wm));
    else
    writel((addr >> 8), vfe.base + VFE_BUS_WM_IMAGE_ADDR(wm));
    dev_dbg(vfe.camss.dev, "wm:%d, image buf addr:0x%x\n",
    wm, addr);
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
    static const struct camss_video_ops vfe_video_ops_gen3 = {
    .queue_buffer = vfe_queue_buffer_v2,
    .flush_buffers = vfe_flush_buffers,
    };
#[no_mangle]
unsafe extern "C" fn vfe_subdev_init(dev: *mut device, vfe: *mut vfe_device) {
    static void vfe_subdev_init(struct device *dev, struct vfe_device *vfe)
    {
    vfe.video_ops = vfe_video_ops_gen3;
    }
#[no_mangle]
unsafe extern "C" fn vfe_global_reset(vfe: *mut vfe_device) {
    static void vfe_global_reset(struct vfe_device *vfe)
    {
    vfe_isr_reset_ack(vfe);
    }
#[no_mangle]
unsafe extern "C" fn vfe_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t vfe_isr(int irq, void *dev)
    {
// nop
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn vfe_halt(vfe: *mut vfe_device) -> c_int {
    static int vfe_halt(struct vfe_device *vfe)
    {
// rely on vfe_disable_output() to stop the VFE
    return 0;
    }
    const struct vfe_hw_ops vfe_ops_gen3 = {
    .global_reset = vfe_global_reset,
    .hw_version = vfe_hw_version,
    .isr = vfe_isr,
    .pm_domain_off = vfe_pm_domain_off,
    .pm_domain_on = vfe_pm_domain_on,
    .reg_update = vfe_reg_update,
    .reg_update_clear = vfe_reg_update_clear,
    .subdev_init = vfe_subdev_init,
    .vfe_disable = vfe_disable,
    .vfe_enable = vfe_enable_v2,
    .vfe_halt = vfe_halt,
    .vfe_wm_start = vfe_wm_start,
    .vfe_wm_stop = vfe_wm_stop,
    .vfe_buf_done = vfe_buf_done,
    .vfe_wm_update = vfe_wm_update,
    };
