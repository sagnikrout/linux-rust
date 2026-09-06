//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/rockchip/rockchip_drm_vop.h
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Author:Mark Yao <mark.yao@rock-chips.com>
//
// major: IP major version, used for IP structure
// minor: big feature change under same structure
//

pub const NUM_YUV2YUV_COEFFICIENTS: c_int = 12;
// AFBC supports a number of configurable modes. Relevant to us is block size
// (16x16 or 32x8), storage modifiers (SPARSE, SPLIT), and the YUV-like
// colourspace transform (YTR). 16x16 SPARSE mode is always used. SPLIT mode
// could be enabled via the hreg_block_split register, but is not currently
// handled. The colourspace transform is implicitly always assumed by the
// decoder, so consumers must use this transform as well.
//
// Failure to match modifiers will cause errors displaying AFBC buffers
// produced by conformant AFBC producers, including Mesa.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vop_data_format {
    VOP_FMT_ARGB8888 = 0,
    VOP_FMT_RGB888,
    VOP_FMT_RGB565,
    VOP_FMT_YUV420SP = 4,
    VOP_FMT_YUV422SP,
    VOP_FMT_YUV444SP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_rect {
    pub width: c_int,
    pub height: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_reg {
    pub mask: u32,
    pub offset: u16,
    pub shift: u8,
    pub write_mask: bool,
    pub relaxed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_afbc {
    pub enable: vop_reg,
    pub win_sel: vop_reg,
    pub format: vop_reg,
    pub rb_swap: vop_reg,
    pub uv_swap: vop_reg,
    pub auto_gating_en: vop_reg,
    pub block_split_en: vop_reg,
    pub pic_vir_width: vop_reg,
    pub tile_num: vop_reg,
    pub hreg_block_split: vop_reg,
    pub pic_offset: vop_reg,
    pub pic_size: vop_reg,
    pub dsp_offset: vop_reg,
    pub transform_offset: vop_reg,
    pub hdr_ptr: vop_reg,
    pub half_block_en: vop_reg,
    pub xmirror: vop_reg,
    pub ymirror: vop_reg,
    pub rotate_270: vop_reg,
    pub rotate_90: vop_reg,
    pub rstn: vop_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_modeset {
    pub htotal_pw: vop_reg,
    pub hact_st_end: vop_reg,
    pub hpost_st_end: vop_reg,
    pub vtotal_pw: vop_reg,
    pub vact_st_end: vop_reg,
    pub vpost_st_end: vop_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_output {
    pub pin_pol: vop_reg,
    pub dp_pin_pol: vop_reg,
    pub dp_dclk_pol: vop_reg,
    pub edp_pin_pol: vop_reg,
    pub edp_dclk_pol: vop_reg,
    pub hdmi_pin_pol: vop_reg,
    pub hdmi_dclk_pol: vop_reg,
    pub mipi_pin_pol: vop_reg,
    pub mipi_dclk_pol: vop_reg,
    pub rgb_pin_pol: vop_reg,
    pub rgb_dclk_pol: vop_reg,
    pub dp_en: vop_reg,
    pub edp_en: vop_reg,
    pub hdmi_en: vop_reg,
    pub mipi_en: vop_reg,
    pub mipi_dual_channel_en: vop_reg,
    pub rgb_en: vop_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_common {
    pub cfg_done: vop_reg,
    pub dsp_blank: vop_reg,
    pub data_blank: vop_reg,
    pub pre_dither_down: vop_reg,
    pub dither_down_sel: vop_reg,
    pub dither_down_mode: vop_reg,
    pub dither_down_en: vop_reg,
    pub dither_up: vop_reg,
    pub dsp_lut_en: vop_reg,
    pub update_gamma_lut: vop_reg,
    pub lut_buffer_index: vop_reg,
    pub gate_en: vop_reg,
    pub mmu_en: vop_reg,
    pub dma_stop: vop_reg,
    pub out_mode: vop_reg,
    pub standby: vop_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_misc {
    pub global_regdone_en: vop_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_intr {
    pub intrs: *const c_int,
    pub nintrs: u32,
    pub line_flag_num: [vop_reg; 2],
    pub enable: vop_reg,
    pub clear: vop_reg,
    pub status: vop_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_scl_extension {
    pub cbcr_vsd_mode: vop_reg,
    pub cbcr_vsu_mode: vop_reg,
    pub cbcr_hsd_mode: vop_reg,
    pub cbcr_ver_scl_mode: vop_reg,
    pub cbcr_hor_scl_mode: vop_reg,
    pub yrgb_vsd_mode: vop_reg,
    pub yrgb_vsu_mode: vop_reg,
    pub yrgb_hsd_mode: vop_reg,
    pub yrgb_ver_scl_mode: vop_reg,
    pub yrgb_hor_scl_mode: vop_reg,
    pub line_load_mode: vop_reg,
    pub cbcr_axi_gather_num: vop_reg,
    pub yrgb_axi_gather_num: vop_reg,
    pub vsd_cbcr_gt2: vop_reg,
    pub vsd_cbcr_gt4: vop_reg,
    pub vsd_yrgb_gt2: vop_reg,
    pub vsd_yrgb_gt4: vop_reg,
    pub bic_coe_sel: vop_reg,
    pub cbcr_axi_gather_en: vop_reg,
    pub yrgb_axi_gather_en: vop_reg,
    pub lb_mode: vop_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_scl_regs {
    pub ext: *const vop_scl_extension,
    pub scale_yrgb_x: vop_reg,
    pub scale_yrgb_y: vop_reg,
    pub scale_cbcr_x: vop_reg,
    pub scale_cbcr_y: vop_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_yuv2yuv_phy {
    pub y2r_coefficients: [vop_reg; NUM_YUV2YUV_COEFFICIENTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_win_phy {
    pub scl: *const vop_scl_regs,
    pub data_formats: *const u32,
    pub nformats: u32,
    pub format_modifiers: *const u64,
    pub enable: vop_reg,
    pub gate: vop_reg,
    pub format: vop_reg,
    pub fmt_10: vop_reg,
    pub rb_swap: vop_reg,
    pub uv_swap: vop_reg,
    pub act_info: vop_reg,
    pub dsp_info: vop_reg,
    pub dsp_st: vop_reg,
    pub yrgb_mst: vop_reg,
    pub uv_mst: vop_reg,
    pub yrgb_vir: vop_reg,
    pub uv_vir: vop_reg,
    pub y_mir_en: vop_reg,
    pub x_mir_en: vop_reg,
    pub dst_alpha_ctl: vop_reg,
    pub src_alpha_ctl: vop_reg,
    pub alpha_pre_mul: vop_reg,
    pub alpha_mode: vop_reg,
    pub alpha_en: vop_reg,
    pub channel: vop_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_win_yuv2yuv_data {
    pub base: u32,
    pub phy: *const vop_yuv2yuv_phy,
    pub y2r_en: vop_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_win_data {
    pub base: u32,
    pub phy: *const vop_win_phy,
    pub type: drm_plane_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vop_data {
    pub version: u32,
    pub intr: *const vop_intr,
    pub common: *const vop_common,
    pub misc: *const vop_misc,
    pub modeset: *const vop_modeset,
    pub output: *const vop_output,
    pub afbc: *const vop_afbc,
    pub win_yuv2yuv: *const vop_win_yuv2yuv_data,
    pub win: *const vop_win_data,
    pub win_size: c_uint,
    pub lut_size: c_uint,
    pub max_output: vop_rect,

    pub feature: u64,
}

// interrupt define

pub const INTR_CLR_SHIFT: c_int = 8;

// src alpha ctrl define

// dst alpha ctrl define

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alpha_mode {
    ALPHA_STRAIGHT,
    ALPHA_INVERSE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum global_blend_mode {
    ALPHA_GLOBAL,
    ALPHA_PER_PIX,
    ALPHA_PER_PIX_GLOBAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alpha_cal_mode {
    ALPHA_SATURATION,
    ALPHA_NO_SATURATION,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum color_mode {
    ALPHA_SRC_PRE_MUL,
    ALPHA_SRC_NO_PRE_MUL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum factor_mode {
    ALPHA_ZERO,
    ALPHA_ONE,
    ALPHA_SRC,
    ALPHA_SRC_INVERSE,
    ALPHA_SRC_GLOBAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scale_mode {
    SCALE_NONE = 0x0,
    SCALE_UP   = 0x1,
    SCALE_DOWN = 0x2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lb_mode {
    LB_YUV_3840X5 = 0x0,
    LB_YUV_2560X8 = 0x1,
    LB_RGB_3840X2 = 0x2,
    LB_RGB_2560X4 = 0x3,
    LB_RGB_1920X5 = 0x4,
    LB_RGB_1280X8 = 0x5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sacle_up_mode {
    SCALE_UP_BIL = 0x0,
    SCALE_UP_BIC = 0x1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scale_down_mode {
    SCALE_DOWN_BIL = 0x0,
    SCALE_DOWN_AVG = 0x1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dither_down_mode {
    RGB888_TO_RGB565 = 0x0,
    RGB888_TO_RGB666 = 0x1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dither_down_mode_sel {
    DITHER_DOWN_ALLEGRO = 0x0,
    DITHER_DOWN_FRC = 0x1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vop_pol {
    HSYNC_POSITIVE = 0,
    VSYNC_POSITIVE = 1,
    DEN_NEGATIVE   = 2
}

pub const SCL_FT_DEFAULT_FIXPOINT_SHIFT: c_int = 12;
pub const SCL_MAX_VSKIPLINES: c_int = 4;
pub const MIN_SCL_FT_AFTER_VSKIP: c_int = 1;

extern "C" {
    pub fn GET_SCL_FT_BILI_DN(_arg: act_height, _arg: dst_h) -> return;
}
