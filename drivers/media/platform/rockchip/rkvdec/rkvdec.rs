//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rkvdec/rkvdec.h
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
// Rockchip Video Decoder driver
//
// Copyright (C) 2019 Collabora, Ltd.
//
// Based on rkvdec driver by Google LLC. (Tomasz Figa <tfiga@chromium.org>)
// Based on s5p-mfc driver by Samsung Electronics Co., Ltd.
// Copyright (C) 2011 Samsung Electronics Co., Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_ctrl_desc {
    pub cfg: v4l2_ctrl_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_ctrls {
    pub ctrls: *const rkvdec_ctrl_desc,
    pub num_ctrls: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_run {
    pub src: *mut vb2_v4l2_buffer,
    pub dst: *mut vb2_v4l2_buffer,
    pub bufs: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vp9_decoded_buffer_info {
// Info needed when the decoded frame serves as a reference frame.
    pub width: c_ushort,
    pub height: c_ushort,
    pub 4: unsigned int bit_depth :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_decoded_buffer {
// Must be the first field in this struct.
    pub base: v4l2_m2m_buffer,
    pub vp9: rkvdec_vp9_decoded_buffer_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_variant_ops {
    pub ctx): *mut *mut irqreturn_t (irq_handler)(struct rkvdec_ctx,
    pub height): *mut *mut u32 (colmv_size)(u16 width, u16,
    pub row_length): *const *const *const *const void (flatten_matrices)(u8 output, u8 input, int matrices, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_variant {
    pub num_regs: c_uint,
    pub coded_fmts: *const rkvdec_coded_fmt_desc,
    pub num_coded_fmts: usize,
    pub rcb_sizes: *const rcb_size_info,
    pub num_rcb_sizes: usize,
    pub ops: *const rkvdec_variant_ops,
    pub has_single_reg_region: bool,
    pub quirks: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_coded_fmt_ops {
    pub f): *mut v4l2_format,
    pub ctx): *mut *mut int (start)(struct rkvdec_ctx,
    pub ctx): *mut *mut void (stop)(struct rkvdec_ctx,
    pub ctx): *mut *mut int (run)(struct rkvdec_ctx,
    pub result): vb2_buffer_state,
    pub ctrl): *mut *mut *mut int (try_ctrl)(struct rkvdec_ctx ctx, struct v4l2_ctrl,
    pub ctrl): *mut v4l2_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkvdec_image_fmt {
    RKVDEC_IMG_FMT_ANY = 0,
    RKVDEC_IMG_FMT_420_8BIT,
    RKVDEC_IMG_FMT_420_10BIT,
    RKVDEC_IMG_FMT_422_8BIT,
    RKVDEC_IMG_FMT_422_10BIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_decoded_fmt_desc {
    pub fourcc: u32,
    pub image_fmt: rkvdec_image_fmt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_coded_fmt_desc {
    pub fourcc: u32,
    pub frmsize: v4l2_frmsize_stepwise,
    pub ctrls: *const rkvdec_ctrls,
    pub ops: *const rkvdec_coded_fmt_ops,
    pub num_decoded_fmts: c_uint,
    pub decoded_fmts: *const rkvdec_decoded_fmt_desc,
    pub subsystem_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_dev {
    pub v4l2_dev: v4l2_device,
    pub mdev: media_device,
    pub vdev: video_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub dev: *mut device,
    pub clocks: *mut clk_bulk_data,
    pub num_clocks: c_uint,
    pub axi_clk: *mut clk,
    pub regs: *mut void __iomem,
    pub link: *mut void __iomem,
    pub /: *mut *mut mutex vdev_lock; / serializes ioctls,
    pub watchdog_work: delayed_work,
    pub sram_pool: *mut gen_pool,
    pub iommu_domain: *mut iommu_domain,
    pub empty_domain: *mut iommu_domain,
    pub variant: *const rkvdec_variant,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_ctx {
    pub fh: v4l2_fh,
    pub coded_fmt: v4l2_format,
    pub decoded_fmt: v4l2_format,
    pub coded_fmt_desc: *const rkvdec_coded_fmt_desc,
    pub ctrl_hdl: v4l2_ctrl_handler,
    pub dev: *mut rkvdec_dev,
    pub image_fmt: rkvdec_image_fmt,
    pub rcb_config: *mut rkvdec_rcb_config,
    pub colmv_offset: u32,
    pub priv: *mut c_void,
    pub 1: u8 has_sps_st_rps:,
    pub 1: u8 has_sps_lt_rps:,
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), rkvdec_ctx: struct, _arg: fh) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkvdec_alloc_type {
    RKVDEC_ALLOC_DMA  = 0,
    RKVDEC_ALLOC_SRAM = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_aux_buf {
    pub cpu: *mut c_void,
    pub dma: dma_addr_t,
    pub size: usize,
    pub type: rkvdec_alloc_type,
}

extern "C" {
    pub fn rkvdec_run_preamble(ctx: *mut rkvdec_ctx, run: *mut rkvdec_run);
}
extern "C" {
    pub fn rkvdec_run_postamble(ctx: *mut rkvdec_ctx, run: *mut rkvdec_run);
}
extern "C" {
    pub fn rkvdec_memcpy_toio(dst: *mut void __iomem, src: *mut c_void, len: usize);
}
extern "C" {
    pub fn rkvdec_schedule_watchdog(rkvdec: *mut rkvdec_dev, timeout_threshold: u32);
}
extern "C" {
    pub fn rkvdec_quirks_disable_qos(ctx: *mut rkvdec_ctx);
}
// RKVDEC ops
// VDPU381 ops
// VDPU383 ops
