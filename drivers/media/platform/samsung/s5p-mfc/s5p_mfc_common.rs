//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-mfc/s5p_mfc_common.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Samsung S5P Multi Format Codec v 5.0
//
// This file contains definitions of enums and structs used by the codec
// driver.
//
// Copyright (C) 2011 Samsung Electronics Co., Ltd.
// Kamil Debski, <k.debski@samsung.com>
//

// Definitions related to MFC memory
// Offset base used to differentiate between CAPTURE and OUTPUT
// while mmaping

pub const BANK_L_CTX: c_int = 0;
pub const BANK_R_CTX: c_int = 1;
pub const BANK_CTX_NUM: c_int = 2;
pub const MFC_BANK1_ALIGN_ORDER: c_int = 13;
pub const MFC_BANK2_ALIGN_ORDER: c_int = 13;
pub const MFC_BASE_ALIGN_ORDER: c_int = 17;
pub const MFC_FW_MAX_VERSIONS: c_int = 2;

// MFC definitions
pub const MFC_MAX_EXTRA_DPB: c_int = 5;
pub const MFC_MAX_BUFFERS: c_int = 32;
pub const MFC_NUM_CONTEXTS: c_int = 4;
// Interrupt timeout
pub const MFC_INT_TIMEOUT: c_int = 2000;
// Busy wait timeout
pub const MFC_BW_TIMEOUT: c_int = 500;
// Watchdog interval
pub const MFC_WATCHDOG_INTERVAL: c_int = 1000;
// After how many executions watchdog should assume lock up
pub const MFC_WATCHDOG_CNT: c_int = 10;

pub const MFC_ENC_CAP_PLANE_COUNT: c_int = 1;
pub const MFC_ENC_OUT_PLANE_COUNT: c_int = 2;
pub const VB2_MAX_PLANE_COUNT: c_int = 3;
pub const STUFF_BYTE: c_int = 4;
pub const MFC_MAX_CTRLS: c_int = 128;

pub const S5P_MFC_CODEC_H264_DEC: c_int = 0;
pub const S5P_MFC_CODEC_H264_MVC_DEC: c_int = 1;
pub const S5P_MFC_CODEC_VC1_DEC: c_int = 2;
pub const S5P_MFC_CODEC_MPEG4_DEC: c_int = 3;
pub const S5P_MFC_CODEC_MPEG2_DEC: c_int = 4;
pub const S5P_MFC_CODEC_H263_DEC: c_int = 5;
pub const S5P_MFC_CODEC_VC1RCV_DEC: c_int = 6;
pub const S5P_MFC_CODEC_VP8_DEC: c_int = 7;
pub const S5P_MFC_CODEC_HEVC_DEC: c_int = 17;
pub const S5P_MFC_CODEC_VP9_DEC: c_int = 18;
pub const S5P_MFC_CODEC_H264_ENC: c_int = 20;
pub const S5P_MFC_CODEC_H264_MVC_ENC: c_int = 21;
pub const S5P_MFC_CODEC_MPEG4_ENC: c_int = 22;
pub const S5P_MFC_CODEC_H263_ENC: c_int = 23;
pub const S5P_MFC_CODEC_VP8_ENC: c_int = 24;
pub const S5P_MFC_CODEC_HEVC_ENC: c_int = 26;
pub const S5P_MFC_R2H_CMD_EMPTY: c_int = 0;
pub const S5P_MFC_R2H_CMD_SYS_INIT_RET: c_int = 1;
pub const S5P_MFC_R2H_CMD_OPEN_INSTANCE_RET: c_int = 2;
pub const S5P_MFC_R2H_CMD_SEQ_DONE_RET: c_int = 3;
pub const S5P_MFC_R2H_CMD_INIT_BUFFERS_RET: c_int = 4;
pub const S5P_MFC_R2H_CMD_CLOSE_INSTANCE_RET: c_int = 6;
pub const S5P_MFC_R2H_CMD_SLEEP_RET: c_int = 7;
pub const S5P_MFC_R2H_CMD_WAKEUP_RET: c_int = 8;
pub const S5P_MFC_R2H_CMD_COMPLETE_SEQ_RET: c_int = 9;
pub const S5P_MFC_R2H_CMD_DPB_FLUSH_RET: c_int = 10;
pub const S5P_MFC_R2H_CMD_NAL_ABORT_RET: c_int = 11;
pub const S5P_MFC_R2H_CMD_FW_STATUS_RET: c_int = 12;
pub const S5P_MFC_R2H_CMD_FRAME_DONE_RET: c_int = 13;
pub const S5P_MFC_R2H_CMD_FIELD_DONE_RET: c_int = 14;
pub const S5P_MFC_R2H_CMD_SLICE_DONE_RET: c_int = 15;
pub const S5P_MFC_R2H_CMD_ENC_BUFFER_FUL_RET: c_int = 16;
pub const S5P_MFC_R2H_CMD_ERR_RET: c_int = 32;
pub const MFC_MAX_CLOCKS: c_int = 4;

