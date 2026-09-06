//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/vsp1.h
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
// vsp1.h  --  R-Car VSP1 API
//
// Copyright (C) 2015 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

// -----------------------------------------------------------------------------
// VSP1 DU interface
//
extern "C" {
    pub fn vsp1_du_init(dev: *mut device) -> c_int;
}

//
// struct vsp1_du_lif_config - VSP LIF configuration
// @width: output frame width
// @height: output frame height
// @interlaced: true for interlaced pipelines
// @callback: frame completion callback function (optional). When a callback
// is provided, the VSP driver guarantees that it will be called once
// and only once for each vsp1_du_atomic_flush() call.
// @callback_data: data to be passed to the frame completion callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_du_lif_config {
    pub width: c_uint,
    pub height: c_uint,
    pub interlaced: bool,
    pub crc): *mut *mut *mut void (callback)(void data, unsigned int status, u32,
    pub callback_data: *mut c_void,
}

extern "C" {
    pub fn vsp1_du_disable(dev: *mut device, pipe_index: c_uint) -> c_int;
}
//
// struct vsp1_du_atomic_config - VSP atomic configuration parameters
// @pixelformat: plane pixel format (V4L2 4CC)
// @pitch: line pitch in bytes for the first plane
// @mem: DMA memory address for each plane of the frame buffer
// @src: source rectangle in the frame buffer (integer coordinates)
// @dst: destination rectangle on the display (integer coordinates)
// @alpha: alpha value (0: fully transparent, 255: fully opaque)
// @zpos: Z position of the plane (from 0 to number of planes minus 1)
// @premult: true for premultiplied alpha
// @color_encoding: color encoding (valid for YUV formats only)
// @color_range: color range (valid for YUV formats only)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_du_atomic_config {
    pub pixelformat: u32,
    pub pitch: c_uint,
    pub mem: [dma_addr_t; 3],
    pub src: v4l2_rect,
    pub dst: v4l2_rect,
    pub alpha: c_uint,
    pub zpos: c_uint,
    pub premult: bool,
    pub color_encoding: v4l2_ycbcr_encoding,
    pub color_range: v4l2_quantization,
}

//
// enum vsp1_du_crc_source - Source used for CRC calculation
// @VSP1_DU_CRC_NONE: CRC calculation disabled
// @VSP1_DU_CRC_PLANE: Perform CRC calculation on an input plane
// @VSP1_DU_CRC_OUTPUT: Perform CRC calculation on the composed output
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vsp1_du_crc_source {
    VSP1_DU_CRC_NONE,
    VSP1_DU_CRC_PLANE,
    VSP1_DU_CRC_OUTPUT,
}

//
// struct vsp1_du_crc_config - VSP CRC computation configuration parameters
// @source: source for CRC calculation
// @index: index of the CRC source plane (when source is set to plane)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_du_crc_config {
    pub source: vsp1_du_crc_source,
    pub index: c_uint,
}

//
// struct vsp1_du_writeback_config - VSP writeback configuration parameters
// @pixelformat: plane pixel format (V4L2 4CC)
// @pitch: line pitch in bytes for the first plane
// @mem: DMA memory address for each plane of the frame buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_du_writeback_config {
    pub pixelformat: u32,
    pub pitch: c_uint,
    pub mem: [dma_addr_t; 3],
}

//
// struct vsp1_du_atomic_pipe_config - VSP atomic pipe configuration parameters
// @crc: CRC computation configuration
// @writeback: writeback configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_du_atomic_pipe_config {
    pub crc: vsp1_du_crc_config,
    pub writeback: vsp1_du_writeback_config,
}

extern "C" {
    pub fn vsp1_du_atomic_begin(dev: *mut device, pipe_index: c_uint);
}
extern "C" {
    pub fn vsp1_du_map_sg(dev: *mut device, sgt: *mut sg_table) -> c_int;
}
extern "C" {
    pub fn vsp1_du_unmap_sg(dev: *mut device, sgt: *mut sg_table);
}
// -----------------------------------------------------------------------------
// VSP1 ISP interface
//
// struct vsp1_isp_buffer_desc - Describe a buffer allocated by VSPX
// @size: Byte size of the buffer allocated by VSPX
// @cpu_addr: CPU-mapped address of a buffer allocated by VSPX
// @dma_addr: bus address of a buffer allocated by VSPX
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_isp_buffer_desc {
    pub size: usize,
    pub cpu_addr: *mut c_void,
    pub dma_addr: dma_addr_t,
}

//
// struct vsp1_isp_job_desc - Describe a VSPX buffer transfer request
// @config: ConfigDMA buffer descriptor
// @config.pairs: number of reg-value pairs in the ConfigDMA buffer
// @config.mem: bus address of the ConfigDMA buffer
// @img: RAW image buffer descriptor
// @img.fmt: RAW image format
// @img.mem: bus address of the RAW image buffer
// @dl: pointer to the display list populated by the VSPX driver in the
// vsp1_isp_job_prepare() function
//
// Describe a transfer request for the VSPX to perform on behalf of the ISP.
// The job descriptor contains an optional ConfigDMA buffer and one RAW image
// buffer. Set config.pairs to 0 if no ConfigDMA buffer should be transferred.
// The minimum number of config.pairs that can be written using ConfigDMA is 17.
// A number of pairs < 16 corrupts the output image. A number of pairs == 16
// freezes the VSPX operation. If the ISP driver has to write less than 17 pairs
// it shall pad the buffer with writes directed to registers that have no effect
// or avoid using ConfigDMA at all for such small write sequences.
//
// The ISP driver shall pass an instance this type to the vsp1_isp_job_prepare()
// function that will populate the display list pointer @dl using the @config
// and @img descriptors. When the job has to be run on the VSPX, the descriptor
// shall be passed to vsp1_isp_job_run() which consumes the display list.
//
// Job descriptors not yet run shall be released with a call to
// vsp1_isp_job_release() when stopping the streaming in order to properly
// release the resources acquired by vsp1_isp_job_prepare().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_isp_job_desc {
    pub pairs: c_uint,
    pub mem: dma_addr_t,
    pub config: },
    pub fmt: v4l2_pix_format_mplane,
    pub mem: dma_addr_t,
    pub img: },
    pub dl: *mut vsp1_dl_list,
}

//
// struct vsp1_vspx_frame_end - VSPX frame end callback data
// @vspx_frame_end: Frame end callback. Called after a transfer job has been
// completed. If the job includes both a ConfigDMA and a
// RAW image, the callback is called after both have been
// transferred
// @frame_end_data: Frame end callback data, passed to vspx_frame_end
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_vspx_frame_end {
    pub data): *mut *mut void (vspx_frame_end)(void,
    pub frame_end_data: *mut c_void,
}

extern "C" {
    pub fn vsp1_isp_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn vsp1_isp_stop_streaming(dev: *mut device);
}
extern "C" {
    pub fn vsp1_isp_job_run(dev: *mut device, job: *mut vsp1_isp_job_desc) -> c_int;
}
extern "C" {
    pub fn vsp1_isp_job_release(dev: *mut device, job: *mut vsp1_isp_job_desc);
}
