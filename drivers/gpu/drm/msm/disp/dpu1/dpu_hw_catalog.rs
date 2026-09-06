//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_hw_catalog.h
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
// Copyright (c) 2022-2023, Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright (c) 2015-2018, 2020 The Linux Foundation. All rights reserved.
//

//
// Max hardware block count: For ex: max 12 SSPP pipes or
// 5 ctl paths. In all cases, it can have max 12 hardware blocks
// based on current design
//
pub const MAX_BLOCKS: c_int = 12;
pub const DPU_HW_BLK_NAME_LEN: c_int = 16;
pub const DPU_MAX_IMG_WIDTH: c_uint = 0x3fff;
pub const DPU_MAX_IMG_HEIGHT: c_uint = 0x3fff;
pub const CRTC_DUAL_MIXERS: c_int = 2;
pub const MAX_XIN_COUNT: c_int = 16;
//
// SSPP sub-blocks/features
// @DPU_SSPP_SCALER_QSEED2,  QSEED2 algorithm support
// @DPU_SSPP_SCALER_QSEED3_COMPATIBLE,  QSEED3-compatible alogorithm support (includes QSEED3, QSEED3LITE and QSEED4)
// @DPU_SSPP_SCALER_RGB,     RGB Scaler, supported by RGB pipes
// @DPU_SSPP_CSC,            Support of Color space converion
// @DPU_SSPP_CSC_10BIT,      Support of 10-bit Color space conversion
// @DPU_SSPP_CURSOR,         SSPP can be used as a cursor layer
// @DPU_SSPP_QOS,            SSPP support QoS control, danger/safe/creq
// @DPU_SSPP_EXCL_RECT,      SSPP supports exclusion rect
// @DPU_SSPP_SMART_DMA_V1,   SmartDMA 1.0 support
// @DPU_SSPP_SMART_DMA_V2,   SmartDMA 2.0 support
// @DPU_SSPP_TS_PREFILL      Supports prefill with traffic shaper
// @DPU_SSPP_TS_PREFILL_REC1 Supports prefill with traffic shaper multirec
// @DPU_SSPP_CDP             Supports client driven prefetch
// @DPU_SSPP_INLINE_ROTATION Support inline rotation
// @DPU_SSPP_MAX             maximum value
//
// MIXER sub-blocks/features
// @DPU_MIXER_SOURCESPLIT     Layer mixer supports source-split configuration
// @DPU_MIXER_MAX             maximum value
//
// DSPP sub-blocks
// @DPU_DSPP_PCC             Panel color correction block
// @DPU_DSPP_GC              Gamma correction block
//
// CTL sub-blocks
// @DPU_CTL_SPLIT_DISPLAY:	CTL supports video mode split display
// @DPU_CTL_MAX
//
// WB sub-blocks and features
// @DPU_WB_LINE_MODE        Writeback module supports line/linear mode
// @DPU_WB_BLOCK_MODE       Writeback module supports block mode read
// @DPU_WB_CHROMA_DOWN,     Writeback chroma down block,
// @DPU_WB_DOWNSCALE,       Writeback integer downscaler,
// @DPU_WB_DITHER,          Dither block
// @DPU_WB_TRAFFIC_SHAPER,  Writeback traffic shaper bloc
// @DPU_WB_UBWC,            Writeback Universal bandwidth compression
// @DPU_WB_YUV_CONFIG       Writeback supports output of YUV colorspace
// @DPU_WB_PIPE_ALPHA       Writeback supports pipe alpha
// @DPU_WB_XY_ROI_OFFSET    Writeback supports x/y-offset of out ROI in
// the destination image
// @DPU_WB_QOS,             Writeback supports QoS control, danger/safe/creq
// @DPU_WB_QOS_8LVL,        Writeback supports 8-level QoS control
// @DPU_WB_CDP              Writeback supports client driven prefetch
// @DPU_WB_CROP             CWB supports cropping
// @DPU_WB_MAX              maximum value
//
// VBIF sub-blocks and features
// @DPU_VBIF_QOS_OTLIM        VBIF supports OT Limit
// @DPU_VBIF_QOS_REMAP        VBIF supports QoS priority remap
// @DPU_VBIF_MAX              maximum value
//
// DSC sub-blocks/features
// @DPU_DSC_NATIVE_42x_EN     Supports NATIVE_422_EN and NATIVE_420_EN encoding
// @DPU_DSC_MAX
//
// MACRO DPU_HW_BLK_INFO - information of HW blocks inside DPU
// @name:              string name for debug purposes
// @id:                enum identifying this block
// @base:              register base offset to mdss
// @len:               length of hardware block
//