//
// enum s5p_mfc_fmt_type - type of the pixelformat
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s5p_mfc_fmt_type {
    MFC_FMT_DEC,
    MFC_FMT_ENC,
    MFC_FMT_RAW,
}

//
// enum s5p_mfc_inst_type - The type of an MFC instance.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s5p_mfc_inst_type {
    MFCINST_INVALID,
    MFCINST_DECODER,
    MFCINST_ENCODER,
}

//
// enum s5p_mfc_inst_state - The state of an MFC instance.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s5p_mfc_inst_state {
    MFCINST_FREE = 0,
    MFCINST_INIT = 100,
    MFCINST_GOT_INST,
    MFCINST_HEAD_PARSED,
    MFCINST_HEAD_PRODUCED,
    MFCINST_BUFS_SET,
    MFCINST_RUNNING,
    MFCINST_FINISHING,
    MFCINST_FINISHED,
    MFCINST_RETURN_INST,
    MFCINST_ERROR,
    MFCINST_ABORT,
    MFCINST_FLUSH,
    MFCINST_RES_CHANGE_INIT,
    MFCINST_RES_CHANGE_FLUSH,
    MFCINST_RES_CHANGE_END,
    MFCINST_NAL_ABORT,
}

//
// enum s5p_mfc_queue_state - The state of buffer queue.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s5p_mfc_queue_state {
    QUEUE_FREE,
    QUEUE_BUFS_REQUESTED,
    QUEUE_BUFS_QUERIED,
    QUEUE_BUFS_MMAPED,
}

//
// enum s5p_mfc_decode_arg - type of frame decoding
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s5p_mfc_decode_arg {
    MFC_DEC_FRAME,
    MFC_DEC_LAST_FRAME,
    MFC_DEC_RES_CHANGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s5p_mfc_fw_ver {
    MFC_FW_V1,
    MFC_FW_V2,
}

//
// struct s5p_mfc_buf - MFC buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_buf {
    pub b: *mut vb2_v4l2_buffer,
    pub list: list_head,
    pub luma: usize,
    pub chroma: usize,
    pub chroma_1: usize,
    pub raw: },
    pub stream: usize,
    pub cookie: },
    pub flags: c_int,
}

