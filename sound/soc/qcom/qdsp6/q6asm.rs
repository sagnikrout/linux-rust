//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/qcom/qdsp6/q6asm.h
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

// ASM client callback events
pub const CMD_PAUSE: c_uint = 0x0001;
pub const ASM_CLIENT_EVENT_CMD_PAUSE_DONE: c_uint = 0x1001;
pub const CMD_FLUSH: c_uint = 0x0002;
pub const ASM_CLIENT_EVENT_CMD_FLUSH_DONE: c_uint = 0x1002;
pub const CMD_EOS: c_uint = 0x0003;
pub const ASM_CLIENT_EVENT_CMD_EOS_DONE: c_uint = 0x1003;
pub const CMD_CLOSE: c_uint = 0x0004;
pub const ASM_CLIENT_EVENT_CMD_CLOSE_DONE: c_uint = 0x1004;
pub const CMD_OUT_FLUSH: c_uint = 0x0005;
pub const ASM_CLIENT_EVENT_CMD_OUT_FLUSH_DONE: c_uint = 0x1005;
pub const CMD_SUSPEND: c_uint = 0x0006;
pub const ASM_CLIENT_EVENT_CMD_SUSPEND_DONE: c_uint = 0x1006;
pub const ASM_CLIENT_EVENT_CMD_RUN_DONE: c_uint = 0x1008;
pub const ASM_CLIENT_EVENT_DATA_WRITE_DONE: c_uint = 0x1009;
pub const ASM_CLIENT_EVENT_DATA_READ_DONE: c_uint = 0x100a;

pub const ASM_WRITE_TOKEN_LEN_SHIFT: c_int = 16;
pub const MAX_SESSIONS: c_int = 8;
pub const FORMAT_LINEAR_PCM: c_uint = 0x0000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6asm_flac_cfg {
    pub sample_rate: u32,
    pub ext_sample_rate: u32,
    pub min_frame_size: u32,
    pub max_frame_size: u32,
    pub stream_info_present: u16,
    pub min_blk_size: u16,
    pub max_blk_size: u16,
    pub ch_cfg: u16,
    pub sample_size: u16,
    pub md5_sum: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6asm_wma_cfg {
    pub fmtag: u32,
    pub num_channels: u32,
    pub sample_rate: u32,
    pub bytes_per_sec: u32,
    pub block_align: u32,
    pub bits_per_sample: u32,
    pub channel_mask: u32,
    pub enc_options: u32,
    pub adv_enc_options: u32,
    pub adv_enc_options2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6asm_alac_cfg {
    pub frame_length: u32,
    pub compatible_version: u8,
    pub bit_depth: u8,
    pub pb: u8,
    pub mb: u8,
    pub kb: u8,
    pub num_channels: u8,
    pub max_run: u16,
    pub max_frame_bytes: u32,
    pub avg_bit_rate: u32,
    pub sample_rate: u32,
    pub channel_layout_tag: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6asm_ape_cfg {
    pub compatible_version: u16,
    pub compression_level: u16,
    pub format_flags: u32,
    pub blocks_per_frame: u32,
    pub final_frame_blocks: u32,
    pub total_frames: u32,
    pub bits_per_sample: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub seek_table_present: u32,
}

extern "C" {
    pub fn q6asm_audio_client_free(ac: *mut audio_client);
}
extern "C" {
    pub fn q6asm_read(ac: *mut audio_client, stream_id: u32) -> c_int;
}
extern "C" {
    pub fn q6asm_cmd(ac: *mut audio_client, stream_id: u32, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn q6asm_cmd_nowait(ac: *mut audio_client, stream_id: u32, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn q6asm_get_session_id(c: *mut audio_client) -> c_int;
}
extern "C" {
    pub fn q6asm_unmap_memory_regions(dir: c_uint, ac: *mut audio_client) -> c_int;
}
extern "C" {
    pub fn q6asm_get_hw_pointer(ac: *mut audio_client, dir: c_uint) -> c_int;
}
