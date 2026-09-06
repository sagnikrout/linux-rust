//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/camss/camss-vfe.h
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
// camss-vfe.h
//
// Qualcomm MSM Camera Subsystem - VFE (Video Front End) Module
//
// Copyright (c) 2013-2015, The Linux Foundation. All rights reserved.
// Copyright (C) 2015-2018 Linaro Ltd.
//

pub const MSM_VFE_PAD_SINK: c_int = 0;
pub const MSM_VFE_PAD_SRC: c_int = 1;
pub const MSM_VFE_PADS_NUM: c_int = 2;
pub const MSM_VFE_IMAGE_MASTERS_NUM: c_int = 7;
pub const MSM_VFE_COMPOSITE_IRQ_NUM: c_int = 4;
// VFE halt timeout
pub const VFE_HALT_TIMEOUT_MS: c_int = 100;
// Frame drop value. VAL + UPDATES - 1 should not exceed 31
pub const VFE_FRAME_DROP_VAL: c_int = 30;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vfe_output_state {
    VFE_OUTPUT_OFF,
    VFE_OUTPUT_RESERVED,
    VFE_OUTPUT_SINGLE,
    VFE_OUTPUT_CONTINUOUS,
    VFE_OUTPUT_IDLE,
    VFE_OUTPUT_STOPPING,
    VFE_OUTPUT_ON,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vfe_line_id {
    VFE_LINE_NONE = -1,
    VFE_LINE_RDI0 = 0,
    VFE_LINE_RDI1 = 1,
    VFE_LINE_RDI2 = 2,
    VFE_LINE_PIX = 3,
    VFE_LINE_NUM_MAX = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfe_output {
    pub wm_num: u8,
    pub wm_idx: [u8; 3],
    pub buf: [*mut camss_buffer; 2],
    pub last_buffer: *mut camss_buffer,
    pub pending_bufs: list_head,
    pub drop_update_idx: c_uint,
    pub active_buf: c_int,
    pub wait_sof: c_int,
    pub gen1: },
    pub active_num: c_int,
    pub gen2: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfe_line {
    pub id: vfe_line_id,
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; MSM_VFE_PADS_NUM],
    pub fmt: [v4l2_mbus_framefmt; MSM_VFE_PADS_NUM],
    pub compose: v4l2_rect,
    pub crop: v4l2_rect,
    pub video_out: camss_video,
    pub output: vfe_output,
    pub formats: *const camss_format_info,
    pub nformats: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfe_hw_ops {
    pub vfe): *mut *mut void (enable_irq)(struct vfe_device,
    pub vfe): *mut *mut void (global_reset)(struct vfe_device,
    pub vfe): *mut *mut u32 (hw_version)(struct vfe_device,
    pub dev): *mut *mut irqreturn_t (isr)(int irq, void,
    pub value1): *mut *mut *mut *mut void (isr_read)(struct vfe_device vfe, u32 value0, u32,
    pub vfe): *mut *mut void (pm_domain_off)(struct vfe_device,
    pub vfe): *mut *mut int (pm_domain_on)(struct vfe_device,
    pub line_id): *mut *mut *mut void (reg_update)(struct vfe_device vfe, enum vfe_line_id,
    pub line_id): vfe_line_id,
    pub vfe): *mut *mut *mut void (subdev_init)(struct device dev, struct vfe_device,
    pub line): *mut *mut int (vfe_disable)(struct vfe_line,
    pub line): *mut *mut int (vfe_enable)(struct vfe_line,
    pub vfe): *mut *mut int (vfe_halt)(struct vfe_device,
    pub vfe): *mut *mut void (violation_read)(struct vfe_device,
    pub line): *mut vfe_line,
    pub wm): *mut *mut *mut void (vfe_wm_stop)(struct vfe_device vfe, u8,
    pub port_id): *mut *mut *mut void (vfe_buf_done)(struct vfe_device vfe, int,
    pub line): *mut vfe_line,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfe_isr_ops {
    pub vfe): *mut *mut void (reset_ack)(struct vfe_device,
    pub vfe): *mut *mut void (halt_ack)(struct vfe_device,
    pub line_id): *mut *mut *mut void (reg_update)(struct vfe_device vfe, enum vfe_line_id,
    pub line_id): *mut *mut *mut void (sof)(struct vfe_device vfe, enum vfe_line_id,
    pub comp): *mut *mut *mut void (comp_done)(struct vfe_device vfe, u8,
    pub wm): *mut *mut *mut void (wm_done)(struct vfe_device vfe, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfe_subdev_resources {
    pub is_lite: bool,
    pub line_num: u8,
    pub has_pd: bool,
    pub pd_name: *mut c_char,
    pub has_vbif: bool,
    pub vbif_name: *mut c_char,
    pub hw_ops: *const vfe_hw_ops,
    pub formats_rdi: *const camss_formats,
    pub formats_pix: *const camss_formats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfe_device {
    pub camss: *mut camss,
    pub id: u8,
    pub base: *mut void __iomem,
    pub vbif_base: *mut void __iomem,
    pub irq: u32,
    pub irq_name: [c_char; 30],
    pub clock: *mut camss_clock,
    pub nclocks: c_int,
    pub reset_complete: completion,
    pub halt_complete: completion,
    pub power_lock: mutex,
    pub power_count: c_int,
    pub stream_lock: mutex,
    pub stream_count: c_int,
    pub output_lock: spinlock_t,
    pub wm_output_map: [vfe_line_id; MSM_VFE_IMAGE_MASTERS_NUM],
    pub line: [vfe_line; VFE_LINE_NUM_MAX],
    pub reg_update: u32,
    pub was_streaming: u8,
    pub res: *const vfe_subdev_resources,
    pub ops_gen1: *const vfe_hw_ops_gen1,
    pub isr_ops: vfe_isr_ops,
    pub video_ops: camss_video_ops,
    pub genpd: *mut device,
    pub genpd_link: *mut device_link,
}

extern "C" {
    pub fn msm_vfe_genpd_cleanup(vfe: *mut vfe_device);
}
extern "C" {
    pub fn msm_vfe_unregister_entities(vfe: *mut vfe_device);
}
//
// vfe_buf_add_pending - Add output buffer to list of pending
// @output: VFE output
// @buffer: Video buffer
//
extern "C" {
    pub fn vfe_buf_add_pending(output: *mut vfe_output, buffer: *mut camss_buffer);
}
extern "C" {
    pub fn vfe_flush_buffers(vid: *mut camss_video, state: vb2_buffer_state) -> c_int;
}
//
// vfe_isr_comp_done - Process composite image done interrupt
// @vfe: VFE Device
// @comp: Composite image id
//
extern "C" {
    pub fn vfe_isr_comp_done(vfe: *mut vfe_device, comp: u8);
}
extern "C" {
    pub fn vfe_isr_reset_ack(vfe: *mut vfe_device);
}
extern "C" {
    pub fn vfe_put_output(line: *mut vfe_line) -> c_int;
}
extern "C" {
    pub fn vfe_release_wm(vfe: *mut vfe_device, wm: u8) -> c_int;
}
extern "C" {
    pub fn vfe_reserve_wm(vfe: *mut vfe_device, line_id: vfe_line_id) -> c_int;
}
//
// vfe_reset - Trigger reset on VFE module and wait to complete
// @vfe: VFE device
//
// Return 0 on success or a negative error code otherwise
//
extern "C" {
    pub fn vfe_reset(vfe: *mut vfe_device) -> c_int;
}
//
// vfe_disable - Disable streaming on VFE line
// @line: VFE line
//
// Return 0 on success or a negative error code otherwise
//
extern "C" {
    pub fn vfe_disable(line: *mut vfe_line) -> c_int;
}
//
// vfe_pm_domain_off - Disable power domains specific to this VFE.
// @vfe: VFE Device
//
extern "C" {
    pub fn vfe_pm_domain_off(vfe: *mut vfe_device);
}
//
// vfe_pm_domain_on - Enable power domains specific to this VFE.
// @vfe: VFE Device
//
extern "C" {
    pub fn vfe_pm_domain_on(vfe: *mut vfe_device) -> c_int;
}
extern "C" {
    pub fn vfe_get(vfe: *mut vfe_device) -> c_int;
}
extern "C" {
    pub fn vfe_put(vfe: *mut vfe_device);
}
//
// vfe_is_lite - Return if VFE is VFE lite.
// @vfe: VFE Device
//
// Some VFE lites have a different register layout.
//
// Return whether VFE is VFE lite
//
extern "C" {
    pub fn vfe_is_lite(vfe: *mut vfe_device) -> bool;
}
//
// vfe_hw_version - Process write master done interrupt
// @vfe: VFE Device
//
// Return vfe hw version
//
extern "C" {
    pub fn vfe_hw_version(vfe: *mut vfe_device) -> u32;
}
//
// vfe_enable - Enable streaming on VFE line
// @line: VFE line
//
// Return 0 on success or a negative error code otherwise
//
extern "C" {
    pub fn vfe_enable_v2(line: *mut vfe_line) -> c_int;
}
//
// vfe_buf_done - Process write master done interrupt
// @vfe: VFE Device
// @wm: Write master id
//
extern "C" {
    pub fn vfe_buf_done(vfe: *mut vfe_device, wm: c_int);
}
//
// vfe_get_output_v2 - Get vfe output line
// line: VFE line
//
// Return 0 on success or a negative error code otherwise
//
extern "C" {
    pub fn vfe_get_output_v2(line: *mut vfe_line) -> c_int;
}
//
// vfe_enable_output_v2 - Enable vfe output line
// line: VFE line
//
// Return 0 on success or a negative error code otherwise
//
extern "C" {
    pub fn vfe_enable_output_v2(line: *mut vfe_line) -> c_int;
}
//
// vfe_queue_buffer_v2 - Add empty buffer
// @vid: Video device structure
// @buf: Buffer to be enqueued
//
// Add an empty buffer - depending on the current number of buffers it will be
// put in pending buffer queue or directly given to the hardware to be filled.
//
// Return 0 on success or a negative error code otherwise
//
