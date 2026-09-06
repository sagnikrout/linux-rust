//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/verisilicon/hantro_hw.h
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
// Hantro VPU codec driver
//
// Copyright 2018 Google LLC.
// Tomasz Figa <tfiga@chromium.org>
//

pub const DEC_8190_ALIGN_MASK: c_uint = 0x07U;
pub const MB_DIM: c_int = 16;
pub const TILE_MB_DIM: c_int = 4;

pub const FMT_MIN_WIDTH: c_int = 48;
pub const FMT_MIN_HEIGHT: c_int = 48;
pub const FMT_HD_WIDTH: c_int = 1280;
pub const FMT_HD_HEIGHT: c_int = 720;
pub const FMT_FHD_WIDTH: c_int = 1920;
pub const FMT_FHD_HEIGHT: c_int = 1088;
pub const FMT_UHD_WIDTH: c_int = 3840;
pub const FMT_UHD_HEIGHT: c_int = 2160;
pub const FMT_4K_WIDTH: c_int = 4096;
pub const FMT_4K_HEIGHT: c_int = 2304;

pub const MAX_POSTPROC_BUFFERS: c_int = 64;

// blocks, with Cb CB first then Cr CB following
//
pub const CBS_CHROMA_H: c_int = 4;
//
// struct hantro_aux_buf - auxiliary DMA buffer for hardware data
//
// @cpu:	CPU pointer to the buffer.
// @dma:	DMA address of the buffer.
// @size:	Size of the buffer.
// @attrs:	Attributes of the DMA mapping.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_aux_buf {
    pub cpu: *mut c_void,
    pub dma: dma_addr_t,
    pub size: usize,
    pub attrs: c_ulong,
}

// Max. number of entries in the DPB (HW limitation).
pub const HANTRO_H264_DPB_SIZE: c_int = 16;
//
// struct hantro_h264_dec_ctrls
//
// @decode:	Decode params
// @scaling:	Scaling info
// @sps:	SPS info
// @pps:	PPS info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_h264_dec_ctrls {
    pub decode: *const v4l2_ctrl_h264_decode_params,
    pub scaling: *const v4l2_ctrl_h264_scaling_matrix,
    pub sps: *const v4l2_ctrl_h264_sps,
    pub pps: *const v4l2_ctrl_h264_pps,
}

//
// struct hantro_h264_dec_reflists
//
// @p:		P reflist
// @b0:		B0 reflist
// @b1:		B1 reflist
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_h264_dec_reflists {
    pub p: [v4l2_h264_reference; V4L2_H264_REF_LIST_LEN],
    pub b0: [v4l2_h264_reference; V4L2_H264_REF_LIST_LEN],
    pub b1: [v4l2_h264_reference; V4L2_H264_REF_LIST_LEN],
}

//
// struct hantro_h264_dec_hw_ctx
//
// @priv:	Private auxiliary buffer for hardware.
// @dpb:	DPB
// @reflists:	P/B0/B1 reflists
// @ctrls:	V4L2 controls attached to a run
// @dpb_longterm: DPB long-term
// @dpb_valid:	  DPB valid
// @cur_poc:	Current picture order count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_h264_dec_hw_ctx {
    pub priv: hantro_aux_buf,
    pub dpb: [v4l2_h264_dpb_entry; HANTRO_H264_DPB_SIZE],
    pub reflists: hantro_h264_dec_reflists,
    pub ctrls: hantro_h264_dec_ctrls,
    pub dpb_longterm: u32,
    pub dpb_valid: u32,
    pub cur_poc: i32,
}

//
// struct hantro_hevc_dec_ctrls
// @decode_params: Decode params
// @scaling:	Scaling matrix
// @sps:	SPS info
// @pps:	PPS info
// @hevc_hdr_skip_length: the number of data (in bits) to skip in the
// slice segment header syntax after 'slice type'
// token
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_hevc_dec_ctrls {
    pub decode_params: *const v4l2_ctrl_hevc_decode_params,
    pub scaling: *const v4l2_ctrl_hevc_scaling_matrix,
    pub sps: *const v4l2_ctrl_hevc_sps,
    pub pps: *const v4l2_ctrl_hevc_pps,
    pub hevc_hdr_skip_length: u32,
}

