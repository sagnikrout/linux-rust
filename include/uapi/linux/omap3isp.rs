//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/omap3isp.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// omap3isp.h
//
// TI OMAP3 ISP - User-space API
//
// Copyright (C) 2010 Nokia Corporation
// Copyright (C) 2009 Texas Instruments, Inc.
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA
// 02110-1301 USA
//

//
// Private IOCTLs
//
// VIDIOC_OMAP3ISP_CCDC_CFG: Set CCDC configuration
// VIDIOC_OMAP3ISP_PRV_CFG: Set preview engine configuration
// VIDIOC_OMAP3ISP_AEWB_CFG: Set AEWB module configuration
// VIDIOC_OMAP3ISP_HIST_CFG: Set histogram module configuration
// VIDIOC_OMAP3ISP_AF_CFG: Set auto-focus module configuration
// VIDIOC_OMAP3ISP_STAT_REQ: Read statistics (AEWB/AF/histogram) data
// VIDIOC_OMAP3ISP_STAT_EN: Enable/disable a statistics module
//

//
// Events
//
// V4L2_EVENT_OMAP3ISP_AEWB: AEWB statistics data ready
// V4L2_EVENT_OMAP3ISP_AF: AF statistics data ready
// V4L2_EVENT_OMAP3ISP_HIST: Histogram statistics data ready
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_stat_event_status {
    pub frame_number: __u32,
    pub config_counter: __u16,
    pub buf_err: __u8,
}

// AE/AWB related structures and flags
// H3A Range Constants
pub const OMAP3ISP_AEWB_MAX_SATURATION_LIM: c_int = 1023;
pub const OMAP3ISP_AEWB_MIN_WIN_H: c_int = 2;
pub const OMAP3ISP_AEWB_MAX_WIN_H: c_int = 256;
pub const OMAP3ISP_AEWB_MIN_WIN_W: c_int = 6;
pub const OMAP3ISP_AEWB_MAX_WIN_W: c_int = 256;
pub const OMAP3ISP_AEWB_MIN_WINVC: c_int = 1;
pub const OMAP3ISP_AEWB_MIN_WINHC: c_int = 1;
pub const OMAP3ISP_AEWB_MAX_WINVC: c_int = 128;
pub const OMAP3ISP_AEWB_MAX_WINHC: c_int = 36;
pub const OMAP3ISP_AEWB_MAX_WINSTART: c_int = 4095;
pub const OMAP3ISP_AEWB_MIN_SUB_INC: c_int = 2;
pub const OMAP3ISP_AEWB_MAX_SUB_INC: c_int = 32;
pub const OMAP3ISP_AEWB_MAX_BUF_SIZE: c_int = 83600;
pub const OMAP3ISP_AF_IIRSH_MIN: c_int = 0;
pub const OMAP3ISP_AF_IIRSH_MAX: c_int = 4095;
pub const OMAP3ISP_AF_PAXEL_HORIZONTAL_COUNT_MIN: c_int = 1;
pub const OMAP3ISP_AF_PAXEL_HORIZONTAL_COUNT_MAX: c_int = 36;
pub const OMAP3ISP_AF_PAXEL_VERTICAL_COUNT_MIN: c_int = 1;
pub const OMAP3ISP_AF_PAXEL_VERTICAL_COUNT_MAX: c_int = 128;
pub const OMAP3ISP_AF_PAXEL_INCREMENT_MIN: c_int = 2;
pub const OMAP3ISP_AF_PAXEL_INCREMENT_MAX: c_int = 32;
pub const OMAP3ISP_AF_PAXEL_HEIGHT_MIN: c_int = 2;
pub const OMAP3ISP_AF_PAXEL_HEIGHT_MAX: c_int = 256;
pub const OMAP3ISP_AF_PAXEL_WIDTH_MIN: c_int = 16;
pub const OMAP3ISP_AF_PAXEL_WIDTH_MAX: c_int = 256;
pub const OMAP3ISP_AF_PAXEL_HZSTART_MIN: c_int = 1;
pub const OMAP3ISP_AF_PAXEL_HZSTART_MAX: c_int = 4095;
pub const OMAP3ISP_AF_PAXEL_VTSTART_MIN: c_int = 0;
pub const OMAP3ISP_AF_PAXEL_VTSTART_MAX: c_int = 4095;
pub const OMAP3ISP_AF_THRESHOLD_MAX: c_int = 255;
pub const OMAP3ISP_AF_COEF_MAX: c_int = 4095;
pub const OMAP3ISP_AF_PAXEL_SIZE: c_int = 48;
pub const OMAP3ISP_AF_MAX_BUF_SIZE: c_int = 221184;
//
// struct omap3isp_h3a_aewb_config - AE AWB configuration reset values
// saturation_limit: Saturation limit.
// @win_height: Window Height. Range 2 - 256, even values only.
// @win_width: Window Width. Range 6 - 256, even values only.
// @ver_win_count: Vertical Window Count. Range 1 - 128.
// @hor_win_count: Horizontal Window Count. Range 1 - 36.
// @ver_win_start: Vertical Window Start. Range 0 - 4095.
// @hor_win_start: Horizontal Window Start. Range 0 - 4095.
// @blk_ver_win_start: Black Vertical Windows Start. Range 0 - 4095.
// @blk_win_height: Black Window Height. Range 2 - 256, even values only.
// @subsample_ver_inc: Subsample Vertical points increment Range 2 - 32, even
// values only.
// @subsample_hor_inc: Subsample Horizontal points increment Range 2 - 32, even
// values only.
// @alaw_enable: AEW ALAW EN flag.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_h3a_aewb_config {
//
// Common fields.
// They should be the first ones and must be in the same order as in
// ispstat_generic_config struct.
//
    pub buf_size: __u32,
    pub config_counter: __u16,