//
// struct dpu_scaler_blk: Scaler information
// @name: string name for debug purposes
// @base: offset of this sub-block relative to the block offset
// @len: register block length of this sub-block
// @version: qseed block revision, on QSEED3+ platforms this is the value of
// scaler_blk.base + QSEED3_HW_VERSION registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_scaler_blk {
    pub name: [c_char; DPU_HW_BLK_NAME_LEN],
    pub base: u32,
    pub len: u32,
    pub version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_csc_blk {
    pub name: [c_char; DPU_HW_BLK_NAME_LEN],
    pub base: u32,
    pub len: u32,
}

//
// struct dpu_pp_blk : Pixel processing sub-blk information
// @name: string name for debug purposes
// @base: offset of this sub-block relative to the block offset
// @len: register block length of this sub-block
// @version: HW Algorithm version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_pp_blk {
    pub name: [c_char; DPU_HW_BLK_NAME_LEN],
    pub base: u32,
    pub len: u32,
    pub version: u32,
}

//
// struct dpu_dsc_blk - DSC Encoder sub-blk information
// @name: string name for debug purposes
// @base: offset of this sub-block relative to the block offset
// @len: register block length of this sub-block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_dsc_blk {
    pub name: [c_char; DPU_HW_BLK_NAME_LEN],
    pub base: u32,
    pub len: u32,
}

//
// struct dpu_sspp_v13_rec_blk - SSPP REC sub-blk information
// @name: string name for debug purposes
// @base: offset of this sub-block relative to the block offset
// @len: register block length of this sub-block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_sspp_v13_rec_blk {
    pub name: [c_char; DPU_HW_BLK_NAME_LEN],
    pub base: u32,
    pub len: u32,
}

//
// enum dpu_qos_lut_usage - define QoS LUT use cases
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_qos_lut_usage {
    DPU_QOS_LUT_USAGE_LINEAR,
    DPU_QOS_LUT_USAGE_MACROTILE,
    DPU_QOS_LUT_USAGE_NRT,
    DPU_QOS_LUT_USAGE_MAX,
}

//
// struct dpu_qos_lut_entry - define QoS LUT table entry
// @fl: fill level, or zero on last entry to indicate default lut
// @lut: lut to use if equal to or less than fill level
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_qos_lut_entry {
    pub fl: u32,
    pub lut: u64,
}

//
// struct dpu_qos_lut_tbl - define QoS LUT table
// @nentry: number of entry in this table
// @entries: Pointer to table entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_qos_lut_tbl {
    pub nentry: u32,
    pub entries: *const dpu_qos_lut_entry,
}

//
// struct dpu_rotation_cfg - define inline rotation config
// @rot_maxheight: max pre rotated height allowed for rotation
// @rot_num_formats: number of elements in @rot_format_list
// @rot_format_list: list of supported rotator formats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_rotation_cfg {
    pub rot_maxheight: u32,
    pub rot_num_formats: usize,
    pub rot_format_list: *const u32,
}

//
// struct dpu_caps - define DPU capabilities
// @max_mixer_width    max layer mixer line width support.
// @max_mixer_blendstages max layer mixer blend stages or
// supported z order
// @has_src_split      source split feature status
// @has_dim_layer      dim layer feature status
// @has_idle_pc        indicate if idle power collapse feature is supported
// @has_3d_merge       indicate if 3D merge is supported
// @max_linewidth      max linewidth for sspp
// @pixel_ram_size     size of latency hiding and de-tiling buffer in bytes
// @max_hdeci_exp      max horizontal decimation supported (max is 2^value)
// @max_vdeci_exp      max vertical decimation supported (max is 2^value)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_caps {
    pub max_mixer_width: u32,
    pub max_mixer_blendstages: u32,
    pub has_src_split: bool,
    pub has_dim_layer: bool,
    pub has_idle_pc: bool,
    pub has_3d_merge: bool,