//
// struct s5p_mfc_pm - power management data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_pm {
    pub clock_gate: *mut clk,
    pub clk_names: *const *const c_char,
    pub clocks: [*mut clk; MFC_MAX_CLOCKS],
    pub num_clocks: c_int,
    pub use_clock_gating: bool,
    pub device: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_buf_size_v5 {
    pub h264_ctx: c_uint,
    pub non_h264_ctx: c_uint,
    pub dsc: c_uint,
    pub shm: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_buf_size_v6 {
    pub dev_ctx: c_uint,
    pub h264_dec_ctx: c_uint,
    pub other_dec_ctx: c_uint,
    pub h264_enc_ctx: c_uint,
    pub hevc_enc_ctx: c_uint,
    pub other_enc_ctx: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_buf_size {
    pub fw: c_uint,
    pub cpb: c_uint,
    pub priv: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_variant {
    pub version: c_uint,
    pub port_num: c_uint,
    pub version_bit: u32,
    pub buf_size: *const s5p_mfc_buf_size,
    pub fw_name: [*const c_char; MFC_FW_MAX_VERSIONS],
    pub clk_names: [*const c_char; MFC_MAX_CLOCKS],
    pub num_clocks: c_int,
    pub use_clock_gating: bool,
}

//
// struct s5p_mfc_priv_buf - represents internal used buffer
// @ofs:		offset of each buffer, will be used for MFC
// @virt:		kernel virtual address, only valid when the
// buffer accessed by driver
// @dma:		DMA address, only valid when kernel DMA API used
// @size:		size of the buffer
// @ctx:		memory context (bank) used for this allocation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_priv_buf {
    pub ofs: c_ulong,
    pub virt: *mut c_void,
    pub dma: dma_addr_t,
    pub size: usize,
    pub ctx: c_uint,
}

//
// struct s5p_mfc_dev - The struct containing driver internal parameters.
//
// @v4l2_dev:		v4l2_device
// @vfd_dec:		video device for decoding
// @vfd_enc:		video device for encoding
// @plat_dev:		platform device
// @mem_dev:		child devices of the memory banks
// @regs_base:		base address of the MFC hw registers
// @irq:		irq resource
// @dec_ctrl_handler:	control framework handler for decoding
// @enc_ctrl_handler:	control framework handler for encoding
// @pm:			power management control
// @variant:		MFC hardware variant information
// @num_inst:		counter of active MFC instances
// @irqlock:		lock for operations on videobuf2 queues
// @condlock:		lock for changing/checking if a context is ready to be
// processed
// @mfc_mutex:		lock for video_device
// @int_cond:		variable used by the waitqueue
// @int_type:		type of last interrupt
// @int_err:		error number for last interrupt
// @queue:		waitqueue for waiting for completion of device commands
// @fw_buf:		the firmware buffer data structure
// @mem_size:		size of the firmware operation memory
// @mem_base:		base DMA address of the firmware operation memory
// @mem_bitmap:		bitmap for managing MFC internal buffer allocations
// @mem_virt:		virtual address of the firmware operation memory
// @dma_base:		address of the beginning of memory banks
// @hw_lock:		used for hardware locking
// @ctx:		array of driver contexts
// @curr_ctx:		number of the currently running context
// @ctx_work_bits:	used to mark which contexts are waiting for hardware
// @watchdog_cnt:	counter for the watchdog
// @watchdog_timer:	timer for the watchdog
// @watchdog_workqueue:	workqueue for the watchdog
// @watchdog_work:	worker for the watchdog
// @enter_suspend:	flag set when entering suspend
// @ctx_buf:		common context memory (MFCv6)
// @warn_start:		hardware error code from which warnings start
// @mfc_ops:		ops structure holding HW operation function pointers
// @mfc_cmds:		cmd structure holding HW commands function pointers
// @mfc_regs:		structure holding MFC registers
// @fw_ver:		loaded firmware sub-version
// @fw_get_done:	flag set when request_firmware() is complete and
// copied into fw_buf
// @risc_on:		flag indicates RISC is on or off
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_dev {
    pub v4l2_dev: v4l2_device,
    pub vfd_dec: *mut video_device,
    pub vfd_enc: *mut video_device,
    pub plat_dev: *mut platform_device,
    pub mem_dev: [*mut device; BANK_CTX_NUM],
    pub regs_base: *mut void __iomem,
    pub irq: c_int,
    pub dec_ctrl_handler: v4l2_ctrl_handler,
    pub enc_ctrl_handler: v4l2_ctrl_handler,
    pub pm: s5p_mfc_pm,
    pub variant: *const s5p_mfc_variant,
    pub num_inst: c_int,
    pub /: *mut *mut spinlock_t irqlock; / lock when operating on context,
    pub is: *mut *mut spinlock_t condlock; / lock when changing/checking if a context,
    pub /: *mut *mut mutex mfc_mutex; / video_device lock,
    pub int_cond: c_int,
    pub int_type: c_int,
    pub int_err: c_uint,
    pub queue: wait_queue_head_t,
    pub fw_buf: s5p_mfc_priv_buf,
    pub mem_size: usize,
    pub mem_base: dma_addr_t,
    pub mem_bitmap: *mut c_ulong,
    pub mem_virt: *mut c_void,
    pub dma_base: [dma_addr_t; BANK_CTX_NUM],
    pub hw_lock: c_ulong,
    pub ctx: [*mut s5p_mfc_ctx; MFC_NUM_CONTEXTS],
    pub curr_ctx: c_int,
    pub ctx_work_bits: c_ulong,
    pub watchdog_cnt: core::sync::atomic::AtomicI32,
    pub watchdog_timer: timer_list,
    pub watchdog_workqueue: *mut workqueue_struct,
    pub watchdog_work: work_struct,
    pub enter_suspend: c_ulong,
    pub ctx_buf: s5p_mfc_priv_buf,
    pub warn_start: c_int,
    pub mfc_ops: *const s5p_mfc_hw_ops,
    pub mfc_cmds: *const s5p_mfc_hw_cmds,
    pub mfc_regs: *const s5p_mfc_regs,
    pub fw_ver: s5p_mfc_fw_ver,
    pub fw_get_done: bool,
    pub /: *mut *mut bool risc_on; / indicates if RISC is on or off,
}

//
// struct s5p_mfc_h264_enc_params - encoding parameters for h264
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_h264_enc_params {
    pub profile: v4l2_mpeg_video_h264_profile,
    pub loop_filter_mode: v4l2_mpeg_video_h264_loop_filter_mode,
    pub loop_filter_alpha: i8,
    pub loop_filter_beta: i8,
    pub entropy_mode: v4l2_mpeg_video_h264_entropy_mode,
    pub max_ref_pic: u8,
    pub num_ref_pic_4p: u8,
    pub _8x8_transform: c_int,
    pub rc_mb_dark: c_int,
    pub rc_mb_smooth: c_int,
    pub rc_mb_static: c_int,
    pub rc_mb_activity: c_int,
    pub vui_sar: c_int,
    pub vui_sar_idc: u8,
    pub vui_ext_sar_width: u16,
    pub vui_ext_sar_height: u16,
    pub open_gop: c_int,
    pub open_gop_size: u16,
    pub rc_frame_qp: u8,
    pub rc_min_qp: u8,
    pub rc_max_qp: u8,
    pub rc_p_frame_qp: u8,
    pub rc_b_frame_qp: u8,
    pub level_v4l2: v4l2_mpeg_video_h264_level,
    pub level: c_int,
    pub cpb_size: u16,
    pub interlace: c_int,
    pub hier_qp: u8,
    pub hier_qp_type: u8,
    pub hier_qp_layer: u8,
    pub hier_qp_layer_qp: [u8; 7],
    pub sei_frame_packing: u8,
    pub sei_fp_curr_frame_0: u8,
    pub sei_fp_arrangement_type: u8,
    pub fmo: u8,
    pub fmo_map_type: u8,
    pub fmo_slice_grp: u8,
    pub fmo_chg_dir: u8,
    pub fmo_chg_rate: u32,
    pub fmo_run_len: [u32; 4],
    pub aso: u8,
    pub aso_slice_order: [u32; 8],
}

//
// struct s5p_mfc_mpeg4_enc_params - encoding parameters for h263 and mpeg4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_mpeg4_enc_params {
// MPEG4 Only
    pub profile: v4l2_mpeg_video_mpeg4_profile,
    pub quarter_pixel: c_int,
// Common for MPEG4, H263
    pub vop_time_res: u16,
    pub vop_frm_delta: u16,
    pub rc_frame_qp: u8,
    pub rc_min_qp: u8,
    pub rc_max_qp: u8,
    pub rc_p_frame_qp: u8,
    pub rc_b_frame_qp: u8,
    pub level_v4l2: v4l2_mpeg_video_mpeg4_level,
    pub level: c_int,
}

//
// struct s5p_mfc_vp8_enc_params - encoding parameters for vp8
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_vp8_enc_params {
    pub imd_4x4: u8,
    pub num_partitions: v4l2_vp8_num_partitions,
    pub num_ref: v4l2_vp8_num_ref_frames,
    pub filter_level: u8,
    pub filter_sharpness: u8,
    pub golden_frame_ref_period: u32,
    pub golden_frame_sel: v4l2_vp8_golden_frame_sel,
    pub hier_layer: u8,
    pub hier_layer_qp: [u8; 3],
    pub rc_min_qp: u8,
    pub rc_max_qp: u8,
    pub rc_frame_qp: u8,
    pub rc_p_frame_qp: u8,
    pub profile: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_hevc_enc_params {
    pub profile: v4l2_mpeg_video_hevc_profile,
    pub level: c_int,
    pub level_v4l2: v4l2_mpeg_video_h264_level,
    pub tier: u8,
    pub rc_framerate: u32,
    pub rc_min_qp: u8,
    pub rc_max_qp: u8,
    pub rc_lcu_dark: u8,
    pub rc_lcu_smooth: u8,
    pub rc_lcu_static: u8,
    pub rc_lcu_activity: u8,
    pub rc_frame_qp: u8,
    pub rc_p_frame_qp: u8,
    pub rc_b_frame_qp: u8,
    pub max_partition_depth: u8,
    pub num_refs_for_p: u8,
    pub refreshtype: u8,
    pub refreshperiod: u16,
    pub lf_beta_offset_div2: i32,
    pub lf_tc_offset_div2: i32,
    pub loopfilter: u8,
    pub loopfilter_disable: u8,
    pub loopfilter_across: u8,
    pub nal_control_length_filed: u8,
    pub nal_control_user_ref: u8,
    pub nal_control_store_ref: u8,
    pub const_intra_period_enable: u8,
    pub lossless_cu_enable: u8,
    pub wavefront_enable: u8,
    pub enable_ltr: u8,
    pub hier_qp_enable: u8,
    pub hier_qp_type: v4l2_mpeg_video_hevc_hier_coding_type,
    pub num_hier_layer: u8,
    pub hier_qp_layer: [u8; 7],
    pub hier_bit_layer: [u32; 7],
    pub sign_data_hiding: u8,
    pub general_pb_enable: u8,
    pub temporal_id_enable: u8,
    pub strong_intra_smooth: u8,
    pub intra_pu_split_disable: u8,
    pub tmv_prediction_disable: u8,
    pub max_num_merge_mv: u8,
    pub eco_mode_enable: u8,
    pub encoding_nostartcode_enable: u8,
    pub size_of_length_field: u8,
    pub prepend_sps_pps_to_idr: u8,
}

//
// struct s5p_mfc_enc_params - general encoding parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_enc_params {
    pub width: u16,
    pub height: u16,
    pub mv_h_range: u32,
    pub mv_v_range: u32,
    pub gop_size: u16,
    pub slice_mode: v4l2_mpeg_video_multi_slice_mode,
    pub slice_mb: u16,
    pub slice_bit: u32,
    pub intra_refresh_mb: u16,
    pub pad: c_int,
    pub pad_luma: u8,
    pub pad_cb: u8,
    pub pad_cr: u8,
    pub rc_frame: c_int,
    pub rc_mb: c_int,
    pub rc_bitrate: u32,
    pub rc_reaction_coeff: u16,
    pub vbv_size: u16,
    pub vbv_delay: u32,
    pub seq_hdr_mode: v4l2_mpeg_video_header_mode,
    pub frame_skip_mode: v4l2_mpeg_mfc51_video_frame_skip_mode,
    pub fixed_target_bit: c_int,
    pub num_b_frame: u8,
    pub rc_framerate_num: u32,
    pub rc_framerate_denom: u32,
    pub h264: s5p_mfc_h264_enc_params,
    pub mpeg4: s5p_mfc_mpeg4_enc_params,
    pub vp8: s5p_mfc_vp8_enc_params,
    pub hevc: s5p_mfc_hevc_enc_params,
    pub codec: },
}

