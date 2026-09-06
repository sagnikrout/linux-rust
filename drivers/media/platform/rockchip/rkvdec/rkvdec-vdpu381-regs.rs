//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rkvdec/rkvdec-vdpu381-regs.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Rockchip VDPU381 Video Decoder driver registers description
//
// Copyright (C) 2024 Collabora, Ltd.
// Detlev Casanova <detlev.casanova@collabora.com>
//

pub const VDPU381_MODE_HEVC: c_int = 0;
pub const VDPU381_MODE_H264: c_int = 1;
pub const VDPU381_MODE_VP9: c_int = 2;
pub const VDPU381_MODE_AVS2: c_int = 3;
pub const MAX_SLICE_NUMBER: c_uint = 0x3fff;

pub const VDPU381_REG_DEC_E: c_uint = 0x028;
pub const VDPU381_DEC_E_BIT: c_int = 1;
pub const VDPU381_REG_IMPORTANT_EN: c_uint = 0x02c;

pub const VDPU381_REG_STA_INT: c_uint = 0x380;

// base: OFFSET_COMMON_REGS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vdpu381_regs_common {
    pub 1: u32 in_endian :,
    pub 1: u32 in_swap32_e :,
    pub 1: u32 in_swap64_e :,
    pub 1: u32 str_endian :,
    pub 1: u32 str_swap32_e :,
    pub 1: u32 str_swap64_e :,
    pub 1: u32 out_endian :,
    pub 1: u32 out_swap32_e :,
    pub 1: u32 out_cbcr_swap :,
    pub 1: u32 out_swap64_e :,
    pub 22: u32 reserved :,
    pub reg008_in_out: },
    pub 10: u32 dec_mode :,
    pub 22: u32 reserved :,
    pub reg009_dec_mode: },
    pub 1: u32 dec_e :,
    pub 31: u32 reserved :,
    pub reg010_dec_e: },
    pub 1: u32 reserved0 :,
    pub 1: u32 dec_clkgate_e :,
    pub 1: u32 dec_e_strmd_clkgate_dis :,
    pub 1: u32 reserved1 :,
    pub 1: u32 dec_irq_dis :,
    pub 1: u32 dec_timeout_e :,
    pub 1: u32 buf_empty_en :,
    pub 3: u32 reserved2 :,
    pub 1: u32 dec_e_rewrite_valid :,
    pub 9: u32 reserved3 :,
    pub 1: u32 softrst_en_p :,
    pub 1: u32 force_softreset_valid :,
    pub 2: u32 reserved4 :,
    pub 1: u32 pix_range_det_e :,
    pub 7: u32 reserved5 :,
    pub reg011_important_en: },
    pub 1: u32 reserved0 :,
    pub 1: u32 colmv_compress_en :,
    pub 1: u32 fbc_e :,
    pub 1: u32 reserved1 :,
    pub 1: u32 buspr_slot_disable :,
    pub 1: u32 error_info_en :,
    pub 1: u32 collect_info_en :,
    pub 1: u32 error_auto_rst_disable :,
    pub 1: u32 scanlist_addr_valid_en :,
    pub 1: u32 scale_down_en :,
    pub 1: u32 error_cfg_wr_disable :,
    pub 21: u32 reserved2 :,
    pub reg012_secondary_en: },
    pub 1: u32 reserved0 :,
    pub 1: u32 req_timeout_rst_sel :,
    pub 1: u32 reserved1 :,
    pub 1: u32 dec_commonirq_mode :,
    pub 2: u32 reserved2 :,
    pub 1: u32 stmerror_waitdecfifo_empty :,
    pub 5: u32 reserved3 :,
    pub 1: u32 allow_not_wr_unref_bframe :,
    pub 1: u32 fbc_output_wr_disable :,
    pub 4: u32 reserved4 :,
    pub 1: u32 error_mode :,
    pub 2: u32 reserved5 :,
    pub 1: u32 ycacherd_prior :,
    pub 2: u32 reserved6 :,
    pub 1: u32 cur_pic_is_idr :,
    pub 1: u32 reserved7 :,
    pub 1: u32 right_auto_rst_disable :,
    pub 1: u32 frame_end_err_rst_flag :,
    pub 1: u32 rd_prior_mode :,
    pub 1: u32 rd_ctrl_prior_mode :,
    pub 1: u32 reserved8 :,
    pub 1: u32 filter_outbuf_mode :,
    pub reg013_en_mode_set: },
    pub 1: u32 fbc_force_uncompress :,
    pub 2: u32 reserved0 :,
    pub 1: u32 allow_16x8_cp_flag :,
    pub 2: u32 reserved1 :,
    pub 1: u32 fbc_h264_exten_4or8_flag :,
    pub 25: u32 reserved2 :,
    pub reg014_fbc_param_set: },
    pub 1: u32 rlc_mode_direct_write :,
    pub 1: u32 rlc_mode :,
    pub 3: u32 reserved0 :,
    pub 7: u32 strm_start_bit :,
    pub 20: u32 reserved1 :,
    pub reg015_stream_param_set: },
    pub reg016_stream_len: u32,
    pub 25: u32 slice_num :,
    pub 7: u32 reserved :,
    pub reg017_slice_number: },
    pub 16: u32 y_hor_virstride :,
    pub 16: u32 reserved :,
    pub reg018_y_hor_stride: },
    pub 16: u32 uv_hor_virstride :,
    pub 16: u32 reserved :,
    pub reg019_uv_hor_stride: },
    pub 28: u32 y_virstride :,
    pub 4: u32 reserved :,
    pub reg020_y_stride: },
    pub 1: u32 inter_error_prc_mode :,
    pub 1: u32 error_intra_mode :,
    pub 1: u32 error_deb_en :,
    pub 5: u32 picidx_replace :,
    pub 1: u32 error_spread_e :,
    pub 3: u32 reserved0 :,
    pub 1: u32 error_inter_pred_cross_slice :,
    pub 11: u32 reserved1 :,
    pub 1: u32 roi_error_ctu_cal_en :,
    pub 7: u32 reserved2 :,
    pub reg021_error_ctrl_set: },
    pub 12: u32 roi_x_ctu_offset_st :,
    pub 4: u32 reserved0 :,
    pub 12: u32 roi_y_ctu_offset_st :,
    pub 4: u32 reserved1 :,
    pub reg022_err_roi_ctu_offset_start: },
    pub 12: u32 roi_x_ctu_offset_end :,
    pub 4: u32 reserved0 :,
    pub 12: u32 roi_y_ctu_offset_end :,
    pub 4: u32 reserved1 :,
    pub reg023_err_roi_ctu_offset_end: },
    pub 32: u32 cabac_err_en_lowbits :,
    pub reg024_cabac_error_en_lowbits: },
    pub 30: u32 cabac_err_en_highbits :,
    pub 2: u32 reserved :,
    pub reg025_cabac_error_en_highbits: },
    pub 1: u32 inter_auto_gating_e :,
    pub 1: u32 filterd_auto_gating_e :,
    pub 1: u32 strmd_auto_gating_e :,
    pub 1: u32 mcp_auto_gating_e :,
    pub 1: u32 busifd_auto_gating_e :,
    pub 3: u32 reserved0 :,
    pub 1: u32 dec_ctrl_auto_gating_e :,
    pub 1: u32 intra_auto_gating_e :,
    pub 1: u32 mc_auto_gating_e :,
    pub 1: u32 transd_auto_gating_e :,
    pub 4: u32 reserved1 :,
    pub 1: u32 sram_auto_gating_e :,
    pub 1: u32 cru_auto_gating_e :,
    pub 13: u32 reserved2 :,
    pub 1: u32 reg_cfg_gating_en :,
    pub reg026_block_gating_en: },
    pub 16: u32 core_safe_x_pixels :,
    pub 16: u32 core_safe_y_pixels :,
    pub reg027_core_safe_pixels: },
    pub 3: u32 vp9_wr_prob_idx :,
    pub 1: u32 reserved0 :,
    pub 3: u32 vp9_rd_prob_idx :,
    pub 1: u32 reserved1 :,
    pub 1: u32 ref_req_advance_flag :,
    pub 1: u32 colmv_req_advance_flag :,
    pub 1: u32 poc_only_highbit_flag :,
    pub 1: u32 poc_arb_flag :,
    pub 4: u32 reserved2 :,
    pub 10: u32 film_idx :,
    pub 2: u32 reserved3 :,
    pub 1: u32 pu_req_mismatch_dis :,
    pub 1: u32 colmv_req_mismatch_dis :,
    pub 2: u32 reserved4 :,
    pub reg028_multiply_core_ctrl: },
    pub 2: u32 scale_down_hor_ratio :,
    pub 6: u32 reserved0 :,
    pub 2: u32 scale_down_vrz_ratio :,
    pub 22: u32 reserved1 :,
    pub reg029_scale_down_ctrl: },
    pub 20: u32 y_scale_down_tile8x8_hor_stride :,
    pub 12: u32 reserved0 :,
    pub reg030_y_scale_down_tile8x8_hor_stride: },
    pub 20: u32 uv_scale_down8x8_tile_hor_stride :,
    pub 12: u32 reserved0 :,
    pub reg031_uv_scale_down_tile8x8_hor_stride: },
    pub reg032_timeout_threshold: u32,
    pub __packed: },
