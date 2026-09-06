//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dc_hw_types.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

//
// Data types for Virtual HW Layer of DAL3.
// (see DAL3 design documents for HW Layer definition)
//
// The intended uses are:
// 1. Generation pseudocode sequences for HW programming.
// 2. Implementation of real HW programming by HW Sequencer of DAL3.
//
// Note: do *not* add any types which are *not* used for HW programming - this
// will ensure separation of Logic layer from HW layer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union large_integer {
    pub low_part: u32,
    pub high_part: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_plane_addr_type {
    PLN_ADDR_TYPE_GRAPHICS = 0,
    PLN_ADDR_TYPE_3DLUT,
    PLN_ADDR_TYPE_GRPH_STEREO,
    PLN_ADDR_TYPE_VIDEO_PROGRESSIVE,
    PLN_ADDR_TYPE_RGBEA
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_plane_address {
    pub type: dc_plane_addr_type,
    pub tmz_surface: u8,
    pub addr: PHYSICAL_ADDRESS_LOC,
    pub cursor_cache_addr: PHYSICAL_ADDRESS_LOC,
    pub meta_addr: PHYSICAL_ADDRESS_LOC,
    pub dcc_const_color: large_integer,
    pub grph: },
    pub addr: PHYSICAL_ADDRESS_LOC,
    pub lut3d: },
// stereo
    pub left_addr: PHYSICAL_ADDRESS_LOC,
    pub left_meta_addr: PHYSICAL_ADDRESS_LOC,
    pub left_dcc_const_color: large_integer,
    pub right_addr: PHYSICAL_ADDRESS_LOC,
    pub right_meta_addr: PHYSICAL_ADDRESS_LOC,
    pub right_dcc_const_color: large_integer,
    pub left_alpha_addr: PHYSICAL_ADDRESS_LOC,
    pub left_alpha_meta_addr: PHYSICAL_ADDRESS_LOC,
    pub left_alpha_dcc_const_color: large_integer,
    pub right_alpha_addr: PHYSICAL_ADDRESS_LOC,
    pub right_alpha_meta_addr: PHYSICAL_ADDRESS_LOC,
    pub right_alpha_dcc_const_color: large_integer,
    pub grph_stereo: },