// SSPP limits
    pub max_linewidth: u32,
    pub pixel_ram_size: u32,
    pub max_hdeci_exp: u32,
    pub max_vdeci_exp: u32,
}

//
// struct dpu_sspp_sub_blks : SSPP sub-blocks
// common: Pointer to common configurations shared by sub blocks
// @max_per_pipe_bw: maximum allowable bandwidth of this pipe in kBps
// @qseed_ver: qseed version
// @scaler_blk:
// @csc_blk:
// @format_list: Pointer to list of supported formats
// @num_formats: Number of supported formats
// @dpu_rotation_cfg: inline rotation configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_sspp_sub_blks {
    pub max_per_pipe_bw: u32,
    pub qseed_ver: u32,
    pub scaler_blk: dpu_scaler_blk,
    pub csc_blk: dpu_pp_blk,
    pub sspp_rec0_blk: dpu_sspp_v13_rec_blk,
    pub sspp_rec1_blk: dpu_sspp_v13_rec_blk,
    pub format_list: *const u32,
    pub num_formats: u32,
    pub rotation_cfg: *const dpu_rotation_cfg,
}

//
// struct dpu_lm_sub_blks:      information of mixer block
// @maxwidth:               Max pixel width supported by this mixer
// @maxblendstages:         Max number of blend-stages supported
// @blendstage_base:        Blend-stage register base offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_lm_sub_blks {
    pub maxblendstages: u32,
    pub blendstage_base: [u32; MAX_BLOCKS],
}

//
// struct dpu_dspp_sub_blks: Information of DSPP block
// @pcc: pixel color correction block
// @gc: gamma correction block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_dspp_sub_blks {
    pub pcc: dpu_pp_blk,
    pub gc: dpu_pp_blk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_pingpong_sub_blks {
    pub dither: dpu_pp_blk,
}

//
// struct dpu_dsc_sub_blks - DSC sub-blks
// @enc: DSC encoder sub-block
// @ctl: DSC controller sub-block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_dsc_sub_blks {
    pub enc: dpu_dsc_blk,
    pub ctl: dpu_dsc_blk,
}

//
// dpu_clk_ctrl_type - Defines top level clock control signals
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpu_clk_ctrl_type {
    DPU_CLK_CTRL_NONE,
    DPU_CLK_CTRL_VIG0,
    DPU_CLK_CTRL_VIG1,
    DPU_CLK_CTRL_VIG2,
    DPU_CLK_CTRL_VIG3,
    DPU_CLK_CTRL_VIG4,
    DPU_CLK_CTRL_RGB0,
    DPU_CLK_CTRL_RGB1,
    DPU_CLK_CTRL_RGB2,
    DPU_CLK_CTRL_RGB3,
    DPU_CLK_CTRL_DMA0,
    DPU_CLK_CTRL_DMA1,
    DPU_CLK_CTRL_DMA2,
    DPU_CLK_CTRL_DMA3,
    DPU_CLK_CTRL_DMA4,
    DPU_CLK_CTRL_DMA5,
    DPU_CLK_CTRL_CURSOR0,
    DPU_CLK_CTRL_CURSOR1,
    DPU_CLK_CTRL_INLINE_ROT0_SSPP,
    DPU_CLK_CTRL_REG_DMA,
    DPU_CLK_CTRL_WB2,
    DPU_CLK_CTRL_MAX,
}

// struct dpu_clk_ctrl_reg : Clock control register
// @reg_off:           register offset
// @bit_off:           bit offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_clk_ctrl_reg {
    pub reg_off: u32,
    pub bit_off: u32,
}

// struct dpu_mdp_cfg : MDP TOP-BLK instance info
// @id:                index identifying this block
// @base:              register base offset to mdss
// @clk_ctrls          clock control register definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_mdp_cfg {
    pub clk_ctrls: [dpu_clk_ctrl_reg; DPU_CLK_CTRL_MAX],
}

