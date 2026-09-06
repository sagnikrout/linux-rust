//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_hfi_gen1_defines.h
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

pub const HFI_VIDEO_ARCH_OX: c_uint = 0x1;
pub const HFI_SESSION_TYPE_ENC: c_int = 1;
pub const HFI_SESSION_TYPE_DEC: c_int = 2;
pub const HFI_VIDEO_CODEC_H264: c_uint = 0x00000002;
pub const HFI_VIDEO_CODEC_HEVC: c_uint = 0x00002000;
pub const HFI_VIDEO_CODEC_VP9: c_uint = 0x00004000;
pub const HFI_ERR_NONE: c_uint = 0x0;
pub const HFI_CMD_SYS_INIT: c_uint = 0x10001;
pub const HFI_CMD_SYS_PC_PREP: c_uint = 0x10002;
pub const HFI_CMD_SYS_SET_PROPERTY: c_uint = 0x10005;
pub const HFI_CMD_SYS_GET_PROPERTY: c_uint = 0x10006;
pub const HFI_CMD_SYS_SESSION_INIT: c_uint = 0x10007;
pub const HFI_CMD_SYS_SESSION_END: c_uint = 0x10008;
pub const HFI_CMD_SESSION_SET_PROPERTY: c_uint = 0x11001;
pub const HFI_CMD_SESSION_SET_BUFFERS: c_uint = 0x11002;
pub const HFI_CMD_SESSION_LOAD_RESOURCES: c_uint = 0x211001;
pub const HFI_CMD_SESSION_START: c_uint = 0x211002;
pub const HFI_CMD_SESSION_STOP: c_uint = 0x211003;
pub const HFI_CMD_SESSION_EMPTY_BUFFER: c_uint = 0x211004;
pub const HFI_CMD_SESSION_FILL_BUFFER: c_uint = 0x211005;
pub const HFI_CMD_SESSION_FLUSH: c_uint = 0x211008;
pub const HFI_CMD_SESSION_RELEASE_BUFFERS: c_uint = 0x21100b;
pub const HFI_CMD_SESSION_RELEASE_RESOURCES: c_uint = 0x21100c;
pub const HFI_CMD_SESSION_CONTINUE: c_uint = 0x21100d;
pub const HFI_ERR_SESSION_UNSUPPORTED_SETTING: c_uint = 0x1008;
pub const HFI_ERR_SESSION_UNSUPPORTED_STREAM: c_uint = 0x100d;
pub const HFI_ERR_SESSION_UNSUPPORT_BUFFERTYPE: c_uint = 0x1010;
pub const HFI_ERR_SESSION_INVALID_SCALE_FACTOR: c_uint = 0x1012;
pub const HFI_ERR_SESSION_UPSCALE_NOT_SUPPORTED: c_uint = 0x1013;
pub const HFI_EVENT_SYS_ERROR: c_uint = 0x1;
pub const HFI_EVENT_SESSION_ERROR: c_uint = 0x2;
pub const HFI_EVENT_DATA_SEQUENCE_CHANGED_SUFFICIENT_BUF_RESOURCES: c_uint = 0x1000001;
pub const HFI_EVENT_DATA_SEQUENCE_CHANGED_INSUFFICIENT_BUF_RESOURCES: c_uint = 0x1000002;
pub const HFI_EVENT_SESSION_SEQUENCE_CHANGED: c_uint = 0x1000003;
pub const HFI_BUFFERFLAG_EOS: c_uint = 0x00000001;
pub const HFI_BUFFERFLAG_TIMESTAMPINVALID: c_uint = 0x00000100;
pub const HFI_FLUSH_OUTPUT: c_uint = 0x1000002;
pub const HFI_FLUSH_OUTPUT2: c_uint = 0x1000003;
pub const HFI_FLUSH_ALL: c_uint = 0x1000004;
pub const HFI_INDEX_EXTRADATA_INPUT_CROP: c_uint = 0x0700000e;
pub const HFI_PROPERTY_PARAM_BUFFER_COUNT_ACTUAL: c_uint = 0x201001;
pub const HFI_PROPERTY_PARAM_UNCOMPRESSED_PLANE_ACTUAL_CONSTRAINTS_INFO: c_uint = 0x201002;
pub const HFI_PROPERTY_PARAM_BUFFER_ALLOC_MODE: c_uint = 0x201008;
pub const HFI_PROPERTY_PARAM_BUFFER_SIZE_ACTUAL: c_uint = 0x20100c;
pub const HFI_PROPERTY_CONFIG_BUFFER_REQUIREMENTS: c_uint = 0x202001;
pub const HFI_PROPERTY_PARAM_VDEC_DPB_COUNTS: c_uint = 0x120300e;
pub const HFI_PROPERTY_CONFIG_VDEC_ENTROPY: c_uint = 0x1204004;
pub const HFI_BUFFER_INPUT: c_uint = 0x1;
pub const HFI_BUFFER_OUTPUT: c_uint = 0x2;
pub const HFI_BUFFER_OUTPUT2: c_uint = 0x3;
pub const HFI_BUFFER_INTERNAL_PERSIST: c_uint = 0x4;
pub const HFI_BUFFER_INTERNAL_PERSIST_1: c_uint = 0x5;
pub const HFI_BUFFER_INTERNAL_SCRATCH: c_uint = 0x6;
pub const HFI_BUFFER_INTERNAL_SCRATCH_1: c_uint = 0x7;
pub const HFI_BUFFER_INTERNAL_SCRATCH_2: c_uint = 0x8;
pub const HFI_PROPERTY_SYS_CODEC_POWER_PLANE_CTRL: c_uint = 0x5;
pub const HFI_PROPERTY_SYS_IMAGE_VERSION: c_uint = 0x6;
pub const HFI_PROPERTY_PARAM_FRAME_SIZE: c_uint = 0x1001;
pub const HFI_PROPERTY_PARAM_UNCOMPRESSED_PLANE_ACTUAL_INFO: c_uint = 0x1002;
pub const HFI_PROPERTY_PARAM_UNCOMPRESSED_FORMAT_SELECT: c_uint = 0x1003;
pub const HFI_PROPERTY_PARAM_PROFILE_LEVEL_CURRENT: c_uint = 0x1005;
pub const HFI_PROPERTY_PARAM_WORK_MODE: c_uint = 0x1015;
pub const HFI_PROPERTY_PARAM_WORK_ROUTE: c_uint = 0x1017;
pub const HFI_PROPERTY_CONFIG_FRAME_RATE: c_uint = 0x2001;
pub const HFI_PROPERTY_CONFIG_VIDEOCORES_USAGE: c_uint = 0x2002;
pub const HFI_PROPERTY_PARAM_VDEC_MULTI_STREAM: c_uint = 0x1003001;
pub const HFI_PROPERTY_PARAM_VDEC_PIXEL_BITDEPTH: c_uint = 0x1003007;
pub const HFI_PROPERTY_PARAM_VDEC_PIC_STRUCT: c_uint = 0x1003009;
pub const HFI_PROPERTY_PARAM_VDEC_COLOUR_SPACE: c_uint = 0x100300a;
pub const HFI_CORE_ID_1: c_int = 1;
pub const HFI_COLOR_FORMAT_NV12: c_uint = 0x02;
pub const HFI_COLOR_FORMAT_NV12_UBWC: c_uint = 0x8002;
pub const HFI_MSG_SYS_INIT: c_uint = 0x20001;
pub const HFI_MSG_SYS_SESSION_INIT: c_uint = 0x20006;
pub const HFI_MSG_SYS_SESSION_END: c_uint = 0x20007;
pub const HFI_MSG_SYS_COV: c_uint = 0x20009;
pub const HFI_MSG_SYS_PROPERTY_INFO: c_uint = 0x2000a;
pub const HFI_MSG_EVENT_NOTIFY: c_uint = 0x21001;
pub const HFI_MSG_SESSION_LOAD_RESOURCES: c_uint = 0x221001;
pub const HFI_MSG_SESSION_START: c_uint = 0x221002;
pub const HFI_MSG_SESSION_STOP: c_uint = 0x221003;
pub const HFI_MSG_SESSION_FLUSH: c_uint = 0x221006;
pub const HFI_MSG_SESSION_EMPTY_BUFFER: c_uint = 0x221007;
pub const HFI_MSG_SESSION_FILL_BUFFER: c_uint = 0x221008;
pub const HFI_MSG_SESSION_RELEASE_RESOURCES: c_uint = 0x22100a;
pub const HFI_MSG_SESSION_RELEASE_BUFFERS: c_uint = 0x22100c;
pub const HFI_GEN1_PICTURE_I: c_uint = 0x00000001;
pub const HFI_GEN1_PICTURE_P: c_uint = 0x00000002;
pub const HFI_GEN1_PICTURE_B: c_uint = 0x00000004;
pub const HFI_GEN1_PICTURE_IDR: c_uint = 0x00000008;
pub const HFI_FRAME_NOTCODED: c_uint = 0x7f002000;
pub const HFI_FRAME_YUV: c_uint = 0x7f004000;
pub const HFI_UNUSED_PICT: c_uint = 0x10000000;
pub const HFI_BUFFERFLAG_DATACORRUPT: c_uint = 0x00000008;
pub const HFI_BUFFERFLAG_DROP_FRAME: c_uint = 0x20000000;
pub const HFI_RATE_CONTROL_OFF: c_uint = 0x1000001;
pub const HFI_RATE_CONTROL_VBR_VFR: c_uint = 0x1000002;
pub const HFI_RATE_CONTROL_VBR_CFR: c_uint = 0x1000003;
pub const HFI_RATE_CONTROL_CBR_VFR: c_uint = 0x1000004;
pub const HFI_RATE_CONTROL_CBR_CFR: c_uint = 0x1000005;
pub const HFI_RATE_CONTROL_CQ: c_uint = 0x1000008;
pub const HFI_H264_ENTROPY_CAVLC: c_uint = 0x1;
pub const HFI_H264_ENTROPY_CABAC: c_uint = 0x2;
pub const HFI_PROPERTY_PARAM_VENC_H264_ENTROPY_CONTROL: c_uint = 0x2005002;
pub const HFI_PROPERTY_PARAM_VENC_H264_DEBLOCK_CONTROL: c_uint = 0x2005003;
pub const HFI_PROPERTY_PARAM_VENC_RATE_CONTROL: c_uint = 0x2005004;
pub const HFI_PROPERTY_PARAM_VENC_SESSION_QP_RANGE_V2: c_uint = 0x2005009;
pub const HFI_INTRA_REFRESH_NONE: c_uint = 0x1;
pub const HFI_INTRA_REFRESH_CYCLIC: c_uint = 0x2;
pub const HFI_INTRA_REFRESH_ADAPTIVE: c_uint = 0x3;
pub const HFI_INTRA_REFRESH_CYCLIC_ADAPTIVE: c_uint = 0x4;
pub const HFI_INTRA_REFRESH_RANDOM: c_uint = 0x5;
pub const HFI_PROPERTY_PARAM_VENC_INTRA_REFRESH: c_uint = 0x200500d;
pub const HFI_LTR_MODE_DISABLE: c_uint = 0x0;
pub const HFI_LTR_MODE_MANUAL: c_uint = 0x1;
pub const HFI_LTR_MODE_PERIODIC: c_uint = 0x2;
pub const HFI_PROPERTY_PARAM_VENC_LTRMODE: c_uint = 0x200501c;
pub const HFI_PROPERTY_PARAM_VENC_MAX_NUM_B_FRAMES: c_uint = 0x2005020;
pub const HFI_PROPERTY_PARAM_VENC_HIER_P_MAX_NUM_ENH_LAYER: c_uint = 0x2005026;
pub const HFI_PROPERTY_CONFIG_VENC_TARGET_BITRATE: c_uint = 0x2006001;
pub const HFI_PROPERTY_CONFIG_VENC_INTRA_PERIOD: c_uint = 0x2006003;
pub const HFI_PROPERTY_CONFIG_VENC_REQUEST_SYNC_FRAME: c_uint = 0x2006004;
pub const HFI_PROPERTY_CONFIG_VENC_MARKLTRFRAME: c_uint = 0x2006009;
pub const HFI_PROPERTY_CONFIG_VENC_USELTRFRAME: c_uint = 0x200600a;
pub const HFI_PROPERTY_CONFIG_VENC_SYNC_FRAME_SEQUENCE_HEADER: c_uint = 0x2006008;
pub const HFI_PROPERTY_CONFIG_VENC_HIER_P_ENH_LAYER: c_uint = 0x200600b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_pkt_hdr {
    pub size: u32,
    pub pkt_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_hdr_pkt {
    pub hdr: hfi_pkt_hdr,
    pub session_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_open_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub session_domain: u32,
    pub session_codec: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_pkt {
    pub shdr: hfi_session_hdr_pkt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_init_pkt {
    pub hdr: hfi_pkt_hdr,
    pub arch_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_set_property_pkt {
    pub hdr: hfi_pkt_hdr,
    pub num_properties: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_get_property_pkt {
    pub hdr: hfi_pkt_hdr,
    pub num_properties: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_set_property_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub num_properties: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_sys_pc_prep_pkt {
    pub hdr: hfi_pkt_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_set_buffers_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub buffer_type: u32,
    pub buffer_size: u32,
    pub extradata_size: u32,
    pub min_buffer_size: u32,
    pub num_buffers: u32,
    pub buffer_info: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_empty_buffer_compressed_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub time_stamp_hi: u32,
    pub time_stamp_lo: u32,
    pub flags: u32,
    pub mark_target: u32,
    pub mark_data: u32,
    pub offset: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub input_tag: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_empty_buffer_uncompressed_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub view_id: u32,
    pub time_stamp_hi: u32,
    pub time_stamp_lo: u32,
    pub flags: u32,
    pub mark_target: u32,
    pub mark_data: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub offset: u32,
    pub input_tag: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_fill_buffer_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub stream_id: u32,
    pub offset: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub output_tag: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_flush_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub flush_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_release_buffer_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub buffer_type: u32,
    pub buffer_size: u32,
    pub extradata_size: u32,
    pub response_req: u32,
    pub num_buffers: u32,
    pub buffer_info: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_info {
    pub buffer_addr: u32,
    pub extradata_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_event_notify_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub event_id: u32,
    pub event_data1: u32,
    pub event_data2: u32,
    pub ext_event_data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_init_done_pkt {
    pub hdr: hfi_pkt_hdr,
    pub error_type: u32,
    pub num_properties: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_hdr_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub error_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_init_done_pkt {
    pub shdr: hfi_msg_session_hdr_pkt,
    pub num_properties: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_property_info_pkt {
    pub hdr: hfi_pkt_hdr,
    pub num_properties: u32,
    pub property: u32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_flush_done_pkt {
    pub shdr: hfi_msg_session_hdr_pkt,
    pub flush_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_enable {
    pub enable: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_profile_level {
    pub profile: u32,
    pub level: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_framesize {
    pub buffer_type: u32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_videocores_usage_type {
    pub video_core_enable_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_video_work_mode {
    pub video_work_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_video_work_route {
    pub video_work_route: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_bit_depth {
    pub buffer_type: u32,
    pub bit_depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_pic_struct {
    pub progressive_only: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_colour_space {
    pub colour_space: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_extradata_input_crop {
    pub size: u32,
    pub version: u32,
    pub port_index: u32,
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_dpb_counts {
    pub max_dpb_count: u32,
    pub max_ref_frames: u32,
    pub max_dec_buffering: u32,
    pub max_reorder_frames: u32,
    pub fw_min_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_uncompressed_format_select {
    pub buffer_type: u32,
    pub format: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_uncompressed_plane_constraints {
    pub stride_multiples: u32,
    pub max_stride: u32,
    pub min_plane_buffer_height_multiple: u32,
    pub buffer_alignment: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_uncompressed_plane_actual_constraints_info {
    pub buffer_type: u32,
    pub num_planes: u32,
    pub plane_format: [hfi_uncompressed_plane_constraints; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_uncompressed_plane_actual {
    pub actual_stride: c_int,
    pub actual_plane_buffer_height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_uncompressed_plane_actual_info {
    pub buffer_type: u32,
    pub num_planes: u32,
    pub plane_format: [hfi_uncompressed_plane_actual; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_count_actual {
    pub type: u32,
    pub count_actual: u32,
    pub count_min_host: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_size_actual {
    pub type: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_multi_stream {
    pub buffer_type: u32,
    pub enable: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_requirements {
    pub type: u32,
    pub size: u32,
    pub region_size: u32,
    pub hold_count: u32,
    pub count_min: u32,
    pub count_actual: u32,
    pub contiguous: u32,
    pub alignment: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_bitrate {
    pub bitrate: u32,
    pub layer_id: u32,
}

pub const HFI_H264_CABAC_MODEL_0: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_h264_entropy_control {
    pub entropy_mode: u32,
    pub cabac_model: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_quantization_v2 {
    pub qp_packed: u32,
    pub layer_id: u32,
    pub enable: u32,
    pub reserved: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_quantization_range_v2 {
    pub min_qp: hfi_quantization_v2,
    pub max_qp: hfi_quantization_v2,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_framerate {
    pub buffer_type: u32,
    pub framerate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_intra_refresh {
    pub mode: u32,
    pub mbs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_ltr_mode {
    pub mode: u32,
    pub count: u32,
    pub trust_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_ltr_use {
    pub ref_ltr: u32,
    pub use_constrnt: u32,
    pub frames: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_ltr_mark {
    pub mark_frame: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_max_num_b_frames {
    pub max_num_b_frames: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_intra_period {
    pub pframes: u32,
    pub bframes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_event_data {
    pub error: u32,
    pub height: u32,
    pub width: u32,
    pub event_type: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub tag: u32,
    pub profile: u32,
    pub level: u32,
    pub bit_depth: u32,
    pub pic_struct: u32,
    pub colour_space: u32,
    pub entropy_mode: u32,
    pub buf_count: u32,
    pub top: u32 left,,
    pub height: u32 width,,
    pub input_crop: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_empty_buffer_done_pkt {
    pub shdr: hfi_msg_session_hdr_pkt,
    pub offset: u32,
    pub filled_len: u32,
    pub input_tag: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_fbd_compressed_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub time_stamp_hi: u32,
    pub time_stamp_lo: u32,
    pub error_type: u32,
    pub flags: u32,
    pub mark_target: u32,
    pub mark_data: u32,
    pub stats: u32,
    pub offset: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub input_tag: u32,
    pub output_tag: u32,
    pub picture_type: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_fbd_uncompressed_plane0_pkt {
    pub shdr: hfi_session_hdr_pkt,
    pub stream_id: u32,
    pub view_id: u32,
    pub error_type: u32,
    pub time_stamp_hi: u32,
    pub time_stamp_lo: u32,
    pub flags: u32,
    pub mark_target: u32,
    pub mark_data: u32,
    pub stats: u32,
    pub alloc_len: u32,
    pub filled_len: u32,
    pub offset: u32,
    pub frame_width: u32,
    pub frame_height: u32,
    pub start_x_coord: u32,
    pub start_y_coord: u32,
    pub input_tag: u32,
    pub input_tag2: u32,
    pub output_tag: u32,
    pub picture_type: u32,
    pub packet_buffer: u32,
    pub extradata_buffer: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_session_release_buffers_done_pkt {
    pub shdr: hfi_msg_session_hdr_pkt,
    pub num_buffers: u32,
    pub buffer_info: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_debug_pkt {
    pub hdr: hfi_pkt_hdr,
    pub msg_type: u32,
    pub msg_size: u32,
    pub time_stamp_hi: u32,
    pub time_stamp_lo: u32,
    pub msg_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_msg_sys_coverage_pkt {
    pub hdr: hfi_pkt_hdr,
    pub msg_size: u32,
    pub time_stamp_hi: u32,
    pub time_stamp_lo: u32,
    pub msg_data: [u8; ],
}
