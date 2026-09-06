//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/qxl/qxl_dev.h
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


//
// Redistributions of source code must retain the above copyright
// Redistributions in binary form must reproduce the above copyright
// Neither the name of the copyright holder nor the names of its
//

// Macro flag: #define H_QXL_DEV

//
// from spice-protocol
// Release 0.10.0
//
// enums.h
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SpiceImageType {
    SPICE_IMAGE_TYPE_BITMAP,
    SPICE_IMAGE_TYPE_QUIC,
    SPICE_IMAGE_TYPE_RESERVED,
    SPICE_IMAGE_TYPE_LZ_PLT = 100,
    SPICE_IMAGE_TYPE_LZ_RGB,
    SPICE_IMAGE_TYPE_GLZ_RGB,
    SPICE_IMAGE_TYPE_FROM_CACHE,
    SPICE_IMAGE_TYPE_SURFACE,
    SPICE_IMAGE_TYPE_JPEG,
    SPICE_IMAGE_TYPE_FROM_CACHE_LOSSLESS,
    SPICE_IMAGE_TYPE_ZLIB_GLZ_RGB,
    SPICE_IMAGE_TYPE_JPEG_ALPHA,

    SPICE_IMAGE_TYPE_ENUM_END
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SpiceBitmapFmt {
    SPICE_BITMAP_FMT_INVALID,
    SPICE_BITMAP_FMT_1BIT_LE,
    SPICE_BITMAP_FMT_1BIT_BE,
    SPICE_BITMAP_FMT_4BIT_LE,
    SPICE_BITMAP_FMT_4BIT_BE,
    SPICE_BITMAP_FMT_8BIT,
    SPICE_BITMAP_FMT_16BIT,
    SPICE_BITMAP_FMT_24BIT,
    SPICE_BITMAP_FMT_32BIT,
    SPICE_BITMAP_FMT_RGBA,

    SPICE_BITMAP_FMT_ENUM_END
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SpiceSurfaceFmt {
    SPICE_SURFACE_FMT_INVALID,
    SPICE_SURFACE_FMT_1_A,
    SPICE_SURFACE_FMT_8_A = 8,
    SPICE_SURFACE_FMT_16_555 = 16,
    SPICE_SURFACE_FMT_32_xRGB = 32,
    SPICE_SURFACE_FMT_16_565 = 80,
    SPICE_SURFACE_FMT_32_ARGB = 96,

    SPICE_SURFACE_FMT_ENUM_END
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SpiceClipType {
    SPICE_CLIP_TYPE_NONE,
    SPICE_CLIP_TYPE_RECTS,

    SPICE_CLIP_TYPE_ENUM_END
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SpiceRopd {
    SPICE_ROPD_INVERS_SRC = (1 << 0),
    SPICE_ROPD_INVERS_BRUSH = (1 << 1),
    SPICE_ROPD_INVERS_DEST = (1 << 2),
    SPICE_ROPD_OP_PUT = (1 << 3),
    SPICE_ROPD_OP_OR = (1 << 4),
    SPICE_ROPD_OP_AND = (1 << 5),
    SPICE_ROPD_OP_XOR = (1 << 6),
    SPICE_ROPD_OP_BLACKNESS = (1 << 7),
    SPICE_ROPD_OP_WHITENESS = (1 << 8),
    SPICE_ROPD_OP_INVERS = (1 << 9),
    SPICE_ROPD_INVERS_RES = (1 << 10),

    SPICE_ROPD_MASK = 0x7ff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SpiceBrushType {
    SPICE_BRUSH_TYPE_NONE,
    SPICE_BRUSH_TYPE_SOLID,
    SPICE_BRUSH_TYPE_PATTERN,

    SPICE_BRUSH_TYPE_ENUM_END
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SpiceCursorType {
    SPICE_CURSOR_TYPE_ALPHA,
    SPICE_CURSOR_TYPE_MONO,
    SPICE_CURSOR_TYPE_COLOR4,
    SPICE_CURSOR_TYPE_COLOR8,
    SPICE_CURSOR_TYPE_COLOR16,
    SPICE_CURSOR_TYPE_COLOR24,
    SPICE_CURSOR_TYPE_COLOR32,

    SPICE_CURSOR_TYPE_ENUM_END
}

// qxl_dev.h

// 0x100-0x11f reserved for spice, 0x1ff used for unstable work
pub const QXL_DEVICE_ID_STABLE: c_uint = 0x0100;
pub const QXL_DEVICE_ID_DEVEL: c_uint = 0x01ff;
pub const QXL_REVISION_DEVEL: c_uint = 0x01;

// qxl-1 compat: append only
// appended for qxl-2
// appended for qxl-3
// appended for qxl-4
pub type QXLPHYSICAL = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_point_fix {
    pub x: QXLFIXED,
    pub y: QXLFIXED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_point_1_6 {
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_rect {
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_urect {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
}

// qxl-1 compat: append only
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_rom {
    pub magic: u32,
    pub id: u32,
    pub update_id: u32,
    pub compression_level: u32,
    pub log_level: u32,
    pub /: *mut *mut uint32_t mode; / qxl-1,
    pub modes_offset: u32,
    pub num_io_pages: u32,
    pub /: *mut *mut uint32_t pages_offset; / qxl-1,
    pub /: *mut *mut uint32_t draw_area_offset; / qxl-1,
    pub /: *mut *mut uint32_t surface0_area_size; / qxl-1 name: draw_area_size,
    pub ram_header_offset: u32,
    pub mm_clock: u32,
// appended for qxl-2
    pub n_surfaces: u32,
    pub flags: u64,
    pub slots_start: u8,
    pub slots_end: u8,
    pub slot_gen_bits: u8,
    pub slot_id_bits: u8,
    pub slot_generation: u8,
// appended for qxl-4
    pub client_present: u8,
    pub client_capabilities: [u8; 58],
    pub client_monitors_config_crc: u32,
    pub count: u16,
    pub padding: u16,
    pub heads: [qxl_urect; 64],
    pub client_monitors_config: },
}

// qxl-1 compat: fixed
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_mode {
    pub id: u32,
    pub x_res: u32,
    pub y_res: u32,
    pub bits: u32,
    pub stride: u32,
    pub x_mili: u32,
    pub y_mili: u32,
    pub orientation: u32,
}

// qxl-1 compat: fixed
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_modes {
    pub n_modes: u32,
    pub modes: [qxl_mode; ],
}

// qxl-1 compat: append only
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qxl_cmd_type {
    QXL_CMD_NOP,
    QXL_CMD_DRAW,
    QXL_CMD_UPDATE,
    QXL_CMD_CURSOR,
    QXL_CMD_MESSAGE,
    QXL_CMD_SURFACE,
}

// qxl-1 compat: fixed
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_command {
    pub data: QXLPHYSICAL,
    pub type: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_command_ext {
    pub cmd: qxl_command,
    pub group_id: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_mem_slot {
    pub mem_start: u64,
    pub mem_end: u64,
}

pub const QXL_SURF_TYPE_PRIMARY: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_surface_create {
    pub width: u32,
    pub height: u32,
    pub stride: i32,
    pub format: u32,
    pub position: u32,
    pub mouse_mode: u32,
    pub flags: u32,
    pub type: u32,
    pub mem: QXLPHYSICAL,
}

pub const QXL_COMMAND_RING_SIZE: c_int = 32;
pub const QXL_CURSOR_RING_SIZE: c_int = 32;
pub const QXL_RELEASE_RING_SIZE: c_int = 8;
pub const QXL_LOG_BUF_SIZE: c_int = 4096;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_ring_header {
    pub num_items: u32,
    pub prod: u32,
    pub notify_on_prod: u32,
    pub cons: u32,
    pub notify_on_cons: u32,
}

// qxl-1 compat: append only
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_ram_header {
    pub magic: u32,
    pub int_pending: u32,
    pub int_mask: u32,
    pub log_buf: [u8; QXL_LOG_BUF_SIZE],
    pub cmd_ring_hdr: qxl_ring_header,
    pub cmd_ring: [qxl_command; QXL_COMMAND_RING_SIZE],
    pub cursor_ring_hdr: qxl_ring_header,
    pub cursor_ring: [qxl_command; QXL_CURSOR_RING_SIZE],
    pub release_ring_hdr: qxl_ring_header,
    pub release_ring: [u64; QXL_RELEASE_RING_SIZE],
    pub update_area: qxl_rect,
// appended for qxl-2
    pub update_surface: u32,
    pub mem_slot: qxl_mem_slot,
    pub create_surface: qxl_surface_create,
    pub flags: u64,
// appended for qxl-4
// used by QXL_IO_MONITORS_CONFIG_ASYNC
    pub monitors_config: QXLPHYSICAL,
    pub guest_capabilities: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union qxl_release_info {
    pub /: *mut *mut uint64_t id; / in,
    pub /: *mut *mut uint64_t next; / out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_release_info_ext {
    pub info: *mut qxl_release_info,
    pub group_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_data_chunk {
    pub data_size: u32,
    pub prev_chunk: QXLPHYSICAL,
    pub next_chunk: QXLPHYSICAL,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_message {
    pub release_info: qxl_release_info,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_compat_update_cmd {
    pub release_info: qxl_release_info,
    pub area: qxl_rect,
    pub update_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_update_cmd {
    pub release_info: qxl_release_info,
    pub area: qxl_rect,
    pub update_id: u32,
    pub surface_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_cursor_header {
    pub unique: u64,
    pub type: u16,
    pub width: u16,
    pub height: u16,
    pub hot_spot_x: u16,
    pub hot_spot_y: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_cursor {
    pub header: qxl_cursor_header,
    pub data_size: u32,
    pub chunk: qxl_data_chunk,
}

pub const QXL_CURSOR_DEVICE_DATA_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_cursor_cmd {
    pub release_info: qxl_release_info,
    pub type: u8,
    pub position: qxl_point_1_6,
    pub visible: u8,
    pub shape: QXLPHYSICAL,
    pub set: },
    pub length: u16,
    pub frequency: u16,
    pub trail: },
    pub position: qxl_point_1_6,
    pub u: },
// todo: dynamic size from rom
    pub device_data: [u8; QXL_CURSOR_DEVICE_DATA_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_raster_glyph {
    pub render_pos: qxl_point,
    pub glyph_origin: qxl_point,
    pub width: u16,
    pub height: u16,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_string {
    pub data_size: u32,
    pub length: u16,
    pub flags: u16,
    pub chunk: qxl_data_chunk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_copy_bits {
    pub src_pos: qxl_point,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qxl_effect_type {
    QXL_EFFECT_BLEND = 0,
    QXL_EFFECT_OPAQUE = 1,
    QXL_EFFECT_REVERT_ON_DUP = 2,
    QXL_EFFECT_BLACKNESS_ON_DUP = 3,
    QXL_EFFECT_WHITENESS_ON_DUP = 4,
    QXL_EFFECT_NOP_ON_DUP = 5,
    QXL_EFFECT_NOP = 6,
    QXL_EFFECT_OPAQUE_BRUSH = 7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_pattern {
    pub pat: QXLPHYSICAL,
    pub pos: qxl_point,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_brush {
    pub type: u32,
    pub color: u32,
    pub pattern: qxl_pattern,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_q_mask {
    pub flags: u8,
    pub pos: qxl_point,
    pub bitmap: QXLPHYSICAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_fill {
    pub brush: qxl_brush,
    pub rop_descriptor: u16,
    pub mask: qxl_q_mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_opaque {
    pub src_bitmap: QXLPHYSICAL,
    pub src_area: qxl_rect,
    pub brush: qxl_brush,
    pub rop_descriptor: u16,
    pub scale_mode: u8,
    pub mask: qxl_q_mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_copy {
    pub src_bitmap: QXLPHYSICAL,
    pub src_area: qxl_rect,
    pub rop_descriptor: u16,
    pub scale_mode: u8,
    pub mask: qxl_q_mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_transparent {
    pub src_bitmap: QXLPHYSICAL,
    pub src_area: qxl_rect,
    pub src_color: u32,
    pub true_color: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_alpha_blend {
    pub alpha_flags: u16,
    pub alpha: u8,
    pub src_bitmap: QXLPHYSICAL,
    pub src_area: qxl_rect,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_compat_alpha_blend {
    pub alpha: u8,
    pub src_bitmap: QXLPHYSICAL,
    pub src_area: qxl_rect,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_rop_3 {
    pub src_bitmap: QXLPHYSICAL,
    pub src_area: qxl_rect,
    pub brush: qxl_brush,
    pub rop3: u8,
    pub scale_mode: u8,
    pub mask: qxl_q_mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_line_attr {
    pub flags: u8,
    pub join_style: u8,
    pub end_style: u8,
    pub style_nseg: u8,
    pub width: QXLFIXED,
    pub miter_limit: QXLFIXED,
    pub style: QXLPHYSICAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_stroke {
    pub path: QXLPHYSICAL,
    pub attr: qxl_line_attr,
    pub brush: qxl_brush,
    pub fore_mode: u16,
    pub back_mode: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_text {
    pub str: QXLPHYSICAL,
    pub back_area: qxl_rect,
    pub fore_brush: qxl_brush,
    pub back_brush: qxl_brush,
    pub fore_mode: u16,
    pub back_mode: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_mask {
    pub mask: qxl_q_mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_clip {
    pub type: u32,
    pub data: QXLPHYSICAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qxl_operator {
    QXL_OP_CLEAR			 = 0x00,
    QXL_OP_SOURCE			 = 0x01,
    QXL_OP_DST			 = 0x02,
    QXL_OP_OVER			 = 0x03,
    QXL_OP_OVER_REVERSE		 = 0x04,
    QXL_OP_IN			 = 0x05,
    QXL_OP_IN_REVERSE		 = 0x06,
    QXL_OP_OUT			 = 0x07,
    QXL_OP_OUT_REVERSE		 = 0x08,
    QXL_OP_ATOP			 = 0x09,
    QXL_OP_ATOP_REVERSE		 = 0x0a,
    QXL_OP_XOR			 = 0x0b,
    QXL_OP_ADD			 = 0x0c,
    QXL_OP_SATURATE			 = 0x0d,
// Note the jump here from 0x0d to 0x30
    QXL_OP_MULTIPLY			 = 0x30,
    QXL_OP_SCREEN			 = 0x31,
    QXL_OP_OVERLAY			 = 0x32,
    QXL_OP_DARKEN			 = 0x33,
    QXL_OP_LIGHTEN			 = 0x34,
    QXL_OP_COLOR_DODGE		 = 0x35,
    QXL_OP_COLOR_BURN		 = 0x36,
    QXL_OP_HARD_LIGHT		 = 0x37,
    QXL_OP_SOFT_LIGHT		 = 0x38,
    QXL_OP_DIFFERENCE		 = 0x39,
    QXL_OP_EXCLUSION		 = 0x3a,
    QXL_OP_HSL_HUE			 = 0x3b,
    QXL_OP_HSL_SATURATION		 = 0x3c,
    QXL_OP_HSL_COLOR		 = 0x3d,
    QXL_OP_HSL_LUMINOSITY		 = 0x3e
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_transform {
    pub t00: u32,
    pub t01: u32,
    pub t02: u32,
    pub t10: u32,
    pub t11: u32,
    pub t12: u32,
}

// The flags field has the following bit fields:
//
// operator:		[  0 -  7 ]
// src_filter:		[  8 - 10 ]
// mask_filter:		[ 11 - 13 ]
// src_repeat:		[ 14 - 15 ]
// mask_repeat:		[ 16 - 17 ]
// component_alpha:		[ 18 - 18 ]
// reserved:		[ 19 - 31 ]
//
// The repeat and filter values are those of pixman:
// REPEAT_NONE =		0
// REPEAT_NORMAL =		1
// REPEAT_PAD =		2
// REPEAT_REFLECT =	3
//
// The filter values are:
// FILTER_NEAREST =	0
// FILTER_BILINEAR	=	1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_composite {
    pub flags: u32,
    pub src: QXLPHYSICAL,
    pub /: *mut *mut QXLPHYSICAL src_transform; / May be NULL,
    pub /: *mut *mut QXLPHYSICAL mask; / May be NULL,
    pub /: *mut *mut QXLPHYSICAL mask_transform; / May be NULL,
    pub src_origin: qxl_point_1_6,
    pub mask_origin: qxl_point_1_6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_compat_drawable {
    pub release_info: qxl_release_info,
    pub effect: u8,
    pub type: u8,
    pub bitmap_offset: u16,
    pub bitmap_area: qxl_rect,
    pub bbox: qxl_rect,
    pub clip: qxl_clip,
    pub mm_time: u32,
    pub fill: qxl_fill,
    pub opaque: qxl_opaque,
    pub copy: qxl_copy,
    pub transparent: qxl_transparent,
    pub alpha_blend: qxl_compat_alpha_blend,
    pub copy_bits: qxl_copy_bits,
    pub blend: qxl_copy,
    pub rop3: qxl_rop_3,
    pub stroke: qxl_stroke,
    pub text: qxl_text,
    pub blackness: qxl_mask,
    pub invers: qxl_mask,
    pub whiteness: qxl_mask,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_drawable {
    pub release_info: qxl_release_info,
    pub surface_id: u32,
    pub effect: u8,
    pub type: u8,
    pub self_bitmap: u8,
    pub self_bitmap_area: qxl_rect,
    pub bbox: qxl_rect,
    pub clip: qxl_clip,
    pub mm_time: u32,
    pub surfaces_dest: [i32; 3],
    pub surfaces_rects: [qxl_rect; 3],
    pub fill: qxl_fill,
    pub opaque: qxl_opaque,
    pub copy: qxl_copy,
    pub transparent: qxl_transparent,
    pub alpha_blend: qxl_alpha_blend,
    pub copy_bits: qxl_copy_bits,
    pub blend: qxl_copy,
    pub rop3: qxl_rop_3,
    pub stroke: qxl_stroke,
    pub text: qxl_text,
    pub blackness: qxl_mask,
    pub invers: qxl_mask,
    pub whiteness: qxl_mask,
    pub composite: qxl_composite,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qxl_surface_cmd_type {
    QXL_SURFACE_CMD_CREATE,
    QXL_SURFACE_CMD_DESTROY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_surface {
    pub format: u32,
    pub width: u32,
    pub height: u32,
    pub stride: i32,
    pub data: QXLPHYSICAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_surface_cmd {
    pub release_info: qxl_release_info,
    pub surface_id: u32,
    pub type: u8,
    pub flags: u32,
    pub surface_create: qxl_surface,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_clip_rects {
    pub num_rects: u32,
    pub chunk: qxl_data_chunk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_path_seg {
    pub flags: u32,
    pub count: u32,
    pub points: [qxl_point_fix; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_path {
    pub data_size: u32,
    pub chunk: qxl_data_chunk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_image_id {
    pub group: u32,
    pub unique: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union qxl_image_id_union {
    pub id: qxl_image_id,
    pub value: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qxl_image_flags {
    QXL_IMAGE_CACHE = (1 << 0),
    QXL_IMAGE_HIGH_BITS_SET = (1 << 1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qxl_bitmap_flags {
    QXL_BITMAP_DIRECT = (1 << 0),
    QXL_BITMAP_UNSTABLE = (1 << 1),
    QXL_BITMAP_TOP_DOWN = (1 << 2), /* == SPICE_BITMAP_FLAGS_TOP_DOWN */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_image_descriptor {
    pub id: u64,
    pub type: u8,
    pub flags: u8,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_palette {
    pub unique: u64,
    pub num_ents: u16,
    pub ents: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_bitmap {
    pub format: u8,
    pub flags: u8,
    pub x: u32,
    pub y: u32,
    pub stride: u32,
    pub palette: QXLPHYSICAL,
    pub /: *mut *mut QXLPHYSICAL data; / data[0] ?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_surface_id {
    pub surface_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_encoder_data {
    pub data_size: u32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_image {
    pub descriptor: qxl_image_descriptor,
    pub bitmap: qxl_bitmap,
    pub quic: qxl_encoder_data,
    pub surface_image: qxl_surface_id,
    pub u: },
}

// A QXLHead is a single monitor output backed by a QXLSurface.
// x and y offsets are unsigned since they are used in relation to
// the given surface, not the same as the x, y coordinates in the guest
// screen reference frame.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_head {
    pub id: u32,
    pub surface_id: u32,
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qxl_monitors_config {
    pub count: u16,
    pub the: *mut *mut uint16_t max_allowed; / If it is 0 no fixed limit is given by,
    pub heads: [qxl_head; ],
}