// struct dpu_ctl_cfg : MDP CTL instance info
// @id:                index identifying this block
// @base:              register base offset to mdss
// @features           bit mask identifying sub-blocks/features
// @intr_start:        interrupt index for CTL_START
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_ctl_cfg {
    pub features: c_ulong,
    pub intr_start: c_uint,
}

//
// struct dpu_sspp_cfg - information of source pipes
// @id:                index identifying this block
// @base               register offset of this block
// @features           bit mask identifying sub-blocks/features
// @sblk:              SSPP sub-blocks information
// @xin_id:            bus client identifier
// @clk_ctrl           clock control identifier
// @type               sspp type identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_sspp_cfg {
    pub features: c_ulong,
    pub sblk: *const dpu_sspp_sub_blks,
    pub xin_id: u32,
    pub clk_ctrl: dpu_clk_ctrl_type,
    pub type: u32,
}

//
// struct dpu_lm_cfg - information of layer mixer blocks
// @id:                index identifying this block
// @base               register offset of this block
// @features           bit mask identifying sub-blocks/features
// @sblk:              LM Sub-blocks information
// @pingpong:          ID of connected PingPong, PINGPONG_NONE if unsupported
// @lm_pair:           ID of LM that can be controlled by same CTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_lm_cfg {
    pub features: c_ulong,
    pub sblk: *const dpu_lm_sub_blks,
    pub pingpong: u32,
    pub dspp: u32,
    pub lm_pair: c_ulong,
}

//
// struct dpu_dspp_cfg - information of DSPP blocks
// @id                 enum identifying this block
// @base               register offset of this block
// supported by this block
// @sblk               sub-blocks information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_dspp_cfg {
    pub sblk: *const dpu_dspp_sub_blks,
}

//
// struct dpu_pingpong_cfg - information of PING-PONG blocks
// @id                 enum identifying this block
// @base               register offset of this block
// @intr_done:         index for PINGPONG done interrupt
// @intr_rdptr:        index for PINGPONG readpointer done interrupt
// @sblk               sub-blocks information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_pingpong_cfg {
    pub merge_3d: u32,
    pub intr_done: c_uint,
    pub intr_rdptr: c_uint,
    pub sblk: *const dpu_pingpong_sub_blks,
}

//
// struct dpu_merge_3d_cfg - information of DSPP blocks
// @id                 enum identifying this block
// @base               register offset of this block
// @sblk               sub-blocks information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_merge_3d_cfg {
    pub sblk: *const dpu_merge_3d_sub_blks,
}

//
// struct dpu_dsc_cfg - information of DSC blocks
// @id                 enum identifying this block
// @base               register offset of this block
// @len:               length of hardware block
// @features           bit mask identifying sub-blocks/features
// @sblk:              sub-blocks information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_dsc_cfg {
    pub features: c_ulong,
    pub sblk: *const dpu_dsc_sub_blks,
}

//
// struct dpu_intf_cfg - information of timing engine blocks
// @id                 enum identifying this block
// @base               register offset of this block
// @type:              Interface type(DSI, DP, HDMI)
// @controller_id:     Controller Instance ID in case of multiple of intf type
// @prog_fetch_lines_worst_case	Worst case latency num lines needed to prefetch
// @intr_underrun:	index for INTF underrun interrupt
// @intr_vsync:	        index for INTF VSYNC interrupt
// @intr_tear_rd_ptr:  Index for INTF TEAR_RD_PTR interrupt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_intf_cfg {
    pub type*/: *mut *mut u32 type; / interface,
    pub controller_id: u32,
    pub prog_fetch_lines_worst_case: u32,
    pub intr_underrun: c_uint,
    pub intr_vsync: c_uint,
    pub intr_tear_rd_ptr: c_uint,
}

//
// struct dpu_wb_cfg - information of writeback blocks
// @DPU_HW_BLK_INFO:    refer to the description above for DPU_HW_BLK_INFO
// @maxlinewidth:       max line width supported by writeback block
// @xin_id:             bus client identifier
// @intr_wb_done:       interrupt index for WB_DONE
// @format_list:	    list of formats supported by this writeback block
// @num_formats:	    number of formats supported by this writeback block
// @clk_ctrl:	        clock control identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_wb_cfg {
    pub features: c_ulong,
    pub maxlinewidth: u32,
    pub xin_id: u32,
    pub intr_wb_done: c_uint,
    pub format_list: *const u32,
    pub num_formats: u32,
    pub clk_ctrl: dpu_clk_ctrl_type,
}