// Private fields
    pub saturation_limit: __u16,
    pub win_height: __u16,
    pub win_width: __u16,
    pub ver_win_count: __u16,
    pub hor_win_count: __u16,
    pub ver_win_start: __u16,
    pub hor_win_start: __u16,
    pub blk_ver_win_start: __u16,
    pub blk_win_height: __u16,
    pub subsample_ver_inc: __u16,
    pub subsample_hor_inc: __u16,
    pub alaw_enable: __u8,
}

//
// struct omap3isp_stat_data - Statistic data sent to or received from user
// @ts: Timestamp of returned framestats.
// @buf: Pointer to pass to user.
// @buf_size: Size of buffer.
// @frame_number: Frame number of requested stats.
// @cur_frame: Current frame number being processed.
// @config_counter: Number of the configuration associated with the data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_stat_data {

    pub tv_sec: __s64,
    pub tv_usec: __s64,
    pub ts: },

    pub ts: timeval,

    pub buf: *mut void __user,
    pub buf_size: __u32,
    pub frame_number: __u16,
    pub cur_frame: __u16,
    pub config_counter: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_stat_data_time32 {
    pub tv_sec: __s32,
    pub tv_usec: __s32,
    pub ts: },
    pub buf: __u32,
    pub buf_size: __u32,
    pub frame_number: __u16,
    pub cur_frame: __u16,
    pub config_counter: __u16,
}

// Histogram related structs
// Flags for number of bins
pub const OMAP3ISP_HIST_BINS_32: c_int = 0;
pub const OMAP3ISP_HIST_BINS_64: c_int = 1;
pub const OMAP3ISP_HIST_BINS_128: c_int = 2;
pub const OMAP3ISP_HIST_BINS_256: c_int = 3;
// Number of bins * 4 colors * 4-bytes word

