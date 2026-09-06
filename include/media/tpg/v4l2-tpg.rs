//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/tpg/v4l2-tpg.h
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
// v4l2-tpg.h - Test Pattern Generator
//
// Copyright 2014 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpg_rbg_color8 {
    pub b: unsigned char r, g,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpg_rbg_color16 {
    pub b: __u16 r, g,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpg_color {
    TPG_COLOR_CSC_WHITE,
    TPG_COLOR_CSC_YELLOW,
    TPG_COLOR_CSC_CYAN,
    TPG_COLOR_CSC_GREEN,
    TPG_COLOR_CSC_MAGENTA,
    TPG_COLOR_CSC_RED,
    TPG_COLOR_CSC_BLUE,
    TPG_COLOR_CSC_BLACK,
    TPG_COLOR_75_YELLOW,
    TPG_COLOR_75_CYAN,
    TPG_COLOR_75_GREEN,
    TPG_COLOR_75_MAGENTA,
    TPG_COLOR_75_RED,
    TPG_COLOR_75_BLUE,
    TPG_COLOR_100_WHITE,
    TPG_COLOR_100_YELLOW,
    TPG_COLOR_100_CYAN,
    TPG_COLOR_100_GREEN,
    TPG_COLOR_100_MAGENTA,
    TPG_COLOR_100_RED,
    TPG_COLOR_100_BLUE,
    TPG_COLOR_100_BLACK,
    TPG_COLOR_TEXTFG,
    TPG_COLOR_TEXTBG,
    TPG_COLOR_RANDOM,
    TPG_COLOR_RAMP,
    TPG_COLOR_MAX = TPG_COLOR_RAMP + 256
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpg_pattern {
    TPG_PAT_75_COLORBAR,
    TPG_PAT_100_COLORBAR,
    TPG_PAT_CSC_COLORBAR,
    TPG_PAT_100_HCOLORBAR,
    TPG_PAT_100_COLORSQUARES,
    TPG_PAT_BLACK,
    TPG_PAT_WHITE,
    TPG_PAT_RED,
    TPG_PAT_GREEN,
    TPG_PAT_BLUE,
    TPG_PAT_CHECKERS_16X16,
    TPG_PAT_CHECKERS_2X2,
    TPG_PAT_CHECKERS_1X1,
    TPG_PAT_COLOR_CHECKERS_2X2,
    TPG_PAT_COLOR_CHECKERS_1X1,
    TPG_PAT_ALTERNATING_HLINES,
    TPG_PAT_ALTERNATING_VLINES,
    TPG_PAT_CROSS_1_PIXEL,
    TPG_PAT_CROSS_2_PIXELS,
    TPG_PAT_CROSS_10_PIXELS,
    TPG_PAT_GRAY_RAMP,

// Must be the last pattern
    TPG_PAT_NOISE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpg_quality {
    TPG_QUAL_COLOR,
    TPG_QUAL_GRAY,
    TPG_QUAL_NOISE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpg_video_aspect {
    TPG_VIDEO_ASPECT_IMAGE,
    TPG_VIDEO_ASPECT_4X3,
    TPG_VIDEO_ASPECT_14X9_CENTRE,
    TPG_VIDEO_ASPECT_16X9_CENTRE,
    TPG_VIDEO_ASPECT_16X9_ANAMORPHIC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpg_pixel_aspect {
    TPG_PIXEL_ASPECT_SQUARE,
    TPG_PIXEL_ASPECT_NTSC,
    TPG_PIXEL_ASPECT_PAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpg_move_mode {
    TPG_MOVE_NEG_FAST,
    TPG_MOVE_NEG,
    TPG_MOVE_NEG_SLOW,
    TPG_MOVE_NONE,
    TPG_MOVE_POS_SLOW,
    TPG_MOVE_POS,
    TPG_MOVE_POS_FAST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tgp_color_enc {
    TGP_COLOR_ENC_RGB,
    TGP_COLOR_ENC_YCBCR,
    TGP_COLOR_ENC_HSV,
    TGP_COLOR_ENC_LUMA,
}

pub const TPG_MAX_PLANES: c_int = 3;
pub const TPG_MAX_PAT_LINES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpg_data {
// Source frame size
    pub src_height: unsigned src_width,,
// Buffer height
    pub buf_height: unsigned,
// Scaled output frame size
    pub scaled_width: unsigned,
    pub field: u32,
    pub field_alternate: bool,
// crop coordinates are frame-based
    pub crop: v4l2_rect,
// compose coordinates are format-based
    pub compose: v4l2_rect,
// border and square coordinates are frame-based
    pub border: v4l2_rect,
    pub square: v4l2_rect,
// Color-related fields
    pub qual: tpg_quality,
    pub qual_offset: unsigned,
    pub alpha_component: u8,
    pub alpha_red_only: bool,
    pub brightness: u8,
    pub contrast: u8,
    pub saturation: u8,
    pub hue: i16,
    pub fourcc: u32,
    pub color_enc: tgp_color_enc,
    pub colorspace: u32,
    pub xfer_func: u32,
    pub ycbcr_enc: u32,
    pub hsv_enc: u32,
//
// Stores the actual transfer function, i.e. will never be
// V4L2_XFER_FUNC_DEFAULT.
//
    pub real_xfer_func: u32,
//
// Stores the actual Y'CbCr encoding, i.e. will never be
// V4L2_YCBCR_ENC_DEFAULT.
//
    pub real_hsv_enc: u32,
    pub real_ycbcr_enc: u32,
    pub quantization: u32,
//
// Stores the actual quantization, i.e. will never be
// V4L2_QUANTIZATION_DEFAULT.
//
    pub real_quantization: u32,
    pub vid_aspect: tpg_video_aspect,
    pub pix_aspect: tpg_pixel_aspect,
    pub rgb_range: unsigned,
    pub real_rgb_range: unsigned,
    pub buffers: unsigned,
    pub planes: unsigned,
    pub interleaved: bool,
    pub vdownsampling: [u8; TPG_MAX_PLANES],
    pub hdownsampling: [u8; TPG_MAX_PLANES],
//
// horizontal positions must be ANDed with this value to enforce
// correct boundaries for packed YUYV values.
//
    pub hmask: [unsigned; TPG_MAX_PLANES],
// Used to store the colors in native format, either RGB or YUV
    pub colors: [u8; TPG_COLOR_MAX][3],
    pub textbg: [u8 textfg[TPG_MAX_PLANES][8],; TPG_MAX_PLANES][8],
// size in bytes for two pixels in each plane
    pub twopixelsize: [unsigned; TPG_MAX_PLANES],
    pub bytesperline: [unsigned; TPG_MAX_PLANES],
// Configuration
    pub pattern: tpg_pattern,
    pub hflip: bool,
    pub vflip: bool,
    pub perc_fill: unsigned,
    pub perc_fill_blank: bool,
    pub show_border: bool,
    pub show_square: bool,
    pub insert_sav: bool,
    pub insert_eav: bool,
    pub insert_hdmi_video_guard_band: bool,
// Test pattern movement
    pub mv_hor_mode: tpg_move_mode,
    pub mv_hor_count: c_int,
    pub mv_hor_step: c_int,
    pub mv_vert_mode: tpg_move_mode,
    pub mv_vert_count: c_int,
    pub mv_vert_step: c_int,
    pub recalc_colors: bool,
    pub recalc_lines: bool,
    pub recalc_square_border: bool,
// Used to store TPG_MAX_PAT_LINES lines, each with up to two planes
    pub max_line_width: unsigned,
    pub lines: [*mut u8; TPG_MAX_PAT_LINES][TPG_MAX_PLANES],
    pub downsampled_lines: [*mut u8; TPG_MAX_PAT_LINES][TPG_MAX_PLANES],
    pub random_line: [*mut u8; TPG_MAX_PLANES],
    pub contrast_line: [*mut u8; TPG_MAX_PLANES],
    pub black_line: [*mut u8; TPG_MAX_PLANES],
}

extern "C" {
    pub fn tpg_init(tpg: *mut tpg_data, w: unsigned, h: unsigned);
}
extern "C" {
    pub fn tpg_alloc(tpg: *mut tpg_data, max_w: unsigned) -> c_int;
}
extern "C" {
    pub fn tpg_free(tpg: *mut tpg_data);
}
extern "C" {
    pub fn tpg_log_status(tpg: *mut tpg_data);
}
extern "C" {
    pub fn tpg_set_font(f: *const u8);
}
extern "C" {
    pub fn tpg_g_interleaved_plane(tpg: *const tpg_data, buf_line: unsigned) -> unsigned;
}
extern "C" {
    pub fn tpg_s_fourcc(tpg: *mut tpg_data, fourcc: u32) -> bool;
}
extern "C" {
    pub fn tpg_hdiv(_arg: tpg, _arg: plane, _arg: tpg_hscale(tpg, _arg: x)) -> return;
}
extern "C" {
    pub fn tpg_g_bytesperline(_arg: tpg, _arg: plane) -> return;
}
//
// This inserts 4 pixels of the RGB color 0xab55ab at the left hand side of the
// image. This is only done for 3 or 4 byte RGB pixel formats. This pixel value
// equals the Video Guard Band value as defined by HDMI (see section 5.2.2.1
// in the HDMI 1.3 Specification) that preceeds the first actual pixel. If the
// HDMI receiver doesn't handle this correctly, then it might keep skipping
// these Video Guard Band patterns and end up with a shorter video line. So this
// is a nice pattern to test with.
//
extern "C" {
    pub fn tpg_update_mv_step(tpg: *mut tpg_data);
}