//
// struct hantro_hevc_dec_hw_ctx
// @tile_sizes:		Tile sizes buffer
// @tile_filter:	Tile vertical filter buffer
// @tile_sao:		Tile SAO buffer
// @tile_bsd:		Tile BSD control buffer
// @ref_bufs:		Internal reference buffers
// @scaling_lists:	Scaling lists buffer
// @ref_bufs_poc:	Internal reference buffers picture order count
// @ref_bufs_used:	Bitfield of used reference buffers
// @ctrls:		V4L2 controls attached to a run
// @num_tile_cols_allocated: number of allocated tiles
// @use_compression:	use reference buffer compression
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_hevc_dec_hw_ctx {
    pub tile_sizes: hantro_aux_buf,
    pub tile_filter: hantro_aux_buf,
    pub tile_sao: hantro_aux_buf,
    pub tile_bsd: hantro_aux_buf,
    pub ref_bufs: [hantro_aux_buf; NUM_REF_PICTURES],
    pub scaling_lists: hantro_aux_buf,
    pub ref_bufs_poc: [i32; NUM_REF_PICTURES],
    pub ref_bufs_used: u32,
    pub ctrls: hantro_hevc_dec_ctrls,
    pub num_tile_cols_allocated: c_uint,
    pub use_compression: bool,
}

//
// struct hantro_mpeg2_dec_hw_ctx
//
// @qtable:		Quantization table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_mpeg2_dec_hw_ctx {
    pub qtable: hantro_aux_buf,
}

//
// struct hantro_vp8_dec_hw_ctx
//
// @segment_map:	Segment map buffer.
// @prob_tbl:		Probability table buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_vp8_dec_hw_ctx {
    pub segment_map: hantro_aux_buf,
    pub prob_tbl: hantro_aux_buf,
}

//
// struct hantro_vp9_frame_info
//
// @valid: frame info valid flag
// @frame_context_idx: index of frame context
// @reference_mode: inter prediction type
// @tx_mode: transform mode
// @interpolation_filter: filter selection for inter prediction
// @flags: frame flags
// @timestamp: frame timestamp
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_vp9_frame_info {
    pub 1: u32 valid :,
    pub 2: u32 frame_context_idx :,
    pub 2: u32 reference_mode :,
    pub 3: u32 tx_mode :,
    pub 3: u32 interpolation_filter :,
    pub flags: u32,
    pub timestamp: u64,
}

pub const MAX_SB_COLS: c_int = 64;
pub const MAX_SB_ROWS: c_int = 34;
//
// struct hantro_vp9_dec_hw_ctx
//
// @tile_edge: auxiliary DMA buffer for tile edge processing
// @segment_map: auxiliary DMA buffer for segment map
// @misc: auxiliary DMA buffer for tile info, probabilities and hw counters
// @cnts: vp9 library struct for abstracting hw counters access
// @probability_tables: VP9 probability tables implied by the spec
// @frame_context: VP9 frame contexts
// @cur: current frame information
// @last: last frame information
// @bsd_ctrl_offset: bsd offset into tile_edge
// @segment_map_size: size of segment map
// @ctx_counters_offset: hw counters offset into misc
// @tile_info_offset: tile info offset into misc
// @tile_r_info: per-tile information array
// @tile_c_info: per-tile information array
// @last_tile_r: last number of tile rows
// @last_tile_c: last number of tile cols
// @last_sbs_r: last number of superblock rows
// @last_sbs_c: last number of superblock cols
// @active_segment: number of active segment (alternating between 0 and 1)
// @feature_enabled: segmentation feature enabled flags
// @feature_data: segmentation feature data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_vp9_dec_hw_ctx {
    pub tile_edge: hantro_aux_buf,
    pub segment_map: hantro_aux_buf,
    pub misc: hantro_aux_buf,
    pub cnts: v4l2_vp9_frame_symbol_counts,
    pub probability_tables: v4l2_vp9_frame_context,
    pub frame_context: [v4l2_vp9_frame_context; 4],
    pub cur: hantro_vp9_frame_info,
    pub last: hantro_vp9_frame_info,
    pub bsd_ctrl_offset: c_uint,
    pub segment_map_size: c_uint,
    pub ctx_counters_offset: c_uint,
    pub tile_info_offset: c_uint,
    pub tile_r_info: [c_ushort; MAX_SB_ROWS],
    pub tile_c_info: [c_ushort; MAX_SB_COLS],
    pub last_tile_r: c_uint,
    pub last_tile_c: c_uint,
    pub last_sbs_r: c_uint,
    pub last_sbs_c: c_uint,
    pub active_segment: c_uint,
    pub feature_enabled: [u8; 8],
    pub feature_data: [i16; 8][4],
}

//
// struct hantro_av1_dec_ctrls
// @sequence:		AV1 Sequence
// @tile_group_entry:	AV1 Tile Group entry
// @frame:		AV1 Frame Header OBU
// @film_grain:		AV1 Film Grain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_av1_dec_ctrls {
    pub sequence: *const v4l2_ctrl_av1_sequence,
    pub tile_group_entry: *const v4l2_ctrl_av1_tile_group_entry,
    pub frame: *const v4l2_ctrl_av1_frame,
    pub film_grain: *const v4l2_ctrl_av1_film_grain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_av1_frame_ref {
    pub width: c_int,
    pub height: c_int,
    pub mi_cols: c_int,
    pub mi_rows: c_int,
    pub timestamp: u64,
    pub frame_type: v4l2_av1_frame_type,
    pub used: bool,
    pub order_hint: u32,
    pub order_hints: [u32; V4L2_AV1_TOTAL_REFS_PER_FRAME],
    pub vb2_ref: *mut vb2_v4l2_buffer,
}