//
// struct s5p_mfc_codec_ops - codec ops, used by encoding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_codec_ops {
// initialization routines
    pub ctx): *mut *mut int (pre_seq_start) (struct s5p_mfc_ctx,
    pub ctx): *mut *mut int (post_seq_start) (struct s5p_mfc_ctx,
// execution routines
    pub ctx): *mut *mut int (pre_frame_start) (struct s5p_mfc_ctx,
    pub ctx): *mut *mut int (post_frame_start) (struct s5p_mfc_ctx,
}

//
// struct s5p_mfc_ctx - This struct contains the instance context
//
// @dev:		pointer to the s5p_mfc_dev of the device
// @fh:			struct v4l2_fh
// @num:		number of the context that this structure describes
// @int_cond:		variable used by the waitqueue
// @int_type:		type of the last interrupt
// @int_err:		error number received from MFC hw in the interrupt
// @queue:		waitqueue that can be used to wait for this context to
// finish
// @src_fmt:		source pixelformat information
// @dst_fmt:		destination pixelformat information
// @vq_src:		vb2 queue for source buffers
// @vq_dst:		vb2 queue for destination buffers
// @src_queue:		driver internal queue for source buffers
// @dst_queue:		driver internal queue for destination buffers
// @src_queue_cnt:	number of buffers queued on the source internal queue
// @dst_queue_cnt:	number of buffers queued on the dest internal queue
// @type:		type of the instance - decoder or encoder
// @state:		state of the context
// @inst_no:		number of hw instance associated with the context
// @img_width:		width of the image that is decoded or encoded
// @img_height:		height of the image that is decoded or encoded
// @buf_width:		width of the buffer for processed image
// @buf_height:		height of the buffer for processed image
// @luma_size:		size of a luma plane
// @chroma_size:	size of a chroma plane
// @mv_size:		size of a motion vectors buffer
// @consumed_stream:	number of bytes that have been used so far from the
// decoding buffer
// @dpb_flush_flag:	flag used to indicate that a DPB buffers are being
// flushed
// @head_processed:	flag mentioning whether the header data is processed
// completely or not
// @bank1:		handle to memory allocated for temporary buffers from
// memory bank 1
// @bank2:		handle to memory allocated for temporary buffers from
// memory bank 2
// @capture_state:	state of the capture buffers queue
// @output_state:	state of the output buffers queue
// @src_bufs:		information on allocated source buffers
// @src_bufs_cnt:	number of allocated source buffers
// @dst_bufs:		information on allocated destination buffers
// @dst_bufs_cnt:	number of allocated destination buffers
// @sequence:		counter for the sequence number for v4l2
// @dec_dst_flag:	flags for buffers queued in the hardware
// @dec_src_buf_size:	size of the buffer for source buffers in decoding
// @codec_mode:		number of codec mode used by MFC hw
// @slice_interface:	slice interface flag
// @loop_filter_mpeg4:	loop filter for MPEG4 flag
// @display_delay:	value of the display delay for H264
// @display_delay_enable:	display delay for H264 enable flag
// @after_packed_pb:	flag used to track buffer when stream is in
// Packed PB format
// @sei_fp_parse:	enable/disable parsing of frame packing SEI information
// @pb_count:		count of the DPB buffers required by MFC hw
// @total_dpb_count:	count of DPB buffers with additional buffers
// requested by the application
// @ctx:		context buffer information
// @dsc:		descriptor buffer information
// @shm:		shared memory buffer information
// @mv_count:		number of MV buffers allocated for decoding
// @enc_params:		encoding parameters for MFC
// @enc_dst_buf_size:	size of the buffers for encoder output
// @luma_dpb_size:	dpb buffer size for luma
// @chroma_dpb_size:	dpb buffer size for chroma
// @me_buffer_size:	size of the motion estimation buffer
// @tmv_buffer_size:	size of temporal predictor motion vector buffer
// @ref_queue:		list of the reference buffers for encoding
// @force_frame_type:	encoder's frame type forcing control
// @ref_queue_cnt:	number of the buffers in the reference list
// @slice_size:		slice size
// @slice_mode:		mode of dividing frames into slices
// @c_ops:		ops for encoding
// @ctrls:		array of controls, used when adding controls to the
// v4l2 control framework
// @ctrl_handler:	handler for v4l2 framework
// @scratch_buf_size:	scratch buffer size
// @is_10bit:		state to check 10bit support
// @is_422:		state to check YUV422 10bit format
// @chroma_size_1:	size of a chroma third plane
// @stride:		size of stride for all planes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_ctx {
    pub dev: *mut s5p_mfc_dev,
    pub fh: v4l2_fh,
    pub num: c_int,
    pub int_cond: c_int,
    pub int_type: c_int,
    pub int_err: c_uint,
    pub queue: wait_queue_head_t,
    pub src_fmt: *const s5p_mfc_fmt,
    pub dst_fmt: *const s5p_mfc_fmt,
    pub vq_src: vb2_queue,
    pub vq_dst: vb2_queue,
    pub src_queue: list_head,
    pub dst_queue: list_head,
    pub src_queue_cnt: c_uint,
    pub dst_queue_cnt: c_uint,
    pub type: s5p_mfc_inst_type,
    pub state: s5p_mfc_inst_state,
    pub inst_no: c_int,