pub const OMAP3ISP_HIST_MEM_SIZE: c_int = 1024;
pub const OMAP3ISP_HIST_MIN_REGIONS: c_int = 1;
pub const OMAP3ISP_HIST_MAX_REGIONS: c_int = 4;
pub const OMAP3ISP_HIST_MAX_WB_GAIN: c_int = 255;
pub const OMAP3ISP_HIST_MIN_WB_GAIN: c_int = 0;
pub const OMAP3ISP_HIST_MAX_BIT_WIDTH: c_int = 14;
pub const OMAP3ISP_HIST_MIN_BIT_WIDTH: c_int = 8;
pub const OMAP3ISP_HIST_MAX_WG: c_int = 4;
pub const OMAP3ISP_HIST_MAX_BUF_SIZE: c_int = 4096;
// Source
pub const OMAP3ISP_HIST_SOURCE_CCDC: c_int = 0;
pub const OMAP3ISP_HIST_SOURCE_MEM: c_int = 1;
// CFA pattern
pub const OMAP3ISP_HIST_CFA_BAYER: c_int = 0;
pub const OMAP3ISP_HIST_CFA_FOVEONX3: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_hist_region {
    pub h_start: __u16,
    pub h_end: __u16,
    pub v_start: __u16,
    pub v_end: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_hist_config {
//
// Common fields.
// They should be the first ones and must be in the same order as in
// ispstat_generic_config struct.
//
    pub buf_size: __u32,
    pub config_counter: __u16,
    pub and: *mut *mut __u8 num_acc_frames; / Num of image frames to be processed,
    pub /: *mut *mut __u16 hist_bins; / number of bins: 32, 64, 128, or 256,
    pub /: *mut *mut __u8 cfa; / BAYER or FOVEON X3,
    pub /: *mut *mut __u8 wg[OMAP3ISP_HIST_MAX_WG]; / White Balance Gain,
    pub /: *mut *mut __u8 num_regions; / number of regions to be configured,
    pub region: [omap3isp_hist_region; OMAP3ISP_HIST_MAX_REGIONS],
}

// Auto Focus related structs
pub const OMAP3ISP_AF_NUM_COEF: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap3isp_h3a_af_fvmode {
    OMAP3ISP_AF_MODE_SUMMED = 0,
    OMAP3ISP_AF_MODE_PEAK = 1
}

// Red, Green, and blue pixel location in the AF windows
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap3isp_h3a_af_rgbpos {
    OMAP3ISP_AF_GR_GB_BAYER = 0,	/* GR and GB as Bayer pattern */
    OMAP3ISP_AF_RG_GB_BAYER = 1,	/* RG and GB as Bayer pattern */
    OMAP3ISP_AF_GR_BG_BAYER = 2,	/* GR and BG as Bayer pattern */
    OMAP3ISP_AF_RG_BG_BAYER = 3,	/* RG and BG as Bayer pattern */
    OMAP3ISP_AF_GG_RB_CUSTOM = 4,	/* GG and RB as custom pattern */
    OMAP3ISP_AF_RB_GG_CUSTOM = 5	/* RB and GG as custom pattern */
}

// Contains the information regarding the Horizontal Median Filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_h3a_af_hmf {
    pub /: *mut *mut __u8 enable; / Status of Horizontal Median Filter,
    pub /: *mut *mut __u8 threshold; / Threshold Value for Horizontal Median Filter,
}

// Contains the information regarding the IIR Filters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_h3a_af_iir {
    pub /: *mut *mut __u16 h_start; / IIR horizontal start,
    pub /: *mut *mut __u16 coeff_set0[OMAP3ISP_AF_NUM_COEF]; / Filter coefficient, set 0,
    pub /: *mut *mut __u16 coeff_set1[OMAP3ISP_AF_NUM_COEF]; / Filter coefficient, set 1,
}

// Contains the information regarding the Paxels Structure in AF Engine
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_h3a_af_paxel {
    pub /: *mut *mut __u16 h_start; / Horizontal Start Position,
    pub /: *mut *mut __u16 v_start; / Vertical Start Position,
    pub /: *mut *mut __u8 width; / Width of the Paxel,
    pub /: *mut *mut __u8 height; / Height of the Paxel,
    pub /: *mut *mut __u8 h_cnt; / Horizontal Count,
    pub /: *mut *mut __u8 v_cnt; / vertical Count,
    pub /: *mut *mut __u8 line_inc; / Line Increment,
}

// Contains the parameters required for hardware set up of AF Engine
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_h3a_af_config {
//
// Common fields.
// They should be the first ones and must be in the same order as in
// ispstat_generic_config struct.
//
    pub buf_size: __u32,
    pub config_counter: __u16,
    pub /: *mut *mut omap3isp_h3a_af_hmf hmf; / HMF configurations,
    pub /: *mut *mut omap3isp_h3a_af_iir iir; / IIR filter configurations,
    pub /: *mut *mut omap3isp_h3a_af_paxel paxel; / Paxel parameters,
    pub /: *mut *mut omap3isp_h3a_af_rgbpos rgb_pos; / RGB Positions,
    pub /: *mut *mut omap3isp_h3a_af_fvmode fvmode; / Accumulator mode,
    pub /: *mut *mut __u8 alaw_enable; / AF ALAW status,
}

// ISP CCDC structs
// Abstraction layer CCDC configurations