//
// struct dpu_cwb_cfg : MDP CWB mux instance info
// @id:                enum identifying this block
// @base:              register base offset to mdss
// @features           bit mask identifying sub-blocks/features
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_cwb_cfg {
}

//
// struct dpu_vbif_dynamic_ot_cfg - dynamic OT setting
// @pps                pixel per seconds
// @ot_limit           OT limit to use up to specified pixel per second
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_vbif_dynamic_ot_cfg {
    pub pps: u64,
    pub ot_limit: u32,
}

//
// struct dpu_vbif_dynamic_ot_tbl - dynamic OT setting table
// @count              length of cfg
// @cfg                pointer to array of configuration settings with
// ascending requirements
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_vbif_dynamic_ot_tbl {
    pub count: u32,
    pub cfg: *const dpu_vbif_dynamic_ot_cfg,
}

//
// struct dpu_vbif_qos_tbl - QoS priority table
// @npriority_lvl      num of priority level
// @priority_lvl       pointer to array of priority level in ascending order
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_vbif_qos_tbl {
    pub npriority_lvl: u32,
    pub priority_lvl: *const u32,
}

//
// struct dpu_vbif_cfg - information of VBIF blocks
// @len:               length of hardware block
// @features           bit mask identifying sub-blocks/features
// @ot_rd_limit        default OT read limit
// @ot_wr_limit        default OT write limit
// @xin_halt_timeout   maximum time (in usec) for xin to halt
// @qos_rp_remap_size  size of VBIF_XINL_QOS_RP_REMAP register space
// @dynamic_ot_rd_tbl  dynamic OT read configuration table
// @dynamic_ot_wr_tbl  dynamic OT write configuration table
// @qos_rt_tbl         real-time QoS priority table
// @qos_nrt_tbl        non-real-time QoS priority table
// @memtype_count      number of defined memtypes
// @memtype            array of xin memtype definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_vbif_cfg {
    pub len: u32,
    pub features: c_ulong,
    pub default_ot_rd_limit: u32,
    pub default_ot_wr_limit: u32,
    pub xin_halt_timeout: u32,
    pub qos_rp_remap_size: u32,
    pub dynamic_ot_rd_tbl: dpu_vbif_dynamic_ot_tbl,
    pub dynamic_ot_wr_tbl: dpu_vbif_dynamic_ot_tbl,
    pub qos_rt_tbl: dpu_vbif_qos_tbl,
    pub qos_nrt_tbl: dpu_vbif_qos_tbl,
    pub memtype_count: u32,
    pub memtype: [u32; MAX_XIN_COUNT],
}

//
// struct dpu_cdm_cfg - information of chroma down blocks
// @name               string name for debug purposes
// @id                 enum identifying this block
// @base               register offset of this block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_cdm_cfg {
}

//
// Define CDP use cases
// @DPU_PERF_CDP_UDAGE_RT: real-time use cases
// @DPU_PERF_CDP_USAGE_NRT: non real-time use cases such as WFD
//
// struct dpu_perf_cdp_cfg - define CDP use case configuration
// @rd_enable: true if read pipe CDP is enabled
// @wr_enable: true if write pipe CDP is enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_perf_cdp_cfg {
    pub rd_enable: bool,
    pub wr_enable: bool,
}

//
// struct dpu_mdss_version - DPU's major and minor versions
// @core_major_ver: DPU core's major version
// @core_minor_ver: DPU core's minor version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_mdss_version {
    pub core_major_ver: u8,
    pub core_minor_ver: u8,
}