//
// struct hantro_av1_dec_hw_ctx
// @db_data_col:	db tile col data buffer
// @db_ctrl_col:	db tile col ctrl buffer
// @cdef_col:		cdef tile col buffer
// @sr_col:		sr tile col buffer
// @lr_col:		lr tile col buffer
// @global_model:	global model buffer
// @tile_info:		tile info buffer
// @segment:		segmentation info buffer
// @film_grain:		film grain buffer
// @prob_tbl:		probability table
// @prob_tbl_out:	probability table output
// @tile_buf:		tile buffer
// @ctrls:		V4L2 controls attached to a run
// @frame_refs:		reference frames info slots
// @ref_frame_sign_bias: array of sign bias
// @num_tile_cols_allocated: number of allocated tiles
// @cdfs:		current probabilities structure
// @cdfs_ndvc:		current mv probabilities structure
// @default_cdfs:	default probabilities structure
// @default_cdfs_ndvc:	default mv probabilties structure
// @cdfs_last:		stored probabilities structures
// @cdfs_last_ndvc:	stored mv probabilities structures
// @current_frame_index: index of the current in frame_refs array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_av1_dec_hw_ctx {
    pub db_data_col: hantro_aux_buf,
    pub db_ctrl_col: hantro_aux_buf,
    pub cdef_col: hantro_aux_buf,
    pub sr_col: hantro_aux_buf,
    pub lr_col: hantro_aux_buf,
    pub global_model: hantro_aux_buf,
    pub tile_info: hantro_aux_buf,
    pub segment: hantro_aux_buf,
    pub film_grain: hantro_aux_buf,
    pub prob_tbl: hantro_aux_buf,
    pub prob_tbl_out: hantro_aux_buf,
    pub tile_buf: hantro_aux_buf,
    pub ctrls: hantro_av1_dec_ctrls,
    pub frame_refs: [hantro_av1_frame_ref; AV1_MAX_FRAME_BUF_COUNT],
    pub ref_frame_sign_bias: [u32; V4L2_AV1_TOTAL_REFS_PER_FRAME],
    pub num_tile_cols_allocated: c_uint,
    pub cdfs: *mut av1cdfs,
    pub cdfs_ndvc: *mut mvcdfs,
    pub default_cdfs: av1cdfs,
    pub default_cdfs_ndvc: mvcdfs,
    pub cdfs_last: [av1cdfs; NUM_REF_FRAMES],
    pub cdfs_last_ndvc: [mvcdfs; NUM_REF_FRAMES],
    pub current_frame_index: c_int,
}

//
// struct hantro_postproc_ctx
//
// @dec_q:		References buffers, in decoder format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_postproc_ctx {
    pub dec_q: [hantro_aux_buf; MAX_POSTPROC_BUFFERS],
}

//
// struct hantro_postproc_ops - post-processor operations
//
// @enable:		Enable the post-processor block. Optional.
// @disable:		Disable the post-processor block. Optional.
// @enum_framesizes:	Enumerate possible scaled output formats.
// Returns zero if OK, a negative value in error cases.
// Optional.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_postproc_ops {
    pub ctx): *mut *mut void (enable)(struct hantro_ctx,
    pub ctx): *mut *mut void (disable)(struct hantro_ctx,
    pub fsize): *mut *mut *mut int (enum_framesizes)(struct hantro_ctx ctx, struct v4l2_frmsizeenum,
}

//
// struct hantro_codec_ops - codec mode specific operations
//
// @init:	If needed, can be used for initialization.
// Optional and called from process context.
// @exit:	If needed, can be used to undo the .init phase.
// Optional and called from process context.
// @run:	Start single {en,de)coding job. Called from atomic context
// to indicate that a pair of buffers is ready and the hardware
// should be programmed and started. Returns zero if OK, a
// negative value in error cases.
// @done:	Read back processing results and additional data from hardware.
// @reset:	Reset the hardware in case of a timeout.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hantro_codec_ops {
    pub ctx): *mut *mut int (init)(struct hantro_ctx,
    pub ctx): *mut *mut void (exit)(struct hantro_ctx,
    pub ctx): *mut *mut int (run)(struct hantro_ctx,
    pub ctx): *mut *mut void (done)(struct hantro_ctx,
    pub ctx): *mut *mut void (reset)(struct hantro_ctx,
}

