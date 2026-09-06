//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amd/isp4/isp4_fw_cmd_resp.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2025 Advanced Micro Devices, Inc.
//
// Two types of command/response channel.
// Type Global Command has one command/response channel.
// Type Stream Command has one command/response channel.
// -----------                                        ------------
// |         |       ---------------------------      |          |
// |         |  ---->|  Global Command         |----> |          |
// |         |       ---------------------------      |          |
// |         |                                        |          |
// |         |       ---------------------------      |          |
// |         |  ---->|   Stream Command        |----> |          |
// |         |       ---------------------------      |          |
// |         |                                        |          |
// |  HOST   |                                        | Firmware |
// |         |                                        |          |
// |         |       --------------------------       |          |
// |         |  <----|  Global Response       |<----  |          |
// |         |       --------------------------       |          |
// |         |                                        |          |
// |         |       --------------------------       |          |
// |         |  <----|  Stream Response       |<----  |          |
// |         |       --------------------------       |          |
// |         |                                        |          |
// -----------                                        ------------
//
// cmd_id is in the format of following type:
// type: indicate command type, global/stream commands.
// group: indicate the command group.
// id: A unique command identification in one type and group.
// |<-Bit31 ~ Bit24->|<-Bit23 ~ Bit16->|<-Bit15 ~ Bit0->|
// |      type       |      group      |       id       |
//
pub const ISP4FW_CMD_TYPE_SHIFT: c_int = 24;
pub const ISP4FW_CMD_GROUP_SHIFT: c_int = 16;

// Stream  Command

// Stream Buffer Command

//
// resp_id is in the format of following type:
// type: indicate command type, global/stream commands.
// group: indicate the command group.
// id: A unique command identification in one type and group.
// |<-Bit31 ~ Bit24->|<-Bit23 ~ Bit16->|<-Bit15 ~ Bit0->|
// |      type       |      group      |       id       |
//
pub const ISP4FW_RESP_GROUP_SHIFT: c_int = 16;

// General Response

// Notification

pub const ISP4FW_CMD_STATUS_SUCCESS: c_int = 0;
pub const ISP4FW_CMD_STATUS_FAIL: c_int = 1;
pub const ISP4FW_CMD_STATUS_SKIPPED: c_int = 2;
pub const ISP4FW_ADDR_SPACE_TYPE_GPU_VA: c_int = 4;

