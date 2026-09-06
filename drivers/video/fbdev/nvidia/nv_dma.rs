//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/nvidia/nv_dma.h
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


// \
//
// GPL Licensing Note - According to Mark Vojkovich, author of the Xorg
// XFree86 'nv' driver, this source code is provided under MIT-style licensing
// where the source code is provided "as is" without warranty of any kind.
// The only usage restriction is for the copyright notices to be retained
// whenever code is used.
//
// Antonino Daplas <adaplas@pol.net> 2005-03-11
//
pub const SURFACE_FORMAT: c_uint = 0x00000300;
pub const SURFACE_FORMAT_DEPTH8: c_uint = 0x00000001;
pub const SURFACE_FORMAT_DEPTH15: c_uint = 0x00000002;
pub const SURFACE_FORMAT_DEPTH16: c_uint = 0x00000004;
pub const SURFACE_FORMAT_DEPTH24: c_uint = 0x00000006;
pub const SURFACE_PITCH: c_uint = 0x00000304;

pub const SURFACE_OFFSET_SRC: c_uint = 0x00000308;
pub const SURFACE_OFFSET_DST: c_uint = 0x0000030C;
pub const ROP_SET: c_uint = 0x00002300;
pub const PATTERN_FORMAT: c_uint = 0x00004300;
pub const PATTERN_FORMAT_DEPTH8: c_uint = 0x00000003;
pub const PATTERN_FORMAT_DEPTH16: c_uint = 0x00000001;
pub const PATTERN_FORMAT_DEPTH24: c_uint = 0x00000003;
pub const PATTERN_COLOR_0: c_uint = 0x00004310;
pub const PATTERN_COLOR_1: c_uint = 0x00004314;
pub const PATTERN_PATTERN_0: c_uint = 0x00004318;
pub const PATTERN_PATTERN_1: c_uint = 0x0000431C;
pub const CLIP_POINT: c_uint = 0x00006300;

pub const CLIP_SIZE: c_uint = 0x00006304;

pub const LINE_FORMAT: c_uint = 0x00008300;
pub const LINE_FORMAT_DEPTH8: c_uint = 0x00000003;
pub const LINE_FORMAT_DEPTH16: c_uint = 0x00000001;
pub const LINE_FORMAT_DEPTH24: c_uint = 0x00000003;
pub const LINE_COLOR: c_uint = 0x00008304;
pub const LINE_MAX_LINES: c_int = 16;
pub const LINE_LINES(i): c_uint = 0x00008400\;

pub const BLIT_POINT_SRC: c_uint = 0x0000A300;

pub const BLIT_POINT_DST: c_uint = 0x0000A304;

pub const BLIT_SIZE: c_uint = 0x0000A308;

pub const RECT_FORMAT: c_uint = 0x0000C300;
pub const RECT_FORMAT_DEPTH8: c_uint = 0x00000003;
pub const RECT_FORMAT_DEPTH16: c_uint = 0x00000001;
pub const RECT_FORMAT_DEPTH24: c_uint = 0x00000003;
pub const RECT_SOLID_COLOR: c_uint = 0x0000C3FC;
pub const RECT_SOLID_RECTS_MAX_RECTS: c_int = 32;
pub const RECT_SOLID_RECTS(i): c_uint = 0x0000C400\;

pub const RECT_EXPAND_ONE_COLOR_CLIP: c_uint = 0x0000C7EC;

pub const RECT_EXPAND_ONE_COLOR_COLOR: c_uint = 0x0000C7F4;
pub const RECT_EXPAND_ONE_COLOR_SIZE: c_uint = 0x0000C7F8;

pub const RECT_EXPAND_ONE_COLOR_POINT: c_uint = 0x0000C7FC;

pub const RECT_EXPAND_ONE_COLOR_DATA_MAX_DWORDS: c_int = 128;
pub const RECT_EXPAND_ONE_COLOR_DATA(i): c_uint = 0x0000C800\;
pub const RECT_EXPAND_TWO_COLOR_CLIP: c_uint = 0x0000CBE4;

pub const RECT_EXPAND_TWO_COLOR_COLOR_0: c_uint = 0x0000CBEC;
pub const RECT_EXPAND_TWO_COLOR_COLOR_1: c_uint = 0x0000CBF0;
pub const RECT_EXPAND_TWO_COLOR_SIZE_IN: c_uint = 0x0000CBF4;

pub const RECT_EXPAND_TWO_COLOR_SIZE_OUT: c_uint = 0x0000CBF8;

pub const RECT_EXPAND_TWO_COLOR_POINT: c_uint = 0x0000CBFC;

pub const RECT_EXPAND_TWO_COLOR_DATA_MAX_DWORDS: c_int = 128;
pub const RECT_EXPAND_TWO_COLOR_DATA(i): c_uint = 0x0000CC00\;
pub const STRETCH_BLIT_FORMAT: c_uint = 0x0000E300;
pub const STRETCH_BLIT_FORMAT_DEPTH8: c_uint = 0x00000004;
pub const STRETCH_BLIT_FORMAT_DEPTH16: c_uint = 0x00000007;
pub const STRETCH_BLIT_FORMAT_DEPTH24: c_uint = 0x00000004;
pub const STRETCH_BLIT_FORMAT_X8R8G8B8: c_uint = 0x00000004;
pub const STRETCH_BLIT_FORMAT_YUYV: c_uint = 0x00000005;
pub const STRETCH_BLIT_FORMAT_UYVY: c_uint = 0x00000006;
pub const STRETCH_BLIT_CLIP_POINT: c_uint = 0x0000E308;

pub const STRETCH_BLIT_CLIP_POINT: c_uint = 0x0000E308;
pub const STRETCH_BLIT_CLIP_SIZE: c_uint = 0x0000E30C;

pub const STRETCH_BLIT_DST_POINT: c_uint = 0x0000E310;

pub const STRETCH_BLIT_DST_SIZE: c_uint = 0x0000E314;

pub const STRETCH_BLIT_DU_DX: c_uint = 0x0000E318;
pub const STRETCH_BLIT_DV_DY: c_uint = 0x0000E31C;
pub const STRETCH_BLIT_SRC_SIZE: c_uint = 0x0000E400;

pub const STRETCH_BLIT_SRC_FORMAT: c_uint = 0x0000E404;

pub const STRETCH_BLIT_SRC_FORMAT_ORIGIN_CENTER: c_uint = 0x00000001;
pub const STRETCH_BLIT_SRC_FORMAT_ORIGIN_CORNER: c_uint = 0x00000002;

pub const STRETCH_BLIT_SRC_FORMAT_FILTER_POINT_SAMPLE: c_uint = 0x00000000;
pub const STRETCH_BLIT_SRC_FORMAT_FILTER_BILINEAR: c_uint = 0x00000001;
pub const STRETCH_BLIT_SRC_OFFSET: c_uint = 0x0000E408;
pub const STRETCH_BLIT_SRC_POINT: c_uint = 0x0000E40C;

