//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_platform_common.h
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const CODED_FRAMES_PROGRESSIVE: c_uint = 0x0;
pub const DEFAULT_MAX_HOST_BUF_COUNT: c_int = 64;
pub const DEFAULT_MAX_HOST_BURST_BUF_COUNT: c_int = 256;
pub const DEFAULT_FPS: c_int = 30;
pub const MAXIMUM_FPS: c_int = 480;

pub const MIN_QP_8BIT: c_int = 1;
pub const MAX_QP: c_int = 51;
pub const MAX_QP_HEVC: c_int = 63;
pub const DEFAULT_QP: c_int = 20;
pub const BITRATE_DEFAULT: c_int = 20000000;

pub const MAX_LTR_FRAME_COUNT_GEN1: c_int = 4;
pub const MAX_LTR_FRAME_COUNT_GEN2: c_int = 2;
pub const MAX_LAYER_HB: c_int = 3;
pub const MAX_AVC_LAYER_HP_HYBRID_LTR: c_int = 5;
pub const MAX_AVC_LAYER_HP_SLIDING_WINDOW: c_int = 3;
pub const MAX_HEVC_LAYER_HP_SLIDING_WINDOW: c_int = 3;
pub const MAX_HEVC_VBR_LAYER_HP_SLIDING_WINDOW: c_int = 5;
pub const MAX_HIER_CODING_LAYER_GEN1: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stage_type {
    STAGE_1 = 1,
    STAGE_2 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pipe_type {
    PIPE_1 = 1,
    PIPE_2 = 2,
    PIPE_4 = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_clk_type {
    IRIS_AXI_CLK, /* AXI0 in case of platforms with multiple AXI clocks */
    IRIS_CTRL_CLK,
    IRIS_AHB_CLK,
    IRIS_HW_CLK,
    IRIS_HW_AHB_CLK,
    IRIS_AXI1_CLK,
    IRIS_CTRL_FREERUN_CLK,
    IRIS_HW_FREERUN_CLK,
    IRIS_BSE_HW_CLK,
    IRIS_VPP0_HW_CLK,
    IRIS_VPP1_HW_CLK,
    IRIS_APV_HW_CLK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_clk_data {
    pub clk_type: platform_clk_type,
    pub clk_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tz_cp_config {
    pub cp_start: u32,
    pub cp_size: u32,
    pub cp_nonpixel_start: u32,
    pub cp_nonpixel_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_inst_caps {
    pub min_frame_width: u32,
    pub max_frame_width: u32,
    pub min_frame_height: u32,
    pub max_frame_height: u32,
    pub max_mbpf: u32,
    pub mb_cycles_vsp: u32,
    pub mb_cycles_vpp: u32,
    pub mb_cycles_fw: u32,
    pub mb_cycles_fw_vpp: u32,
    pub max_frame_rate: u32,
    pub max_operating_rate: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_inst_fw_cap_type {
    PROFILE_H264 = 1,
    PROFILE_HEVC,
    PROFILE_VP9,
    LEVEL_H264,
    LEVEL_HEVC,
    LEVEL_VP9,
    PROFILE_AV1,
    LEVEL_AV1,
    TIER_AV1,
    DRAP,
    FILM_GRAIN,
    SUPER_BLOCK,
    ENH_LAYER_COUNT,
    INPUT_BUF_HOST_MAX_COUNT,
    OUTPUT_BUF_HOST_MAX_COUNT,
    STAGE,
    PIPE,
    POC,
    CODED_FRAMES,
    BIT_DEPTH,
    RAP_FRAME,
    TIER,
    HEADER_MODE,
    PREPEND_SPSPPS_TO_IDR,
    BITRATE,
    BITRATE_PEAK,
    BITRATE_MODE,
    FRAME_SKIP_MODE,
    FRAME_RC_ENABLE,
    GOP_SIZE,
    ENTROPY_MODE,
    MIN_FRAME_QP_H264,
    MIN_FRAME_QP_HEVC,
    MAX_FRAME_QP_H264,
    MAX_FRAME_QP_HEVC,
    I_FRAME_MIN_QP_H264,
    I_FRAME_MIN_QP_HEVC,
    P_FRAME_MIN_QP_H264,
    P_FRAME_MIN_QP_HEVC,
    B_FRAME_MIN_QP_H264,
    B_FRAME_MIN_QP_HEVC,
    I_FRAME_MAX_QP_H264,
    I_FRAME_MAX_QP_HEVC,
    P_FRAME_MAX_QP_H264,
    P_FRAME_MAX_QP_HEVC,
    B_FRAME_MAX_QP_H264,
    B_FRAME_MAX_QP_HEVC,
    I_FRAME_QP_H264,
    I_FRAME_QP_HEVC,
    P_FRAME_QP_H264,
    P_FRAME_QP_HEVC,
    B_FRAME_QP_H264,
    B_FRAME_QP_HEVC,
    ROTATION,
    HFLIP,
    VFLIP,
    IR_TYPE,
    IR_PERIOD,
    LTR_COUNT,
    USE_LTR,
    MARK_LTR,
    B_FRAME,
    LAYER_ENABLE,
    LAYER_TYPE_H264,
    LAYER_TYPE_HEVC,
    LAYER_COUNT_H264,
    LAYER_COUNT_HEVC,
    LAYER0_BITRATE_H264,
    LAYER1_BITRATE_H264,
    LAYER2_BITRATE_H264,
    LAYER3_BITRATE_H264,
    LAYER4_BITRATE_H264,
    LAYER5_BITRATE_H264,
    LAYER0_BITRATE_HEVC,
    LAYER1_BITRATE_HEVC,
    LAYER2_BITRATE_HEVC,
    LAYER3_BITRATE_HEVC,
    LAYER4_BITRATE_HEVC,
    LAYER5_BITRATE_HEVC,
    REQUEST_SYNC_FRAME,
    TIME_DELTA_BASED_RC,
    INST_FW_CAP_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_inst_fw_cap_flags {
    CAP_FLAG_DYNAMIC_ALLOWED	= BIT(0),
    CAP_FLAG_MENU			= BIT(1),
    CAP_FLAG_INPUT_PORT		= BIT(2),
    CAP_FLAG_OUTPUT_PORT		= BIT(3),
    CAP_FLAG_CLIENT_SET		= BIT(4),
    CAP_FLAG_BITMASK		= BIT(5),
    CAP_FLAG_VOLATILE		= BIT(6),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_inst_fw_cap {
    pub cap_id: platform_inst_fw_cap_type,
    pub min: i64,
    pub max: i64,
    pub step_or_mask: i64,
    pub value: i64,
    pub hfi_id: u32,
    pub flags: platform_inst_fw_cap_flags,
    pub cap_id): platform_inst_fw_cap_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bw_info {
    pub mbs_per_sec: u32,
    pub bw_ddr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_core_power {
    pub clk_freq: u64,
    pub icc_bw: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_inst_power {
    pub min_freq: u64,
    pub icc_bw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icc_vote_data {
    pub width: u32 height,,
    pub fps: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_pm_domain_type {
    IRIS_CTRL_POWER_DOMAIN,
    IRIS_HW_POWER_DOMAIN,
    IRIS_VPP0_HW_POWER_DOMAIN,
    IRIS_VPP1_HW_POWER_DOMAIN,
    IRIS_APV_HW_POWER_DOMAIN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_firmware_data {
    pub core): *mut *mut void (init_hfi_ops)(struct iris_core,
    pub core_arch: u32,
    pub inst_fw_caps_dec: *const platform_inst_fw_cap,
    pub inst_fw_caps_dec_size: u32,
    pub inst_fw_caps_enc: *const platform_inst_fw_cap,
    pub inst_fw_caps_enc_size: u32,
    pub dec_input_config_params_default: *const u32,
    pub dec_input_config_params_default_size: c_uint,
    pub dec_input_config_params_hevc: *const u32,
    pub dec_input_config_params_hevc_size: c_uint,
    pub dec_input_config_params_vp9: *const u32,
    pub dec_input_config_params_vp9_size: c_uint,
    pub dec_input_config_params_av1: *const u32,
    pub dec_input_config_params_av1_size: c_uint,
    pub dec_output_config_params: *const u32,
    pub dec_output_config_params_size: c_uint,
    pub enc_input_config_params: *const u32,
    pub enc_input_config_params_size: c_uint,
    pub enc_output_config_params: *const u32,
    pub enc_output_config_params_size: c_uint,
    pub dec_input_prop: *const u32,
    pub dec_input_prop_size: c_uint,
    pub dec_output_prop_avc: *const u32,
    pub dec_output_prop_avc_size: c_uint,
    pub dec_output_prop_hevc: *const u32,
    pub dec_output_prop_hevc_size: c_uint,
    pub dec_output_prop_vp9: *const u32,
    pub dec_output_prop_vp9_size: c_uint,
    pub dec_output_prop_av1: *const u32,
    pub dec_output_prop_av1_size: c_uint,
    pub dec_ip_int_buf_tbl: *const u32,
    pub dec_ip_int_buf_tbl_size: c_uint,
    pub dec_op_int_buf_tbl: *const u32,
    pub dec_op_int_buf_tbl_size: c_uint,
    pub enc_ip_int_buf_tbl: *const u32,
    pub enc_ip_int_buf_tbl_size: c_uint,
    pub enc_op_int_buf_tbl: *const u32,
    pub enc_op_int_buf_tbl_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_firmware_desc {
    pub firmware_data: *const iris_firmware_data,
    pub buffer_type): *mut *mut *mut u32 (get_vpu_buffer_size)(struct iris_inst inst, enum iris_buffer_type,
    pub fwname: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_platform_data {
    pub firmware_desc_gen2: *const *const iris_firmware_desc firmware_desc_gen1,,
    pub vpu_ops: *const vpu_ops,
    pub icc_tbl: *const icc_info,
    pub icc_tbl_size: c_uint,
    pub bw_tbl_dec: *const bw_info,
    pub bw_tbl_dec_size: c_uint,
    pub pmdomain_tbl: *const *const c_char,
    pub pmdomain_tbl_size: c_uint,
    pub opp_pd_tbl: *const *const c_char,
    pub opp_pd_tbl_size: c_uint,
    pub clk_tbl: *const platform_clk_data,
    pub opp_clk_tbl: *const *const c_char,
    pub clk_tbl_size: c_uint,
    pub clk_rst_tbl: *const *const c_char,
    pub clk_rst_tbl_size: c_uint,
    pub controller_rst_tbl: *const *const c_char,
    pub controller_rst_tbl_size: c_uint,
    pub dma_mask: u64,
    pub inst_iris_fmts: *const u32,
    pub inst_iris_fmts_size: u32,
    pub inst_caps: *mut platform_inst_caps,
    pub tz_cp_config_data: *const tz_cp_config,
    pub tz_cp_config_data_size: u32,
    pub num_vpp_pipe: u32,
    pub no_aon: bool,
    pub max_session_count: u32,
// max number of macroblocks per frame supported
    pub max_core_mbpf: u32,
// max number of macroblocks per second supported
    pub max_core_mbps: u32,
}
