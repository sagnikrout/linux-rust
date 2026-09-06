//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/drv-intf/cx2341x.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx2341x_port {
    CX2341X_PORT_MEMORY    = 0,
    CX2341X_PORT_STREAMING = 1,
    CX2341X_PORT_SERIAL    = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx2341x_cap {
    CX2341X_CAP_HAS_SLICED_VBI = 1 << 0,
    CX2341X_CAP_HAS_TS	   = 1 << 1,
    CX2341X_CAP_HAS_AC3	   = 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx2341x_mpeg_params {
// misc
    pub capabilities: u32,
    pub port: cx2341x_port,
    pub width: u16,
    pub height: u16,
    pub is_50hz: u16,
// stream
    pub stream_type: v4l2_mpeg_stream_type,
    pub stream_vbi_fmt: v4l2_mpeg_stream_vbi_fmt,
    pub stream_insert_nav_packets: u16,
// audio
    pub audio_sampling_freq: v4l2_mpeg_audio_sampling_freq,
    pub audio_encoding: v4l2_mpeg_audio_encoding,
    pub audio_l2_bitrate: v4l2_mpeg_audio_l2_bitrate,
    pub audio_ac3_bitrate: v4l2_mpeg_audio_ac3_bitrate,
    pub audio_mode: v4l2_mpeg_audio_mode,
    pub audio_mode_extension: v4l2_mpeg_audio_mode_extension,
    pub audio_emphasis: v4l2_mpeg_audio_emphasis,
    pub audio_crc: v4l2_mpeg_audio_crc,
    pub audio_properties: u32,
    pub audio_mute: u16,
// video
    pub video_encoding: v4l2_mpeg_video_encoding,
    pub video_aspect: v4l2_mpeg_video_aspect,
    pub video_b_frames: u16,
    pub video_gop_size: u16,
    pub video_gop_closure: u16,
    pub video_bitrate_mode: v4l2_mpeg_video_bitrate_mode,
    pub video_bitrate: u32,
    pub video_bitrate_peak: u32,
    pub video_temporal_decimation: u16,
    pub video_mute: u16,
    pub video_mute_yuv: u32,
// encoding filters
    pub video_spatial_filter_mode: v4l2_mpeg_cx2341x_video_spatial_filter_mode,
    pub video_spatial_filter: u16,
    pub video_luma_spatial_filter_type: v4l2_mpeg_cx2341x_video_luma_spatial_filter_type,
    pub video_chroma_spatial_filter_type: v4l2_mpeg_cx2341x_video_chroma_spatial_filter_type,
    pub video_temporal_filter_mode: v4l2_mpeg_cx2341x_video_temporal_filter_mode,
    pub video_temporal_filter: u16,
    pub video_median_filter_type: v4l2_mpeg_cx2341x_video_median_filter_type,
    pub video_luma_median_filter_top: u16,
    pub video_luma_median_filter_bottom: u16,
    pub video_chroma_median_filter_top: u16,
    pub video_chroma_median_filter_bottom: u16,
}

pub const CX2341X_MBOX_MAX_DATA: c_int = 16;
extern "C" {
    pub fn cx2341x_fill_defaults(p: *mut cx2341x_mpeg_params);
}
extern "C" {
    pub fn cx2341x_log_status(p: *const cx2341x_mpeg_params, prefix: *const c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx2341x_handler_ops {
// needed for the video clock freq
    pub val): *mut *mut *mut int (s_audio_sampling_freq)(struct cx2341x_handler hdl, u32,
// needed for dualwatch
    pub val): *mut *mut *mut int (s_audio_mode)(struct cx2341x_handler hdl, u32,
// needed for setting up the video resolution
    pub val): *mut *mut *mut int (s_video_encoding)(struct cx2341x_handler hdl, u32,
// needed for setting up the sliced vbi insertion data structures
    pub val): *mut *mut *mut int (s_stream_vbi_fmt)(struct cx2341x_handler hdl, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx2341x_handler {
    pub capabilities: u32,
    pub port: cx2341x_port,
    pub width: u16,
    pub height: u16,
    pub is_50hz: u16,
    pub audio_properties: u32,
    pub hdl: v4l2_ctrl_handler,
    pub priv: *mut c_void,
    pub func: cx2341x_mbox_func,
    pub ops: *const cx2341x_handler_ops,
    pub stream_vbi_fmt: *mut v4l2_ctrl,
// audio cluster
    pub audio_sampling_freq: *mut v4l2_ctrl,
    pub audio_encoding: *mut v4l2_ctrl,
    pub audio_l2_bitrate: *mut v4l2_ctrl,
    pub audio_mode: *mut v4l2_ctrl,
    pub audio_mode_extension: *mut v4l2_ctrl,
    pub audio_emphasis: *mut v4l2_ctrl,
    pub audio_crc: *mut v4l2_ctrl,
    pub audio_ac3_bitrate: *mut v4l2_ctrl,
}

// video gop cluster
// stream type cluster
// video mute cluster
// video filter mode cluster
// video filter type cluster
// video filter cluster
// video median cluster
extern "C" {
    pub fn cx2341x_handler_set_50hz(cxhdl: *mut cx2341x_handler, is_50hz: c_int);
}
extern "C" {
    pub fn cx2341x_handler_setup(cxhdl: *mut cx2341x_handler) -> c_int;
}
extern "C" {
    pub fn cx2341x_handler_set_busy(cxhdl: *mut cx2341x_handler, busy: c_int);
}
// Firmware names

// Decoder firmware for the cx23415 only

// Firmware API commands
// MPEG decoder API, specific to the cx23415
pub const CX2341X_DEC_PING_FW: c_uint = 0x00;
pub const CX2341X_DEC_START_PLAYBACK: c_uint = 0x01;
pub const CX2341X_DEC_STOP_PLAYBACK: c_uint = 0x02;
pub const CX2341X_DEC_SET_PLAYBACK_SPEED: c_uint = 0x03;
pub const CX2341X_DEC_STEP_VIDEO: c_uint = 0x05;
pub const CX2341X_DEC_SET_DMA_BLOCK_SIZE: c_uint = 0x08;
pub const CX2341X_DEC_GET_XFER_INFO: c_uint = 0x09;
pub const CX2341X_DEC_GET_DMA_STATUS: c_uint = 0x0a;
pub const CX2341X_DEC_SCHED_DMA_FROM_HOST: c_uint = 0x0b;
pub const CX2341X_DEC_PAUSE_PLAYBACK: c_uint = 0x0d;
pub const CX2341X_DEC_HALT_FW: c_uint = 0x0e;
pub const CX2341X_DEC_SET_STANDARD: c_uint = 0x10;
pub const CX2341X_DEC_GET_VERSION: c_uint = 0x11;
pub const CX2341X_DEC_SET_STREAM_INPUT: c_uint = 0x14;
pub const CX2341X_DEC_GET_TIMING_INFO: c_uint = 0x15;
pub const CX2341X_DEC_SET_AUDIO_MODE: c_uint = 0x16;
pub const CX2341X_DEC_SET_EVENT_NOTIFICATION: c_uint = 0x17;
pub const CX2341X_DEC_SET_DISPLAY_BUFFERS: c_uint = 0x18;
pub const CX2341X_DEC_EXTRACT_VBI: c_uint = 0x19;
pub const CX2341X_DEC_SET_DECODER_SOURCE: c_uint = 0x1a;
pub const CX2341X_DEC_SET_PREBUFFERING: c_uint = 0x1e;
// MPEG encoder API
pub const CX2341X_ENC_PING_FW: c_uint = 0x80;
pub const CX2341X_ENC_START_CAPTURE: c_uint = 0x81;
pub const CX2341X_ENC_STOP_CAPTURE: c_uint = 0x82;
pub const CX2341X_ENC_SET_AUDIO_ID: c_uint = 0x89;
pub const CX2341X_ENC_SET_VIDEO_ID: c_uint = 0x8b;
pub const CX2341X_ENC_SET_PCR_ID: c_uint = 0x8d;
pub const CX2341X_ENC_SET_FRAME_RATE: c_uint = 0x8f;
pub const CX2341X_ENC_SET_FRAME_SIZE: c_uint = 0x91;
pub const CX2341X_ENC_SET_BIT_RATE: c_uint = 0x95;
pub const CX2341X_ENC_SET_GOP_PROPERTIES: c_uint = 0x97;
pub const CX2341X_ENC_SET_ASPECT_RATIO: c_uint = 0x99;
pub const CX2341X_ENC_SET_DNR_FILTER_MODE: c_uint = 0x9b;
pub const CX2341X_ENC_SET_DNR_FILTER_PROPS: c_uint = 0x9d;
pub const CX2341X_ENC_SET_CORING_LEVELS: c_uint = 0x9f;
pub const CX2341X_ENC_SET_SPATIAL_FILTER_TYPE: c_uint = 0xa1;
pub const CX2341X_ENC_SET_VBI_LINE: c_uint = 0xb7;
pub const CX2341X_ENC_SET_STREAM_TYPE: c_uint = 0xb9;
pub const CX2341X_ENC_SET_OUTPUT_PORT: c_uint = 0xbb;
pub const CX2341X_ENC_SET_AUDIO_PROPERTIES: c_uint = 0xbd;
pub const CX2341X_ENC_HALT_FW: c_uint = 0xc3;
pub const CX2341X_ENC_GET_VERSION: c_uint = 0xc4;
pub const CX2341X_ENC_SET_GOP_CLOSURE: c_uint = 0xc5;
pub const CX2341X_ENC_GET_SEQ_END: c_uint = 0xc6;
pub const CX2341X_ENC_SET_PGM_INDEX_INFO: c_uint = 0xc7;
pub const CX2341X_ENC_SET_VBI_CONFIG: c_uint = 0xc8;
pub const CX2341X_ENC_SET_DMA_BLOCK_SIZE: c_uint = 0xc9;
pub const CX2341X_ENC_GET_PREV_DMA_INFO_MB_10: c_uint = 0xca;
pub const CX2341X_ENC_GET_PREV_DMA_INFO_MB_9: c_uint = 0xcb;
pub const CX2341X_ENC_SCHED_DMA_TO_HOST: c_uint = 0xcc;
pub const CX2341X_ENC_INITIALIZE_INPUT: c_uint = 0xcd;
pub const CX2341X_ENC_SET_FRAME_DROP_RATE: c_uint = 0xd0;
pub const CX2341X_ENC_PAUSE_ENCODER: c_uint = 0xd2;
pub const CX2341X_ENC_REFRESH_INPUT: c_uint = 0xd3;
pub const CX2341X_ENC_SET_COPYRIGHT: c_uint = 0xd4;
pub const CX2341X_ENC_SET_EVENT_NOTIFICATION: c_uint = 0xd5;
pub const CX2341X_ENC_SET_NUM_VSYNC_LINES: c_uint = 0xd6;
pub const CX2341X_ENC_SET_PLACEHOLDER: c_uint = 0xd7;
pub const CX2341X_ENC_MUTE_VIDEO: c_uint = 0xd9;
pub const CX2341X_ENC_MUTE_AUDIO: c_uint = 0xda;
pub const CX2341X_ENC_SET_VERT_CROP_LINE: c_uint = 0xdb;
pub const CX2341X_ENC_MISC: c_uint = 0xdc;
// OSD API, specific to the cx23415
pub const CX2341X_OSD_GET_FRAMEBUFFER: c_uint = 0x41;
pub const CX2341X_OSD_GET_PIXEL_FORMAT: c_uint = 0x42;
pub const CX2341X_OSD_SET_PIXEL_FORMAT: c_uint = 0x43;
pub const CX2341X_OSD_GET_STATE: c_uint = 0x44;
pub const CX2341X_OSD_SET_STATE: c_uint = 0x45;
pub const CX2341X_OSD_GET_OSD_COORDS: c_uint = 0x46;
pub const CX2341X_OSD_SET_OSD_COORDS: c_uint = 0x47;
pub const CX2341X_OSD_GET_SCREEN_COORDS: c_uint = 0x48;
pub const CX2341X_OSD_SET_SCREEN_COORDS: c_uint = 0x49;
pub const CX2341X_OSD_GET_GLOBAL_ALPHA: c_uint = 0x4a;
pub const CX2341X_OSD_SET_GLOBAL_ALPHA: c_uint = 0x4b;
pub const CX2341X_OSD_SET_BLEND_COORDS: c_uint = 0x4c;
pub const CX2341X_OSD_GET_FLICKER_STATE: c_uint = 0x4f;
pub const CX2341X_OSD_SET_FLICKER_STATE: c_uint = 0x50;
pub const CX2341X_OSD_BLT_COPY: c_uint = 0x52;
pub const CX2341X_OSD_BLT_FILL: c_uint = 0x53;
pub const CX2341X_OSD_BLT_TEXT: c_uint = 0x54;
pub const CX2341X_OSD_SET_FRAMEBUFFER_WINDOW: c_uint = 0x56;
pub const CX2341X_OSD_SET_CHROMA_KEY: c_uint = 0x60;
pub const CX2341X_OSD_GET_ALPHA_CONTENT_INDEX: c_uint = 0x61;
pub const CX2341X_OSD_SET_ALPHA_CONTENT_INDEX: c_uint = 0x62;
