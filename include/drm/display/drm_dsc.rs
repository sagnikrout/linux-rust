//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/display/drm_dsc.h
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


// SPDX-License-Identifier: MIT
// Copyright (C) 2018 Intel Corp.
//
// Authors:
// Manasi Navare <manasi.d.navare@intel.com>
//

// VESA Display Stream Compression DSC 1.2 constants
pub const DSC_NUM_BUF_RANGES: c_int = 15;
pub const DSC_MUX_WORD_SIZE_8_10_BPC: c_int = 48;
pub const DSC_MUX_WORD_SIZE_12_BPC: c_int = 64;
pub const DSC_RC_PIXELS_PER_GROUP: c_int = 3;
pub const DSC_SCALE_DECREMENT_INTERVAL_MAX: c_int = 4095;
pub const DSC_RANGE_BPG_OFFSET_MASK: c_uint = 0x3f;
// DSC Rate Control Constants
pub const DSC_RC_MODEL_SIZE_CONST: c_int = 8192;
pub const DSC_RC_EDGE_FACTOR_CONST: c_int = 6;
pub const DSC_RC_TGT_OFFSET_HI_CONST: c_int = 3;
pub const DSC_RC_TGT_OFFSET_LO_CONST: c_int = 3;
// DSC PPS constants and macros
pub const DSC_PPS_VERSION_MAJOR_SHIFT: c_int = 4;
pub const DSC_PPS_BPC_SHIFT: c_int = 4;
pub const DSC_PPS_MSB_SHIFT: c_int = 8;

pub const DSC_PPS_VBR_EN_SHIFT: c_int = 2;
pub const DSC_PPS_SIMPLE422_SHIFT: c_int = 3;
pub const DSC_PPS_CONVERT_RGB_SHIFT: c_int = 4;
pub const DSC_PPS_BLOCK_PRED_EN_SHIFT: c_int = 5;

pub const DSC_PPS_RC_TGT_OFFSET_HI_SHIFT: c_int = 4;
pub const DSC_PPS_RC_RANGE_MINQP_SHIFT: c_int = 11;
pub const DSC_PPS_RC_RANGE_MAXQP_SHIFT: c_int = 6;
pub const DSC_PPS_NATIVE_420_SHIFT: c_int = 1;
//
// struct drm_dsc_rc_range_parameters - DSC Rate Control range parameters
//
// This defines different rate control parameters used by the DSC engine
// to compress the frame.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dsc_rc_range_parameters {
//
// @range_min_qp: Min Quantization Parameters allowed for this range
//
    pub range_min_qp: u8,
//
// @range_max_qp: Max Quantization Parameters allowed for this range
//
    pub range_max_qp: u8,
//
// @range_bpg_offset:
// Bits/group offset to apply to target for this group
//
    pub range_bpg_offset: u8,
}

//
// struct drm_dsc_config - Parameters required to configure DSC
//
// Driver populates this structure with all the parameters required
// to configure the display stream compression on the source.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dsc_config {
//
// @line_buf_depth:
// Bits per component for previous reconstructed line buffer
//
    pub line_buf_depth: u8,
//
// @bits_per_component: Bits per component to code (8/10/12)
//
    pub bits_per_component: u8,
//
// @convert_rgb:
// Flag to indicate if RGB - YCoCg conversion is needed
// True if RGB input, False if YCoCg input
//
    pub convert_rgb: bool,
//
// @slice_count: Number fo slices per line used by the DSC encoder
//
    pub slice_count: u8,
//
// @slice_width: Width of each slice in pixels
//
    pub slice_width: u16,
//
// @slice_height: Slice height in pixels
//
    pub slice_height: u16,
//
// @simple_422: True if simple 4_2_2 mode is enabled else False
//
    pub simple_422: bool,
//
// @pic_width: Width of the input display frame in pixels
//
    pub pic_width: u16,
//
// @pic_height: Vertical height of the input display frame
//
    pub pic_height: u16,