// Image parameters
    pub img_width: c_int,
    pub img_height: c_int,
    pub buf_width: c_int,
    pub buf_height: c_int,
    pub luma_size: c_int,
    pub chroma_size: c_int,
    pub chroma_size_1: c_int,
    pub mv_size: c_int,
    pub consumed_stream: c_ulong,
    pub dpb_flush_flag: c_uint,
    pub head_processed: c_uint,
    pub bank1: s5p_mfc_priv_buf,
    pub bank2: s5p_mfc_priv_buf,
    pub capture_state: s5p_mfc_queue_state,
    pub output_state: s5p_mfc_queue_state,
    pub src_bufs: [s5p_mfc_buf; MFC_MAX_BUFFERS],
    pub src_bufs_cnt: c_int,
    pub dst_bufs: [s5p_mfc_buf; MFC_MAX_BUFFERS],
    pub dst_bufs_cnt: c_int,
    pub sequence: c_uint,
    pub dec_dst_flag: c_ulong,
    pub dec_src_buf_size: usize,
// Control values
    pub codec_mode: c_int,
    pub slice_interface: c_int,
    pub loop_filter_mpeg4: c_int,
    pub display_delay: c_int,
    pub display_delay_enable: c_int,
    pub after_packed_pb: c_int,
    pub sei_fp_parse: c_int,
    pub pb_count: c_int,
    pub total_dpb_count: c_int,
    pub mv_count: c_int,