// video  progressive
    pub luma_addr: PHYSICAL_ADDRESS_LOC,
    pub luma_meta_addr: PHYSICAL_ADDRESS_LOC,
    pub luma_dcc_const_color: large_integer,
    pub chroma_addr: PHYSICAL_ADDRESS_LOC,
    pub chroma_meta_addr: PHYSICAL_ADDRESS_LOC,
    pub chroma_dcc_const_color: large_integer,
    pub video_progressive: },
    pub addr: PHYSICAL_ADDRESS_LOC,
    pub meta_addr: PHYSICAL_ADDRESS_LOC,
    pub dcc_const_color: large_integer,
    pub alpha_addr: PHYSICAL_ADDRESS_LOC,
    pub alpha_meta_addr: PHYSICAL_ADDRESS_LOC,
    pub alpha_dcc_const_color: large_integer,
    pub rgbea: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_size {
    pub width: c_int,
    pub height: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rect {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plane_size {
// Graphic surface pitch in pixels.
// In LINEAR_GENERAL mode, pitch
// is 32 pixel aligned.
//
    pub surface_pitch: c_int,
    pub chroma_pitch: c_int,
    pub surface_size: rect,
    pub chroma_size: rect,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_plane_dcc_param {
    pub enable: bool,
    pub meta_pitch: c_int,
    pub independent_64b_blks: bool,
    pub dcc_ind_blk: u8,
    pub meta_pitch_c: c_int,
    pub independent_64b_blks_c: bool,
    pub dcc_ind_blk_c: u8,
}

// Displayable pixel format in fb
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum surface_pixel_format {
    SURFACE_PIXEL_FORMAT_GRPH_BEGIN = 0,
// TOBE REMOVED paletta 256 colors
    SURFACE_PIXEL_FORMAT_GRPH_PALETA_256_COLORS =
    SURFACE_PIXEL_FORMAT_GRPH_BEGIN,
// 16 bpp
    SURFACE_PIXEL_FORMAT_GRPH_ARGB1555,
// 16 bpp
    SURFACE_PIXEL_FORMAT_GRPH_RGB565,
// 32 bpp
    SURFACE_PIXEL_FORMAT_GRPH_ARGB8888,
// 32 bpp swaped
    SURFACE_PIXEL_FORMAT_GRPH_ABGR8888,

    SURFACE_PIXEL_FORMAT_GRPH_ARGB2101010,
// swaped
    SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010,
// TOBE REMOVED swaped, XR_BIAS has no differance
// for pixel layout than previous and we can
// delete this after discusion
    SURFACE_PIXEL_FORMAT_GRPH_ABGR2101010_XR_BIAS,
// 64 bpp
    SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616,
// swapped
    SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616,
// float
    SURFACE_PIXEL_FORMAT_GRPH_ARGB16161616F,
// swaped & float
    SURFACE_PIXEL_FORMAT_GRPH_ABGR16161616F,
// grow graphics here if necessary
    SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FIX,
    SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FIX,
    SURFACE_PIXEL_FORMAT_GRPH_RGB111110_FLOAT,
    SURFACE_PIXEL_FORMAT_GRPH_BGR101111_FLOAT,
    SURFACE_PIXEL_FORMAT_GRPH_RGBE,
    SURFACE_PIXEL_FORMAT_GRPH_RGBE_ALPHA,
    SURFACE_PIXEL_FORMAT_VIDEO_BEGIN,
    SURFACE_PIXEL_FORMAT_VIDEO_420_YCbCr =
    SURFACE_PIXEL_FORMAT_VIDEO_BEGIN,
    SURFACE_PIXEL_FORMAT_VIDEO_420_YCrCb,
    SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCbCr,
    SURFACE_PIXEL_FORMAT_VIDEO_420_10bpc_YCrCb,
// Planar 422 Formats
    SURFACE_PIXEL_FORMAT_VIDEO_422_CrCb_P208, // 8 bpc
    SURFACE_PIXEL_FORMAT_VIDEO_422_CbCr_P208, // 8 bpc
    SURFACE_PIXEL_FORMAT_VIDEO_422_CrCb_P210, // 10 bpc
    SURFACE_PIXEL_FORMAT_VIDEO_422_CbCr_P210, // 10 bpc
    SURFACE_PIXEL_FORMAT_VIDEO_422_CrCb_P212, // 12 bpc
    SURFACE_PIXEL_FORMAT_VIDEO_422_CbCr_P212, // 12 bpc
    SURFACE_PIXEL_FORMAT_SUBSAMPLE_END,
    SURFACE_PIXEL_FORMAT_VIDEO_ACrYCb2101010 =
    SURFACE_PIXEL_FORMAT_SUBSAMPLE_END,
// packed 422 formats, they should reside here as the necessity for programming chroma parameters are determined wrt SURFACE_PIXEL_FORMAT_SUBSAMPLE_END
// Packed 422 Format, 8bpc
    SURFACE_PIXEL_FORMAT_VIDEO_422_YCrYCb,
    SURFACE_PIXEL_FORMAT_VIDEO_422_YCbYCr,
    SURFACE_PIXEL_FORMAT_VIDEO_422_CrYCbY,
    SURFACE_PIXEL_FORMAT_VIDEO_422_CbYCrY,
// Packed 422 Format, 10bpc
    SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_YCrYCb,
    SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_YCbYCr,
    SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_CrYCbY,
    SURFACE_PIXEL_FORMAT_VIDEO_422_10bpc_CbYCrY,
// Packed 422 Format, 12bpc
    SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_YCrYCb,
    SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_YCbYCr,
    SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_CrYCbY,
    SURFACE_PIXEL_FORMAT_VIDEO_422_12bpc_CbYCrY,
    SURFACE_PIXEL_FORMAT_VIDEO_CrYCbA1010102,
    SURFACE_PIXEL_FORMAT_VIDEO_AYCrCb8888,
    SURFACE_PIXEL_FORMAT_INVALID

// grow 444 video here if necessary
}

// Pixel format
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_pixel_format {
// graph
    PIXEL_FORMAT_UNINITIALIZED,
    PIXEL_FORMAT_INDEX8,
    PIXEL_FORMAT_RGB565,
    PIXEL_FORMAT_ARGB8888,
    PIXEL_FORMAT_ARGB2101010,
    PIXEL_FORMAT_ARGB2101010_XRBIAS,
    PIXEL_FORMAT_FP16,
// video
    PIXEL_FORMAT_420BPP8,
    PIXEL_FORMAT_420BPP10,
// Align with SPL formats
    PIXEL_FORMAT_422BPP8,
    PIXEL_FORMAT_422BPP10,
    PIXEL_FORMAT_422BPP12,
    PIXEL_FORMAT_444BPP8,
    PIXEL_FORMAT_444BPP10,
// end of pixel format definition
    PIXEL_FORMAT_INVALID,

    PIXEL_FORMAT_GRPH_BEGIN = PIXEL_FORMAT_INDEX8,
    PIXEL_FORMAT_GRPH_END = PIXEL_FORMAT_FP16,
    PIXEL_FORMAT_VIDEO_BEGIN = PIXEL_FORMAT_420BPP8,
    PIXEL_FORMAT_VIDEO_END = PIXEL_FORMAT_444BPP10,
    PIXEL_FORMAT_UNKNOWN
}

//
// This structure holds a surface address.  There could be multiple addresses
// in cases such as Stereo 3D, Planar YUV, etc.  Other per-flip attributes such
// as frame durations and DCC format can also be set.
//
pub const DC_MAX_DIRTY_RECTS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_flip_addrs {
    pub address: dc_plane_address,
    pub flip_timestamp_in_us: c_ulonglong,
    pub flip_immediate: bool,
// TODO: add flip duration for FreeSync
    pub triplebuffer_flips: bool,
    pub dirty_rect_count: c_uint,
    pub dirty_rects: [rect; DC_MAX_DIRTY_RECTS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tile_split_values {
    DC_DISPLAY_MICRO_TILING = 0x0,
    DC_THIN_MICRO_TILING = 0x1,
    DC_DEPTH_MICRO_TILING = 0x2,
    DC_ROTATED_MICRO_TILING = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tripleBuffer_enable {
    DC_TRIPLEBUFFER_DISABLE = 0x0,
    DC_TRIPLEBUFFER_ENABLE = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tile_split_values_new {
    DC_SURF_TILE_SPLIT_1KB = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otg_pwa_sync_mode {
    DC_OTG_PWA_FRAME_SYNC_MODE_VSYNC = 0x0,
    DC_OTG_PWA_FRAME_SYNC_MODE_VSTARTUP = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otc_pwa_frame_sync {
    pub pwa_sync_mode: otg_pwa_sync_mode,
    pub pwa_frame_sync_line_offset: u32,
}

// TODO: These values come from hardware spec. We need to readdress this
// if they ever change.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum array_mode_values {
    DC_ARRAY_LINEAR_GENERAL = 0,
    DC_ARRAY_LINEAR_ALLIGNED,
    DC_ARRAY_1D_TILED_THIN1,
    DC_ARRAY_1D_TILED_THICK,
    DC_ARRAY_2D_TILED_THIN1,
    DC_ARRAY_PRT_TILED_THIN1,
    DC_ARRAY_PRT_2D_TILED_THIN1,
    DC_ARRAY_2D_TILED_THICK,
    DC_ARRAY_2D_TILED_X_THICK,
    DC_ARRAY_PRT_TILED_THICK,
    DC_ARRAY_PRT_2D_TILED_THICK,
    DC_ARRAY_PRT_3D_TILED_THIN1,
    DC_ARRAY_3D_TILED_THIN1,
    DC_ARRAY_3D_TILED_THICK,
    DC_ARRAY_3D_TILED_X_THICK,
    DC_ARRAY_PRT_3D_TILED_THICK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tile_mode_values {
    DC_ADDR_SURF_MICRO_TILING_DISPLAY = 0x0,
    DC_ADDR_SURF_MICRO_TILING_NON_DISPLAY = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum swizzle_mode_values {
    DC_SW_LINEAR = 0,
    DC_SW_256B_S = 1,
    DC_SW_256_D = 2,
    DC_SW_256_R = 3,
    DC_SW_4KB_S = 5,
    DC_SW_4KB_D = 6,
    DC_SW_4KB_R = 7,
    DC_SW_64KB_S = 9,
    DC_SW_64KB_D = 10,
    DC_SW_64KB_R = 11,
    DC_SW_VAR_S = 13,
    DC_SW_VAR_D = 14,
    DC_SW_VAR_R = 15,
    DC_SW_64KB_S_T = 17,
    DC_SW_64KB_D_T = 18,
    DC_SW_4KB_S_X = 21,
    DC_SW_4KB_D_X = 22,
    DC_SW_4KB_R_X = 23,
    DC_SW_64KB_S_X = 25,
    DC_SW_64KB_D_X = 26,
    DC_SW_64KB_R_X = 27,
    DC_SW_VAR_S_X = 29,
    DC_SW_VAR_D_X = 30,
    DC_SW_VAR_R_X = 31,
    DC_SW_MAX = 32,
    DC_SW_UNKNOWN = DC_SW_MAX
}

// Definition of swizzle modes with addr3 ASICs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum swizzle_mode_addr3_values {
    DC_ADDR3_SW_LINEAR = 0,
    DC_ADDR3_SW_256B_2D = 1,
    DC_ADDR3_SW_4KB_2D = 2,
    DC_ADDR3_SW_64KB_2D = 3,
    DC_ADDR3_SW_256KB_2D = 4,
    DC_ADDR3_SW_4KB_3D = 5,
    DC_ADDR3_SW_64KB_3D = 6,
    DC_ADDR3_SW_256KB_3D = 7,
    DC_ADDR3_SW_64KB_2D_Z = 8,
    DC_ADDR3_SW_256KB_2D_Z = 9,
    DC_ADDR3_SW_MAX = 10,
    DC_ADDR3_SW_UNKNOWN = DC_ADDR3_SW_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_gfxversion {
    DcGfxVersion7 = 0,
    DcGfxVersion8,
    DcGfxVersion9,
    DcGfxVersion10,
    DcGfxVersion11,
    DcGfxAddr3,
    DcGfxVersionUnknown
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_tiling_info {
    pub enum: unsigned int gfxversion; // Specifies which part of the union to use. Must use DalGfxVersion,
// Specifies the number of memory banks for tiling
// purposes.
// Only applies to 2D and 3D tiling modes.
// POSSIBLE VALUES: 2,4,8,16
//
    pub num_banks: c_uint,
// Specifies the number of tiles in the x direction
// to be incorporated into the same bank.
// Only applies to 2D and 3D tiling modes.
// POSSIBLE VALUES: 1,2,4,8
//
    pub bank_width: c_uint,
    pub bank_width_c: c_uint,
// Specifies the number of tiles in the y direction to
// be incorporated into the same bank.
// Only applies to 2D and 3D tiling modes.
// POSSIBLE VALUES: 1,2,4,8
//
    pub bank_height: c_uint,
    pub bank_height_c: c_uint,
// Specifies the macro tile aspect ratio. Only applies
// to 2D and 3D tiling modes.
//
    pub tile_aspect: c_uint,
    pub tile_aspect_c: c_uint,
// Specifies the number of bytes that will be stored
// contiguously for each tile.
// If the tile data requires more storage than this
// amount, it is split into multiple slices.
// This field must not be larger than
// GB_ADDR_CONFIG.DRAM_ROW_SIZE.
// Only applies to 2D and 3D tiling modes.
// For color render targets, TILE_SPLIT >= 256B.
//
    pub tile_split: tile_split_values,
    pub tile_split_c: tile_split_values,
// Specifies the addressing within a tile.
// 0x0 - DISPLAY_MICRO_TILING
// 0x1 - THIN_MICRO_TILING
// 0x2 - DEPTH_MICRO_TILING
// 0x3 - ROTATED_MICRO_TILING
//
    pub tile_mode: tile_mode_values,
    pub tile_mode_c: tile_mode_values,
// Specifies the number of pipes and how they are
// interleaved in the surface.
// Refer to memory addressing document for complete
// details and constraints.
//
    pub pipe_config: c_uint,
// Specifies the tiling mode of the surface.
// THIN tiles use an 8x8x1 tile size.
// THICK tiles use an 8x8x4 tile size.
// 2D tiling modes rotate banks for successive Z slices
// 3D tiling modes rotate pipes and banks for Z slices
// Refer to memory addressing document for complete
// details and constraints.
//
    pub array_mode: array_mode_values,
    pub gfx8: },
    pub swizzle: swizzle_mode_values,
    pub num_pipes: c_uint,
    pub max_compressed_frags: c_uint,
    pub pipe_interleave: c_uint,
    pub num_banks: c_uint,
    pub num_shader_engines: c_uint,
    pub num_rb_per_se: c_uint,
    pub shaderEnable: bool,
    pub meta_linear: bool,
    pub rb_aligned: bool,
    pub pipe_aligned: bool,
    pub num_pkrs: c_uint,
    pub above*/: *mut *mut } gfx9;/gfx9, gfx10 and,
    pub swizzle: swizzle_mode_addr3_values,
    pub above*/: *mut *mut } gfx_addr3;/gfx with addr3 and,
}

// Rotation angle
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_rotation_angle {
    ROTATION_ANGLE_0 = 0,
    ROTATION_ANGLE_90,
    ROTATION_ANGLE_180,
    ROTATION_ANGLE_270,
    ROTATION_ANGLE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_scan_direction {
    SCAN_DIRECTION_UNKNOWN = 0,
    SCAN_DIRECTION_HORIZONTAL = 1,  /* 0, 180 rotation */
    SCAN_DIRECTION_VERTICAL = 2,    /* 90, 270 rotation */
}

//
// struct dc_cursor_position: Hardware cursor data.
//
// This struct keeps the action information related to the cursor that will be
// sent and received from our DC core.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_cursor_position {
//
// @x: It represents the top left abscissa coordinate of the cursor.
//
    pub x: u32,
//
// @y: It is the top ordinate of the cursor coordinate.
//
    pub y: u32,
//
// @x_hotspot: Define the abscissa point where mouse click happens.
//
    pub x_hotspot: u32,
//
// @y_hotspot: Define the ordinate point where mouse click happens.
//
    pub y_hotspot: u32,
//
// @enable: This parameter indicates whether hardware cursor should be
// enabled.
//
    pub enable: bool,
//
// @translate_by_source: Translate cursor x/y by the source rectangle
// for each plane.
//
    pub translate_by_source: bool,
//
// @use_viewport_for_clip: Use viewport position for clip_x calculation
// instead of clip_rect. Required to protect against clip being overwritten
//
    pub use_viewport_for_clip: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_cursor_mi_param {
    pub pixel_clk_khz: c_uint,
    pub ref_clk_khz: c_uint,
    pub viewport: rect,
    pub recout: rect,
    pub h_scale_ratio: fixed31_32,
    pub v_scale_ratio: fixed31_32,
    pub rotation: dc_rotation_angle,
    pub mirror: bool,
    pub stream: *mut dc_stream_state,
}

// IPP related types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_gamma_type {
    GAMMA_RGB_256 = 1,
    GAMMA_RGB_FLOAT_1024 = 2,
    GAMMA_CS_TFM_1D = 3,
    GAMMA_CUSTOM = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_csc_transform {
    pub matrix: [u16; 12],
    pub enable_adjustment: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_rgb_fixed {
    pub red: fixed31_32,
    pub green: fixed31_32,
    pub blue: fixed31_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_gamma {
    pub refcount: kref,
    pub type: dc_gamma_type,
    pub num_entries: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_gamma_entries {
    pub red: [fixed31_32; GAMMA_MAX_ENTRIES],
    pub green: [fixed31_32; GAMMA_MAX_ENTRIES],
    pub blue: [fixed31_32; GAMMA_MAX_ENTRIES],
    pub entries: },
// private to DC core
    pub ctx: *mut dc_context,
// is_identity is used for RGB256 gamma identity which can also be programmed in INPUT_LUT.
// is_logical_identity indicates the given gamma ramp regardless of type is identity.
//
    pub is_identity: bool,
}

// Used by both ipp amd opp functions
// TODO: to be consolidated with enum color_space
//
// enum dc_cursor_color_format - DC cursor programming mode
//
// This enum is for programming CURSOR_MODE register field. What this register
// should be programmed to depends on OS requested cursor shape flags and what
// we stored in the cursor surface.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_cursor_color_format {
    CURSOR_MODE_MONO,
    CURSOR_MODE_COLOR_1BIT_AND,
    CURSOR_MODE_COLOR_PRE_MULTIPLIED_ALPHA,
    CURSOR_MODE_COLOR_UN_PRE_MULTIPLIED_ALPHA,
    CURSOR_MODE_COLOR_64BIT_FP_PRE_MULTIPLIED,
    CURSOR_MODE_COLOR_64BIT_FP_UN_PRE_MULTIPLIED
}

//
// This is all the parameters required by DAL in order to update the cursor
// attributes, including the new cursor image surface address, size, hotspot
// location, color format, etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dc_cursor_attribute_flags {
    pub ENABLE_MAGNIFICATION:1: u32,
    pub INVERSE_TRANSPARENT_CLAMPING:1: u32,
    pub HORIZONTAL_MIRROR:1: u32,
    pub VERTICAL_MIRROR:1: u32,
    pub INVERT_PIXEL_DATA:1: u32,
    pub ZERO_EXPANSION:1: u32,
    pub MIN_MAX_INVERT:1: u32,
    pub ENABLE_CURSOR_DEGAMMA:1: u32,
    pub RESERVED:24: u32,
    pub bits: },
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_cursor_attributes {
//
// @address: This field represents the framebuffer address associated
// with the cursor. It is important to highlight that this address is
// divided into a high and low parts.
//
    pub address: PHYSICAL_ADDRESS_LOC,
//
// @pitch: Cursor line stride.
//
    pub pitch: u32,
//
// @width: Width should correspond to cursor surface width.
//
    pub width: u32,
//
// @heigh: Height should correspond to cursor surface heigh.
//
    pub height: u32,
//
// @color_format: DC cursor programming mode.
//
    pub color_format: dc_cursor_color_format,
//
// @sdr_white_level: Boosting (SDR) cursor in HDR mode.
//
    pub sdr_white_level: u32,
//
// @rotation_angle: In case we support HW Cursor rotation in the future
//
    pub rotation_angle: dc_rotation_angle,
    pub attribute_flags: dc_cursor_attribute_flags,
    pub force_cursor_to_disp_pref: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_cursor_attributes {
    pub bias: c_int,
    pub scale: c_int,
}

// OPP
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_color_space {
    COLOR_SPACE_UNKNOWN,
    COLOR_SPACE_SRGB,
    COLOR_SPACE_XR_RGB,
    COLOR_SPACE_SRGB_LIMITED,
    COLOR_SPACE_MSREF_SCRGB,
    COLOR_SPACE_YCBCR601,
    COLOR_SPACE_YCBCR709,
    COLOR_SPACE_XV_YCC_709,
    COLOR_SPACE_XV_YCC_601,
    COLOR_SPACE_YCBCR601_LIMITED,
    COLOR_SPACE_YCBCR709_LIMITED,
    COLOR_SPACE_2020_RGB_FULLRANGE,
    COLOR_SPACE_2020_RGB_LIMITEDRANGE,
    COLOR_SPACE_2020_YCBCR_LIMITED,
    COLOR_SPACE_2020_YCBCR_FULL,
    COLOR_SPACE_ADOBERGB,
    COLOR_SPACE_DCIP3,
    COLOR_SPACE_DISPLAYNATIVE,
    COLOR_SPACE_DOLBYVISION,
    COLOR_SPACE_APPCTRL,
    COLOR_SPACE_CUSTOMPOINTS,
    COLOR_SPACE_YCBCR709_BLACK,
    COLOR_SPACE_2020_YCBCR = COLOR_SPACE_2020_YCBCR_LIMITED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_dither_option {
    DITHER_OPTION_DEFAULT,
    DITHER_OPTION_DISABLE,
    DITHER_OPTION_FM6,
    DITHER_OPTION_FM8,
    DITHER_OPTION_FM10,
    DITHER_OPTION_SPATIAL6_FRAME_RANDOM,
    DITHER_OPTION_SPATIAL8_FRAME_RANDOM,
    DITHER_OPTION_SPATIAL10_FRAME_RANDOM,
    DITHER_OPTION_SPATIAL6,
    DITHER_OPTION_SPATIAL8,
    DITHER_OPTION_SPATIAL10,
    DITHER_OPTION_TRUN6,
    DITHER_OPTION_TRUN8,
    DITHER_OPTION_TRUN10,
    DITHER_OPTION_TRUN10_SPATIAL8,
    DITHER_OPTION_TRUN10_SPATIAL6,
    DITHER_OPTION_TRUN10_FM8,
    DITHER_OPTION_TRUN10_FM6,
    DITHER_OPTION_TRUN10_SPATIAL8_FM6,
    DITHER_OPTION_SPATIAL10_FM8,
    DITHER_OPTION_SPATIAL10_FM6,
    DITHER_OPTION_TRUN8_SPATIAL6,
    DITHER_OPTION_TRUN8_FM6,
    DITHER_OPTION_SPATIAL8_FM6,
    DITHER_OPTION_MAX = DITHER_OPTION_SPATIAL8_FM6,
    DITHER_OPTION_INVALID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_quantization_range {
    QUANTIZATION_RANGE_UNKNOWN,
    QUANTIZATION_RANGE_FULL,
    QUANTIZATION_RANGE_LIMITED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_dynamic_expansion {
    DYN_EXPANSION_AUTO,
    DYN_EXPANSION_DISABLE
}

// XFM
// used in  struct dc_plane_state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scaling_taps {
    pub v_taps: u32,
    pub h_taps: u32,
    pub v_taps_c: u32,
    pub h_taps_c: u32,
    pub integer_scaling: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_timing_standard {
    DC_TIMING_STANDARD_UNDEFINED,
    DC_TIMING_STANDARD_DMT,
    DC_TIMING_STANDARD_GTF,
    DC_TIMING_STANDARD_CVT,
    DC_TIMING_STANDARD_CVT_RB,
    DC_TIMING_STANDARD_CEA770,
    DC_TIMING_STANDARD_CEA861,
    DC_TIMING_STANDARD_HDMI,
    DC_TIMING_STANDARD_TV_NTSC,
    DC_TIMING_STANDARD_TV_NTSC_J,
    DC_TIMING_STANDARD_TV_PAL,
    DC_TIMING_STANDARD_TV_PAL_M,
    DC_TIMING_STANDARD_TV_PAL_CN,
    DC_TIMING_STANDARD_TV_SECAM,
    DC_TIMING_STANDARD_EXPLICIT,
// !< For explicit timings from EDID, VBIOS, etc.
    DC_TIMING_STANDARD_USER_OVERRIDE,
// !< For mode timing override by user
    DC_TIMING_STANDARD_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_color_depth {
    COLOR_DEPTH_UNDEFINED,
    COLOR_DEPTH_666,
    COLOR_DEPTH_888,
    COLOR_DEPTH_101010,
    COLOR_DEPTH_121212,
    COLOR_DEPTH_141414,
    COLOR_DEPTH_161616,
    COLOR_DEPTH_999,
    COLOR_DEPTH_111111,
    COLOR_DEPTH_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_pixel_encoding {
    PIXEL_ENCODING_UNDEFINED,
    PIXEL_ENCODING_RGB,
    PIXEL_ENCODING_YCBCR422,
    PIXEL_ENCODING_YCBCR444,
    PIXEL_ENCODING_YCBCR420,
    PIXEL_ENCODING_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_aspect_ratio {
    ASPECT_RATIO_NO_DATA,
    ASPECT_RATIO_4_3,
    ASPECT_RATIO_16_9,
    ASPECT_RATIO_64_27,
    ASPECT_RATIO_256_135,
    ASPECT_RATIO_FUTURE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scanning_type {
    SCANNING_TYPE_NODATA = 0,
    SCANNING_TYPE_OVERSCAN,
    SCANNING_TYPE_UNDERSCAN,
    SCANNING_TYPE_FUTURE,
    SCANNING_TYPE_UNDEFINED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_crtc_timing_flags {
    pub :1: uint32_t INTERLACE,
    pub 1,: *mut *mut uint32_t HSYNC_POSITIVE_POLARITY :1; / when set to,
    pub 1,: *mut *mut uint32_t VSYNC_POSITIVE_POLARITY :1; / when set to,
    pub HORZ_COUNT_BY_TWO:1: u32,
    pub set,: *mut *mut uint32_t EXCLUSIVE_3D :1; / if this bit,
    pub polarity: *mut *mut uint32_t RIGHT_EYE_3D_POLARITY :1; / 1 - means right eye,
    pub subsampled: *mut *mut uint32_t SUB_SAMPLE_3D :1; / 1 - means left/right images,
    pub View,: *mut *mut uint32_t USE_IN_3D_VIEW_ONLY :1; / Do not use this timing in 2D,
    pub timing: *mut *mut uint32_t STEREO_3D_PREFERENCE :1; / Means this is 2D,
    pub :1: uint32_t Y_ONLY,
    pub /: *mut *mut uint32_t YCBCR420 :1; / TODO: shouldn't need this flag, should be a separate pixel format,
    pub /: *mut *mut uint32_t DTD_COUNTER :5; / values 1 to 16,
    pub :1: uint32_t FORCE_HDR,
// HDMI 2.0 - Support scrambling for TMDS character
// rates less than or equal to 340Mcsc
    pub LTE_340MCSC_SCRAMBLE:1: u32,
    pub /: *mut *mut uint32_t DSC : 1; / Use DSC with this timing,
    pub 1: uint32_t VBLANK_SYNCHRONIZABLE:,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_timing_3d_format {
    TIMING_3D_FORMAT_NONE,
    TIMING_3D_FORMAT_FRAME_ALTERNATE, /* No stereosync at all*/
    TIMING_3D_FORMAT_INBAND_FA, /* Inband Frame Alternate (DVI/DP)*/
    TIMING_3D_FORMAT_DP_HDMI_INBAND_FA, /* Inband FA to HDMI Frame Pack*/
// for active DP-HDMI dongle
    TIMING_3D_FORMAT_SIDEBAND_FA, /* Sideband Frame Alternate (eDP)*/
    TIMING_3D_FORMAT_HW_FRAME_PACKING,
    TIMING_3D_FORMAT_SW_FRAME_PACKING,
    TIMING_3D_FORMAT_ROW_INTERLEAVE,
    TIMING_3D_FORMAT_COLUMN_INTERLEAVE,
    TIMING_3D_FORMAT_PIXEL_INTERLEAVE,
    TIMING_3D_FORMAT_SIDE_BY_SIDE,
    TIMING_3D_FORMAT_TOP_AND_BOTTOM,
    TIMING_3D_FORMAT_SBS_SW_PACKED,
// Side-by-side, packed by application/driver into 2D frame
    TIMING_3D_FORMAT_TB_SW_PACKED,
// Top-and-bottom, packed by application/driver into 2D frame

    TIMING_3D_FORMAT_MAX,
}

pub const DC_DSC_QP_SET_SIZE: c_int = 15;
pub const DC_DSC_RC_BUF_THRESH_SIZE: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dsc_rc_params_override {
    pub rc_model_size: i32,
    pub rc_buf_thresh: [i32; DC_DSC_RC_BUF_THRESH_SIZE],
    pub rc_minqp: [i32; DC_DSC_QP_SET_SIZE],
    pub rc_maxqp: [i32; DC_DSC_QP_SET_SIZE],
    pub rc_offset: [i32; DC_DSC_QP_SET_SIZE],
    pub rc_tgt_offset_hi: i32,
    pub rc_tgt_offset_lo: i32,
    pub rc_edge_factor: i32,
    pub rc_quant_incr_limit0: i32,
    pub rc_quant_incr_limit1: i32,
    pub initial_fullness_offset: i32,
    pub initial_delay: i32,
    pub flatness_min_qp: i32,
    pub flatness_max_qp: i32,
    pub flatness_det_thresh: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dsc_config {
    pub /: *mut *mut uint32_t num_slices_h; / Number of DSC slices - horizontal,
    pub /: *mut *mut uint32_t num_slices_v; / Number of DSC slices - vertical,
    pub /: *mut *mut uint32_t bits_per_pixel; / DSC target bitrate in 1/16 of bpp (e.g. 128 -> 8bpp),
    pub /: *mut *mut bool block_pred_enable; / DSC block prediction enable,
    pub /: *mut *mut uint32_t linebuf_depth; / DSC line buffer depth,
    pub /: *mut *mut uint32_t version_minor; / DSC minor version. Full version is formed as 1.version_minor.,
    pub /: *mut *mut bool ycbcr422_simple; / Tell DSC engine to convert YCbCr 4:2:2 to 'YCbCr 4:2:2 simple'.,
    pub /: *mut *mut int32_t rc_buffer_size; / DSC RC buffer block size in bytes,
    pub /: *mut *mut bool is_frl; / indicate if DSC is applied based on HDMI FRL sink's capability,
    pub /: *mut *mut bool is_vic_all_bpp; / indicate of DSC_ALL_BPP = 1,
    pub /: *mut *mut uint32_t total_chunk_kbytes; / total chunk kbytes in EDID,
    pub /: *mut *mut bool is_dp; / indicate if DSC is applied based on DP's capability,
    pub /: *mut *mut uint32_t mst_pbn; / pbn of display on dsc mst hub,
    pub /: *const *const *const dc_dsc_rc_params_override rc_params_ovrd; / DM owned memory. If not NULL, apply custom dsc rc params,
}

//
// struct dc_crtc_timing - Timing parameters used to configure DCN blocks
//
// DCN provides multiple signals and parameters that can be used to adjust
// timing parameters, this struct aggregate multiple of these values for easy
// access. In this struct, fields prefixed with h_* are related to horizontal
// timing, and v_* to vertical timing. Keep in mind that when we talk about
// vertical timings, the values, in general, are described in the number of
// lines; on the other hand, the horizontal values are in pixels.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_crtc_timing {
//
// @h_total: The total number of pixels from the rising edge of HSync
// until the rising edge of the current HSync.
//
    pub h_total: u32,
//
// @h_border_left: The black pixels related to the left border
//
    pub h_border_left: u32,
//
// @h_addressable: It is the range of pixels displayed horizontally.
// For example, if the display resolution is 3840@2160, the horizontal
// addressable area is 3840.
//
    pub h_addressable: u32,
//
// @h_border_right: The black pixels related to the right border
//
    pub h_border_right: u32,
//
// @h_front_porch: Period (in pixels) between HBlank start and the
// rising edge of HSync.
//
    pub h_front_porch: u32,
//
// @h_sync_width: HSync duration in pixels.
//
    pub h_sync_width: u32,
//
// @v_total: It is the total number of lines from the rising edge of
// the previous VSync until the rising edge of the current VSync.
//
// |--------------------------|
// +-+        V_TOTAL         +-+
// | |                        | |
// VSync ---+ +--------- // -----------+ +---
//
    pub v_total: u32,
//
// @v_border_top: The black border on the top.
//
    pub v_border_top: u32,
//
// @v_addressable: It is the range of the scanout at which the
// framebuffer is displayed. For example, if the display resolution is
// 3840@2160, the addressable area is 2160 lines, or if the resolution
// is 1920x1080, the addressable area is 1080 lines.
//
    pub v_addressable: u32,
//
// @v_border_bottom: The black border on the bottom.
//
    pub v_border_bottom: u32,
//
// @v_front_porch: Period (in lines) between VBlank start and rising
// edge of VSync.
// +-+
// VSync            | |
// ----------+ +--------...
// +------------------...
// VBlank   |
// --+
// |-------|
// v_front_porch
//
    pub v_front_porch: u32,
//
// @v_sync_width: VSync signal width in lines.
//
    pub v_sync_width: u32,
//
// @pix_clk_100hz: Pipe pixel precision
//
// This field is used to communicate pixel clocks with 100 Hz accuracy
// from dc_crtc_timing to BIOS command table.
//
    pub pix_clk_100hz: u32,
    pub min_refresh_in_uhz: u32,
    pub max_refresh_in_uhz: u32,
    pub vic: u32,
    pub hdmi_vic: u32,
    pub rid: u32,
    pub fr_index: u32,
    pub frl_uncompressed_video_bandwidth_in_kbps: u32,
    pub timing_3d_format: dc_timing_3d_format,
    pub display_color_depth: dc_color_depth,
    pub pixel_encoding: dc_pixel_encoding,
    pub aspect_ratio: dc_aspect_ratio,
    pub scan_type: scanning_type,
    pub flags: dc_crtc_timing_flags,
    pub /: *mut *mut uint32_t dsc_fixed_bits_per_pixel_x16; / DSC target bitrate in 1/16 of bpp (e.g. 128 -> 8bpp),
    pub dsc_cfg: dc_dsc_config,
// The number of pixels that HBlank has been expanded by from the original EDID timing.
    pub expanded_hblank: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trigger_delay {
    TRIGGER_DELAY_NEXT_PIXEL = 0,
    TRIGGER_DELAY_NEXT_LINE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum crtc_event {
    CRTC_EVENT_VSYNC_RISING = 0,
    CRTC_EVENT_VSYNC_FALLING
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crtc_trigger_info {
    pub enabled: bool,
    pub event_source: *mut dc_stream_state,
    pub event: crtc_event,
    pub delay: trigger_delay,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_crtc_timing_adjust {
    pub v_total_min: u32,
    pub v_total_max: u32,
    pub v_total_mid: u32,
    pub v_total_mid_frame_num: u32,
    pub allow_otg_v_count_halt: u32,
    pub timing_adjust_pending: u8,
}

// Passed on init
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vram_type {
    VIDEO_MEMORY_TYPE_GDDR5  = 2,
    VIDEO_MEMORY_TYPE_DDR3   = 3,
    VIDEO_MEMORY_TYPE_DDR4   = 4,
    VIDEO_MEMORY_TYPE_HBM    = 5,
    VIDEO_MEMORY_TYPE_GDDR6  = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_cnv_out_bpc {
    DWB_CNV_OUT_BPC_8BPC  = 0,
    DWB_CNV_OUT_BPC_10BPC = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_output_depth {
    DWB_OUTPUT_PIXEL_DEPTH_8BPC = 0,
    DWB_OUTPUT_PIXEL_DEPTH_10BPC = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_capture_rate {
    dwb_capture_rate_0 = 0,	/* Every frame is captured. */
    dwb_capture_rate_1 = 1,	/* Every other frame is captured. */
    dwb_capture_rate_2 = 2,	/* Every 3rd frame is captured. */
    dwb_capture_rate_3 = 3,	/* Every 4th frame is captured. */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_scaler_mode {
    dwb_scaler_mode_bypass444 = 0,
    dwb_scaler_mode_rgb444 = 1,
    dwb_scaler_mode_yuv444 = 2,
    dwb_scaler_mode_yuv420 = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_subsample_position {
    DWB_INTERSTITIAL_SUBSAMPLING = 0,
    DWB_COSITED_SUBSAMPLING      = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_stereo_eye_select {
    DWB_STEREO_EYE_LEFT  = 1,		/* Capture left eye only */
    DWB_STEREO_EYE_RIGHT = 2,		/* Capture right eye only */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_stereo_type {
    DWB_STEREO_TYPE_FRAME_PACKING = 0,		/* Frame packing */
    DWB_STEREO_TYPE_FRAME_SEQUENTIAL = 3,	/* Frame sequential */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_out_format {
    DWB_OUT_FORMAT_32BPP_ARGB = 0,
    DWB_OUT_FORMAT_32BPP_RGBA = 1,
    DWB_OUT_FORMAT_64BPP_ARGB = 2,
    DWB_OUT_FORMAT_64BPP_RGBA = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_out_denorm {
    DWB_OUT_DENORM_10BPC = 0,
    DWB_OUT_DENORM_8BPC = 1,
    DWB_OUT_DENORM_BYPASS = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm_gamut_remap_select {
    CM_GAMUT_REMAP_MODE_BYPASS = 0,
    CM_GAMUT_REMAP_MODE_RAMA_COEFF,
    CM_GAMUT_REMAP_MODE_RAMB_COEFF,
    CM_GAMUT_REMAP_MODE_RESERVED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm_gamut_coef_format {
    CM_GAMUT_REMAP_COEF_FORMAT_S2_13 = 0,
    CM_GAMUT_REMAP_COEF_FORMAT_S3_12 = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpcc_gamut_remap_mode_select {
    MPCC_GAMUT_REMAP_MODE_SELECT_0 = 0,
    MPCC_GAMUT_REMAP_MODE_SELECT_1,
    MPCC_GAMUT_REMAP_MODE_SELECT_2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpcc_gamut_remap_id {
    MPCC_OGAM_GAMUT_REMAP,
    MPCC_MCM_FIRST_GAMUT_REMAP,
    MPCC_MCM_SECOND_GAMUT_REMAP,
    MPCC_RMCM_GAMUT_REMAP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cursor_matrix_mode {
    CUR_MATRIX_BYPASS = 0,
    CUR_MATRIX_SET_A,
    CUR_MATRIX_SET_B
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcif_warmup_params {
    pub start_address: large_integer,
    pub address_increment: c_uint,
    pub region_size: c_uint,
    pub p_vmid: c_uint,
}

pub const MCIF_BUF_COUNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcif_buf_params {
    pub luma_address: [c_ulonglong; MCIF_BUF_COUNT],
    pub chroma_address: [c_ulonglong; MCIF_BUF_COUNT],
    pub luma_pitch: c_uint,
    pub chroma_pitch: c_uint,
    pub warmup_pitch: c_uint,
    pub swlock: c_uint,
    pub p_vmid: c_uint,
    pub tmz_id: u8,
}

pub const MAX_TG_COLOR_VALUE: c_uint = 0x3FF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg_color {
// Maximum 10 bits color value
    pub color_r_cr: u16,
    pub color_g_y: u16,
    pub color_b_cb: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fva_adj {
    pub pixel_clock_100hz: c_uint,
    pub max_pixel_clock_100hz: c_uint,
    pub fva_factor: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symclk_state {
    SYMCLK_OFF_TX_OFF,
    SYMCLK_ON_TX_ON,
    SYMCLK_ON_TX_OFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_state {
    pub 1: uint8_t otg :,
    pub 7: uint8_t reserved :,
    pub symclk_ref_cnts: },
    pub symclk_state: symclk_state,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm_hist_tap_point {
    CM_HIST_TAP_POINT_1,
    CM_HIST_TAP_POINT_2,
    CM_HIST_TAP_POINT_3,
    CM_HIST_TAP_POINT_4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm_hist_src {
    CM_HIST_SRC1,
    CM_HIST_SRC2,
    CM_HIST_SRC3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm_hist_format {
    CM_HIST_FORMAT_FIXED_POINT,
    CM_HIST_FORMAT_FP16_POS,
    CM_HIST_FORMAT_FP16_POS_AND_NEG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm_hist_read_channel_mask {
    CM_HIST_READ_DISABLED,
    CM_HIST_READ_CH1,
    CM_HIST_READ_CH2,
    CM_HIST_READ_CH1_CH2,
    CM_HIST_READ_CH3,
    CM_HIST_READ_CH1_CH3,
    CM_HIST_READ_CH2_CH3,
    CM_HIST_READ_ALL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm_hist_src1_mode {
    CM_HIST_SRC1_MODE_R_OR_CR,
    CM_HIST_SRC1_MODE_MAX_RGB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm_hist_src2_mode {
    CM_HIST_SRC2_MODE_G_OR_Y,
    CM_HIST_SRC2_MODE_RGB_TO_Y,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm_hist_src3_mode {
    CM_HIST_SRC3_MODE_B_OR_CB,
    CM_HIST_SRC3_MODE_MIN_RGB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cm_hist_control {
    pub tap_point: cm_hist_tap_point,
    pub channels_enabled: u32,
    pub src_1_select: cm_hist_src1_mode,
    pub src_2_select: cm_hist_src2_mode,
    pub src_3_select: cm_hist_src3_mode,
    pub ch1_src: cm_hist_src,
    pub ch2_src: cm_hist_src,
    pub ch3_src: cm_hist_src,
    pub format: cm_hist_format,
    pub read_channel_mask: cm_hist_read_channel_mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cm_hist {
    pub ch1: [u32; 256],
    pub ch2: [u32; 256],
    pub ch3: [u32; 256],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pregam_mode {
    PREGAM_BYPASS = 0,
    PREGAM_DEGAM,
    PREGAM_REGAM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum degam_lut {
    DEGAM_SRGB = 0,
    DEGAM_GAMMA_22,
    DEGAM_GAMMA_24,
    DEGAM_GAMMA_26,
    DEGAM_BT2020,
    DEGAM_BT2100PQ,
    DEGAM_BT2100HLG
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regam_lut {
    REGAM_20 = 0,
    REGAM_24
}