//
// @rc_tgt_offset_high:
// Offset to bits/group used by RC to determine QP adjustment
//
    pub rc_tgt_offset_high: u8,
//
// @rc_tgt_offset_low:
// Offset to bits/group used by RC to determine QP adjustment
//
    pub rc_tgt_offset_low: u8,
//
// @bits_per_pixel:
// Target bits per pixel with 4 fractional bits, bits_per_pixel << 4
//
    pub bits_per_pixel: u16,
//
// @rc_edge_factor:
// Factor to determine if an edge is present based on the bits produced
//
    pub rc_edge_factor: u8,
//
// @rc_quant_incr_limit1:
// Slow down incrementing once the range reaches this value
//
    pub rc_quant_incr_limit1: u8,
//
// @rc_quant_incr_limit0:
// Slow down incrementing once the range reaches this value
//
    pub rc_quant_incr_limit0: u8,
//
// @initial_xmit_delay:
// Number of pixels to delay the initial transmission
//
    pub initial_xmit_delay: u16,
//
// @initial_dec_delay:
// Initial decoder delay, number of pixel times that the decoder
// accumulates data in its rate buffer before starting to decode
// and output pixels.
//
    pub initial_dec_delay: u16,
//
// @block_pred_enable:
// True if block prediction is used to code any groups within the
// picture. False if BP not used
//
    pub block_pred_enable: bool,
//
// @first_line_bpg_offset:
// Number of additional bits allocated for each group on the first
// line of slice.
//
    pub first_line_bpg_offset: u8,
//
// @initial_offset: Value to use for RC model offset at slice start
//
    pub initial_offset: u16,
//
// @rc_buf_thresh: Thresholds defining each of the buffer ranges
//
    pub 1]: u16 rc_buf_thresh[DSC_NUM_BUF_RANGES -,
//
// @rc_range_params:
// Parameters for each of the RC ranges defined in
// &struct drm_dsc_rc_range_parameters
//
    pub rc_range_params: [drm_dsc_rc_range_parameters; DSC_NUM_BUF_RANGES],
//
// @rc_model_size: Total size of RC model
//
    pub rc_model_size: u16,
//
// @flatness_min_qp: Minimum QP where flatness information is sent
//
    pub flatness_min_qp: u8,
//
// @flatness_max_qp: Maximum QP where flatness information is sent
//
    pub flatness_max_qp: u8,
//
// @initial_scale_value: Initial value for the scale factor
//
    pub initial_scale_value: u8,
//
// @scale_decrement_interval:
// Specifies number of group times between decrementing the scale factor
// at beginning of a slice.
//
    pub scale_decrement_interval: u16,
//
// @scale_increment_interval:
// Number of group times between incrementing the scale factor value
// used at the beginning of a slice.
//
    pub scale_increment_interval: u16,
//
// @nfl_bpg_offset: Non first line BPG offset to be used
//
    pub nfl_bpg_offset: u16,
//
// @slice_bpg_offset: BPG offset used to enforce slice bit
//
    pub slice_bpg_offset: u16,
//
// @final_offset: Final RC linear transformation offset value
//
    pub final_offset: u16,
//
// @vbr_enable: True if VBR mode is enabled, false if disabled
//
    pub vbr_enable: bool,
//
// @mux_word_size: Mux word size (in bits) for SSM mode
//
    pub mux_word_size: u8,
//
// @slice_chunk_size:
// The (max) size in bytes of the "chunks" that are used in slice
// multiplexing.
//
    pub slice_chunk_size: u16,
//
// @rc_bits: Rate control buffer size in bits
//
    pub rc_bits: u16,
//
// @dsc_version_minor: DSC minor version
//
    pub dsc_version_minor: u8,
//
// @dsc_version_major: DSC major version
//
    pub dsc_version_major: u8,
//
// @native_422: True if Native 4:2:2 supported, else false
//
    pub native_422: bool,
//
// @native_420: True if Native 4:2:0 supported else false.
//
    pub native_420: bool,
//
// @second_line_bpg_offset:
// Additional bits/grp for seconnd line of slice for native 4:2:0
//
    pub second_line_bpg_offset: u8,
//
// @nsl_bpg_offset:
// Num of bits deallocated for each grp that is not in second line of
// slice
//
    pub nsl_bpg_offset: u16,
//
// @second_line_offset_adj:
// Offset adjustment for second line in Native 4:2:0 mode
//
    pub second_line_offset_adj: u16,
}

