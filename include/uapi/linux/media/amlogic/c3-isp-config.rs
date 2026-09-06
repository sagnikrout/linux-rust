//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/media/amlogic/c3-isp-config.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Copyright (C) 2024 Amlogic, Inc. All rights reserved
//

//
// Frames are split into zones of almost equal width and height - a zone is a
// rectangular tile of a frame. The metering blocks within the ISP collect
// aggregated statistics per zone.
//

// The maximum number of point on the diagonal of the frame for statistics
pub const C3_ISP_AE_MAX_PT_NUM: c_int = 18;
pub const C3_ISP_AF_MAX_PT_NUM: c_int = 18;
pub const C3_ISP_AWB_MAX_PT_NUM: c_int = 33;
//
// struct c3_isp_awb_zone_stats - AWB statistics of a zone
//
// AWB zone stats is aligned with 8 bytes
//
// @rg: the ratio of R / G in a zone
// @bg: the ratio of B / G in a zone
// @pixel_sum: the total number of pixels used in a zone
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_awb_zone_stats {
    pub rg: __u16,
    pub bg: __u16,
    pub pixel_sum: __u32,
}

//
// struct c3_isp_awb_stats - Auto white balance statistics information.
//
// AWB statistical information of all zones.
//
// @stats: array of auto white balance statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_awb_stats {
    pub stats: [c3_isp_awb_zone_stats; C3_ISP_AWB_MAX_ZONES],
    pub __attribute__((aligned(16))): },
//
// struct c3_isp_ae_zone_stats - AE statistics of a zone
//
// AE zone stats is aligned with 8 bytes.
// This is a 5-bin histogram and the total sum is normalized to 0xffff.
// So hist2 = 0xffff - (hist0 + hist1 + hist3 + hist4)
//
// @hist0: the global normalized pixel count for bin 0
// @hist1: the global normalized pixel count for bin 1
// @hist3: the global normalized pixel count for bin 3
// @hist4: the global normalized pixel count for bin 4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_ae_zone_stats {
    pub hist0: __u16,
    pub hist1: __u16,
    pub hist3: __u16,
    pub hist4: __u16,
}

//
// struct c3_isp_ae_stats - Exposure statistics information
//
// AE statistical information consists of all blocks information and a 1024-bin
// histogram.
//
// @stats: array of auto exposure block statistics
// @reserved: undefined buffer space
// @hist: a 1024-bin histogram for the entire image
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_ae_stats {
    pub stats: [c3_isp_ae_zone_stats; C3_ISP_AE_MAX_ZONES],
    pub reserved: [__u32; 2],
    pub hist: [__u32; 1024],
    pub __attribute__((aligned(16))): },
//
// struct c3_isp_af_zone_stats - AF statistics of a zone
//
// AF zone stats is aligned with 8 bytes.
// The zonal accumulated contrast metrics are stored in floating point format
// with 16 bits mantissa and 5 or 6 bits exponent. Apart from contrast metrics
// we accumulate squared image and quartic image data over the zone.
//
// @i2_mat: the mantissa of zonal squared image pixel sum
// @i4_mat: the mantissa of zonal quartic image pixel sum
// @e4_mat: the mantissa of zonal multi-directional quartic edge sum
// @e4_exp: the exponent of zonal multi-directional quartic edge sum
// @i2_exp: the exponent of zonal squared image pixel sum
// @i4_exp: the exponent of zonal quartic image pixel sum
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_af_zone_stats {
    pub i2_mat: __u16,
    pub i4_mat: __u16,
    pub e4_mat: __u16,
    pub 5: __u16 e4_exp :,
    pub 5: __u16 i2_exp :,
    pub 6: __u16 i4_exp :,
}

//
// struct c3_isp_af_stats - Auto Focus statistics information
//
// AF statistical information of each zone
//
// @stats: array of auto focus block statistics
// @reserved: undefined buffer space
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_af_stats {
    pub stats: [c3_isp_af_zone_stats; C3_ISP_AF_MAX_ZONES],
    pub reserved: [__u32; 2],
    pub __attribute__((aligned(16))): },
