//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/nxp/imx-jpeg/mxc-jpeg.h
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
// i.MX8QXP/i.MX8QM JPEG encoder/decoder v4l2 driver
//
// Copyright 2018-2019 NXP
//

pub const MXC_JPEG_FMT_TYPE_ENC: c_int = 0;
pub const MXC_JPEG_FMT_TYPE_RAW: c_int = 1;
pub const MXC_JPEG_DEFAULT_WIDTH: c_int = 1280;
pub const MXC_JPEG_DEFAULT_HEIGHT: c_int = 720;

pub const MXC_JPEG_MIN_WIDTH: c_int = 64;
pub const MXC_JPEG_MIN_HEIGHT: c_int = 64;
pub const MXC_JPEG_MAX_WIDTH: c_uint = 0x2000;
pub const MXC_JPEG_MAX_HEIGHT: c_uint = 0x2000;
pub const MXC_JPEG_MAX_LINE: c_uint = 0x8000;
pub const MXC_JPEG_MAX_CFG_STREAM: c_uint = 0x1000;
pub const MXC_JPEG_H_ALIGN: c_int = 3;
pub const MXC_JPEG_W_ALIGN: c_int = 3;
pub const MXC_JPEG_MAX_SIZEIMAGE: c_uint = 0xFFFFFC00;
pub const MXC_JPEG_MAX_PLANES: c_int = 2;
pub const MXC_JPEG_PATTERN_WIDTH: c_int = 128;
pub const MXC_JPEG_PATTERN_HEIGHT: c_int = 64;
pub const MXC_JPEG_ADDR_ALIGNMENT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxc_jpeg_enc_state {
    MXC_JPEG_ENCODING	= 0, /* jpeg encode phase */
    MXC_JPEG_ENC_CONF	= 1, /* jpeg encoder config phase */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxc_jpeg_mode {
    MXC_JPEG_DECODE	= 0, /* jpeg decode mode */
    MXC_JPEG_ENCODE	= 1, /* jpeg encode mode */
}

//
// struct mxc_jpeg_fmt - driver's internal color format data
// @name:	format description
// @fourcc:	fourcc code, 0 if not applicable
// @subsampling: subsampling of jpeg components
// @nc:		number of color components
// @depth:	number of bits per pixel
// @mem_planes:	number of memory planes (1 for packed formats)
// @comp_planes:number of component planes, which includes the alpha plane (1 to 4).
// @h_align:	horizontal alignment order (align to 2^h_align)
// @v_align:	vertical alignment order (align to 2^v_align)
// @flags:	flags describing format applicability
// @precision:  jpeg sample precision
// @is_rgb:     is an RGB pixel format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_jpeg_fmt {
    pub name: *const c_char,
    pub fourcc: u32,
    pub subsampling: v4l2_jpeg_chroma_subsampling,
    pub nc: c_int,
    pub depth: c_int,
    pub mem_planes: c_int,
    pub comp_planes: c_int,
    pub h_align: c_int,
    pub v_align: c_int,
    pub flags: u32,
    pub precision: u8,
    pub is_rgb: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_jpeg_desc {
    pub next_descpt_ptr: u32,
    pub buf_base0: u32,
    pub buf_base1: u32,
    pub line_pitch: u32,
    pub stm_bufbase: u32,
    pub stm_bufsize: u32,
    pub imgsize: u32,
    pub stm_ctrl: u32,
// below parameters are valid for v1
    pub mode: u32,
    pub cfg_mode: u32,
    pub quality: u32,
    pub rc_regs_sel: u32,
    pub lumth: u32,
    pub chrth: u32,
    pub nomfrsize_lo: u32,
    pub nomfrsize_hi: u32,
    pub ofbsize_lo: u32,
    pub ofbsize_hi: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_jpeg_q_data {
    pub fmt: *const mxc_jpeg_fmt,
    pub sizeimage: [u32; MXC_JPEG_MAX_PLANES],
    pub bytesperline: [u32; MXC_JPEG_MAX_PLANES],
    pub w: c_int,
    pub w_adjusted: c_int,
    pub h: c_int,
    pub h_adjusted: c_int,
    pub sequence: c_uint,
    pub crop: v4l2_rect,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_jpeg_ctx {
    pub mxc_jpeg: *mut mxc_jpeg_dev,
    pub out_q: mxc_jpeg_q_data,
    pub cap_q: mxc_jpeg_q_data,
    pub fh: v4l2_fh,
    pub enc_state: mxc_jpeg_enc_state,
    pub slot: c_int,
    pub source_change: c_uint,
    pub need_initial_source_change_evt: bool,
    pub header_parsed: bool,
    pub extseq: bool,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub jpeg_quality: u8,
    pub task_timer: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_jpeg_slot_data {
    pub slot: c_int,
    pub used: bool,
    pub descriptor: *mut *mut mxc_jpeg_desc desc; // enc/dec,
    pub descriptor: *mut *mut mxc_jpeg_desc cfg_desc; // configuration,
    pub address: *mut *mut void cfg_stream_vaddr; // configuration bitstream virtual,
    pub cfg_stream_size: c_uint,
    pub desc_handle: dma_addr_t,
    pub address: dma_addr_t cfg_desc_handle; // configuration descriptor dma,
    pub address: dma_addr_t cfg_stream_handle; // configuration bitstream dma,
    pub cfg_dec_size: dma_addr_t,
    pub cfg_dec_vaddr: *mut c_void,
    pub cfg_dec_daddr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_jpeg_enc_ops {
// Manual configuration (v0 hardware) - two-phase process
    pub ctx): *mut *mut void (enter_config_mode)(struct mxc_jpeg_ctx,
    pub ctx): *mut *mut void (exit_config_mode)(struct mxc_jpeg_ctx,
// Descriptor-based configuration (v1 hardware) - single-phase
    pub ctx): *mut *mut void (setup_desc)(struct mxc_jpeg_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_jpeg_dev {
    pub /: *mut *mut spinlock_t hw_lock; / hardware access lock,
    pub mode: c_uint,
    pub /: *mut *mut mutex lock; / v4l2 ioctls serialization,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub pdev: *mut platform_device,
    pub dev: *mut device,
    pub base_reg: *mut void __iomem,
    pub v4l2_dev: v4l2_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub dec_vdev: *mut video_device,
    pub slot_data: mxc_jpeg_slot_data,
    pub num_domains: c_int,
    pub pd_dev: *mut device,
    pub pd_link: *mut device_link,
    pub sram_pool: *mut gen_pool,
    pub enc_cfg_ops: *const mxc_jpeg_enc_ops,
}

//
// struct mxc_jpeg_sof_comp - JPEG Start Of Frame component fields
// @id:				component id
// @v:				vertical sampling
// @h:				horizontal sampling
// @quantization_table_no:	id of quantization table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_jpeg_sof_comp {
    pub id: u8,
    pub :4: u8 v,
    pub :4: u8 h,
    pub quantization_table_no: u8,
    pub __packed: },
pub const MXC_JPEG_MAX_COMPONENTS: c_int = 4;
//
// struct mxc_jpeg_sof - JPEG Start Of Frame marker fields
// @length:		Start of Frame length
// @precision:		precision (bits per pixel per color component)
// @height:		image height
// @width:		image width
// @components_no:	number of color components
// @comp:		component fields for each color component
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_jpeg_sof {
    pub length: u16,
    pub precision: u8,
    pub width: u16 height,,
    pub components_no: u8,
    pub comp: [mxc_jpeg_sof_comp; MXC_JPEG_MAX_COMPONENTS],
    pub __packed: },
//
// struct mxc_jpeg_sos_comp - JPEG Start Of Scan component fields
// @id:			component id
// @huffman_table_no:	id of the Huffman table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_jpeg_sos_comp {
    pub id*/: *mut *mut u8 id; /component,
    pub huffman_table_no: u8,
    pub __packed: },
//
// struct mxc_jpeg_sos - JPEG Start Of Scan marker fields
// @length:		Start of Frame length
// @components_no:	number of color components
// @comp:		SOS component fields for each color component
// @ignorable_bytes:	ignorable bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxc_jpeg_sos {
    pub length: u16,
    pub components_no: u8,
    pub comp: [mxc_jpeg_sos_comp; MXC_JPEG_MAX_COMPONENTS],
    pub ignorable_bytes: [u8; 3],
    pub __packed: },
