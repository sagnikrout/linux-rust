//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mtk-img-ipi.h
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
// Copyright (c) 2022 MediaTek Inc.
// Author: Holmes Chiou <holmes.chiou@mediatek.com>
// Ping-Hsun Wu <ping-hsun.wu@mediatek.com>
//

// ISP-MDP generic input information
pub const IMG_IPI_INIT: c_int = 1;
pub const IMG_IPI_DEINIT: c_int = 2;
pub const IMG_IPI_FRAME: c_int = 3;
pub const IMG_IPI_DEBUG: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_timeval {
    pub tv_sec: u32,
    pub tv_usec: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_addr {
    pub /: *mut *mut u64 va; / Used for Linux OS access,
    pub /: *mut *mut u32 pa; / Used for CM4 access,
    pub /: *mut *mut u32 iova; / Used for IOMMU HW access,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tuning_addr {
    pub present: u64,
    pub /: *mut *mut u32 pa; / Used for CM4 access,
    pub /: *mut *mut u32 iova; / Used for IOMMU HW access,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_sw_addr {
    pub /: *mut *mut u64 va; / Used for APMCU access,
    pub /: *mut *mut u32 pa; / Used for CM4 access,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_plane_format {
    pub size: u32,
    pub stride: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_pix_format {
    pub width: u32,
    pub height: u32,
    pub /: *mut *mut u32 colorformat; / enum mdp_color,
    pub /: *mut *mut u32 ycbcr_prof; / enum mdp_ycbcr_profile,
    pub plane_fmt: [img_plane_format; IMG_MAX_PLANES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_image_buffer {
    pub format: img_pix_format,
    pub iova: [u32; IMG_MAX_PLANES],
// enum mdp_buffer_usage, FD or advanced ISP usages
    pub usage: u32,
    pub __packed: },
pub const IMG_SUBPIXEL_SHIFT: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_input {
    pub buffer: img_image_buffer,
    pub /: *mut *mut u32 flags; / HDR, DRE, dither,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_output {
    pub buffer: img_image_buffer,
    pub crop: img_crop,
    pub rotation: i32,
    pub /: *mut *mut u32 flags; / H-flip, sharpness, dither,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ipi_frameparam {
    pub index: u32,
    pub frame_no: u32,
    pub timestamp: img_timeval,
    pub /: *mut *mut u32 type; / enum mdp_stream_type,
    pub state: u32,
    pub num_inputs: u32,
    pub num_outputs: u32,
    pub drv_data: u64,
    pub inputs: [img_input; IMG_MAX_HW_INPUTS],
    pub outputs: [img_output; IMG_MAX_HW_OUTPUTS],
    pub tuning_data: tuning_addr,
    pub subfrm_data: img_addr,
    pub config_data: img_sw_addr,
    pub self_data: img_sw_addr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_sw_buffer {
    pub /: *mut *mut u64 handle; / Used for APMCU access,
    pub /: *mut *mut u32 scp_addr; / Used for CM4 access,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ipi_param {
    pub usage: u32,
    pub frm_param: img_sw_buffer,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_frameparam {
    pub list_entry: list_head,
    pub frameparam: img_ipi_frameparam,
    pub __packed: },
// Platform config indicator
pub const MT8183: c_int = 8183;
pub const MT8188: c_int = 8195;
pub const MT8195: c_int = 8195;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_config {
    pub config_8183: img_config_8183,
    pub config_8195: img_config_8195,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_compparam {
    pub comp_8183: img_compparam_8183,
    pub comp_8195: img_compparam_8195,
}