//
// struct drm_dsc_picture_parameter_set - Represents 128 bytes of
// Picture Parameter Set
//
// The VESA DSC standard defines picture parameter set (PPS) which display
// stream compression encoders must communicate to decoders.
// The PPS is encapsulated in 128 bytes (PPS 0 through PPS 127). The fields in
// this structure are as per Table 4.1 in Vesa DSC specification v1.1/v1.2.
// The PPS fields that span over more than a byte should be stored in Big Endian
// format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dsc_picture_parameter_set {
//
// @dsc_version:
// PPS0[3:0] - dsc_version_minor: Contains Minor version of DSC
// PPS0[7:4] - dsc_version_major: Contains major version of DSC
//
    pub dsc_version: u8,
//
// @pps_identifier:
// PPS1[7:0] - Application specific identifier that can be
// used to differentiate between different PPS tables.
//
    pub pps_identifier: u8,
//
// @pps_reserved:
// PPS2[7:0]- RESERVED Byte
//
    pub pps_reserved: u8,
//
// @pps_3:
// PPS3[3:0] - linebuf_depth: Contains linebuffer bit depth used to
// generate the bitstream. (0x0 - 16 bits for DSC 1.2, 0x8 - 8 bits,
// 0xA - 10 bits, 0xB - 11 bits, 0xC - 12 bits, 0xD - 13 bits,
// 0xE - 14 bits for DSC1.2, 0xF - 14 bits for DSC 1.2.
// PPS3[7:4] - bits_per_component: Bits per component for the original
// pixels of the encoded picture.
// 0x0 = 16bpc (allowed only when dsc_version_minor = 0x2)
// 0x8 = 8bpc, 0xA = 10bpc, 0xC = 12bpc, 0xE = 14bpc (also
// allowed only when dsc_minor_version = 0x2)
//
    pub pps_3: u8,
//
// @pps_4:
// PPS4[1:0] -These are the most significant 2 bits of
// compressed BPP bits_per_pixel[9:0] syntax element.
// PPS4[2] - vbr_enable: 0 = VBR disabled, 1 = VBR enabled
// PPS4[3] - simple_422: Indicates if decoder drops samples to
// reconstruct the 4:2:2 picture.
// PPS4[4] - Convert_rgb: Indicates if DSC color space conversion is
// active.
// PPS4[5] - blobk_pred_enable: Indicates if BP is used to code any
// groups in picture
// PPS4[7:6] - Reseved bits
//
    pub pps_4: u8,
//
// @bits_per_pixel_low:
// PPS5[7:0] - This indicates the lower significant 8 bits of
// the compressed BPP bits_per_pixel[9:0] element.
//
    pub bits_per_pixel_low: u8,
//
// @pic_height:
// PPS6[7:0], PPS7[7:0] -pic_height: Specifies the number of pixel rows
// within the raster.
//
    pub pic_height: __be16,
//
// @pic_width:
// PPS8[7:0], PPS9[7:0] - pic_width: Number of pixel columns within
// the raster.
//
    pub pic_width: __be16,
//
// @slice_height:
// PPS10[7:0], PPS11[7:0] - Slice height in units of pixels.
//
    pub slice_height: __be16,
//
// @slice_width:
// PPS12[7:0], PPS13[7:0] - Slice width in terms of pixels.
//
    pub slice_width: __be16,
//
// @chunk_size:
// PPS14[7:0], PPS15[7:0] - Size in units of bytes of the chunks
// that are used for slice multiplexing.
//
    pub chunk_size: __be16,