pub const OMAP3ISP_RGB_MAX: c_int = 3;
// Enumeration constants for Alaw input width
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap3isp_alaw_ipwidth {
    OMAP3ISP_ALAW_BIT12_3 = 0x3,
    OMAP3ISP_ALAW_BIT11_2 = 0x4,
    OMAP3ISP_ALAW_BIT10_1 = 0x5,
    OMAP3ISP_ALAW_BIT9_0 = 0x6
}

//
// struct omap3isp_ccdc_lsc_config - LSC configuration
// @offset: Table Offset of the gain table.
// @gain_mode_n: Vertical dimension of a paxel in LSC configuration.
// @gain_mode_m: Horizontal dimension of a paxel in LSC configuration.
// @gain_format: Gain table format.
// @fmtsph: Start pixel horizontal from start of the HS sync pulse.
// @fmtlnh: Number of pixels in horizontal direction to use for the data
// reformatter.
// @fmtslv: Start line from start of VS sync pulse for the data reformatter.
// @fmtlnv: Number of lines in vertical direction for the data reformatter.
// @initial_x: X position, in pixels, of the first active pixel in reference
// to the first active paxel. Must be an even number.
// @initial_y: Y position, in pixels, of the first active pixel in reference
// to the first active paxel. Must be an even number.
// @size: Size of LSC gain table. Filled when loaded from userspace.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_ccdc_lsc_config {
    pub offset: __u16,
    pub gain_mode_n: __u8,
    pub gain_mode_m: __u8,
    pub gain_format: __u8,
    pub fmtsph: __u16,
    pub fmtlnh: __u16,
    pub fmtslv: __u16,
    pub fmtlnv: __u16,
    pub initial_x: __u8,
    pub initial_y: __u8,
    pub size: __u32,
}

//
// struct omap3isp_ccdc_bclamp - Optical & Digital black clamp subtract
// @obgain: Optical black average gain.
// @obstpixel: Start Pixel w.r.t. HS pulse in Optical black sample.
// @oblines: Optical Black Sample lines.
// @oblen: Optical Black Sample Length.
// @dcsubval: Digital Black Clamp subtract value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_ccdc_bclamp {
    pub obgain: __u8,
    pub obstpixel: __u8,
    pub oblines: __u8,
    pub oblen: __u8,
    pub dcsubval: __u16,
}

//
// struct omap3isp_ccdc_fpc - Faulty Pixels Correction
// @fpnum: Number of faulty pixels to be corrected in the frame.
// @fpcaddr: Memory address of the FPC Table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_ccdc_fpc {
    pub fpnum: __u16,
    pub fpcaddr: __u32,
}

//
// struct omap3isp_ccdc_blcomp - Black Level Compensation parameters
// @b_mg: B/Mg pixels. 2's complement. -128 to +127.
// @gb_g: Gb/G pixels. 2's complement. -128 to +127.
// @gr_cy: Gr/Cy pixels. 2's complement. -128 to +127.
// @r_ye: R/Ye pixels. 2's complement. -128 to +127.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_ccdc_blcomp {
    pub b_mg: __u8,
    pub gb_g: __u8,
    pub gr_cy: __u8,
    pub r_ye: __u8,
}

//
// omap3isp_ccdc_culling - Culling parameters
// @v_pattern: Vertical culling pattern.
// @h_odd: Horizontal Culling pattern for odd lines.
// @h_even: Horizontal Culling pattern for even lines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_ccdc_culling {
    pub v_pattern: __u8,
    pub h_odd: __u16,
    pub h_even: __u16,
}

//
// omap3isp_ccdc_update_config - CCDC configuration
// @update: Specifies which CCDC registers should be updated.
// @flag: Specifies which CCDC functions should be enabled.
// @alawip: Enable/Disable A-Law compression.
// @bclamp: Black clamp control register.
// @blcomp: Black level compensation value for RGrGbB Pixels. 2's complement.
// @fpc: Number of faulty pixels corrected in the frame, address of FPC table.
// @cull: Cull control register.
// @lsc: Pointer to LSC gain table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_ccdc_update_config {
    pub update: __u16,
    pub flag: __u16,
    pub alawip: omap3isp_alaw_ipwidth,
    pub bclamp: *mut omap3isp_ccdc_bclamp __user,
    pub blcomp: *mut omap3isp_ccdc_blcomp __user,
    pub fpc: *mut omap3isp_ccdc_fpc __user,
    pub lsc_cfg: *mut omap3isp_ccdc_lsc_config __user,
    pub cull: *mut omap3isp_ccdc_culling __user,
    pub lsc: *mut __u8 __user,
}