//
// struct dpu_perf_cfg - performance control settings
// @max_bw_low         low threshold of maximum bandwidth (kbps)
// @max_bw_high        high threshold of maximum bandwidth (kbps)
// @min_core_ib        minimum bandwidth for core (kbps)
// @min_core_ib        minimum mnoc ib vote in kbps
// @min_llcc_ib        minimum llcc ib vote in kbps
// @min_dram_ib        minimum dram ib vote in kbps
// @undersized_prefill_lines   undersized prefill in lines
// @xtra_prefill_lines         extra prefill latency in lines
// @dest_scale_prefill_lines   destination scaler latency in lines
// @macrotile_perfill_lines    macrotile latency in lines
// @yuv_nv12_prefill_lines     yuv_nv12 latency in lines
// @linear_prefill_lines       linear latency in lines
// @downscaling_prefill_lines  downscaling latency in lines
// @amortizable_theshold minimum y position for traffic shaping prefill
// @min_prefill_lines  minimum pipeline latency in lines
// @clk_inefficiency_factor DPU src clock inefficiency factor
// @bw_inefficiency_factor DPU axi bus bw inefficiency factor
// @safe_lut_tbl: LUT tables for safe signals
// @danger_lut_tbl: LUT tables for danger signals
// @qos_lut_tbl: LUT tables for QoS signals
// @cdp_cfg            cdp use case configurations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_perf_cfg {
    pub max_bw_low: u32,
    pub max_bw_high: u32,
    pub min_core_ib: u32,
    pub min_llcc_ib: u32,
    pub min_dram_ib: u32,
    pub undersized_prefill_lines: u32,
    pub xtra_prefill_lines: u32,
    pub dest_scale_prefill_lines: u32,
    pub macrotile_prefill_lines: u32,
    pub yuv_nv12_prefill_lines: u32,
    pub linear_prefill_lines: u32,
    pub downscaling_prefill_lines: u32,
    pub amortizable_threshold: u32,
    pub min_prefill_lines: u32,
    pub clk_inefficiency_factor: u32,
    pub bw_inefficiency_factor: u32,
    pub safe_lut_tbl: [u32; DPU_QOS_LUT_USAGE_MAX],
    pub danger_lut_tbl: [u32; DPU_QOS_LUT_USAGE_MAX],
    pub qos_lut_tbl: [dpu_qos_lut_tbl; DPU_QOS_LUT_USAGE_MAX],
    pub cdp_cfg: [dpu_perf_cdp_cfg; DPU_PERF_CDP_USAGE_MAX],
}

//
// struct dpu_mdss_cfg - information of MDSS HW
// This is the main catalog data structure representing
// this HW version. Contains dpu's major and minor versions,
// number of instances, register offsets, capabilities of the
// all MDSS HW sub-blocks.
//
// @dma_formats        Supported formats for dma pipe
// @cursor_formats     Supported formats for cursor pipe
// @vig_formats        Supported formats for vig pipe
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_mdss_cfg {
    pub mdss_ver: *const dpu_mdss_version,
    pub caps: *const dpu_caps,
    pub mdp: *const dpu_mdp_cfg,
    pub ctl_count: u32,
    pub ctl: *const dpu_ctl_cfg,
    pub sspp_count: u32,
    pub sspp: *const dpu_sspp_cfg,
    pub mixer_count: u32,
    pub mixer: *const dpu_lm_cfg,
    pub pingpong_count: u32,
    pub pingpong: *const dpu_pingpong_cfg,
    pub merge_3d_count: u32,
    pub merge_3d: *const dpu_merge_3d_cfg,
    pub dsc_count: u32,
    pub dsc: *const dpu_dsc_cfg,
    pub intf_count: u32,
    pub intf: *const dpu_intf_cfg,
    pub vbif: *const dpu_vbif_cfg,
    pub wb_count: u32,
    pub wb: *const dpu_wb_cfg,
    pub cdm: *const dpu_cdm_cfg,
    pub ad_count: u32,
    pub dspp_count: u32,
    pub dspp: *const dpu_dspp_cfg,
    pub cwb_count: u32,
    pub cwb: *const dpu_cwb_cfg,
// Add additional block data structures here
    pub perf: *const dpu_perf_cfg,
    pub dma_formats: *const dpu_format_extended,
    pub cursor_formats: *const dpu_format_extended,
    pub vig_formats: *const dpu_format_extended,
}
