//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rga/rga.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) Rockchip Electronics Co., Ltd.
// Author: Jacob Chen <jacob-chen@iotwrt.com>
//

pub const DEFAULT_WIDTH: c_int = 100;
pub const DEFAULT_HEIGHT: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rga_frame {
// Crop
    pub crop: v4l2_rect,
// Image format
    pub fmt: *mut c_void,
    pub pix: v4l2_pix_format_mplane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rga_dma_desc {
    pub addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_rga_version {
    pub major: u32,
    pub minor: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rga_ctx {
    pub fh: v4l2_fh,
    pub rga: *mut rockchip_rga,
    pub in: rga_frame,
    pub out: rga_frame,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub cmdbuf_virt: *mut c_void,
    pub cmdbuf_phy: dma_addr_t,
    pub cmdbuf_dirty: bool,
    pub osequence: c_int,
    pub csequence: c_int,
// Control values
    pub op: u32,
    pub hflip: u32,
    pub vflip: u32,
    pub rotate: u32,
    pub fill_color: u32,
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), rga_ctx: struct, _arg: fh) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_rga {
    pub v4l2_dev: v4l2_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub vfd: *mut video_device,
    pub dev: *mut device,
    pub grf: *mut regmap,
    pub regs: *mut void __iomem,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub version: rockchip_rga_version,
// vfd lock
    pub mutex: mutex,
// ctrl parm lock
    pub ctrl_lock: spinlock_t,
    pub curr: *mut rga_ctx,
    pub hw: *const rga_hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rga_addrs {
    pub y_addr: dma_addr_t,
    pub u_addr: dma_addr_t,
    pub v_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rga_vb_buffer {
    pub vb_buf: vb2_v4l2_buffer,
    pub queue: list_head,
// RGA MMU mapping for this buffer
    pub dma_desc: *mut rga_dma_desc,
    pub dma_desc_pa: dma_addr_t,
    pub n_desc: usize,
// Plane DMA addresses after the MMU mapping of the buffer
    pub dma_addrs: rga_addrs,
}

extern "C" {
    pub fn container_of(_arg: vb, rga_vb_buffer: struct, _arg: vb_buf) -> return;
}
// RGA Buffers Manage
// RGA Hardware
extern "C" {
    pub fn readl(reg: rga->regs +) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rga_hw {
    pub card_type: *const c_char,
    pub has_internal_iommu: bool,
    pub cmdbuf_size: usize,
    pub min_height: u32 min_width,,
    pub max_height: u32 max_width,,
    pub max_scaling_factor: u8,
    pub stride_alignment: u8,
    pub features: u8,
    pub ctx): *mut *mut void (setup_cmdbuf)(struct rga_ctx,
    pub dst): *mut *mut rga_vb_buffer src, rga_vb_buffer,
    pub rga): *mut *mut bool (handle_irq)(struct rockchip_rga,
    pub rga): *mut *mut void (get_version)(struct rockchip_rga,
    pub is_output): bool,
    pub f): *mut *mut int (enum_format)(struct v4l2_fmtdesc,
}