//
// struct c3_isp_stats_info - V4L2_META_FMT_C3ISP_STATS
//
// Contains ISP statistics
//
// @awb: auto white balance stats
// @ae: auto exposure stats
// @af: auto focus stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_stats_info {
    pub awb: c3_isp_awb_stats,
    pub ae: c3_isp_ae_stats,
    pub af: c3_isp_af_stats,
}

//
// enum c3_isp_params_buffer_version -  C3 ISP parameters block versioning
//
// @C3_ISP_PARAMS_BUFFER_V0: First version of C3 ISP parameters block
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c3_isp_params_buffer_version {
    C3_ISP_PARAMS_BUFFER_V0 = V4L2_ISP_PARAMS_VERSION_V0,
}

//
// enum c3_isp_params_block_type - Enumeration of C3 ISP parameter blocks
//
// Each block configures a specific processing block of the C3 ISP.
// The block type allows the driver to correctly interpret the parameters block
// data.
//
// @C3_ISP_PARAMS_BLOCK_AWB_GAINS: White balance gains
// @C3_ISP_PARAMS_BLOCK_AWB_CONFIG: AWB statistic format configuration for all
// blocks that control how stats are generated
// @C3_ISP_PARAMS_BLOCK_AE_CONFIG: AE statistic format configuration for all
// blocks that control how stats are generated
// @C3_ISP_PARAMS_BLOCK_AF_CONFIG: AF statistic format configuration for all
// blocks that control how stats are generated
// @C3_ISP_PARAMS_BLOCK_PST_GAMMA: post gamma parameters
// @C3_ISP_PARAMS_BLOCK_CCM: Color correction matrix parameters
// @C3_ISP_PARAMS_BLOCK_CSC: Color space conversion parameters
// @C3_ISP_PARAMS_BLOCK_BLC: Black level correction parameters
// @C3_ISP_PARAMS_BLOCK_SENTINEL: First non-valid block index
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c3_isp_params_block_type {
    C3_ISP_PARAMS_BLOCK_AWB_GAINS,
    C3_ISP_PARAMS_BLOCK_AWB_CONFIG,
    C3_ISP_PARAMS_BLOCK_AE_CONFIG,
    C3_ISP_PARAMS_BLOCK_AF_CONFIG,
    C3_ISP_PARAMS_BLOCK_PST_GAMMA,
    C3_ISP_PARAMS_BLOCK_CCM,
    C3_ISP_PARAMS_BLOCK_CSC,
    C3_ISP_PARAMS_BLOCK_BLC,
    C3_ISP_PARAMS_BLOCK_SENTINEL
}

// For backward compatibility

//
// c3_isp_params_block_header - C3 ISP parameter block header
//
// This structure represents the common part of all the ISP configuration
// blocks and is identical to :c:type:`v4l2_isp_params_block_header`.
//
// The type field is one of the values enumerated by
// :c:type:`c3_isp_params_block_type` and specifies how the data should be
// interpreted by the driver.
//
// The flags field is a bitmask of per-block flags C3_ISP_PARAMS_FL_*.
//

//
// struct c3_isp_params_awb_gains - Gains for auto-white balance
//
// This struct allows users to configure the gains for white balance.
// There are four gain settings corresponding to each colour channel in
// the bayer domain. All of the gains are stored in Q4.8 format.
//
// header.type should be set to C3_ISP_PARAMS_BLOCK_AWB_GAINS
// from :c:type:`c3_isp_params_block_type`
//
// @header: The C3 ISP parameters block header
// @gr_gain: Multiplier for Gr channel (Q4.8 format)
// @r_gain: Multiplier for R channel (Q4.8 format)
// @b_gain: Multiplier for B channel (Q4.8 format)
// @gb_gain: Multiplier for Gb channel (Q4.8 format)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_params_awb_gains {
    pub header: c3_isp_params_block_header,
    pub gr_gain: __u16,
    pub r_gain: __u16,
    pub b_gain: __u16,
    pub gb_gain: __u16,
    pub __attribute__((aligned(8))): },