// Preview configurations

// Bit 11 was OMAP3ISP_PREV_GAMMABYPASS, now merged with OMAP3ISP_PREV_GAMMA

pub const OMAP3ISP_PREV_NF_TBL_SIZE: c_int = 64;
pub const OMAP3ISP_PREV_CFA_TBL_SIZE: c_int = 576;

pub const OMAP3ISP_PREV_GAMMA_TBL_SIZE: c_int = 1024;
pub const OMAP3ISP_PREV_YENH_TBL_SIZE: c_int = 128;
pub const OMAP3ISP_PREV_DETECT_CORRECT_CHANNELS: c_int = 4;
//
// struct omap3isp_prev_hmed - Horizontal Median Filter
// @odddist: Distance between consecutive pixels of same color in the odd line.
// @evendist: Distance between consecutive pixels of same color in the even
// line.
// @thres: Horizontal median filter threshold.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_hmed {
    pub odddist: __u8,
    pub evendist: __u8,
    pub thres: __u8,
}

//
// Enumeration for CFA Formats supported by preview
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap3isp_cfa_fmt {
    OMAP3ISP_CFAFMT_BAYER,
    OMAP3ISP_CFAFMT_SONYVGA,
    OMAP3ISP_CFAFMT_RGBFOVEON,
    OMAP3ISP_CFAFMT_DNSPL,
    OMAP3ISP_CFAFMT_HONEYCOMB,
    OMAP3ISP_CFAFMT_RRGGBBFOVEON
}

//
// struct omap3isp_prev_cfa - CFA Interpolation
// @format: CFA Format Enum value supported by preview.
// @gradthrs_vert: CFA Gradient Threshold - Vertical.
// @gradthrs_horz: CFA Gradient Threshold - Horizontal.
// @table: Pointer to the CFA table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_cfa {
    pub format: omap3isp_cfa_fmt,
    pub gradthrs_vert: __u8,
    pub gradthrs_horz: __u8,
    pub table: [__u32; 4][OMAP3ISP_PREV_CFA_BLK_SIZE],
}

//
// struct omap3isp_prev_csup - Chrominance Suppression
// @gain: Gain.
// @thres: Threshold.
// @hypf_en: Flag to enable/disable the High Pass Filter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_csup {
    pub gain: __u8,
    pub thres: __u8,
    pub hypf_en: __u8,
}

//
// struct omap3isp_prev_wbal - White Balance
// @dgain: Digital gain (U10Q8).
// @coef3: White balance gain - COEF 3 (U8Q5).
// @coef2: White balance gain - COEF 2 (U8Q5).
// @coef1: White balance gain - COEF 1 (U8Q5).
// @coef0: White balance gain - COEF 0 (U8Q5).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_wbal {
    pub dgain: __u16,
    pub coef3: __u8,
    pub coef2: __u8,
    pub coef1: __u8,
    pub coef0: __u8,
}

//
// struct omap3isp_prev_blkadj - Black Level Adjustment
// @red: Black level offset adjustment for Red in 2's complement format
// @green: Black level offset adjustment for Green in 2's complement format
// @blue: Black level offset adjustment for Blue in 2's complement format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_blkadj {
// Black level offset adjustment for Red in 2's complement format
    pub red: __u8,
// Black level offset adjustment for Green in 2's complement format
    pub green: __u8,
// Black level offset adjustment for Blue in 2's complement format
    pub blue: __u8,
}

//
// struct omap3isp_prev_rgbtorgb - RGB to RGB Blending
// @matrix: Blending values(S12Q8 format)
// [RR] [GR] [BR]
// [RG] [GG] [BG]
// [RB] [GB] [BB]
// @offset: Blending offset value for R,G,B in 2's complement integer format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_rgbtorgb {
    pub matrix: [__u16; OMAP3ISP_RGB_MAX][OMAP3ISP_RGB_MAX],
    pub offset: [__u16; OMAP3ISP_RGB_MAX],
}