// Buffers
    pub ctx: s5p_mfc_priv_buf,
    pub dsc: s5p_mfc_priv_buf,
    pub shm: s5p_mfc_priv_buf,
    pub enc_params: s5p_mfc_enc_params,
    pub enc_dst_buf_size: usize,
    pub luma_dpb_size: usize,
    pub chroma_dpb_size: usize,
    pub me_buffer_size: usize,
    pub tmv_buffer_size: usize,
    pub force_frame_type: v4l2_mpeg_mfc51_video_force_frame_type,
    pub ref_queue: list_head,
    pub ref_queue_cnt: c_uint,
    pub slice_mode: v4l2_mpeg_video_multi_slice_mode,
    pub mb: c_uint,
    pub bits: c_uint,
    pub slice_size: },
    pub c_ops: *const s5p_mfc_codec_ops,
    pub ctrls: [*mut v4l2_ctrl; MFC_MAX_CTRLS],
    pub ctrl_handler: v4l2_ctrl_handler,
    pub scratch_buf_size: usize,
    pub is_10bit: c_int,
    pub is_422: c_int,
    pub stride: [c_int; VB2_MAX_PLANE_COUNT],
}

//
// struct s5p_mfc_fmt -	structure used to store information about pixelformats
// used by the MFC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_fmt {
    pub fourcc: u32,
    pub codec_mode: u32,
    pub type: s5p_mfc_fmt_type,
    pub num_planes: u32,
    pub versions: u32,
    pub flags: u32,
}