//
// standard ISP pipeline: mipicsi=>isp
//
pub const ISP4FW_MIPI0_ISP_PIPELINE_ID: c_uint = 0x5f91;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp4fw_sensor_id {
// Sensor id for ISP input from MIPI port 0
    ISP4FW_SENSOR_ID_ON_MIPI0  = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp4fw_stream_id {
    ISP4FW_STREAM_ID_INVALID = -1,
    ISP4FW_STREAM_ID_1 = 0,
    ISP4FW_STREAM_ID_2 = 1,
    ISP4FW_STREAM_ID_3 = 2,
    ISP4FW_STREAM_ID_MAXIMUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp4fw_image_format {
// 4:2:0,semi-planar, 8-bit
    ISP4FW_IMAGE_FORMAT_NV12 = 1,
// interleave, 4:2:2, 8-bit
    ISP4FW_IMAGE_FORMAT_YUV422INTERLEAVED = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp4fw_pipe_out_ch {
    ISP4FW_ISP_PIPE_OUT_CH_PREVIEW = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp4fw_yuv_range {
    ISP4FW_ISP_YUV_RANGE_FULL = 0,     /* YUV value range in 0~255 */
    ISP4FW_ISP_YUV_RANGE_NARROW = 1,   /* YUV value range in 16~235 */
    ISP4FW_ISP_YUV_RANGE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp4fw_buffer_type {
    ISP4FW_BUFFER_TYPE_PREVIEW = 8,
    ISP4FW_BUFFER_TYPE_META_INFO = 10,
    ISP4FW_BUFFER_TYPE_MEM_POOL = 15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp4fw_buffer_status {
// The buffer is INVALID
    ISP4FW_BUFFER_STATUS_INVALID,
// The buffer is not filled with image data
    ISP4FW_BUFFER_STATUS_SKIPPED,
// The buffer is available and awaiting to be filled
    ISP4FW_BUFFER_STATUS_EXIST,
// The buffer is filled with image data
    ISP4FW_BUFFER_STATUS_DONE,
// The buffer is unavailable
    ISP4FW_BUFFER_STATUS_LACK,
// The buffer is dirty, probably caused by LMI leakage
    ISP4FW_BUFFER_STATUS_DIRTY,
    ISP4FW_BUFFER_STATUS_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp4fw_buffer_source {
// The buffer is from the stream buffer queue
    ISP4FW_BUFFER_SOURCE_STREAM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_error_code {
    pub code1: u32,
    pub code2: u32,
    pub code3: u32,
    pub code4: u32,
    pub code5: u32,
}

// Command Structure for FW
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_cmd {
    pub cmd_seq_num: u32,
    pub cmd_id: u32,
    pub cmd_param: [u32; 12],
    pub cmd_stream_id: u16,
    pub cmd_silent_resp: u8,
    pub reserved: u8,
    pub cmd_check_sum: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_resp_cmd_done {
//
// The host2fw command seqNum.
// To indicate which command this response refers to.
//
    pub cmd_seq_num: u32,
// The host2fw command id for host double check.
    pub cmd_id: u32,
//
// Indicate the command process status.
// 0 means success. 1 means fail. 2 means skipped
//
    pub cmd_status: u16,
//
// If cmd_status is 1, the command failed. The host can check
// isp4fw_error_code for details.
//
    pub isp4fw_error_code: u16,
// The response payload type varies by cmd.
    pub payload: [u8; 36],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_resp_param_package {
    pub /: *mut *mut u32 package_addr_lo; / The low 32 bit of the pkg address.,
    pub /: *mut *mut u32 package_addr_hi; / The high 32 bit of the pkg address.,
    pub /: *mut *mut u32 package_size; / The total pkg size in bytes.,
    pub /: *mut *mut u32 package_check_sum; / The byte sum of the pkg.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_resp {
    pub resp_seq_num: u32,
    pub resp_id: u32,
    pub cmd_done: isp4fw_resp_cmd_done,
    pub frame_done: isp4fw_resp_param_package,
    pub resp_param: [u32; 12],
    pub param: },
    pub reserved: [u8; 4],
    pub resp_check_sum: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_mipi_pipe_path_cfg {
    pub b_enable: u32,
    pub isp4fw_sensor_id: isp4fw_sensor_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_isp_pipe_path_cfg {
    pub /: *mut *mut u32 isp_pipe_id; / pipe ids for pipeline construction,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_isp_stream_cfg {
// Isp mipi path
    pub mipi_pipe_path_cfg: isp4fw_mipi_pipe_path_cfg,
// Isp pipe path
    pub isp_pipe_path_cfg: isp4fw_isp_pipe_path_cfg,
// enable TNR
    pub b_enable_tnr: u32,
//
// Number of frames for RTA processing.
// Set to 0 to use the firmware's default value.
//
    pub rta_frames_per_proc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_image_prop {
    pub image_format: isp4fw_image_format,
    pub width: u32,
    pub height: u32,
    pub luma_pitch: u32,
    pub chroma_pitch: u32,
    pub yuv_range: isp4fw_yuv_range,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_buffer {
//
// A check num for debug usage, host can set the buf_tags
// to different numbers
//
    pub buf_tags: u32,
    pub value: u32,
    pub 16: u32 space :,
    pub 16: u32 vmid :,
    pub bit: },
    pub vmid_space: },
    pub /: *mut *mut u32 buf_base_a_lo; / Low address of buffer A,
    pub /: *mut *mut u32 buf_base_a_hi; / High address of buffer A,
    pub /: *mut *mut u32 buf_size_a; / Buffer size of buffer A,
    pub /: *mut *mut u32 buf_base_b_lo; / Low address of buffer B,
    pub /: *mut *mut u32 buf_base_b_hi; / High address of buffer B,
    pub /: *mut *mut u32 buf_size_b; / Buffer size of buffer B,
    pub /: *mut *mut u32 buf_base_c_lo; / Low address of buffer C,
    pub /: *mut *mut u32 buf_base_c_hi; / High address of buffer C,
    pub /: *mut *mut u32 buf_size_c; / Buffer size of buffer C,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_buffer_meta_info {
    pub /: *mut *mut u32 enabled; / enabled flag,
    pub /: *mut *mut isp4fw_buffer_status status; / BufferStatus,
    pub /: *mut *mut isp4fw_error_code err; / err code,
    pub /: *mut *mut isp4fw_buffer_source source; / BufferSource,
    pub /: *mut *mut isp4fw_image_prop image_prop; / image_prop,
    pub /: *mut *mut isp4fw_buffer buffer; / buffer info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_meta_info {
    pub /: *mut *mut u32 poc; / frame id,
    pub /: *mut *mut u32 fc_id; / frame ctl id,
    pub /: *mut *mut u32 time_stamp_lo; / timestamp low 32 bits,
    pub /: *mut *mut u32 time_stamp_hi; / timestamp_high 32 bits,
    pub /: *mut *mut isp4fw_buffer_meta_info preview; / preview BufferMetaInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_cmd_send_buffer {
    pub buffer_type: isp4fw_buffer_type,
    pub /: *mut *mut isp4fw_buffer buffer; / buffer info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_cmd_set_out_ch_prop {
    pub /: *mut *mut isp4fw_pipe_out_ch ch; / ISP output channel,
    pub /: *mut *mut isp4fw_image_prop image_prop; / image property,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_cmd_enable_out_ch {
    pub /: *mut *mut isp4fw_pipe_out_ch ch; / ISP output channel,
    pub /: *mut *mut u32 is_enable; / If channel is enabled or not,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4fw_cmd_set_stream_cfg {
    pub /: *mut *mut isp4fw_isp_stream_cfg stream_cfg; / stream path config,
}