//
// @initial_xmit_delay_high:
// PPS16[1:0] - Most Significant two bits of initial transmission delay.
// It specifies the number of pixel times that the encoder waits before
// transmitting data from its rate buffer.
// PPS16[7:2] - Reserved
//
    pub initial_xmit_delay_high: u8,
//
// @initial_xmit_delay_low:
// PPS17[7:0] - Least significant 8 bits of initial transmission delay.
//
    pub initial_xmit_delay_low: u8,
//
// @initial_dec_delay:
//
// PPS18[7:0], PPS19[7:0] - Initial decoding delay which is the number
// of pixel times that the decoder accumulates data in its rate buffer
// before starting to decode and output pixels.
//
    pub initial_dec_delay: __be16,
//
// @pps20_reserved:
//
// PPS20[7:0] - Reserved
//
    pub pps20_reserved: u8,
//
// @initial_scale_value:
// PPS21[5:0] - Initial rcXformScale factor used at beginning
// of a slice.
// PPS21[7:6] - Reserved
//
    pub initial_scale_value: u8,
//
// @scale_increment_interval:
// PPS22[7:0], PPS23[7:0] - Number of group times between incrementing
// the rcXformScale factor at end of a slice.
//
    pub scale_increment_interval: __be16,
//
// @scale_decrement_interval_high:
// PPS24[3:0] - Higher 4 bits indicating number of group times between
// decrementing the rcXformScale factor at beginning of a slice.
// PPS24[7:4] - Reserved
//
    pub scale_decrement_interval_high: u8,
//
// @scale_decrement_interval_low:
// PPS25[7:0] - Lower 8 bits of scale decrement interval
//
    pub scale_decrement_interval_low: u8,
//
// @pps26_reserved:
// PPS26[7:0]
//
    pub pps26_reserved: u8,
//
// @first_line_bpg_offset:
// PPS27[4:0] - Number of additional bits that are allocated
// for each group on first line of a slice.
// PPS27[7:5] - Reserved
//
    pub first_line_bpg_offset: u8,
//
// @nfl_bpg_offset:
// PPS28[7:0], PPS29[7:0] - Number of bits including frac bits
// deallocated for each group for groups after the first line of slice.
//
    pub nfl_bpg_offset: __be16,
//
// @slice_bpg_offset:
// PPS30, PPS31[7:0] - Number of bits that are deallocated for each
// group to enforce the slice constraint.
//
    pub slice_bpg_offset: __be16,
//
// @initial_offset:
// PPS32,33[7:0] - Initial value for rcXformOffset
//
    pub initial_offset: __be16,
//
// @final_offset:
// PPS34,35[7:0] - Maximum end-of-slice value for rcXformOffset
//
    pub final_offset: __be16,
//
// @flatness_min_qp:
// PPS36[4:0] - Minimum QP at which flatness is signaled and
// flatness QP adjustment is made.
// PPS36[7:5] - Reserved
//
    pub flatness_min_qp: u8,
//
// @flatness_max_qp:
// PPS37[4:0] - Max QP at which flatness is signalled and
// the flatness adjustment is made.
// PPS37[7:5] - Reserved
//
    pub flatness_max_qp: u8,
//
// @rc_model_size:
// PPS38,39[7:0] - Number of bits within RC Model.
//
    pub rc_model_size: __be16,
//
// @rc_edge_factor:
// PPS40[3:0] - Ratio of current activity vs, previous
// activity to determine presence of edge.
// PPS40[7:4] - Reserved
//
    pub rc_edge_factor: u8,
//
// @rc_quant_incr_limit0:
// PPS41[4:0] - QP threshold used in short term RC
// PPS41[7:5] - Reserved
//
    pub rc_quant_incr_limit0: u8,
//
// @rc_quant_incr_limit1:
// PPS42[4:0] - QP threshold used in short term RC
// PPS42[7:5] - Reserved
//
    pub rc_quant_incr_limit1: u8,