//
// enum hantro_enc_fmt - source format ID for hardware registers.
//
// @ROCKCHIP_VPU_ENC_FMT_YUV420P: Y/CbCr 4:2:0 planar format
// @ROCKCHIP_VPU_ENC_FMT_YUV420SP: Y/CbCr 4:2:0 semi-planar format
// @ROCKCHIP_VPU_ENC_FMT_YUYV422: YUV 4:2:2 packed format (YUYV)
// @ROCKCHIP_VPU_ENC_FMT_UYVY422: YUV 4:2:2 packed format (UYVY)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hantro_enc_fmt {
    ROCKCHIP_VPU_ENC_FMT_YUV420P = 0,
    ROCKCHIP_VPU_ENC_FMT_YUV420SP = 1,
    ROCKCHIP_VPU_ENC_FMT_YUYV422 = 2,
    ROCKCHIP_VPU_ENC_FMT_UYVY422 = 3,
}

extern "C" {
    pub fn hantro_watchdog(work: *mut work_struct);
}
extern "C" {
    pub fn hantro_start_prepare_run(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_end_prepare_run(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_g1_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hantro_g1_reset(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_h1_jpeg_enc_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn rockchip_vpu2_jpeg_enc_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_h1_jpeg_enc_done(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn rockchip_vpu2_jpeg_enc_done(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_h264_dec_prepare_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn rockchip_vpu2_h264_dec_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_g1_h264_dec_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_h264_dec_init(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_h264_dec_exit(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_hevc_dec_init(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_hevc_dec_exit(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_g2_hevc_dec_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_hevc_dec_prepare_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_hevc_ref_init(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_hevc_get_ref_buf(ctx: *mut hantro_ctx, poc: i32) -> dma_addr_t;
}
extern "C" {
    pub fn hantro_hevc_add_ref_buf(ctx: *mut hantro_ctx, poc: c_int, addr: dma_addr_t) -> c_int;
}
extern "C" {
    pub fn rockchip_vpu981_av1_dec_init(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn rockchip_vpu981_av1_dec_exit(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn rockchip_vpu981_av1_dec_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn rockchip_vpu981_av1_dec_done(ctx: *mut hantro_ctx);
}
//
// There can be up to (CTBs x 64) number of blocks,
// and the motion vector for each block needs 16 bytes.
//
// A decoded 8-bit 4:2:0 NV12 frame may need memory for up to
// 448 bytes per macroblock with additional 32 bytes on
// multi-core variants.
//
// The H264 decoder needs extra space on the output buffers
// to store motion vectors. This is needed for reference
// frames and only if the format is non-post-processed NV12.
//
// Memory layout is as follow:
//
// +---------------------------+
// | Y-plane   256 bytes x MBs |
// +---------------------------+
// | UV-plane  128 bytes x MBs |
// +---------------------------+
// | MV buffer  64 bytes x MBs |
// +---------------------------+
// | MC sync          32 bytes |
// +---------------------------+
//
// A CTB can be 64x64, 32x32 or 16x16.
// Allocated memory for the "worse" case: 16x16
//
extern "C" {
    pub fn round_up(pic_height_in_cbsy: *mut *mut pic_width_in_cbsy, _arg: CBS_SIZE) -> return;
}
extern "C" {
    pub fn round_up(pic_height_in_cbsc: *mut *mut pic_width_in_cbsc, _arg: CBS_SIZE) -> return;
}
extern "C" {
    pub fn DIV_ROUND_UP(_arg: dimension, _arg: 64) -> return;
}
extern "C" {
    pub fn hantro_g2_chroma_offset(ctx: *mut hantro_ctx) -> usize;
}
extern "C" {
    pub fn hantro_g2_motion_vectors_offset(ctx: *mut hantro_ctx) -> usize;
}
extern "C" {
    pub fn hantro_g2_luma_compress_offset(ctx: *mut hantro_ctx) -> usize;
}
extern "C" {
    pub fn hantro_g2_chroma_compress_offset(ctx: *mut hantro_ctx) -> usize;
}
extern "C" {
    pub fn hantro_g1_mpeg2_dec_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn rockchip_vpu2_mpeg2_dec_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_mpeg2_dec_init(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_mpeg2_dec_exit(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_g1_vp8_dec_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn rockchip_vpu2_vp8_dec_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_vp8_dec_init(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_vp8_dec_exit(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_g2_vp9_dec_run(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_g2_vp9_dec_done(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_vp9_dec_init(ctx: *mut hantro_ctx) -> c_int;
}
extern "C" {
    pub fn hantro_vp9_dec_exit(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_g2_check_idle(vpu: *mut hantro_dev);
}
extern "C" {
    pub fn hantro_g2_reset(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_g2_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
