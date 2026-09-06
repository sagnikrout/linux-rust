//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/imagination/e5010-jpeg-enc.h
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
// Imagination E5010 JPEG Encoder driver.
//
// Copyright (C) 2023 Texas Instruments Incorporated - https://www.ti.com
//
// Author: David Huang <d-huang@ti.com>
// Author: Devarsh Thakkar <devarsht@ti.com>
//

pub const MAX_PLANES: c_int = 2;
pub const HEADER_SIZE: c_uint = 0x025D;
pub const MIN_DIMENSION: c_int = 64;
pub const MAX_DIMENSION: c_int = 8192;
pub const DEFAULT_WIDTH: c_int = 640;
pub const DEFAULT_HEIGHT: c_int = 480;

pub const JPEG_MAX_BYTES_PER_PIXEL: c_int = 2;
// JPEG marker definitions
pub const START_OF_IMAGE: c_uint = 0xFFD8;
pub const SOF_BASELINE_DCT: c_uint = 0xFFC0;
pub const END_OF_IMAGE: c_uint = 0xFFD9;
pub const START_OF_SCAN: c_uint = 0xFFDA;
// Definitions for the huffman table specification in the Marker segment
pub const DHT_MARKER: c_uint = 0xFFC4;
pub const LH_DC: c_uint = 0x001F;
pub const LH_AC: c_uint = 0x00B5;
// Definitions for the quantization table specification in the Marker segment
pub const DQT_MARKER: c_uint = 0xFFDB;
pub const ACMAX: c_uint = 0x03FF;
pub const DCMAX: c_uint = 0x07FF;
// Length and precision of the quantization table parameters
pub const LQPQ: c_uint = 0x00430;
pub const QMAX: c_int = 255;
// Misc JPEG header definitions
pub const UC_NUM_COMP: c_int = 3;
pub const PRECISION: c_int = 8;

pub const VERT_SAMPLING_FACTOR_422: c_int = 1;
pub const VERT_SAMPLING_FACTOR_420: c_int = 2;
pub const COMPONENTS_IN_SCAN: c_int = 3;
pub const PELS_IN_BLOCK: c_int = 64;
// Used for Qp table generation
pub const LUMINOSITY: c_int = 10;
pub const CONTRAST: c_int = 1;
pub const INCREASE: c_int = 2;

pub const QP_TABLE_FIELD_OFFSET: c_uint = 0x04;
//
// vb2 queue structure
// contains queue data information
//
// @fmt: format info
// @width: frame width
// @height: frame height
// @bytesperline: bytes per line in memory
// @size_image: image size in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e5010_q_data {
    pub fmt: *mut e5010_fmt,
    pub width: u32,
    pub height: u32,
    pub width_adjusted: u32,
    pub height_adjusted: u32,
    pub sizeimage: [u32; MAX_PLANES],
    pub bytesperline: [u32; MAX_PLANES],
    pub sequence: u32,
    pub crop: v4l2_rect,
    pub crop_set: bool,
}

//
// Driver device structure
// Holds all memory handles and global parameters
// Shared by all instances
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e5010_dev {
    pub dev: *mut device,
    pub v4l2_dev: v4l2_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub vdev: *mut video_device,
    pub core_base: *mut void __iomem,
    pub mmu_base: *mut void __iomem,
    pub clk: *mut clk,
    pub last_context_run: *mut e5010_context,
// Protect access to device data
    pub mutex: mutex,
// Protect access to hardware
    pub hw_lock: spinlock_t,
}

//
// Driver context structure
// One of these exists for every m2m context
// Holds context specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e5010_context {
    pub fh: v4l2_fh,
    pub e5010: *mut e5010_dev,
    pub out_queue: e5010_q_data,
    pub cap_queue: e5010_q_data,
    pub quality: c_int,
    pub update_qp: bool,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub luma_qp: [u8; QP_TABLE_SIZE],
    pub chroma_qp: [u8; QP_TABLE_SIZE],
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), e5010_context: struct, _arg: fh) -> return;
}
//
// Buffer structure
// Contains info for all buffers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e5010_buffer {
    pub buffer: v4l2_m2m_buffer,
}

//
// e5010 format structure
// contains format information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e5010_fmt {
    pub fourcc: u32,
    pub num_planes: c_uint,
    pub type: c_uint,
    pub subsampling: u32,
    pub chroma_order: u32,
    pub frmsize: v4l2_frmsize_stepwise,
}

//
// struct e5010_ctrl - contains info for each supported v4l2 control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e5010_ctrl {
    pub cid: c_uint,
    pub type: v4l2_ctrl_type,
    pub name: [c_uchar; 32],
    pub minimum: c_int,
    pub maximum: c_int,
    pub step: c_int,
    pub default_value: c_int,
    pub compound: c_uchar,
}