//
// enum c3_isp_params_awb_tap_points - Tap points for the AWB statistics
// @C3_ISP_AWB_STATS_TAP_OFE: immediately after the optical frontend block
// @C3_ISP_AWB_STATS_TAP_GE: immediately after the green equal block
// @C3_ISP_AWB_STATS_TAP_BEFORE_WB: immediately before the white balance block
// @C3_ISP_AWB_STATS_TAP_AFTER_WB: immediately after the white balance block
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c3_isp_params_awb_tap_points {
    C3_ISP_AWB_STATS_TAP_OFE = 0,
    C3_ISP_AWB_STATS_TAP_GE,
    C3_ISP_AWB_STATS_TAP_BEFORE_WB,
    C3_ISP_AWB_STATS_TAP_AFTER_WB,
}

//
// struct c3_isp_params_awb_config - Stats settings for auto-white balance
//
// This struct allows the configuration of the statistics generated for auto
// white balance.
//
// header.type should be set to C3_ISP_PARAMS_BLOCK_AWB_CONFIG
// from :c:type:`c3_isp_params_block_type`
//
// @header: the C3 ISP parameters block header
// @tap_point: the tap point from enum c3_isp_params_awb_tap_point
// @satur_vald: AWB statistic over saturation control
// value: 0: disable, 1: enable
// @horiz_zones_num: active number of hotizontal zones [0..32]
// @vert_zones_num: active number of vertical zones [0..24]
// @rg_min: minimum R/G ratio (Q4.8 format)
// @rg_max: maximum R/G ratio (Q4.8 format)
// @bg_min: minimum B/G ratio (Q4.8 format)
// @bg_max: maximum B/G ratio (Q4.8 format)
// @rg_low: R/G ratio trim low (Q4.8 format)
// @rg_high: R/G ratio trim hight (Q4.8 format)
// @bg_low: B/G ratio trim low (Q4.8 format)
// @bg_high: B/G ratio trim high (Q4.8 format)
// @zone_weight: array of weights for AWB statistics zones [0..15]
// @horiz_coord: the horizontal coordinate of points on the diagonal [0..2888]
// @vert_coord: the vertical coordinate of points on the diagonal [0..2240]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_params_awb_config {
    pub header: c3_isp_params_block_header,
    pub tap_point: __u8,
    pub satur_vald: __u8,
    pub horiz_zones_num: __u8,
    pub vert_zones_num: __u8,
    pub rg_min: __u16,
    pub rg_max: __u16,
    pub bg_min: __u16,
    pub bg_max: __u16,
    pub rg_low: __u16,
    pub rg_high: __u16,
    pub bg_low: __u16,
    pub bg_high: __u16,
    pub zone_weight: [__u8; C3_ISP_AWB_MAX_ZONES],
    pub horiz_coord: [__u16; C3_ISP_AWB_MAX_PT_NUM],
    pub vert_coord: [__u16; C3_ISP_AWB_MAX_PT_NUM],
    pub __attribute__((aligned(8))): },
//
// enum c3_isp_params_ae_tap_points - Tap points for the AE statistics
// @C3_ISP_AE_STATS_TAP_GE: immediately after the green equal block
// @C3_ISP_AE_STATS_TAP_MLS: immediately after the mesh lens shading block
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c3_isp_params_ae_tap_points {
    C3_ISP_AE_STATS_TAP_GE = 0,
    C3_ISP_AE_STATS_TAP_MLS,
}

//
// struct c3_isp_params_ae_config - Stats settings for auto-exposure
//
// This struct allows the configuration of the statistics generated for
// auto exposure.
//
// header.type should be set to C3_ISP_PARAMS_BLOCK_AE_CONFIG
// from :c:type:`c3_isp_params_block_type`
//
// @header: the C3 ISP parameters block header
// @horiz_zones_num: active number of horizontal zones [0..17]
// @vert_zones_num: active number of vertical zones [0..15]
// @tap_point: the tap point from enum c3_isp_params_ae_tap_point
// @zone_weight: array of weights for AE statistics zones [0..15]
// @horiz_coord: the horizontal coordinate of points on the diagonal [0..2888]
// @vert_coord: the vertical coordinate of points on the diagonal [0..2240]
// @reserved: applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_params_ae_config {
    pub header: c3_isp_params_block_header,
    pub tap_point: __u8,
    pub horiz_zones_num: __u8,
    pub vert_zones_num: __u8,
    pub zone_weight: [__u8; C3_ISP_AE_MAX_ZONES],
    pub horiz_coord: [__u16; C3_ISP_AE_MAX_PT_NUM],
    pub vert_coord: [__u16; C3_ISP_AE_MAX_PT_NUM],
    pub reserved: [__u16; 3],
    pub __attribute__((aligned(8))): },
