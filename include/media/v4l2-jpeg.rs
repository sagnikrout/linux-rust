//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-jpeg.h
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
// V4L2 JPEG helpers header
//
// Copyright (C) 2019 Pengutronix, Philipp Zabel <kernel@pengutronix.de>
//
// For reference, see JPEG ITU-T.81 (ISO/IEC 10918-1)
//

pub const V4L2_JPEG_MAX_COMPONENTS: c_int = 4;
pub const V4L2_JPEG_MAX_TABLES: c_int = 4;
//
// Prefixes used to generate huffman table class and destination identifiers as
// described below:
//
// V4L2_JPEG_LUM_HT | V4L2_JPEG_DC_HT : Prefix for Luma DC coefficients
// huffman table
// V4L2_JPEG_LUM_HT | V4L2_JPEG_AC_HT : Prefix for Luma AC coefficients
// huffman table
// V4L2_JPEG_CHR_HT | V4L2_JPEG_DC_HT : Prefix for Chroma DC coefficients
// huffman table
// V4L2_JPEG_CHR_HT | V4L2_JPEG_AC_HT : Prefix for Chroma AC coefficients
// huffman table
//
pub const V4L2_JPEG_LUM_HT: c_uint = 0x00;
pub const V4L2_JPEG_CHR_HT: c_uint = 0x01;
pub const V4L2_JPEG_DC_HT: c_uint = 0x00;
pub const V4L2_JPEG_AC_HT: c_uint = 0x10;
// Length of reference huffman tables as provided in Table K.3 of ITU-T.81
pub const V4L2_JPEG_REF_HT_AC_LEN: c_int = 178;
pub const V4L2_JPEG_REF_HT_DC_LEN: c_int = 28;
// Array size for 8x8 block of samples or DCT coefficient
pub const V4L2_JPEG_PIXELS_IN_BLOCK: c_int = 64;
//
// struct v4l2_jpeg_reference - reference into the JPEG buffer
// @start: pointer to the start of the referenced segment or table
// @length: size of the referenced segment or table
//
// Wnen referencing marker segments, start points right after the marker code,
// and length is the size of the segment parameters, excluding the marker code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_jpeg_reference {
    pub start: *mut u8,
    pub length: usize,
}

// B.2.2 Frame header syntax
//
// struct v4l2_jpeg_frame_component_spec - frame component-specification
// @component_identifier: C[i]
// @horizontal_sampling_factor: H[i]
// @vertical_sampling_factor: V[i]
// @quantization_table_selector: quantization table destination selector Tq[i]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_jpeg_frame_component_spec {
    pub component_identifier: u8,
    pub horizontal_sampling_factor: u8,
    pub vertical_sampling_factor: u8,
    pub quantization_table_selector: u8,
}

//
// struct v4l2_jpeg_frame_header - JPEG frame header
// @height: Y
// @width: X
// @precision: P
// @num_components: Nf
// @component: component-specification, see v4l2_jpeg_frame_component_spec
// @subsampling: decoded subsampling from component-specification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_jpeg_frame_header {
    pub height: u16,
    pub width: u16,
    pub precision: u8,
    pub num_components: u8,
    pub component: [v4l2_jpeg_frame_component_spec; V4L2_JPEG_MAX_COMPONENTS],
    pub subsampling: v4l2_jpeg_chroma_subsampling,
}

// B.2.3 Scan header syntax
//
// struct v4l2_jpeg_scan_component_spec - scan component-specification
// @component_selector: Cs[j]
// @dc_entropy_coding_table_selector: Td[j]
// @ac_entropy_coding_table_selector: Ta[j]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_jpeg_scan_component_spec {
    pub component_selector: u8,
    pub dc_entropy_coding_table_selector: u8,
    pub ac_entropy_coding_table_selector: u8,
}

//
// struct v4l2_jpeg_scan_header - JPEG scan header
// @num_components: Ns
// @component: component-specification, see v4l2_jpeg_scan_component_spec
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_jpeg_scan_header {
    pub /: *mut *mut u8 num_components; / Ns,
    pub component: [v4l2_jpeg_scan_component_spec; V4L2_JPEG_MAX_COMPONENTS],
// Ss, Se, Ah, and Al are not used by any driver
}

//
// enum v4l2_jpeg_app14_tf - APP14 transform flag
// According to Rec. ITU-T T.872 (06/2012) 6.5.3
// APP14 segment is for color encoding, it contains a transform flag,
// which may have values of 0, 1 and 2 and are interpreted as follows:
// @V4L2_JPEG_APP14_TF_CMYK_RGB: CMYK for images encoded with four components
// RGB for images encoded with three components
// @V4L2_JPEG_APP14_TF_YCBCR: an image encoded with three components using YCbCr
// @V4L2_JPEG_APP14_TF_YCCK: an image encoded with four components using YCCK
// @V4L2_JPEG_APP14_TF_UNKNOWN: indicate app14 is not present
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_jpeg_app14_tf {
    V4L2_JPEG_APP14_TF_CMYK_RGB	= 0,
    V4L2_JPEG_APP14_TF_YCBCR	= 1,
    V4L2_JPEG_APP14_TF_YCCK		= 2,
    V4L2_JPEG_APP14_TF_UNKNOWN	= -1,
}

//
// struct v4l2_jpeg_header - parsed JPEG header
// @sof: pointer to frame header and size
// @sos: pointer to scan header and size
// @num_dht: number of entries in @dht
// @dht: pointers to huffman tables and sizes
// @num_dqt: number of entries in @dqt
// @dqt: pointers to quantization tables and sizes
// @frame: parsed frame header
// @scan: pointer to parsed scan header, optional
// @quantization_tables: references to four quantization tables, optional
// @huffman_tables: references to four Huffman tables in DC0, DC1, AC0, AC1
// order, optional
// @restart_interval: number of MCU per restart interval, Ri
// @ecs_offset: buffer offset in bytes to the entropy coded segment
// @app14_tf: transform flag from app14 data
//
// When this structure is passed to v4l2_jpeg_parse_header, the optional scan,
// quantization_tables, and huffman_tables pointers must be initialized to NULL
// or point at valid memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_jpeg_header {
    pub sof: v4l2_jpeg_reference,
    pub sos: v4l2_jpeg_reference,
    pub num_dht: c_uint,
    pub dht: [v4l2_jpeg_reference; V4L2_JPEG_MAX_TABLES],
    pub num_dqt: c_uint,
    pub dqt: [v4l2_jpeg_reference; V4L2_JPEG_MAX_TABLES],
    pub frame: v4l2_jpeg_frame_header,
    pub scan: *mut v4l2_jpeg_scan_header,
    pub quantization_tables: *mut v4l2_jpeg_reference,
    pub huffman_tables: *mut v4l2_jpeg_reference,
    pub restart_interval: u16,
    pub ecs_offset: usize,
    pub app14_tf: v4l2_jpeg_app14_tf,
}

extern "C" {
    pub fn v4l2_jpeg_parse_header(buf: *mut c_void, len: usize, out: *mut v4l2_jpeg_header) -> c_int;
}
