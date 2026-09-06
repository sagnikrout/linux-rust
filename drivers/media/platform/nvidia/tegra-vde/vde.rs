//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/nvidia/tegra-vde/vde.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// NVIDIA Tegra Video decoder driver
//
// Copyright (C) 2016-2019 GRATE-DRIVER project
//

pub const ICMDQUE_WR: c_uint = 0x00;
pub const CMDQUE_CONTROL: c_uint = 0x08;
pub const INTR_STATUS: c_uint = 0x18;
pub const BSE_INT_ENB: c_uint = 0x40;
pub const BSE_CONFIG: c_uint = 0x44;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_video_frame {
    pub y_dmabuf_attachment: *mut dma_buf_attachment,
    pub cb_dmabuf_attachment: *mut dma_buf_attachment,
    pub cr_dmabuf_attachment: *mut dma_buf_attachment,
    pub aux_dmabuf_attachment: *mut dma_buf_attachment,
    pub y_addr: dma_addr_t,
    pub cb_addr: dma_addr_t,
    pub cr_addr: dma_addr_t,
    pub aux_addr: dma_addr_t,
    pub frame_num: u32,
    pub flags: u32,
    pub luma_atoms_pitch: u32,
    pub chroma_atoms_pitch: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_coded_fmt_desc {
    pub fourcc: u32,
    pub frmsize: v4l2_frmsize_stepwise,
    pub num_decoded_fmts: c_uint,
    pub decoded_fmts: *const u32,
    pub ctx): *mut *mut int (decode_run)(struct tegra_ctx,
    pub ctx): *mut *mut int (decode_wait)(struct tegra_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_vde_soc {
    pub supports_ref_pic_marking: bool,
    pub coded_fmts: *const tegra_coded_fmt_desc,
    pub num_coded_fmts: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_vde_bo {
    pub iova: *mut iova,
    pub sgt: sg_table,
    pub vde: *mut tegra_vde,
    pub dma_dir: dma_data_direction,
    pub dma_attrs: c_ulong,
    pub dma_handle: dma_addr_t,
    pub dma_addr: dma_addr_t,
    pub dma_cookie: *mut c_void,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_vde {
    pub sxe: *mut void __iomem,
    pub bsev: *mut void __iomem,
    pub mbe: *mut void __iomem,
    pub ppe: *mut void __iomem,
    pub mce: *mut void __iomem,
    pub tfe: *mut void __iomem,
    pub ppb: *mut void __iomem,
    pub vdma: *mut void __iomem,
    pub frameid: *mut void __iomem,
    pub dev: *mut device,
    pub lock: mutex,
    pub map_lock: mutex,
    pub map_list: list_head,
    pub rst: *mut reset_control,
    pub rst_mc: *mut reset_control,
    pub pmc: *mut tegra_pmc,
    pub iram_pool: *mut gen_pool,
    pub decode_completion: completion,
    pub clk: *mut clk,
    pub domain: *mut iommu_domain,
    pub group: *mut iommu_group,
    pub iova: iova_domain,
    pub iova_resv_static_addresses: *mut iova,
    pub iova_resv_last_page: *mut iova,
    pub soc: *const tegra_vde_soc,
    pub secure_bo: *mut tegra_vde_bo,
    pub bitstream_data_addr: dma_addr_t,
    pub iram_lists_addr: dma_addr_t,
    pub iram: *mut u32,
    pub v4l2_dev: v4l2_device,
    pub m2m: *mut v4l2_m2m_dev,
    pub mdev: media_device,
    pub vdev: video_device,
    pub v4l2_lock: mutex,
    pub wq: *mut workqueue_struct,
    pub 1]: tegra_video_frame frames[V4L2_H264_NUM_DPB_ENTRIES +,
}

extern "C" {
    pub fn tegra_vde_free_bo(bo: *mut tegra_vde_bo);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_ctx_h264 {
    pub decode_params: *const v4l2_ctrl_h264_decode_params,
    pub sps: *const v4l2_ctrl_h264_sps,
    pub pps: *const v4l2_ctrl_h264_pps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_ctx {
    pub vde: *mut tegra_vde,
    pub h264: tegra_ctx_h264,
    pub work: work_struct,
    pub fh: v4l2_fh,
    pub hdl: v4l2_ctrl_handler,
    pub coded_fmt: v4l2_format,
    pub decoded_fmt: v4l2_format,
    pub coded_fmt_desc: *const tegra_coded_fmt_desc,
    pub ctrls: [*mut v4l2_ctrl; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_m2m_buffer {
    pub m2m: v4l2_m2m_buffer,
    pub a: [*mut dma_buf_attachment; VB2_MAX_PLANES],
    pub dma_base: [dma_addr_t; VB2_MAX_PLANES],
    pub dma_addr: [dma_addr_t; VB2_MAX_PLANES],
    pub iova: [*mut iova; VB2_MAX_PLANES],
    pub aux: *mut tegra_vde_bo,
    pub b_frame: bool,
}

extern "C" {
    pub fn container_of(_arg: m2m, tegra_m2m_buffer: struct, _arg: m2m) -> return;
}
extern "C" {
    pub fn tegra_vde_prepare_control_data(ctx: *mut tegra_ctx, id: u32);
}
extern "C" {
    pub fn tegra_vde_readl(vde: *mut tegra_vde, base: *mut void __iomem, offset: u32) -> u32;
}
extern "C" {
    pub fn tegra_vde_h264_decode_run(ctx: *mut tegra_ctx) -> c_int;
}
extern "C" {
    pub fn tegra_vde_h264_decode_wait(ctx: *mut tegra_ctx) -> c_int;
}
extern "C" {
    pub fn tegra_vde_iommu_init(vde: *mut tegra_vde) -> c_int;
}
extern "C" {
    pub fn tegra_vde_iommu_deinit(vde: *mut tegra_vde);
}
extern "C" {
    pub fn tegra_vde_iommu_unmap(vde: *mut tegra_vde, iova: *mut iova);
}
extern "C" {
    pub fn tegra_vde_dmabuf_cache_unmap_sync(vde: *mut tegra_vde);
}
extern "C" {
    pub fn tegra_vde_dmabuf_cache_unmap_all(vde: *mut tegra_vde);
}
extern "C" {
    pub fn tegra_vde_v4l2_init(vde: *mut tegra_vde) -> c_int;
}
extern "C" {
    pub fn tegra_vde_v4l2_deinit(vde: *mut tegra_vde);
}