// base: OFFSET_COMMON_ADDR_REGS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vdpu381_regs_common_addr {
    pub rlc_base: u32,
    pub rlcwrite_base: u32,
    pub decout_base: u32,
    pub colmv_cur_base: u32,
    pub error_ref_base: u32,
    pub rcb_base: [u32; 10],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vdpu381_h26x_set {
    pub 1: u32 h26x_frame_orslice :,
    pub 1: u32 h26x_rps_mode :,
    pub 1: u32 h26x_stream_mode :,
    pub 1: u32 h26x_stream_lastpacket :,
    pub 1: u32 h264_firstslice_flag :,
    pub 27: u32 reserved :,
    pub __packed: },
// base: OFFSET_CODEC_PARAMS_REGS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vdpu381_regs_h264_params {
    pub reg064_h26x_set: rkvdec_vdpu381_h26x_set,
    pub reg065_cur_top_poc: u32,
    pub reg066_cur_bot_poc: u32,
    pub reg067_098_ref_poc: [u32; 32],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vdpu381_h264_info {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vdpu381_h264_ref_info {
    pub 1: u32 ref_field :,
    pub 1: u32 ref_topfield_used :,
    pub 1: u32 ref_botfield_used :,
    pub 1: u32 ref_colmv_use_flag :,
    pub 4: u32 reserved :,
    pub ref_info: [} __packed; 4],
    pub reg099_102_ref_info_regs: [} __packed; 4],
    pub reserved_103_111: [u32; 9],
    pub 1: u32 avs2_ref_error_field :,
    pub 1: u32 avs2_ref_error_topfield :,
    pub 1: u32 ref_error_topfield_used :,
    pub 1: u32 ref_error_botfield_used :,
    pub 28: u32 reserved :,
    pub reg112_error_ref_info: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vdpu381_regs_hevc_params {
    pub reg064_h26x_set: rkvdec_vdpu381_h26x_set,
    pub reg065_cur_top_poc: u32,
    pub reg066_cur_bot_poc: u32,
    pub reg067_082_ref_poc: [u32; 16],
    pub reserved_083_098: [u32; 16],
    pub 1: u32 hevc_ref_valid_0 :,
    pub 1: u32 hevc_ref_valid_1 :,
    pub 1: u32 hevc_ref_valid_2 :,
    pub 1: u32 hevc_ref_valid_3 :,
    pub 4: u32 reserve0 :,
    pub 1: u32 hevc_ref_valid_4 :,
    pub 1: u32 hevc_ref_valid_5 :,
    pub 1: u32 hevc_ref_valid_6 :,
    pub 1: u32 hevc_ref_valid_7 :,
    pub 4: u32 reserve1 :,
    pub 1: u32 hevc_ref_valid_8 :,
    pub 1: u32 hevc_ref_valid_9 :,
    pub 1: u32 hevc_ref_valid_10 :,
    pub 1: u32 hevc_ref_valid_11 :,
    pub 4: u32 reserve2 :,
    pub 1: u32 hevc_ref_valid_12 :,
    pub 1: u32 hevc_ref_valid_13 :,
    pub 1: u32 hevc_ref_valid_14 :,
    pub 5: u32 reserve3 :,
    pub reg099_hevc_ref_valid: },
    pub reserved_100_102: [u32; 3],
    pub 16: u32 ref_pic_layer_same_with_cur :,
    pub 16: u32 reserve :,
    pub reg103_hevc_mvc0: },
    pub 1: u32 poc_lsb_not_present_flag :,
    pub 6: u32 num_direct_ref_layers :,
    pub 1: u32 reserve0 :,
    pub 6: u32 num_reflayer_pics :,
    pub 1: u32 default_ref_layers_active_flag :,
    pub 1: u32 max_one_active_ref_layer_flag :,
    pub 1: u32 poc_reset_info_present_flag :,
    pub 1: u32 vps_poc_lsb_aligned_flag :,
    pub 1: u32 mvc_poc15_valid_flag :,
    pub 13: u32 reserve1 :,
    pub reg104_hevc_mvc1: },
    pub reserved_105_111: [u32; 7],
    pub 1: u32 avs2_ref_error_field :,
    pub 1: u32 avs2_ref_error_topfield :,
    pub 1: u32 ref_error_topfield_used :,
    pub 1: u32 ref_error_botfield_used :,
    pub 28: u32 reserve :,
    pub reg112_hevc_ref_info: },
    pub __packed: },
