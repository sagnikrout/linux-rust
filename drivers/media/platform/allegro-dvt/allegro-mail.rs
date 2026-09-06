//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/allegro-dvt/allegro-mail.h
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
// Copyright (C) 2019 Pengutronix, Michael Tretter <kernel@pengutronix.de>
//
// Allegro VCU firmware mailbox mail definitions
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcu_msg_type {
    MCU_MSG_TYPE_INIT = 0x0000,
    MCU_MSG_TYPE_CREATE_CHANNEL = 0x0005,
    MCU_MSG_TYPE_DESTROY_CHANNEL = 0x0006,
    MCU_MSG_TYPE_ENCODE_FRAME = 0x0007,
    MCU_MSG_TYPE_PUT_STREAM_BUFFER = 0x0012,
    MCU_MSG_TYPE_PUSH_BUFFER_INTERMEDIATE = 0x000e,
    MCU_MSG_TYPE_PUSH_BUFFER_REFERENCE = 0x000f,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcu_msg_version {
    MCU_MSG_VERSION_2018_2,
    MCU_MSG_VERSION_2019_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu_msg_header {
    pub type: mcu_msg_type,
    pub version: mcu_msg_version,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu_msg_init_request {
    pub header: mcu_msg_header,
    pub /: *mut *mut u32 reserved0; / maybe a unused channel id,
    pub suballoc_dma: u32,
    pub suballoc_size: u32,
    pub encoder_buffer_size: i32,
    pub encoder_buffer_color_depth: i32,
    pub num_cores: i32,
    pub clk_rate: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu_msg_init_response {
    pub header: mcu_msg_header,
    pub reserved0: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_channel_param {
    pub version: mcu_msg_version,
    pub layer_id: u32,
    pub width: u16,
    pub height: u16,
    pub videomode: u32,
    pub format: u32,
    pub colorspace: u32,
    pub src_mode: u32,
    pub src_bit_depth: u32,
    pub profile: u8,
    pub constraint_set_flags: u16,
    pub codec: u32,
    pub level: u16,
    pub tier: u16,
    pub log2_max_poc: u32,
    pub log2_max_frame_num: u32,
    pub temporal_mvp_enable: u32,
    pub enable_reordering: u32,
    pub dbf_ovr_en: u32,
    pub override_lf: u32,
    pub num_ref_idx_l0: u32,
    pub num_ref_idx_l1: u32,
    pub custom_lda: u32,
    pub rdo_cost_mode: u32,
    pub lf: u32,
    pub lf_x_tile: u32,
    pub lf_x_slice: u32,
    pub beta_offset: i8,
    pub tc_offset: i8,
    pub reserved10: u16,
    pub unknown11: u32,
    pub unknown12: u32,
    pub num_slices: u16,
    pub encoder_buffer_offset: u32,
    pub encoder_buffer_enabled: u32,
    pub clip_hrz_range: u16,
    pub clip_vrt_range: u16,
    pub me_range: [u16; 4],
    pub max_cu_size: u8,
    pub min_cu_size: u8,
    pub max_tu_size: u8,
    pub min_tu_size: u8,
    pub max_transfo_depth_inter: u8,
    pub max_transfo_depth_intra: u8,
    pub reserved20: u16,
    pub entropy_mode: u32,
    pub wp_mode: u32,
// rate control param
    pub rate_control_mode: u32,
    pub initial_rem_delay: u32,
    pub cpb_size: u32,
    pub framerate: u16,
    pub clk_ratio: u16,
    pub target_bitrate: u32,
    pub max_bitrate: u32,
    pub initial_qp: u16,
    pub min_qp: u16,
    pub max_qp: u16,
    pub ip_delta: i16,
    pub pb_delta: i16,
    pub golden_ref: u16,
    pub golden_delta: u16,
    pub golden_ref_frequency: u16,
    pub rate_control_option: u32,
    pub num_pixel: u32,
    pub max_psnr: u16,
    pub max_pixel_value: u16,
    pub maxpicturesize: [u32; 3],
// gop param
    pub gop_ctrl_mode: u32,
    pub freq_idr: u32,
    pub freq_lt: u32,
    pub gdr_mode: u32,
    pub gop_length: u16,
    pub num_b: u8,
    pub freq_golden_ref: u8,
    pub enable_lt: u32,
    pub tmpdqp: u32,
    pub subframe_latency: u32,
    pub lda_control_mode: u32,
    pub unknown41: u32,
    pub lda_factors: [u32; 6],
    pub max_num_merge_cand: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu_msg_create_channel {
    pub header: mcu_msg_header,
    pub user_id: u32,
    pub blob: *mut u32,
    pub blob_size: usize,
    pub blob_mcu_addr: u32,
    pub ep1_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu_msg_create_channel_response {
    pub header: mcu_msg_header,
    pub channel_id: u32,
    pub user_id: u32,
    pub options: u32,
    pub num_core: u32,
    pub num_ref_idx_l0: u32,
    pub num_ref_idx_l1: u32,
    pub int_buffers_count: u32,
    pub int_buffers_size: u32,
    pub rec_buffers_count: u32,
    pub rec_buffers_size: u32,
    pub reserved: u32,
    pub error_code: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu_msg_destroy_channel {
    pub header: mcu_msg_header,
    pub channel_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu_msg_destroy_channel_response {
    pub header: mcu_msg_header,
    pub channel_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu_msg_push_buffers_internal_buffer {
    pub dma_addr: u32,
    pub mcu_addr: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu_msg_push_buffers_internal {
    pub header: mcu_msg_header,
    pub channel_id: u32,
    pub num_buffers: usize,
    pub __counted_by(num_buffers): mcu_msg_push_buffers_internal_buffer buffer[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu_msg_put_stream_buffer {
    pub header: mcu_msg_header,
    pub channel_id: u32,
    pub dma_addr: u32,
    pub mcu_addr: u32,
    pub size: u32,
    pub offset: u32,
    pub dst_handle: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu_msg_encode_frame {
    pub header: mcu_msg_header,
    pub channel_id: u32,
    pub reserved: u32,
    pub encoding_options: u32,

    pub pps_qp: i16,
    pub padding: u16,
    pub user_param: u64,
    pub src_handle: u64,
    pub request_options: u32,

// u32 scene_change_delay (optional)
// rate control param (optional)
// gop param (optional)
// dynamic resolution params (optional)
    pub src_y: u32,
    pub src_uv: u32,
    pub is_10_bit: u32,
    pub stride: u32,
    pub format: u32,
    pub ep2: u32,
    pub ep2_v: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcu_msg_encode_frame_response {
    pub header: mcu_msg_header,
    pub channel_id: u32,
    pub /: *mut *mut u64 dst_handle; / see mcu_msg_put_stream_buffer,
    pub /: *mut *mut u64 user_param; / see mcu_msg_encode_frame,
    pub /: *mut *mut u64 src_handle; / see mcu_msg_encode_frame,
    pub skip: u16,
    pub is_ref: u16,
    pub initial_removal_delay: u32,
    pub dpb_output_delay: u32,
    pub size: u32,
    pub frame_tag_size: u32,
    pub stuffing: i32,
    pub filler: i32,
    pub num_column: u16,
    pub num_row: u16,
    pub qp: u16,
    pub num_ref_idx_l0: u8,
    pub num_ref_idx_l1: u8,
    pub partition_table_offset: u32,
    pub partition_table_size: i32,
    pub sum_complex: u32,
    pub tile_width: [i32; 4],
    pub tile_height: [i32; 22],
    pub error_code: u32,
    pub slice_type: u32,
pub const AL_ENC_SLICE_TYPE_B: c_int = 0;
pub const AL_ENC_SLICE_TYPE_P: c_int = 1;
pub const AL_ENC_SLICE_TYPE_I: c_int = 2;
    pub pic_struct: u32,
    pub is_idr: u8,
    pub is_first_slice: u8,
    pub is_last_slice: u8,
    pub reserved: u8,
    pub pps_qp: u16,
    pub reserved1: u16,
    pub reserved2: u32,
    pub reserved3: u32,
    pub reserved4: u32,
    pub reserved5: u32,
    pub reserved6: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mcu_msg_response {
    pub header: mcu_msg_header,
    pub init: mcu_msg_init_response,
    pub create_channel: mcu_msg_create_channel_response,
    pub destroy_channel: mcu_msg_destroy_channel_response,
    pub encode_frame: mcu_msg_encode_frame_response,
}

extern "C" {
    pub fn allegro_encode_config_blob(dst: *mut u32, param: *mut create_channel_param) -> isize;
}
extern "C" {
    pub fn allegro_decode_mail(msg: *mut c_void, src: *mut u32) -> c_int;
}
extern "C" {
    pub fn allegro_encode_mail(dst: *mut u32, msg: *mut c_void) -> isize;
}
