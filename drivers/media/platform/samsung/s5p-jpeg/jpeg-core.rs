//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-jpeg/jpeg-core.h
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
// linux/drivers/media/platform/samsung/s5p-jpeg/jpeg-core.h
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

pub const JPEG_MAX_CLOCKS: c_int = 4;
// JPEG compression quality setting
pub const S5P_JPEG_COMPR_QUAL_BEST: c_int = 0;
pub const S5P_JPEG_COMPR_QUAL_WORST: c_int = 3;
// JPEG RGB to YCbCr conversion matrix coefficients
pub const S5P_JPEG_COEF11: c_uint = 0x4d;
pub const S5P_JPEG_COEF12: c_uint = 0x97;
pub const S5P_JPEG_COEF13: c_uint = 0x1e;
pub const S5P_JPEG_COEF21: c_uint = 0x2c;
pub const S5P_JPEG_COEF22: c_uint = 0x57;
pub const S5P_JPEG_COEF23: c_uint = 0x83;
pub const S5P_JPEG_COEF31: c_uint = 0x83;
pub const S5P_JPEG_COEF32: c_uint = 0x6e;
pub const S5P_JPEG_COEF33: c_uint = 0x13;
pub const EXYNOS3250_IRQ_TIMEOUT: c_uint = 0x10000000;
// Flags that indicate a format can be used for capture/output

pub const S5P_JPEG_ENCODE: c_int = 0;
pub const S5P_JPEG_DECODE: c_int = 1;