//
// enum c3_isp_params_af_tap_points - Tap points for the AF statistics
// @C3_ISP_AF_STATS_TAP_SNR: immediately after the spatial noise reduce block
// @C3_ISP_AF_STATS_TAP_DMS: immediately after the demosaic block
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c3_isp_params_af_tap_points {
    C3_ISP_AF_STATS_TAP_SNR = 0,
    C3_ISP_AF_STATS_TAP_DMS,
}

//
// struct c3_isp_params_af_config - Stats settings for auto-focus
//
// This struct allows the configuration of the statistics generated for
// auto focus.
//
// header.type should be set to C3_ISP_PARAMS_BLOCK_AF_CONFIG
// from :c:type:`c3_isp_params_block_type`
//
// @header: the C3 ISP parameters block header
// @tap_point: the tap point from enum c3_isp_params_af_tap_point
// @horiz_zones_num: active number of hotizontal zones [0..17]
// @vert_zones_num: active number of vertical zones [0..15]
// @reserved: applications must zero this array
// @horiz_coord: the horizontal coordinate of points on the diagonal [0..2888]
// @vert_coord: the vertical coordinate of points on the diagonal [0..2240]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_params_af_config {
    pub header: c3_isp_params_block_header,
    pub tap_point: __u8,
    pub horiz_zones_num: __u8,
    pub vert_zones_num: __u8,
    pub reserved: [__u8; 5],
    pub horiz_coord: [__u16; C3_ISP_AF_MAX_PT_NUM],
    pub vert_coord: [__u16; C3_ISP_AF_MAX_PT_NUM],
    pub __attribute__((aligned(8))): },
//
// struct c3_isp_params_pst_gamma - Post gamma configuration
//
// This struct allows the configuration of the look up table for
// post gamma. The gamma curve consists of 129 points, so need to
// set lut[129].
//
// header.type should be set to C3_ISP_PARAMS_BLOCK_PST_GAMMA
// from :c:type:`c3_isp_params_block_type`
//
// @header: the C3 ISP parameters block header
// @lut: lookup table for P-Stitch gamma [0..1023]
// @reserved: applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_params_pst_gamma {
    pub header: c3_isp_params_block_header,
    pub lut: [__u16; 129],
    pub reserved: [__u16; 3],
    pub __attribute__((aligned(8))): },
//
// struct c3_isp_params_ccm - ISP CCM configuration
//
// This struct allows the configuration of the matrix for
// color correction. The matrix consists of 3 x 3 points,
// so need to set matrix[3][3].
//
// header.type should be set to C3_ISP_PARAMS_BLOCK_CCM
// from :c:type:`c3_isp_params_block_type`
//
// @header: the C3 ISP parameters block header
// @matrix: a 3 x 3 matrix used for color correction,
// the value of matrix[x][y] is orig_value x 256. [-4096..4095]
// @reserved: applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_params_ccm {
    pub header: c3_isp_params_block_header,
    pub matrix: [__s16; 3][3],
    pub reserved: [__u16; 3],
    pub __attribute__((aligned(8))): },
//
// struct c3_isp_params_csc - ISP Color Space Conversion configuration
//
// This struct allows the configuration of the matrix for color space
// conversion. The matrix consists of 3 x 3 points, so need to set matrix[3][3].
//
// header.type should be set to C3_ISP_PARAMS_BLOCK_CSC
// from :c:type:`c3_isp_params_block_type`
//
// @header: the C3 ISP parameters block header
// @matrix: a 3x3 matrix used for the color space conversion,
// the value of matrix[x][y] is orig_value x 256. [-4096..4095]
// @reserved: applications must zero this array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_params_csc {
    pub header: c3_isp_params_block_header,
    pub matrix: [__s16; 3][3],
    pub reserved: [__u16; 3],
    pub __attribute__((aligned(8))): },