//
// struct mfc_control -	structure used to store information about MFC controls
// it is used to initialize the control framework.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfc_control {
    pub id: __u32,
    pub type: v4l2_ctrl_type,
    pub /: *mut *mut __u8 name[32]; / Whatever,
    pub /: *mut *mut __s32 minimum; / Note signedness,
    pub maximum: __s32,
    pub step: __s32,
    pub menu_skip_mask: __u32,
    pub default_value: __s32,
    pub flags: __u32,
    pub reserved: [__u32; 2],
    pub is_volatile: __u8,
}

// Macro for making hardware specific calls

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), s5p_mfc_ctx: struct, _arg: fh) -> return;
}

extern "C" {
    pub fn clear_work_bit(ctx: *mut s5p_mfc_ctx);
}
extern "C" {
    pub fn set_work_bit(ctx: *mut s5p_mfc_ctx);
}
extern "C" {
    pub fn clear_work_bit_irqsave(ctx: *mut s5p_mfc_ctx);
}
extern "C" {
    pub fn set_work_bit_irqsave(ctx: *mut s5p_mfc_ctx);
}
extern "C" {
    pub fn s5p_mfc_get_new_ctx(dev: *mut s5p_mfc_dev) -> c_int;
}
extern "C" {
    pub fn s5p_mfc_cleanup_queue(lh: *mut list_head, vq: *mut vb2_queue);
}