//
// struct omap3isp_prev_csc - Color Space Conversion from RGB-YCbYCr
// @matrix: Color space conversion coefficients(S10Q8)
// [CSCRY]  [CSCGY]  [CSCBY]
// [CSCRCB] [CSCGCB] [CSCBCB]
// [CSCRCR] [CSCGCR] [CSCBCR]
// @offset: CSC offset values for Y offset, CB offset and CR offset respectively
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_csc {
    pub matrix: [__u16; OMAP3ISP_RGB_MAX][OMAP3ISP_RGB_MAX],
    pub offset: [__s16; OMAP3ISP_RGB_MAX],
}

//
// struct omap3isp_prev_yclimit - Y, C Value Limit
// @minC: Minimum C value
// @maxC: Maximum C value
// @minY: Minimum Y value
// @maxY: Maximum Y value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_yclimit {
    pub minC: __u8,
    pub maxC: __u8,
    pub minY: __u8,
    pub maxY: __u8,
}

//
// struct omap3isp_prev_dcor - Defect correction
// @couplet_mode_en: Flag to enable or disable the couplet dc Correction in NF
// @detect_correct: Thresholds for correction bit 0:10 detect 16:25 correct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_dcor {
    pub couplet_mode_en: __u8,
    pub detect_correct: [__u32; OMAP3ISP_PREV_DETECT_CORRECT_CHANNELS],
}

//
// struct omap3isp_prev_nf - Noise Filter
// @spread: Spread value to be used in Noise Filter
// @table: Pointer to the Noise Filter table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_nf {
    pub spread: __u8,
    pub table: [__u32; OMAP3ISP_PREV_NF_TBL_SIZE],
}

//
// struct omap3isp_prev_gtables - Gamma correction tables
// @red: Array for red gamma table.
// @green: Array for green gamma table.
// @blue: Array for blue gamma table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_gtables {
    pub red: [__u32; OMAP3ISP_PREV_GAMMA_TBL_SIZE],
    pub green: [__u32; OMAP3ISP_PREV_GAMMA_TBL_SIZE],
    pub blue: [__u32; OMAP3ISP_PREV_GAMMA_TBL_SIZE],
}

//
// struct omap3isp_prev_luma - Luma enhancement
// @table: Array for luma enhancement table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_luma {
    pub table: [__u32; OMAP3ISP_PREV_YENH_TBL_SIZE],
}

//
// struct omap3isp_prev_update_config - Preview engine configuration (user)
// @update: Specifies which ISP Preview registers should be updated.
// @flag: Specifies which ISP Preview functions should be enabled.
// @shading_shift: 3bit value of shift used in shading compensation.
// @luma: Pointer to luma enhancement structure.
// @hmed: Pointer to structure containing the odd and even distance.
// between the pixels in the image along with the filter threshold.
// @cfa: Pointer to structure containing the CFA interpolation table, CFA.
// format in the image, vertical and horizontal gradient threshold.
// @csup: Pointer to Structure for Chrominance Suppression coefficients.
// @wbal: Pointer to structure for White Balance.
// @blkadj: Pointer to structure for Black Adjustment.
// @rgb2rgb: Pointer to structure for RGB to RGB Blending.
// @csc: Pointer to structure for Color Space Conversion from RGB-YCbYCr.
// @yclimit: Pointer to structure for Y, C Value Limit.
// @dcor: Pointer to structure for defect correction.
// @nf: Pointer to structure for Noise Filter
// @gamma: Pointer to gamma structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap3isp_prev_update_config {
    pub update: __u32,
    pub flag: __u32,
    pub shading_shift: __u32,
    pub luma: *mut omap3isp_prev_luma __user,
    pub hmed: *mut omap3isp_prev_hmed __user,
    pub cfa: *mut omap3isp_prev_cfa __user,
    pub csup: *mut omap3isp_prev_csup __user,
    pub wbal: *mut omap3isp_prev_wbal __user,
    pub blkadj: *mut omap3isp_prev_blkadj __user,
    pub rgb2rgb: *mut omap3isp_prev_rgbtorgb __user,
    pub csc: *mut omap3isp_prev_csc __user,
    pub yclimit: *mut omap3isp_prev_yclimit __user,
    pub dcor: *mut omap3isp_prev_dcor __user,
    pub nf: *mut omap3isp_prev_nf __user,
    pub gamma: *mut omap3isp_prev_gtables __user,
}
