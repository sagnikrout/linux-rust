//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/chips-media/wave5/wave5-vpuapi.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Wave5 series multi-standard codec IP - helper definitions
//
// Copyright (C) 2021-2023 CHIPS&MEDIA INC
//

// Macro flag: #define VPUAPI_H_INCLUDED

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum product_id {
    PRODUCT_ID_515,
    PRODUCT_ID_521,
    PRODUCT_ID_511,
    PRODUCT_ID_517,
    PRODUCT_ID_NONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_instance_type {
    VPU_INST_TYPE_DEC = 0,
    VPU_INST_TYPE_ENC = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_instance_state {
    VPU_INST_STATE_NONE = 0,
    VPU_INST_STATE_OPEN = 1,
    VPU_INST_STATE_INIT_SEQ = 2,
    VPU_INST_STATE_PIC_RUN = 3,
    VPU_INST_STATE_STOP = 4
}

// Maximum available on hardware.
pub const WAVE5_MAX_FBS: c_int = 32;

//
// common struct and definition
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cod_std {
    STD_AVC = 0,
    STD_HEVC = 12,
    STD_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wave_std {
    W_HEVC_DEC = 0x00,
    W_HEVC_ENC = 0x01,
    W_AVC_DEC = 0x02,
    W_AVC_ENC = 0x03,
    STD_UNKNOWN = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum set_param_option {
    OPT_COMMON = 0, /* SET_PARAM command option for encoding sequence */
    OPT_CUSTOM_GOP = 1, /* SET_PARAM command option for setting custom GOP */
    OPT_CUSTOM_HEADER = 2, /* SET_PARAM command option for setting custom VPS/SPS/PPS */
    OPT_VUI = 3, /* SET_PARAM command option for encoding VUI */
    OPT_CHANGE_PARAM = 0x10,
}

//
// PROFILE & LEVEL
//
// HEVC
pub const HEVC_PROFILE_MAIN: c_int = 1;
pub const HEVC_PROFILE_MAIN10: c_int = 2;
pub const HEVC_PROFILE_STILLPICTURE: c_int = 3;
pub const HEVC_PROFILE_MAIN10_STILLPICTURE: c_int = 2;
// H.264 profile for encoder
pub const H264_PROFILE_BP: c_int = 1;
pub const H264_PROFILE_MP: c_int = 2;
pub const H264_PROFILE_EXTENDED: c_int = 3;
pub const H264_PROFILE_HP: c_int = 4;
pub const H264_PROFILE_HIGH10: c_int = 5;
pub const H264_PROFILE_HIGH422: c_int = 6;
pub const H264_PROFILE_HIGH444: c_int = 7;
//
// error codes
//
// utility macros
//
// Initialize sequence firmware command mode
pub const INIT_SEQ_NORMAL: c_int = 1;
// Decode firmware command mode
pub const DEC_PIC_NORMAL: c_int = 0;
// bit_alloc_mode
pub const BIT_ALLOC_MODE_FIXED_RATIO: c_int = 2;
// bit_rate
pub const MAX_BIT_RATE: c_int = 700000000;
// decoding_refresh_type
pub const DEC_REFRESH_TYPE_NON_IRAP: c_int = 0;
pub const DEC_REFRESH_TYPE_CRA: c_int = 1;
pub const DEC_REFRESH_TYPE_IDR: c_int = 2;
// depend_slice_mode
pub const DEPEND_SLICE_MODE_RECOMMENDED: c_int = 1;
pub const DEPEND_SLICE_MODE_BOOST: c_int = 2;
pub const DEPEND_SLICE_MODE_FAST: c_int = 3;
// hvs_max_delta_qp
pub const MAX_HVS_MAX_DELTA_QP: c_int = 51;
// intra_refresh_mode
pub const REFRESH_MODE_CTU_ROWS: c_int = 1;
pub const REFRESH_MODE_CTU_COLUMNS: c_int = 2;
pub const REFRESH_MODE_CTU_STEP_SIZE: c_int = 3;
pub const REFRESH_MODE_CTUS: c_int = 4;
// intra_mb_refresh_mode
pub const REFRESH_MB_MODE_NONE: c_int = 0;
pub const REFRESH_MB_MODE_CTU_ROWS: c_int = 1;
pub const REFRESH_MB_MODE_CTU_COLUMNS: c_int = 2;
pub const REFRESH_MB_MODE_CTU_STEP_SIZE: c_int = 3;
// intra_qp
pub const MAX_INTRA_QP: c_int = 63;
// nr_inter_weight_*
pub const MAX_INTER_WEIGHT: c_int = 31;
// nr_intra_weight_*
pub const MAX_INTRA_WEIGHT: c_int = 31;
// nr_noise_sigma_*
pub const MAX_NOISE_SIGMA: c_int = 255;
// bitstream_buffer_size
pub const MIN_BITSTREAM_BUFFER_SIZE: c_int = 1024;

// vbv_buffer_size
pub const MIN_VBV_BUFFER_SIZE: c_int = 10;
pub const MAX_VBV_BUFFER_SIZE: c_int = 3000;
pub const BUFFER_MARGIN: c_int = 4096;
pub const MAX_FIRMWARE_CALL_RETRY: c_int = 10;
pub const VDI_LITTLE_ENDIAN: c_uint = 0x0;
//
// Parameters of DEC_SET_SEQ_CHANGE_MASK
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum codec_command {
    ENABLE_ROTATION,
    ENABLE_MIRRORING,
    SET_MIRROR_DIRECTION,
    SET_ROTATION_ANGLE,
    DEC_GET_QUEUE_STATUS,
    ENC_GET_QUEUE_STATUS,
    DEC_RESET_FRAMEBUF_INFO,
    DEC_GET_SEQ_INFO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mirror_direction {
    MIRDIR_NONE, /* no mirroring */
    MIRDIR_VER, /* vertical mirroring */
    MIRDIR_HOR, /* horizontal mirroring */
    MIRDIR_HOR_VER /* horizontal and vertical mirroring */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum frame_buffer_format {
    FORMAT_ERR = -1,
    FORMAT_420 = 0, /* 8bit */
    FORMAT_422, /* 8bit */
    FORMAT_224, /* 8bit */
    FORMAT_444, /* 8bit */
    FORMAT_400, /* 8bit */

// little endian perspective
// | addr 0 | addr 1 |
    FORMAT_420_P10_16BIT_MSB = 5, /* lsb |000000xx|xxxxxxxx | msb */
    FORMAT_420_P10_16BIT_LSB, /* lsb |xxxxxxx |xx000000 | msb */
    FORMAT_420_P10_32BIT_MSB, /* lsb |00xxxxxxxxxxxxxxxxxxxxxxxxxxx| msb */
    FORMAT_420_P10_32BIT_LSB, /* lsb |xxxxxxxxxxxxxxxxxxxxxxxxxxx00| msb */

// 4:2:2 packed format
// little endian perspective
// | addr 0 | addr 1 |
    FORMAT_422_P10_16BIT_MSB, /* lsb |000000xx |xxxxxxxx | msb */
    FORMAT_422_P10_16BIT_LSB, /* lsb |xxxxxxxx |xx000000 | msb */
    FORMAT_422_P10_32BIT_MSB, /* lsb |00xxxxxxxxxxxxxxxxxxxxxxxxxxx| msb */
    FORMAT_422_P10_32BIT_LSB, /* lsb |xxxxxxxxxxxxxxxxxxxxxxxxxxx00| msb */

    FORMAT_YUYV, /* 8bit packed format : Y0U0Y1V0 Y2U1Y3V1 ... */
    FORMAT_YUYV_P10_16BIT_MSB,
    FORMAT_YUYV_P10_16BIT_LSB,
    FORMAT_YUYV_P10_32BIT_MSB,
    FORMAT_YUYV_P10_32BIT_LSB,

    FORMAT_YVYU, /* 8bit packed format : Y0V0Y1U0 Y2V1Y3U1 ... */
    FORMAT_YVYU_P10_16BIT_MSB,
    FORMAT_YVYU_P10_16BIT_LSB,
    FORMAT_YVYU_P10_32BIT_MSB,
    FORMAT_YVYU_P10_32BIT_LSB,

    FORMAT_UYVY, /* 8bit packed format : U0Y0V0Y1 U1Y2V1Y3 ... */
    FORMAT_UYVY_P10_16BIT_MSB,
    FORMAT_UYVY_P10_16BIT_LSB,
    FORMAT_UYVY_P10_32BIT_MSB,
    FORMAT_UYVY_P10_32BIT_LSB,

    FORMAT_VYUY, /* 8bit packed format : V0Y0U0Y1 V1Y2U1Y3 ... */
    FORMAT_VYUY_P10_16BIT_MSB,
    FORMAT_VYUY_P10_16BIT_LSB,
    FORMAT_VYUY_P10_32BIT_MSB,
    FORMAT_VYUY_P10_32BIT_LSB,

    FORMAT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum packed_format_num {
    NOT_PACKED = 0,
    PACKED_YUYV,
    PACKED_YVYU,
    PACKED_UYVY,
    PACKED_VYUY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wave5_interrupt_bit {
    INT_WAVE5_INIT_VPU = 0,
    INT_WAVE5_WAKEUP_VPU = 1,
    INT_WAVE5_SLEEP_VPU = 2,
    INT_WAVE5_CREATE_INSTANCE = 3,
    INT_WAVE5_FLUSH_INSTANCE = 4,
    INT_WAVE5_DESTROY_INSTANCE = 5,
    INT_WAVE5_INIT_SEQ = 6,
    INT_WAVE5_SET_FRAMEBUF = 7,
    INT_WAVE5_DEC_PIC = 8,
    INT_WAVE5_ENC_PIC = 8,
    INT_WAVE5_ENC_SET_PARAM = 9,
    INT_WAVE5_DEC_QUERY = 14,
    INT_WAVE5_BSBUF_EMPTY = 15,
    INT_WAVE5_BSBUF_FULL = 15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pic_type {
    PIC_TYPE_I = 0,
    PIC_TYPE_P = 1,
    PIC_TYPE_B = 2,
    PIC_TYPE_IDR = 5, /* H.264/H.265 IDR (Instantaneous Decoder Refresh) picture */
    PIC_TYPE_MAX /* no meaning */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sw_reset_mode {
    SW_RESET_SAFETY,
    SW_RESET_FORCE,
    SW_RESET_ON_BOOT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tiled_map_type {
    LINEAR_FRAME_MAP = 0, /* linear frame map type */
    COMPRESSED_FRAME_MAP = 17, /* compressed frame map type*/
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum temporal_id_mode {
    TEMPORAL_ID_MODE_ABSOLUTE,
    TEMPORAL_ID_MODE_RELATIVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_attr {
    pub product_id: u32,
    pub /: *mut *mut char product_name[8]; / product name in ascii code,
    pub product_version: u32,
    pub fw_version: u32,
    pub customer_id: u32,
    pub /: *mut *mut u32 support_decoders; / bitmask,
    pub /: *mut *mut u32 support_encoders; / bitmask,
    pub 1: u32 support_backbone:,
    pub 1: u32 support_avc10bit_enc:,
    pub 1: u32 support_hevc10bit_enc:,
    pub 1: u32 support_hevc10bit_dec:,
    pub 1: u32 support_vcore_backbone:,
    pub 1: u32 support_vcpu_backbone:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frame_buffer {
    pub buf_y: dma_addr_t,
    pub buf_cb: dma_addr_t,
    pub buf_cr: dma_addr_t,
    pub buf_y_size: c_uint,
    pub buf_cb_size: c_uint,
    pub buf_cr_size: c_uint,
    pub map_type: tiled_map_type,
    pub /: *mut *mut unsigned int stride; / horizontal stride for the given frame buffer,
    pub /: *mut *mut unsigned int width; / width of the given frame buffer,
    pub /: *mut *mut unsigned int height; / height of the given frame buffer,
    pub /: *mut *mut size_t size; / size of the given frame buffer,
    pub sequence_no: c_uint,
    pub update_fb_info: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_rect {
    pub /: *mut *mut unsigned int left; / horizontal pixel offset from left edge,
    pub /: *mut *mut unsigned int top; / vertical pixel offset from top edge,
    pub /: *mut *mut unsigned int right; / horizontal pixel offset from right edge,
    pub /: *mut *mut unsigned int bottom; / vertical pixel offset from bottom edge,
}

//
// decode struct and definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dec_open_param {
    pub bitstream_buffer: dma_addr_t,
    pub bitstream_buffer_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dec_initial_info {
    pub pic_width: u32,
    pub pic_height: u32,
    pub pic_crop_rect: vpu_rect,
    pub /: *mut *mut u32 min_frame_buffer_count; / between 1 to 16,
    pub profile: u32,
    pub /: *mut *mut u32 luma_bitdepth; / bit-depth of the luma sample,
    pub /: *mut *mut u32 chroma_bitdepth; / bit-depth of the chroma sample,
    pub seq_init_err_reason: u32,
    pub /: *mut *mut dma_addr_t rd_ptr; / read pointer of bitstream buffer,
    pub /: *mut *mut dma_addr_t wr_ptr; / write pointer of bitstream buffer,
    pub sequence_no: u32,
    pub vlc_buf_size: u32,
    pub param_buf_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dec_output_info {
//
// This is a frame buffer index for the picture to be displayed at the moment
// among frame buffers which are registered using vpu_dec_register_frame_buffer().
// Frame data that will be displayed is stored in the frame buffer with this index
// When there is no display delay, this index is always the equal to
// index_frame_decoded, however, if displaying is delayed (for display
// reordering in AVC or B-frames in VC1), this index might be different to
// index_frame_decoded. By checking this index, HOST applications can easily figure
// out whether sequence decoding has been finished or not.
//
// -3(0xFFFD) or -2(0xFFFE) : when a display output cannot be given due to picture
// reordering or skip option
// -1(0xFFFF) : when there is no more output for display at the end of sequence
// decoding
//
    pub index_frame_display: i32,
//
// This is the frame buffer index of the decoded picture among the frame buffers which were
// registered using vpu_dec_register_frame_buffer(). The currently decoded frame is stored
// into the frame buffer specified by this index.
//
// -2 : indicates that no decoded output is generated because decoder meets EOS
// (end of sequence) or skip
// -1 : indicates that the decoder fails to decode a picture because there is no available
// frame buffer
//
    pub index_frame_decoded: i32,
    pub index_frame_decoded_for_tiled: i32,
    pub nal_type: u32,
    pub pic_type: c_uint,
    pub rc_display: vpu_rect,
    pub disp_pic_width: c_uint,
    pub disp_pic_height: c_uint,
    pub rc_decoded: vpu_rect,
    pub dec_pic_width: u32,
    pub dec_pic_height: u32,
    pub decoded_poc: i32,
    pub /: *mut *mut int temporal_id; / temporal ID of the picture,
    pub /: *mut *mut dma_addr_t rd_ptr; / stream buffer read pointer for the current decoder instance,
    pub /: *mut *mut dma_addr_t wr_ptr; / stream buffer write pointer for the current decoder instance,
    pub disp_frame: frame_buffer,
    pub /: *mut *mut u32 frame_display_flag; / it reports a frame buffer flag to be displayed,
//
// this variable reports that sequence has been changed while H.264/AVC stream decoding.
// if it is 1, HOST application can get the new sequence information by calling
// vpu_dec_get_initial_info() or wave5_vpu_dec_issue_seq_init().
//
// for H.265/HEVC decoder, each bit has a different meaning as follows.
//
// sequence_changed[5] : it indicates that the profile_idc has been changed
// sequence_changed[16] : it indicates that the resolution has been changed
// sequence_changed[19] : it indicates that the required number of frame buffer has
// been changed.
//
    pub /: *mut *mut unsigned int frame_cycle; / reports the number of cycles for processing a frame,
    pub sequence_no: u32,
    pub /: *mut *mut u32 dec_host_cmd_tick; / tick of DEC_PIC command for the picture,
    pub /: *mut *mut u32 dec_decode_end_tick; / end tick of decoding slices of the picture,
    pub sequence_changed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue_status_info {
    pub instance_queue_count: u32,
    pub report_queue_count: u32,
}

//
// encode struct and definition
//
pub const MAX_NUM_TEMPORAL_LAYER: c_int = 7;
pub const MAX_NUM_SPATIAL_LAYER: c_int = 3;
pub const MAX_GOP_NUM: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct custom_gop_pic_param {
    pub /: *mut *mut u32 pic_type; / picture type of nth picture in the custom GOP,
    pub /: *mut *mut u32 poc_offset; / POC of nth picture in the custom GOP,
    pub /: *mut *mut u32 pic_qp; / quantization parameter of nth picture in the custom GOP,
    pub /: *mut *mut u32 use_multi_ref_p; / use multiref pic for P picture. valid only if PIC_TYPE is P,
    pub /: *mut *mut u32 ref_poc_l0; / POC of reference L0 of nth picture in the custom GOP,
    pub /: *mut *mut u32 ref_poc_l1; / POC of reference L1 of nth picture in the custom GOP,
    pub /: *mut *mut s32 temporal_id; / temporal ID of nth picture in the custom GOP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enc_wave_param {
//
// profile indicator (HEVC only)
//
// 0 : the firmware determines a profile according to the internal_bit_depth
// 1 : main profile
// 2 : main10 profile
// 3 : main still picture profile
// In the AVC encoder, a profile cannot be set by the host application.
// The firmware decides it based on internal_bit_depth.
// profile = HIGH (bitdepth 8) profile = HIGH10 (bitdepth 10)
//
    pub profile: u32,
    pub /: *mut *mut *mut u32 level; / level indicator (level  10),
    pub /: *mut *mut u32 internal_bit_depth: 4; / 8/10,
    pub /: *mut *mut u32 gop_preset_idx: 4; / 0 - 9,
    pub /: *mut *mut u32 decoding_refresh_type: 2; / 0=non-IRAP, 1=CRA, 2=IDR,
    pub /: *mut *mut u32 intra_qp; / quantization parameter of intra picture,
    pub /: *mut *mut u32 intra_period; / period of intra picture in GOP size,
    pub /: *mut *mut u32 conf_win_top; / top offset of conformance window,
    pub /: *mut *mut u32 conf_win_bot; / bottom offset of conformance window,
    pub /: *mut *mut u32 conf_win_left; / left offset of conformance window,
    pub /: *mut *mut u32 conf_win_right; / right offset of conformance window,
    pub 3: u32 intra_refresh_mode:,
//
// Argument for intra_ctu_refresh_mode.
//
// Depending on intra_refresh_mode, it can mean one of the following:
// - intra_ctu_refresh_mode (1) -> number of consecutive CTU rows
// - intra_ctu_refresh_mode (2) -> the number of consecutive CTU columns
// - intra_ctu_refresh_mode (3) -> step size in CTU
// - intra_ctu_refresh_mode (4) -> number of intra ct_us to be encoded in a picture
//
    pub intra_refresh_arg: u32,
//
// 0 : custom setting
// 1 : recommended encoder parameters (slow encoding speed, highest picture quality)
// 2 : boost mode (normal encoding speed, moderate picture quality)
// 3 : fast mode (fast encoding speed, low picture quality)
//
    pub 2: u32 depend_slice_mode :,
    pub depend_slice_mode_arg: u32,
    pub 1=slice-in-ctu-number*/: *mut *mut u32 independ_slice_mode : 1; / 0=no-multi-slice,,
    pub independ_slice_mode_arg: u32,
    pub 2: u32 max_num_merge:,
    pub /: *mut *mut s32 beta_offset_div2: 4; / sets beta_offset_div2 for deblocking filter,
    pub /: *mut *mut s32 tc_offset_div2: 4; / sets tc_offset_div3 for deblocking filter,
    pub /: *mut *mut u32 hvs_qp_scale: 4; / QP scaling factor for CU QP adjust if hvs_qp_scale_enable is 1,
    pub /: *mut *mut u32 hvs_max_delta_qp; / maximum delta QP for HVS,
    pub /: *mut *mut s32 chroma_cb_qp_offset; / the value of chroma(cb) QP offset,
    pub /: *mut *mut s32 chroma_cr_qp_offset; / the value of chroma(cr) QP offset,
    pub initial_rc_qp: i32,
    pub nr_intra_weight_y: u32,
    pub /: *mut *mut u32 nr_intra_weight_cb; / weight to cb noise level for intra picture (0 ~ 31),
    pub /: *mut *mut u32 nr_intra_weight_cr; / weight to cr noise level for intra picture (0 ~ 31),
    pub nr_inter_weight_y: u32,
    pub /: *mut *mut u32 nr_inter_weight_cb; / weight to cb noise level for inter picture (0 ~ 31),
    pub /: *mut *mut u32 nr_inter_weight_cr; / weight to cr noise level for inter picture (0 ~ 31),
    pub /: *mut *mut u32 min_qp_i; / minimum QP of I picture for rate control,
    pub /: *mut *mut u32 max_qp_i; / maximum QP of I picture for rate control,
    pub /: *mut *mut u32 min_qp_p; / minimum QP of P picture for rate control,
    pub /: *mut *mut u32 max_qp_p; / maximum QP of P picture for rate control,
    pub /: *mut *mut u32 min_qp_b; / minimum QP of B picture for rate control,
    pub /: *mut *mut u32 max_qp_b; / maximum QP of B picture for rate control,
    pub /: *mut *mut u32 avc_idr_period; / period of IDR picture (0 ~ 1024). 0 - implies an infinite period,
    pub /: *mut *mut u32 avc_slice_arg; / the number of MB for a slice when avc_slice_mode is set with 1,
    pub /: *mut *mut u32 intra_mb_refresh_mode: 2; / 0=none, 1=row, 2=column, 3=step-size-in-mb,
//
// Argument for intra_mb_refresh_mode.
//
// intra_mb_refresh_mode (1) -> number of consecutive MB rows
// intra_mb_refresh_mode (2) ->the number of consecutive MB columns
// intra_mb_refresh_mode (3) -> step size in MB
//
    pub intra_mb_refresh_arg: u32,
    pub rc_weight_param: u32,
    pub rc_weight_buf: u32,
// flags
    pub /: *mut *mut u32 en_still_picture: 1; / still picture profile,
    pub /: *mut *mut u32 tier: 1; / 0=main, 1=high,
    pub /: *mut *mut u32 avc_slice_mode: 1; / 0=none, 1=slice-in-mb-number,
    pub /: *mut *mut u32 entropy_coding_mode: 1; / 0=CAVLC, 1=CABAC,
    pub /: *mut *mut u32 lossless_enable: 1; / enable lossless encoding,
    pub /: *mut *mut u32 const_intra_pred_flag: 1; / enable constrained intra prediction,
    pub /: *mut *mut u32 tmvp_enable: 1; / enable temporal motion vector prediction,
    pub 1: u32 wpp_enable:,
    pub /: *mut *mut u32 disable_deblk: 1; / disable in-loop deblocking filtering,
    pub 1: u32 lf_cross_slice_boundary_enable:,
    pub 1: u32 skip_intra_trans:,
    pub /: *mut *mut u32 sao_enable: 1; / enable SAO (sample adaptive offset),
    pub /: *mut *mut u32 intra_nx_n_enable: 1; / enables intra nx_n p_us,
    pub /: *mut *mut u32 cu_level_rc_enable: 1; / enable CU level rate control,
    pub /: *mut *mut u32 hvs_qp_enable: 1; / enable CU QP adjustment for subjective quality enhancement,
    pub /: *mut *mut u32 strong_intra_smooth_enable: 1; / enable strong intra smoothing,
    pub /: *mut *mut u32 rdo_skip: 1; / skip RDO (rate distortion optimization),
    pub /: *mut *mut u32 lambda_scaling_enable: 1; / enable lambda scaling using custom GOP,
    pub /: *mut *mut u32 transform8x8_enable: 1; / enable 8x8 intra prediction and 8x8 transform,
    pub /: *mut *mut u32 mb_level_rc_enable: 1; / enable MB-level rate control,
    pub /: *mut *mut u32 forced_idr_header_enable: 1; / enable header encoding before IDR frame,
    pub /: *mut *mut u32 constraint_set1_flag: 1; / enable CBP,
    pub /: *mut *mut u32 bg_detection: 1; / enable background detection,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enc_open_param {
    pub bitstream_buffer: dma_addr_t,
    pub bitstream_buffer_size: c_uint,
    pub /: *mut *mut u32 pic_width; / width of a picture to be encoded in unit of sample,
    pub /: *mut *mut u32 pic_height; / height of a picture to be encoded in unit of sample,
    pub /: *mut *mut u32 frame_rate_info;/ desired fps,
    pub vbv_buffer_size: u32,
    pub /: *mut *mut u32 bit_rate; / target bitrate in bps,
    pub wave_param: enc_wave_param,
    pub /: *mut *mut packed_format_num packed_format; / <<vpuapi_h_packed_format_num>>,
    pub src_format: frame_buffer_format,
    pub line_buf_int_en: bool,
    pub /: *mut *mut u32 rc_enable : 1; / rate control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enc_initial_info {
    pub /: *mut *mut u32 min_frame_buffer_count; / minimum number of frame buffers,
    pub /: *mut *mut u32 min_src_frame_count; / minimum number of source buffers,
    pub seq_init_err_reason: u32,
    pub warn_info: u32,
    pub /: *mut *mut u32 vlc_buf_size; / size of task buffer,
    pub /: *mut *mut u32 param_buf_size; / size of task buffer,
}

//
// Flags to encode NAL units explicitly
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enc_code_opt {
    pub 1: u32 implicit_header_encode:,
    pub 1: u32 encode_vcl:,
    pub 1: u32 encode_vps:,
    pub 1: u32 encode_sps:,
    pub 1: u32 encode_pps:,
    pub 1: u32 encode_aud:,
    pub 1: u32 encode_eos:,
    pub 1: u32 encode_eob:,
    pub 1: u32 encode_vui:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enc_param {
    pub source_frame: *mut frame_buffer,
    pub pic_stream_buffer_addr: u32,
    pub pic_stream_buffer_size: u64,
    pub /: *mut *mut u32 src_idx; / source frame buffer index,
    pub code_option: enc_code_opt,
    pub /: *mut *mut u64 pts; / presentation timestamp (PTS) of the input source,
    pub src_end_flag: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enc_output_info {
    pub bitstream_buffer: u32,
    pub /: *mut *mut u32 bitstream_size; / byte size of encoded bitstream,
    pub /: *mut *mut u32 pic_type: 2; / <<vpuapi_h_pic_type>>,
    pub recon_frame_index: i32,
    pub rd_ptr: dma_addr_t,
    pub wr_ptr: dma_addr_t,
    pub /: *mut *mut u32 enc_pic_byte; / number of encoded picture bytes,
    pub /: *mut *mut s32 enc_src_idx; / source buffer index of the currently encoded picture,
    pub enc_vcl_nut: u32,
    pub /: *mut *mut u32 error_reason; / error reason of the currently encoded picture,
    pub /: *mut *mut u32 warn_info; / warning information on the currently encoded picture,
    pub frame*/: *mut *mut unsigned int frame_cycle; / param for reporting the cycle number of encoding one,
    pub pts: u64,
    pub /: *mut *mut u32 enc_host_cmd_tick; / tick of ENC_PIC command for the picture,
    pub /: *mut *mut u32 enc_encode_end_tick; / end tick of encoding slices of the picture,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enc_pic_code_option {
    CODEOPT_ENC_HEADER_IMPLICIT = BIT(0),
    CODEOPT_ENC_VCL = BIT(1), /* flag to encode VCL nal unit explicitly */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gop_preset_idx {
    PRESET_IDX_CUSTOM_GOP = 0, /* user defined GOP structure */
    PRESET_IDX_ALL_I = 1, /* all intra, gopsize = 1 */
    PRESET_IDX_IPP = 2, /* consecutive P, cyclic gopsize = 1 */
    PRESET_IDX_IBBB = 3, /* consecutive B, cyclic gopsize = 1 */
    PRESET_IDX_IBPBP = 4, /* gopsize = 2 */
    PRESET_IDX_IBBBP = 5, /* gopsize = 4 */
    PRESET_IDX_IPPPP = 6, /* consecutive P, cyclic gopsize = 4 */
    PRESET_IDX_IBBBB = 7, /* consecutive B, cyclic gopsize = 4 */
    PRESET_IDX_RA_IB = 8, /* random access, cyclic gopsize = 8 */
    PRESET_IDX_IPP_SINGLE = 9, /* consecutive P, cyclic gopsize = 1, with single ref */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_axi_info {
    pub use_ip_enable: u32,
    pub use_bit_enable: u32,
    pub 1: u32 use_lf_row_enable:,
    pub 1: u32 use_enc_rdo_enable:,
    pub 1: u32 use_enc_lf_enable:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dec_info {
    pub open_param: dec_open_param,
    pub initial_info: dec_initial_info,
    pub /: *mut *mut dec_initial_info new_seq_info; / temporal new sequence information,
    pub stream_wr_ptr: u32,
    pub stream_rd_ptr: u32,
    pub frame_display_flag: u32,
    pub stream_buf_start_addr: dma_addr_t,
    pub stream_buf_end_addr: dma_addr_t,
    pub stream_buf_size: u32,
    pub vb_mv: [vpu_buf; MAX_REG_FRAME],
    pub vb_fbc_y_tbl: [vpu_buf; MAX_REG_FRAME],
    pub vb_fbc_c_tbl: [vpu_buf; MAX_REG_FRAME],
    pub 7: unsigned int num_of_decoding_fbs:,
    pub 7: unsigned int num_of_display_fbs:,
    pub stride: c_uint,
    pub sec_axi_info: sec_axi_info,
    pub user_data_buf_addr: dma_addr_t,
    pub user_data_enable: u32,
    pub user_data_buf_size: u32,
    pub vb_work: vpu_buf,
    pub vb_task: vpu_buf,
    pub dec_out_info: [dec_output_info; WAVE5_MAX_FBS],
    pub seq_change_mask: u32,
    pub temp_id_select_mode: temporal_id_mode,
    pub target_temp_id: u32,
    pub target_spatial_id: u32,
    pub instance_queue_count: u32,
    pub report_queue_count: u32,
    pub cycle_per_tick: u32,
    pub product_code: u32,
    pub vlc_buf_size: u32,
    pub param_buf_size: u32,
    pub initial_info_obtained: bool,
    pub reorder_enable: bool,
    pub first_cycle_check: bool,
    pub 1: u32 stream_endflag:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enc_info {
    pub open_param: enc_open_param,
    pub initial_info: enc_initial_info,
    pub stream_rd_ptr: u32,
    pub stream_wr_ptr: u32,
    pub stream_buf_start_addr: dma_addr_t,
    pub stream_buf_end_addr: dma_addr_t,
    pub stream_buf_size: u32,
    pub num_frame_buffers: c_uint,
    pub stride: c_uint,
    pub rotation_enable: bool,
    pub mirror_enable: bool,
    pub mirror_direction: mirror_direction,
    pub rotation_angle: c_uint,
    pub initial_info_obtained: bool,
    pub sec_axi_info: sec_axi_info,
    pub line_buf_int_en: bool,
    pub vb_work: vpu_buf,
    pub /: *mut *mut vpu_buf vb_mv; / col_mv buffer,
    pub /: *mut *mut vpu_buf vb_fbc_y_tbl; / FBC luma table buffer,
    pub /: *mut *mut vpu_buf vb_fbc_c_tbl; / FBC chroma table buffer,
    pub /: *mut *mut vpu_buf vb_sub_sam_buf; / sub-sampled buffer for ME,
    pub vb_task: vpu_buf,
    pub /: *mut *mut u64 cur_pts; / current timestamp in 90_k_hz,
    pub /: *mut *mut u64 pts_map[32]; / PTS mapped with source frame index,
    pub instance_queue_count: u32,
    pub report_queue_count: u32,
    pub first_cycle_check: bool,
    pub cycle_per_tick: u32,
    pub product_code: u32,
    pub vlc_buf_size: u32,
    pub param_buf_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_device {
    pub dev: *mut device,
    pub v4l2_dev: v4l2_device,
    pub v4l2_m2m_dec_dev: *mut v4l2_m2m_dev,
    pub v4l2_m2m_enc_dev: *mut v4l2_m2m_dev,
    pub instances: list_head,
    pub video_dev_dec: *mut video_device,
    pub video_dev_enc: *mut video_device,
    pub /: *mut *mut mutex dev_lock; / lock for the src, dst v4l2 queues,
    pub /: *mut *mut mutex hw_lock; / lock hw configurations,
    pub irq_lock: mutex,
    pub irq: c_int,
    pub product: product_id,
    pub attr: vpu_attr,
    pub common_mem: vpu_buf,
    pub last_performance_cycles: u32,
    pub sram_size: u32,
    pub sram_pool: *mut gen_pool,
    pub sram_buf: vpu_buf,
    pub vdb_register: *mut void __iomem,
    pub product_code: u32,
    pub inst_ida: ida,
    pub clks: *mut clk_bulk_data,
    pub hrtimer: hrtimer,
    pub work: kthread_work,
    pub worker: *mut kthread_worker,
    pub vpu_poll_interval: c_int,
    pub num_clks: c_int,
    pub irq_thread: *mut task_struct,
    pub happens*/: *mut *mut semaphore irq_sem; / signal to irq_thread when interrupt,
    pub resets: *mut reset_control,
    pub /: *mut *mut spinlock_t irq_spinlock; / protect instances list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_instance_ops {
    pub inst): *mut *mut void (finish_process)(struct vpu_instance,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_instance {
    pub list: list_head,
    pub v4l2_fh: v4l2_fh,
    pub v4l2_m2m_dev: *mut v4l2_m2m_dev,
    pub v4l2_ctrl_hdl: v4l2_ctrl_handler,
    pub dev: *mut vpu_device,
    pub irq_done: completion,
    pub src_fmt: v4l2_pix_format_mplane,
    pub dst_fmt: v4l2_pix_format_mplane,
    pub colorspace: v4l2_colorspace,
    pub xfer_func: v4l2_xfer_func,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub quantization: v4l2_quantization,
    pub irq_status: kfifo,
    pub state: vpu_instance_state,
    pub type: vpu_instance_type,
    pub ops: *const vpu_instance_ops,
    pub /: *mut *mut spinlock_t state_spinlock; / This protects the instance state,
    pub std: wave_std,
    pub id: i32,
    pub enc_info: enc_info,
    pub dec_info: dec_info,
    pub codec_info: *mut },
    pub frame_buf: [frame_buffer; MAX_REG_FRAME],
    pub frame_vbuf: [vpu_buf; MAX_REG_FRAME],
    pub fbc_buf_count: u32,
    pub queued_src_buf_num: u32,
    pub queued_dst_buf_num: u32,
    pub avail_src_bufs: list_head,
    pub avail_dst_bufs: list_head,
    pub conf_win: v4l2_rect,
    pub timestamp: u64,
    pub output_format: frame_buffer_format,
    pub cbcr_interleave: bool,
    pub nv21: bool,
    pub eos: bool,
    pub /: *mut *mut bool sent_eos; / check if EOS is sent to application,
    pub WAVE5_SYSERR_QUEUEING_FAIL*/: *mut *mut bool retry; / retry to feed bitstream if failure reason is,
    pub /: *mut *mut int queuing_num; / count of bitstream queued,
    pub /: *mut *mut mutex feed_lock; / lock for feeding bitstream buffers,
    pub /: *mut *mut bool queuing_fail; / if there is the queuing failure,
    pub empty_queue: bool,
    pub bitstream_vbuf: vpu_buf,
    pub last_rd_ptr: dma_addr_t,
    pub remaining_consumed_bytes: usize,
    pub needs_reallocation: bool,
    pub min_src_buf_count: c_uint,
    pub rot_angle: c_uint,
    pub mirror_direction: c_uint,
    pub bit_depth: c_uint,
    pub frame_rate: c_uint,
    pub vbv_buf_size: c_uint,
    pub rc_mode: c_uint,
    pub rc_enable: c_uint,
    pub bit_rate: c_uint,
    pub encode_aud: c_uint,
    pub enc_param: enc_wave_param,
}

extern "C" {
    pub fn wave5_vdi_write_register(vpu_dev: *mut vpu_device, addr: u32, data: u32);
}
extern "C" {
    pub fn wave5_vdi_read_register(vpu_dev: *mut vpu_device, addr: u32) -> u32;
}
extern "C" {
    pub fn wave5_vdi_clear_memory(vpu_dev: *mut vpu_device, vb: *mut vpu_buf) -> c_int;
}
extern "C" {
    pub fn wave5_vdi_allocate_dma_memory(vpu_dev: *mut vpu_device, vb: *mut vpu_buf) -> c_int;
}
extern "C" {
    pub fn wave5_vdi_free_dma_memory(vpu_dev: *mut vpu_device, vb: *mut vpu_buf) -> c_int;
}
extern "C" {
    pub fn wave5_vdi_allocate_sram(vpu_dev: *mut vpu_device);
}
extern "C" {
    pub fn wave5_vdi_free_sram(vpu_dev: *mut vpu_device);
}
extern "C" {
    pub fn wave5_vpu_init_with_bitcode(dev: *mut device, bitcode: *mut u8, size: usize) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_flush_instance(inst: *mut vpu_instance) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_get_version_info(dev: *mut device, revision: *mut u32, product_id: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_open(inst: *mut vpu_instance, open_param: *mut dec_open_param) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_close(inst: *mut vpu_instance, fail_res: *mut u32) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_issue_seq_init(inst: *mut vpu_instance) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_complete_seq_init(inst: *mut vpu_instance, info: *mut dec_initial_info) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_start_one_frame(inst: *mut vpu_instance, res_fail: *mut u32) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_get_output_info(inst: *mut vpu_instance, info: *mut dec_output_info) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_set_rd_ptr(inst: *mut vpu_instance, addr: dma_addr_t, update_wr_ptr: c_int) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_get_rd_ptr(inst: *mut vpu_instance) -> dma_addr_t;
}
extern "C" {
    pub fn wave5_vpu_dec_reset_framebuffer(inst: *mut vpu_instance, index: c_uint) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_give_command(inst: *mut vpu_instance, cmd: codec_command, parameter: *mut c_void) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_update_bitstream_buffer(inst: *mut vpu_instance, size: usize) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_clr_disp_flag(inst: *mut vpu_instance, index: c_int) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_set_disp_flag(inst: *mut vpu_instance, index: c_int) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_enc_open(inst: *mut vpu_instance, open_param: *mut enc_open_param) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_enc_close(inst: *mut vpu_instance, fail_res: *mut u32) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_enc_issue_seq_init(inst: *mut vpu_instance) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_enc_complete_seq_init(inst: *mut vpu_instance, info: *mut enc_initial_info) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_enc_get_output_info(inst: *mut vpu_instance, info: *mut enc_output_info) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_enc_give_command(inst: *mut vpu_instance, cmd: codec_command, parameter: *mut c_void) -> c_int;
}
