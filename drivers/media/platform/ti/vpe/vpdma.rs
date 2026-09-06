//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/vpe/vpdma.h
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
// Copyright (c) 2013 Texas Instruments Inc.
//
// David Griego, <dagriego@biglakesoftware.com>
// Dale Farnsworth, <dale@farnsworth.org>
// Archit Taneja, <archit@ti.com>
//
pub const VPDMA_MAX_NUM_LIST: c_int = 8;
//
// A vpdma_buf tracks the size, DMA address and mapping status of each
// driver DMA area.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpdma_buf {
    pub addr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub size: usize,
    pub mapped: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpdma_desc_list {
    pub buf: vpdma_buf,
    pub next: *mut c_void,
    pub type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpdma_data {
    pub base: *mut void __iomem,
    pub pdev: *mut platform_device,
    pub lock: spinlock_t,
    pub hwlist_used: [bool; VPDMA_MAX_NUM_LIST],
    pub hwlist_priv: [*mut c_void; VPDMA_MAX_NUM_LIST],
// callback to VPE driver when the firmware is loaded
    pub pdev): *mut *mut void (cb)(struct platform_device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpdma_data_format_type {
    VPDMA_DATA_FMT_TYPE_YUV,
    VPDMA_DATA_FMT_TYPE_RGB,
    VPDMA_DATA_FMT_TYPE_MISC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpdma_data_format {
    pub type: vpdma_data_format_type,
    pub data_type: c_int,
    pub depth: u8,
}

// line stride of source and dest
// buffers should be 16 byte aligned
//

pub const VPDMA_LIST_TYPE_NORMAL: c_int = 0;
pub const VPDMA_LIST_TYPE_SELF_MODIFYING: c_int = 1;
pub const VPDMA_LIST_TYPE_DOORBELL: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpdma_yuv_formats {
    VPDMA_DATA_FMT_Y444 = 0,
    VPDMA_DATA_FMT_Y422,
    VPDMA_DATA_FMT_Y420,
    VPDMA_DATA_FMT_C444,
    VPDMA_DATA_FMT_C422,
    VPDMA_DATA_FMT_C420,
    VPDMA_DATA_FMT_CB420,
    VPDMA_DATA_FMT_YCR422,
    VPDMA_DATA_FMT_YC444,
    VPDMA_DATA_FMT_CRY422,
    VPDMA_DATA_FMT_CBY422,
    VPDMA_DATA_FMT_YCB422,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpdma_rgb_formats {
    VPDMA_DATA_FMT_RGB565 = 0,
    VPDMA_DATA_FMT_ARGB16_1555,
    VPDMA_DATA_FMT_ARGB16,
    VPDMA_DATA_FMT_RGBA16_5551,
    VPDMA_DATA_FMT_RGBA16,
    VPDMA_DATA_FMT_ARGB24,
    VPDMA_DATA_FMT_RGB24,
    VPDMA_DATA_FMT_ARGB32,
    VPDMA_DATA_FMT_RGBA24,
    VPDMA_DATA_FMT_RGBA32,
    VPDMA_DATA_FMT_BGR565,
    VPDMA_DATA_FMT_ABGR16_1555,
    VPDMA_DATA_FMT_ABGR16,
    VPDMA_DATA_FMT_BGRA16_5551,
    VPDMA_DATA_FMT_BGRA16,
    VPDMA_DATA_FMT_ABGR24,
    VPDMA_DATA_FMT_BGR24,
    VPDMA_DATA_FMT_ABGR32,
    VPDMA_DATA_FMT_BGRA24,
    VPDMA_DATA_FMT_BGRA32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpdma_raw_formats {
    VPDMA_DATA_FMT_RAW8 = 0,
    VPDMA_DATA_FMT_RAW16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpdma_misc_formats {
    VPDMA_DATA_FMT_MV = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpdma_frame_start_event {
    VPDMA_FSEVENT_HDMI_FID = 0,
    VPDMA_FSEVENT_DVO2_FID,
    VPDMA_FSEVENT_HDCOMP_FID,
    VPDMA_FSEVENT_SD_FID,
    VPDMA_FSEVENT_LM_FID0,
    VPDMA_FSEVENT_LM_FID1,
    VPDMA_FSEVENT_LM_FID2,
    VPDMA_FSEVENT_CHANNEL_ACTIVE,
}

// max width configurations
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpdma_max_width {
    MAX_OUT_WIDTH_UNLIMITED = 0,
    MAX_OUT_WIDTH_REG1,
    MAX_OUT_WIDTH_REG2,
    MAX_OUT_WIDTH_REG3,
    MAX_OUT_WIDTH_352,
    MAX_OUT_WIDTH_768,
    MAX_OUT_WIDTH_1280,
    MAX_OUT_WIDTH_1920,
}

// max height configurations
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpdma_max_height {
    MAX_OUT_HEIGHT_UNLIMITED = 0,
    MAX_OUT_HEIGHT_REG1,
    MAX_OUT_HEIGHT_REG2,
    MAX_OUT_HEIGHT_REG3,
    MAX_OUT_HEIGHT_288,
    MAX_OUT_HEIGHT_576,
    MAX_OUT_HEIGHT_720,
    MAX_OUT_HEIGHT_1080,
}

//
// VPDMA channel numbers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpdma_channel {
    VPE_CHAN_LUMA1_IN,
    VPE_CHAN_CHROMA1_IN,
    VPE_CHAN_LUMA2_IN,
    VPE_CHAN_CHROMA2_IN,
    VPE_CHAN_LUMA3_IN,
    VPE_CHAN_CHROMA3_IN,
    VPE_CHAN_MV_IN,
    VPE_CHAN_MV_OUT,
    VPE_CHAN_LUMA_OUT,
    VPE_CHAN_CHROMA_OUT,
    VPE_CHAN_RGB_OUT,
}

pub const VIP_CHAN_VIP2_OFFSET: c_int = 70;
pub const VIP_CHAN_MULT_PORTB_OFFSET: c_int = 16;
pub const VIP_CHAN_YUV_PORTB_OFFSET: c_int = 2;
pub const VIP_CHAN_RGB_PORTB_OFFSET: c_int = 1;
pub const VPDMA_MAX_CHANNELS: c_int = 256;
// flags for VPDMA data descriptors

//
// client identifiers used for configuration descriptors
//
pub const CFD_MMR_CLIENT: c_int = 0;
pub const CFD_SC_CLIENT: c_int = 4;
// Address data block header format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpdma_adb_hdr {
    pub offset: u32,
    pub nwords: u32,
    pub reserved0: u32,
    pub reserved1: u32,
}

// helpers for creating ADB headers for config descriptors MMRs as client

// vpdma descriptor buffer allocation and management
extern "C" {
    pub fn vpdma_alloc_desc_buf(buf: *mut vpdma_buf, size: usize) -> c_int;
}
extern "C" {
    pub fn vpdma_free_desc_buf(buf: *mut vpdma_buf);
}
extern "C" {
    pub fn vpdma_map_desc_buf(vpdma: *mut vpdma_data, buf: *mut vpdma_buf) -> c_int;
}
extern "C" {
    pub fn vpdma_unmap_desc_buf(vpdma: *mut vpdma_data, buf: *mut vpdma_buf);
}
// vpdma descriptor list funcs
extern "C" {
    pub fn vpdma_create_desc_list(list: *mut vpdma_desc_list, size: usize, type: c_int) -> c_int;
}
extern "C" {
    pub fn vpdma_reset_desc_list(list: *mut vpdma_desc_list);
}
extern "C" {
    pub fn vpdma_free_desc_list(list: *mut vpdma_desc_list);
}
extern "C" {
    pub fn vpdma_list_busy(vpdma: *mut vpdma_data, list_num: c_int) -> bool;
}
// VPDMA hardware list funcs
extern "C" {
    pub fn vpdma_hwlist_alloc(vpdma: *mut vpdma_data, priv: *mut c_void) -> c_int;
}
// helpers for creating vpdma descriptors
// vpdma list interrupt management
extern "C" {
    pub fn vpdma_get_list_stat(vpdma: *mut vpdma_data, irq_num: c_int) -> c_uint;
}
extern "C" {
    pub fn vpdma_get_list_mask(vpdma: *mut vpdma_data, irq_num: c_int) -> c_uint;
}
// vpdma client configuration
extern "C" {
    pub fn vpdma_dump_regs(vpdma: *mut vpdma_data);
}
// initialize vpdma, passed with VPE's platform device pointer
// load vpdma firmware
extern "C" {
    pub fn vpdma_load_firmware(vpdma: *mut vpdma_data) -> c_int;
}
