//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_instance.h
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

pub const DEFAULT_WIDTH: c_int = 320;
pub const DEFAULT_HEIGHT: c_int = 240;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iris_fmt_type_out {
    IRIS_FMT_H264,
    IRIS_FMT_HEVC,
    IRIS_FMT_VP9,
    IRIS_FMT_AV1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iris_fmt_type_cap {
    IRIS_FMT_NV12,
    IRIS_FMT_QC08C,
    IRIS_FMT_TP10,
    IRIS_FMT_QC10C,
}

//
// struct iris_inst - holds per video instance parameters
//
// @list: used for attach an instance to the core
// @core: pointer to core structure
// @session_id: id of current video session
// @hfi_session_ops: iris HFI session ops
// @ctx_q_lock: lock to serialize queues related ioctls
// @lock: lock to seralise forward and reverse threads
// @fh: reference of v4l2 file handler
// @fmt_src: structure of v4l2_format for source
// @fmt_dst: structure of v4l2_format for destination
// @ctrl_handler: reference of v4l2 ctrl handler
// @domain: domain type: encoder or decoder
// @crop: structure of crop info
// @compose: structure of compose info
// @completion: structure of signal completions
// @flush_completion: structure of signal completions for flush cmd
// @flush_responses_pending: counter to track number of pending flush responses
// @fw_caps: array of supported instance firmware capabilities
// @buffers: array of different iris buffers
// @fw_min_count: minimnum count of buffers needed by fw
// @state: instance state
// @sub_state: instance sub state
// @once_per_session_set: boolean to set once per session property
// @max_input_data_size: max size of input data
// @power: structure of power info
// @icc_data: structure of interconnect data
// @m2m_dev:	a reference to m2m device structure
// @m2m_ctx:	a reference to m2m context structure
// @sequence_cap: a sequence counter for capture queue
// @sequence_out: a sequence counter for output queue
// @tss: timestamp metadata
// @metadata_idx: index for metadata buffer
// @codec: codec type
// @last_buffer_dequeued: a flag to indicate that last buffer is sent by driver
// @last_buf_ns: start time of received input buffer for current one second FPS window
// @frame_counter: input buffer counter for current one second FPS window
// @frame_rate: frame rate of current instance
// @operating_rate: operating rate of current instance
// @hfi_rc_type: rate control type
// @enc_raw_width: source image width for encoder instance
// @enc_raw_height: source image height for encoder instance
// @enc_scale_width: scale width for encoder instance
// @enc_scale_height: scale height for encoder instance
// @hfi_layer_type: hierarchical coding layer type
// @hfi_layer_count: hierarchical coding layer count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_inst {
    pub list: list_head,
    pub core: *mut iris_core,
    pub session_id: u32,
    pub hfi_session_ops: *const iris_hfi_session_ops,
    pub /: *mut *mut mutex ctx_q_lock;/ lock to serialize queues related ioctls,
    pub /: *mut *mut mutex lock; / lock to serialize forward and reverse threads,
    pub fh: v4l2_fh,
    pub fmt_src: *mut v4l2_format,
    pub fmt_dst: *mut v4l2_format,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub domain: domain_type,
    pub crop: iris_hfi_rect_desc,
    pub compose: iris_hfi_rect_desc,
    pub completion: completion,
    pub flush_completion: completion,
    pub flush_responses_pending: u32,
    pub fw_caps: [platform_inst_fw_cap; INST_FW_CAP_MAX],
    pub buffers: [iris_buffers; BUF_TYPE_MAX],
    pub fw_min_count: u32,
    pub state: iris_inst_state,
    pub sub_state: iris_inst_sub_state,
    pub once_per_session_set: bool,
    pub max_input_data_size: usize,
    pub power: iris_inst_power,
    pub icc_data: icc_vote_data,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub m2m_ctx: *mut v4l2_m2m_ctx,
    pub sequence_cap: u32,
    pub sequence_out: u32,
    pub tss: [iris_ts_metadata; VIDEO_MAX_FRAME],
    pub metadata_idx: u32,
    pub codec: u32,
    pub last_buffer_dequeued: bool,
    pub last_buf_ns: u64,
    pub frame_counter: u32,
    pub frame_rate: u32,
    pub operating_rate: u32,
    pub hfi_rc_type: u32,
    pub enc_raw_width: u32,
    pub enc_raw_height: u32,
    pub enc_scale_width: u32,
    pub enc_scale_height: u32,
    pub hfi_layer_type: u32,
    pub hfi_layer_count: u32,
}