//
// @rc_tgt_offset:
// PPS43[3:0] - Lower end of the variability range around the target
// bits per group that is allowed by short term RC.
// PPS43[7:4]- Upper end of the variability range around the target
// bits per group that i allowed by short term rc.
//
    pub rc_tgt_offset: u8,
//
// @rc_buf_thresh:
// PPS44[7:0] - PPS57[7:0] - Specifies the thresholds in RC model for
// the 15 ranges defined by 14 thresholds.
//
    pub 1]: u8 rc_buf_thresh[DSC_NUM_BUF_RANGES -,
//
// @rc_range_parameters:
// PPS58[7:0] - PPS87[7:0]
// Parameters that correspond to each of the 15 ranges.
//
    pub rc_range_parameters: [__be16; DSC_NUM_BUF_RANGES],
//
// @native_422_420:
// PPS88[0] - 0 = Native 4:2:2 not used
// 1 = Native 4:2:2 used
// PPS88[1] - 0 = Native 4:2:0 not use
// 1 = Native 4:2:0 used
// PPS88[7:2] - Reserved 6 bits
//
    pub native_422_420: u8,
//
// @second_line_bpg_offset:
// PPS89[4:0] - Additional bits/group budget for the
// second line of a slice in Native 4:2:0 mode.
// Set to 0 if DSC minor version is 1 or native420 is 0.
// PPS89[7:5] - Reserved
//
    pub second_line_bpg_offset: u8,
//
// @nsl_bpg_offset:
// PPS90[7:0], PPS91[7:0] - Number of bits that are deallocated
// for each group that is not in the second line of a slice.
//
    pub nsl_bpg_offset: __be16,
//
// @second_line_offset_adj:
// PPS92[7:0], PPS93[7:0] - Used as offset adjustment for the second
// line in Native 4:2:0 mode.
//
    pub second_line_offset_adj: __be16,
//
// @pps_long_94_reserved:
// PPS 94, 95, 96, 97 - Reserved
//
    pub pps_long_94_reserved: u32,
//
// @pps_long_98_reserved:
// PPS 98, 99, 100, 101 - Reserved
//
    pub pps_long_98_reserved: u32,
//
// @pps_long_102_reserved:
// PPS 102, 103, 104, 105 - Reserved
//
    pub pps_long_102_reserved: u32,
//
// @pps_long_106_reserved:
// PPS 106, 107, 108, 109 - reserved
//
    pub pps_long_106_reserved: u32,
//
// @pps_long_110_reserved:
// PPS 110, 111, 112, 113 - reserved
//
    pub pps_long_110_reserved: u32,
//
// @pps_long_114_reserved:
// PPS 114 - 117 - reserved
//
    pub pps_long_114_reserved: u32,
//
// @pps_long_118_reserved:
// PPS 118 - 121 - reserved
//
    pub pps_long_118_reserved: u32,
//
// @pps_long_122_reserved:
// PPS 122- 125 - reserved
//
    pub pps_long_122_reserved: u32,
//
// @pps_short_126_reserved:
// PPS 126, 127 - reserved
//
    pub pps_short_126_reserved: __be16,
    pub __packed: },
//
// struct drm_dsc_pps_infoframe - DSC infoframe carrying the Picture Parameter
// Set Metadata
//
// This structure represents the DSC PPS infoframe required to send the Picture
// Parameter Set metadata required before enabling VESA Display Stream
// Compression. This is based on the DP Secondary Data Packet structure and
// comprises of SDP Header as defined &struct dp_sdp_header in drm_dp_helper.h
// and PPS payload defined in &struct drm_dsc_picture_parameter_set.
//
// @pps_header: Header for PPS as per DP SDP header format of type
// &struct dp_sdp_header
// @pps_payload: PPS payload fields as per DSC specification Table 4-1
// as represented in &struct drm_dsc_picture_parameter_set
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dsc_pps_infoframe {
    pub pps_header: dp_sdp_header,
    pub pps_payload: drm_dsc_picture_parameter_set,
    pub __packed: },