//
// struct c3_isp_params_blc - ISP Black Level Correction configuration
//
// This struct allows the configuration of the block level offset for each
// color channel.
//
// header.type should be set to C3_ISP_PARAMS_BLOCK_BLC
// from :c:type:`c3_isp_params_block_type`
//
// @header: the C3 ISP parameters block header
// @gr_ofst: Gr blc offset (Q4.12 format)
// @r_ofst: R blc offset (Q4.12 format)
// @b_ofst: B blc offset (Q4.12 format)
// @gb_ofst: Gb blc offset(Q4.12 format)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_params_blc {
    pub header: c3_isp_params_block_header,
    pub gr_ofst: __u16,
    pub r_ofst: __u16,
    pub b_ofst: __u16,
    pub gb_ofst: __u16,
}

//
// define C3_ISP_PARAMS_MAX_SIZE - Maximum size of all C3 ISP Parameters
//
// Though the parameters for the C3 ISP are passed as optional blocks, the
// driver still needs to know the absolute maximum size so that it can allocate
// a buffer sized appropriately to accommodate userspace attempting to set all
// possible parameters in a single frame.
//

//
// struct c3_isp_params_cfg - C3 ISP configuration parameters
//
// This is the driver-specific implementation of
// :c:type:`v4l2_isp_params_buffer`.
//
// Currently only C3_ISP_PARAM_BUFFER_V0 is supported.
//
// The expected memory layout of the parameters buffer is::
//
// +-------------------- struct c3_isp_params_cfg ---- ------------------+
// | version = C3_ISP_PARAM_BUFFER_V0;                                   |
// | data_size = sizeof(struct c3_isp_params_awb_gains) +                |
// |              sizeof(struct c3_isp_params_awb_config);       |
// | +------------------------- data  ---------------------------------+ |
// | | +------------ struct c3_isp_params_awb_gains) ------------------+ |
// | | | +---------  struct c3_isp_params_block_header header -----+ | | |
// | | | | type = C3_ISP_PARAMS_BLOCK_AWB_GAINS;                   | | | |
// | | | | flags = C3_ISP_PARAMS_BLOCK_FL_NONE;                    | | | |
// | | | | size = sizeof(struct c3_isp_params_awb_gains);          | | | |
// | | | +---------------------------------------------------------+ | | |
// | | | gr_gain = ...;                                              | | |
// | | | r_gain = ...;                                               | | |
// | | | b_gain = ...;                                               | | |
// | | | gb_gain = ...;                                              | | |
// | | +------------------ struct c3_isp_params_awb_config ----------+ | |
// | | | +---------- struct c3_isp_param_block_header header ------+ | | |
// | | | | type = C3_ISP_PARAMS_BLOCK_AWB_CONFIG;                  | | | |
// | | | | flags = C3_ISP_PARAMS_BLOCK_FL_NONE;                    | | | |
// | | | | size = sizeof(struct c3_isp_params_awb_config)          | | | |
// | | | +---------------------------------------------------------+ | | |
// | | | tap_point = ...;                                            | | |
// | | | satur_vald = ...;                                           | | |
// | | | horiz_zones_num = ...;                                      | | |
// | | | vert_zones_num = ...;                                       | | |
// | | +-------------------------------------------------------------+ | |
// | +-----------------------------------------------------------------+ |
// +---------------------------------------------------------------------+
//
// @version: The C3 ISP parameters buffer version
// @data_size: The C3 ISP configuration data effective size, excluding this
// header
// @data: The C3 ISP configuration blocks data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_params_cfg {
    pub version: __u32,
    pub data_size: __u32,
    pub data: [__u8; C3_ISP_PARAMS_MAX_SIZE],
}

// Make sure the header is type-convertible to the generic v4l2 params one

