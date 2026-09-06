//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/dwb.h
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


// Copyright 2012-17 Advanced Micro Devices, Inc.
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

pub const DWB_SW_V2: c_int = 1;
pub const DWB_MCIF_BUF_COUNT: c_int = 4;
// forward declaration of mcif_wb struct
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_sw_version {
    dwb_ver_1_0 = 1,
    dwb_ver_2_0 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_source {
    dwb_src_scl = 0,	/* for DCE7x/9x, DCN won't support. */
    dwb_src_blnd,		/* for DCE7x/9x */
    dwb_src_fmt,		/* for DCE7x/9x */
    dwb_src_otg0 = 0x100,	/* for DCN1.x/DCN2.x, register: mmDWB_SOURCE_SELECT */
    dwb_src_otg1,		/* for DCN1.x/DCN2.x */
    dwb_src_otg2,		/* for DCN1.x/DCN2.x */
    dwb_src_otg3,		/* for DCN1.x/DCN2.x */
}

// DCN1.x, DCN2.x support 2 pipes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_pipe {
    dwb_pipe0 = 0,
    dwb_pipe1,
    dwb_pipe_max_num,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_frame_capture_enable {
    DWB_FRAME_CAPTURE_DISABLE = 0,
    DWB_FRAME_CAPTURE_ENABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wbscl_coef_filter_type_sel {
    WBSCL_COEF_LUMA_VERT_FILTER = 0,
    WBSCL_COEF_CHROMA_VERT_FILTER = 1,
    WBSCL_COEF_LUMA_HORZ_FILTER = 2,
    WBSCL_COEF_CHROMA_HORZ_FILTER = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_boundary_mode {
    DWBSCL_BOUNDARY_MODE_EDGE  = 0,
    DWBSCL_BOUNDARY_MODE_BLACK = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_output_csc_mode {
    DWB_OUTPUT_CSC_DISABLE = 0,
    DWB_OUTPUT_CSC_COEF_A = 1,
    DWB_OUTPUT_CSC_COEF_B = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_ogam_lut_mode {
    DWB_OGAM_MODE_BYPASS,
    DWB_OGAM_RAMA_LUT,
    DWB_OGAM_RAMB_LUT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_color_volume {
    DWB_SRGB_BT709 = 0,	//SDR
    DWB_PQ = 1,	//HDR
    DWB_HLG = 2,	//HDR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwb_color_space {
    DWB_SRGB = 0,	//SDR
    DWB_BT709 = 1,	//SDR
    DWB_BT2020 = 2,	//HDR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwb_efc_hdr_metadata {
// display chromaticities and white point in units of 0.00001
    pub chromaticity_green_x: c_uint,
    pub chromaticity_green_y: c_uint,
    pub chromaticity_blue_x: c_uint,
    pub chromaticity_blue_y: c_uint,
    pub chromaticity_red_x: c_uint,
    pub chromaticity_red_y: c_uint,
    pub chromaticity_white_point_x: c_uint,
    pub chromaticity_white_point_y: c_uint,
// in units of candelas per square meter
    pub min_luminance: c_uint,
    pub max_luminance: c_uint,
// in units of nits
    pub maximum_content_light_level: c_uint,
    pub maximum_frame_average_light_level: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwb_efc_display_settings {
    pub inputColorVolume: c_uint,
    pub inputColorSpace: c_uint,
    pub inputBitDepthMinus8: c_uint,
    pub hdr_metadata: dwb_efc_hdr_metadata,
    pub Black: unsigned int dwbOutputBlack; // 0 - Normal, 1 - Output,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwb_warmup_params {
    pub /: *mut *mut bool warmup_en; / false: normal mode, true: enable pattern generator,
    pub /: *mut *mut bool warmup_mode; / false: 420, true: 444,
    pub /: *mut *mut bool warmup_depth; / false: 8bit, true: 10bit,
    pub /: *mut *mut int warmup_data; / Data to be sent by pattern generator (same for each pixel component),
    pub /: *mut *mut int warmup_width; / Pattern width (pixels),
    pub /: *mut *mut int warmup_height; / Pattern height (lines),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwb_caps {
    pub /: *mut *mut dce_version hw_version; / DCN engine version.,
    pub /: *mut *mut dwb_sw_version sw_version; / DWB sw implementation version.,
    pub /: *mut *mut unsigned int reserved[6]; / Reserved for future use, MUST BE 0.,
    pub adapter_id: c_uint,
    pub /: *mut *mut unsigned int num_pipes; / number of DWB pipes,
    pub :1: unsigned int support_dwb,
    pub :1: unsigned int support_ogam,
    pub :1: unsigned int support_wbscl,
    pub :1: unsigned int support_ocsc,
    pub :1: unsigned int support_stereo,
    pub :1: unsigned int support_4k_120p,
    pub caps: },
    pub /: *mut *mut unsigned int reserved2[10]; / Reserved for future use, MUST BE 0.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwbc {
    pub funcs: *const dwbc_funcs,
    pub ctx: *mut dc_context,
    pub inst: c_int,
    pub mcif: *mut mcif_wb,
    pub status: bool,
    pub inputSrcSelect: c_int,
    pub dwb_output_black: bool,
    pub tf: dc_transfer_func_predefined,
    pub output_color_space: dc_color_space,
    pub dwb_is_efc_transition: bool,
    pub dwb_is_drc: bool,
    pub inst*/: *mut *mut int wb_src_plane_inst;/hubp, mpcc,,
    pub mask_id: u32,
    pub otg_inst: c_int,
    pub mvc_cfg: bool,
    pub params: dc_dwb_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwbc_funcs {
    pub caps): *mut dwb_caps,
    pub params): *mut dc_dwb_params,
    pub dwbc): *mut *mut bool (disable)(struct dwbc,
    pub params): *mut dc_dwb_params,
    pub dwbc): *mut dwbc,
    pub enable): dwb_frame_capture_enable,
    pub params): *mut dc_dwb_params,
    pub stereo_params): *mut dwb_stereo_params,
    pub is_new_content): bool,
    pub warmup_params): *mut dwb_warmup_params,
    pub over_run): *mut c_uint,

    pub mode): dwb_output_csc_mode,
    pub in_transfer_func_dwb_ogam): *const dc_transfer_func,

// TODO: merge with output_transfer_func?
    pub in_transfer_func_dwb_ogam): *const dc_transfer_func,
    pub time_stamp): *mut *mut dwbc dwbc, uint32_t,
    pub dwbc): *mut dwbc,
}