// base: OFFSET_CODEC_ADDR_REGS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vdpu381_regs_h26x_addr {
    pub reserved_160: u32,
    pub reg161_pps_base: u32,
    pub reserved_162: u32,
    pub reg163_rps_base: u32,
    pub reg164_180_ref_base: [u32; 16],
    pub reg181_scanlist_addr: u32,
    pub reg182_198_colmv_base: [u32; 16],
    pub reg199_cabactbl_base: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vdpu381_regs_h26x_highpoc {
    pub 4: u32 ref0_poc_highbit :,
    pub 4: u32 ref1_poc_highbit :,
    pub 4: u32 ref2_poc_highbit :,
    pub 4: u32 ref3_poc_highbit :,
    pub 4: u32 ref4_poc_highbit :,
    pub 4: u32 ref5_poc_highbit :,
    pub 4: u32 ref6_poc_highbit :,
    pub 4: u32 ref7_poc_highbit :,
    pub reg200_203_ref_poc_highbit: [}; 4],
    pub 4: u32 cur_poc_highbit :,
    pub 28: u32 reserved :,
    pub reg204_cur_poc_highbit: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vdpu381_regs_h264 {
    pub common: rkvdec_vdpu381_regs_common,
    pub h264_param: rkvdec_vdpu381_regs_h264_params,
    pub common_addr: rkvdec_vdpu381_regs_common_addr,
    pub h264_addr: rkvdec_vdpu381_regs_h26x_addr,
    pub h264_highpoc: rkvdec_vdpu381_regs_h26x_highpoc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vdpu381_regs_hevc {
    pub common: rkvdec_vdpu381_regs_common,
    pub hevc_param: rkvdec_vdpu381_regs_hevc_params,
    pub common_addr: rkvdec_vdpu381_regs_common_addr,
    pub hevc_addr: rkvdec_vdpu381_regs_h26x_addr,
    pub hevc_highpoc: rkvdec_vdpu381_regs_h26x_highpoc,
    pub __packed: },
