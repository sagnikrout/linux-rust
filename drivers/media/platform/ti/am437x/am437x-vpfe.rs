//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/am437x/am437x-vpfe.h
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
// Copyright (C) 2013 - 2014 Texas Instruments, Inc.
//
// Benoit Parrot <bparrot@ti.com>
// Lad, Prabhakar <prabhakar.csengg@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpfe_pin_pol {
    VPFE_PINPOL_POSITIVE = 0,
    VPFE_PINPOL_NEGATIVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpfe_hw_if_type {
// Raw Bayer
    VPFE_RAW_BAYER = 0,
// BT656 - 8 bit
    VPFE_BT656,
// BT656 - 10 bit
    VPFE_BT656_10BIT,
// YCbCr - 8 bit with external sync
    VPFE_YCBCR_SYNC_8,
// YCbCr - 16 bit with external sync
    VPFE_YCBCR_SYNC_16,
}

// interface description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_hw_if_param {
    pub if_type: vpfe_hw_if_type,
    pub hdpol: vpfe_pin_pol,
    pub vdpol: vpfe_pin_pol,
    pub bus_width: c_uint,
}

pub const VPFE_MAX_SUBDEV: c_int = 1;
pub const VPFE_MAX_INPUTS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_std_info {
    pub active_pixels: c_int,
    pub active_lines: c_int,
// current frame format
    pub frame_format: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_route {
    pub input: u32,
    pub output: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_subdev_info {
// Sub device group id
    pub grp_id: c_int,
// inputs available at the sub device
    pub inputs: [v4l2_input; VPFE_MAX_INPUTS],
// Sub dev routing information for each input
    pub routes: *mut vpfe_route,
// check if sub dev supports routing
    pub can_route: c_int,
// ccdc bus/interface configuration
    pub vpfe_param: vpfe_hw_if_param,
    pub sd: *mut v4l2_subdev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_config {
// information about each subdev
    pub sub_devs: [vpfe_subdev_info; VPFE_MAX_SUBDEV],
// Flat array, arranged in groups
    pub asd: [*mut v4l2_async_connection; VPFE_MAX_SUBDEV],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_cap_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ccdc_pixfmt {
    CCDC_PIXFMT_RAW = 0,
    CCDC_PIXFMT_YCBCR_16BIT,
    CCDC_PIXFMT_YCBCR_8BIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ccdc_frmfmt {
    CCDC_FRMFMT_PROGRESSIVE = 0,
    CCDC_FRMFMT_INTERLACED,
}

// PIXEL ORDER IN MEMORY from LSB to MSB
// only applicable for 8-bit input mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ccdc_pixorder {
    CCDC_PIXORDER_YCBYCR,
    CCDC_PIXORDER_CBYCRY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ccdc_buftype {
    CCDC_BUFTYPE_FLD_INTERLEAVED,
    CCDC_BUFTYPE_FLD_SEPARATED
}

// returns the highest bit used for the gamma
// returns the highest bit used for this data size
// Structure for CCDC configuration parameters for raw capture mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccdc_params_raw {
// pixel format
    pub pix_fmt: ccdc_pixfmt,
// progressive or interlaced frame
    pub frm_fmt: ccdc_frmfmt,
    pub win: v4l2_rect,
// Current Format Bytes Per Pixels
    pub bytesperpixel: c_uint,
// Current Format Bytes per Lines
// (Aligned to 32 bytes) used for HORZ_INFO
//
    pub bytesperline: c_uint,
// field id polarity
    pub fid_pol: vpfe_pin_pol,
// vertical sync polarity
    pub vd_pol: vpfe_pin_pol,
// horizontal sync polarity
    pub hd_pol: vpfe_pin_pol,
// interleaved or separated fields
    pub buf_type: ccdc_buftype,
//
// enable to store the image in inverse
// order in memory(bottom to top)
//
    pub image_invert_enable: c_uchar,
// configurable parameters
    pub config_params: vpfe_ccdc_config_params_raw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccdc_params_ycbcr {
// pixel format
    pub pix_fmt: ccdc_pixfmt,
// progressive or interlaced frame
    pub frm_fmt: ccdc_frmfmt,
    pub win: v4l2_rect,
// Current Format Bytes Per Pixels
    pub bytesperpixel: c_uint,
// Current Format Bytes per Lines
// (Aligned to 32 bytes) used for HORZ_INFO
//
    pub bytesperline: c_uint,
// field id polarity
    pub fid_pol: vpfe_pin_pol,
// vertical sync polarity
    pub vd_pol: vpfe_pin_pol,
// horizontal sync polarity
    pub hd_pol: vpfe_pin_pol,
// enable BT.656 embedded sync mode
    pub bt656_enable: c_int,
// cb:y:cr:y or y:cb:y:cr in memory
    pub pix_order: ccdc_pixorder,
// interleaved or separated fields
    pub buf_type: ccdc_buftype,
}

//
// CCDC operational configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccdc_config {
// CCDC interface type
    pub if_type: vpfe_hw_if_type,
// Raw Bayer configuration
    pub bayer: ccdc_params_raw,
// YCbCr configuration
    pub ycbcr: ccdc_params_ycbcr,
// ccdc base address
    pub base_addr: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_ccdc {
    pub ccdc_cfg: ccdc_config,
    pub sizeof(u32)]: u32 ccdc_ctx[VPFE_REG_END /,
}

//
// struct vpfe_fmt - VPFE media bus format information
// fourcc: V4L2 pixel format code
// code: V4L2 media bus format code
// bitsperpixel: Bits per pixel over the bus
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_fmt {
    pub fourcc: u32,
    pub code: u32,
    pub bitsperpixel: u32,
}

//
// When formats[] is modified make sure to adjust this value also.
// Expect compile time warnings if VPFE_NUM_FORMATS is smaller then
// the number of elements in formats[].
//
pub const VPFE_NUM_FORMATS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpfe_device {
// V4l2 specific parameters
// Identifies video device for this channel
    pub video_dev: video_device,
// sub devices
    pub sd: *mut v4l2_subdev,
// vpfe cfg
    pub cfg: *mut vpfe_config,
// V4l2 device
    pub v4l2_dev: v4l2_device,
// parent device
    pub pdev: *mut device,
// subdevice async Notifier
    pub notifier: v4l2_async_notifier,
// Indicates id of the field which is being displayed
    pub field: unsigned,
    pub sequence: unsigned,
// current interface type
    pub vpfe_if_params: vpfe_hw_if_param,
// ptr to currently selected sub device
    pub current_subdev: *mut vpfe_subdev_info,
// current input at the sub device
    pub current_input: c_int,
// Keeps track of the information about the standard
    pub std_info: vpfe_std_info,
// std index into std table
    pub std_index: c_int,
// IRQs used when CCDC output to SDRAM
    pub irq: c_uint,
// Pointer pointing to current v4l2_buffer
    pub cur_frm: *mut vpfe_cap_buffer,
// Pointer pointing to next v4l2_buffer
    pub next_frm: *mut vpfe_cap_buffer,
// Used to store pixel format
    pub fmt: v4l2_format,
// Used to keep a reference to the current vpfe_fmt
    pub current_vpfe_fmt: *mut vpfe_fmt,
    pub active_fmt: [*mut vpfe_fmt; VPFE_NUM_FORMATS],
    pub num_active_fmt: c_uint,
//
// used when IMP is chained to store the crop window which
// is different from the image window
//
    pub crop: v4l2_rect,
// Buffer queue used in vb2
    pub buffer_queue: vb2_queue,
// Queue of filled frames
    pub dma_queue: list_head,
// IRQ lock for DMA queue
    pub dma_queue_lock: spinlock_t,
// lock used to access this structure
    pub lock: mutex,
//
// offset where second field starts from the starting of the
// buffer for field separated YCbCr formats
//
    pub field_off: u32,
    pub ccdc: vpfe_ccdc,
    pub stopping: c_int,
    pub capture_stop: completion,
}