pub const FMT_TYPE_OUTPUT: c_int = 0;
pub const FMT_TYPE_CAPTURE: c_int = 1;
pub const SJPEG_SUBSAMPLING_444: c_uint = 0x11;
pub const SJPEG_SUBSAMPLING_422: c_uint = 0x21;
pub const SJPEG_SUBSAMPLING_420: c_uint = 0x22;
pub const S5P_JPEG_MAX_MARKER: c_int = 4;
// Version numbers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sjpeg_version {
    SJPEG_S5P,
    SJPEG_EXYNOS3250,
    SJPEG_EXYNOS4,
    SJPEG_EXYNOS5420,
    SJPEG_EXYNOS5433,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum exynos4_jpeg_result {
    OK_ENC_OR_DEC,
    ERR_PROT,
    ERR_DEC_INVALID_FORMAT,
    ERR_MULTI_SCAN,
    ERR_FRAME,
    ERR_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum exynos4_jpeg_img_quality_level {
    QUALITY_LEVEL_1 = 0,	/* high */
    QUALITY_LEVEL_2,
    QUALITY_LEVEL_3,
    QUALITY_LEVEL_4,	/* low */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s5p_jpeg_ctx_state {
    JPEGCTX_RUNNING = 0,
    JPEGCTX_RESOLUTION_CHANGE,
}

//
// struct s5p_jpeg - JPEG IP abstraction
// @lock:		the mutex protecting this structure
// @slock:		spinlock protecting the device contexts
// @v4l2_dev:		v4l2 device for mem2mem mode
// @vfd_encoder:	video device node for encoder mem2mem mode
// @vfd_decoder:	video device node for decoder mem2mem mode
// @m2m_dev:		v4l2 mem2mem device data
// @regs:		JPEG IP registers mapping
// @irq:		JPEG IP irq
// @irq_ret:		JPEG IP irq result value
// @clocks:		JPEG IP clock(s)
// @dev:		JPEG IP struct device
// @variant:		driver variant to be used
// @irq_status:		interrupt flags set during single encode/decode
// operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_jpeg {
    pub lock: mutex,
    pub slock: spinlock_t,
    pub v4l2_dev: v4l2_device,
    pub vfd_encoder: *mut video_device,
    pub vfd_decoder: *mut video_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub regs: *mut void __iomem,
    pub irq: c_uint,
    pub irq_ret: exynos4_jpeg_result,
    pub clocks: [*mut clk; JPEG_MAX_CLOCKS],
    pub dev: *mut device,
    pub variant: *mut s5p_jpeg_variant,
    pub irq_status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_jpeg_variant {
    pub version: c_uint,
    pub fmt_ver_flag: c_uint,
    pub hw3250_compat:1: c_uint,
    pub htbl_reinit:1: c_uint,
    pub hw_ex4_compat:1: c_uint,
    pub m2m_ops: *const v4l2_m2m_ops,
    pub priv): *mut *mut irqreturn_t (jpeg_irq)(int irq, void,
    pub clk_names: [*const c_char; JPEG_MAX_CLOCKS],
    pub num_clocks: c_int,
}

//
// struct s5p_jpeg_fmt - driver's internal color format data
// @fourcc:	the fourcc code, 0 if not applicable
// @depth:	number of bits per pixel
// @colplanes:	number of color planes (1 for packed formats)
// @memplanes:	number of memory planes (1 for packed formats)
// @h_align:	horizontal alignment order (align to 2^h_align)
// @v_align:	vertical alignment order (align to 2^v_align)
// @subsampling:subsampling of a raw format or a JPEG
// @flags:	flags describing format applicability
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_jpeg_fmt {
    pub fourcc: u32,
    pub depth: c_int,
    pub colplanes: c_int,
    pub memplanes: c_int,
    pub h_align: c_int,
    pub v_align: c_int,
    pub subsampling: c_int,
    pub flags: u32,
}

//
// struct s5p_jpeg_marker - collection of markers from jpeg header
// @marker:	markers' positions relative to the buffer beginning
// @len:	markers' payload lengths (without length field)
// @n:		number of markers in collection
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_jpeg_marker {
    pub marker: [u32; S5P_JPEG_MAX_MARKER],
    pub len: [u32; S5P_JPEG_MAX_MARKER],
    pub n: u32,
}

//
// struct s5p_jpeg_q_data - parameters of one queue
// @fmt:	driver-specific format of this queue
// @w:		image width
// @h:		image height
// @sos:	JPEG_MARKER_SOS's position relative to the buffer beginning
// @dht:	JPEG_MARKER_DHT' positions relative to the buffer beginning
// @dqt:	JPEG_MARKER_DQT' positions relative to the buffer beginning
// @sof:	JPEG_MARKER_SOF0's position relative to the buffer beginning
// @sof_len:	JPEG_MARKER_SOF0's payload length (without length field itself)
// @size:	image buffer size in bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_jpeg_q_data {
    pub fmt: *mut s5p_jpeg_fmt,
    pub w: u32,
    pub h: u32,
    pub sos: u32,
    pub dht: s5p_jpeg_marker,
    pub dqt: s5p_jpeg_marker,
    pub sof: u32,
    pub sof_len: u32,
    pub size: u32,
}

//
// struct s5p_jpeg_ctx - the device context data
// @jpeg:		JPEG IP device for this context
// @mode:		compression (encode) operation or decompression (decode)
// @compr_quality:	destination image quality in compression (encode) mode
// @restart_interval:	JPEG restart interval for JPEG encoding
// @subsampling:	subsampling of a raw format or a JPEG
// @out_q:		source (output) queue information
// @cap_q:		destination (capture) queue queue information
// @scale_factor:	scale factor for JPEG decoding
// @crop_rect:		a rectangle representing crop area of the output buffer
// @fh:			V4L2 file handle
// @hdr_parsed:		set if header has been parsed during decompression
// @crop_altered:	set if crop rectangle has been altered by the user space
// @ctrl_handler:	controls handler
// @state:		state of the context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_jpeg_ctx {
    pub jpeg: *mut s5p_jpeg,
    pub mode: c_uint,
    pub compr_quality: c_ushort,
    pub restart_interval: c_ushort,
    pub subsampling: c_ushort,
    pub out_q: s5p_jpeg_q_data,
    pub cap_q: s5p_jpeg_q_data,
    pub scale_factor: c_uint,
    pub crop_rect: v4l2_rect,
    pub fh: v4l2_fh,
    pub hdr_parsed: bool,
    pub crop_altered: bool,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub state: s5p_jpeg_ctx_state,
}

//
// struct s5p_jpeg_buffer - description of memory containing input JPEG data
// @size:	buffer size
// @curr:	current position in the buffer
// @data:	pointer to the data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_jpeg_buffer {
    pub size: c_ulong,
    pub curr: c_ulong,
    pub data: c_ulong,
}

//
// struct s5p_jpeg_addr - JPEG converter physical address set for DMA
// @y:   luminance plane physical address
// @cb:  Cb plane physical address
// @cr:  Cr plane physical address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_jpeg_addr {
    pub y: u32,
    pub cb: u32,
    pub cr: u32,
}
