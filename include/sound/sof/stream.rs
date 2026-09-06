//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/stream.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//

//
// Stream configuration.
//
pub const SOF_IPC_MAX_CHANNELS: c_int = 8;
// common sample rates for use in masks

// continuous and non-standard rates for flexibility

// generic PCM flags for runtime settings

// stream PCM frame format
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_frame {
    SOF_IPC_FRAME_S16_LE = 0,
    SOF_IPC_FRAME_S24_4LE,
    SOF_IPC_FRAME_S32_LE,
    SOF_IPC_FRAME_FLOAT,
// other formats here
}

// stream buffer format
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_buffer_format {
    SOF_IPC_BUFFER_INTERLEAVED,
    SOF_IPC_BUFFER_NONINTERLEAVED,
// other formats here
}

// stream direction
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_stream_direction {
    SOF_IPC_STREAM_PLAYBACK = 0,
    SOF_IPC_STREAM_CAPTURE,
}

// stream ring info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_host_buffer {
    pub hdr: sof_ipc_hdr,
    pub phy_addr: u32,
    pub pages: u32,
    pub size: u32,
    pub reserved: [u32; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_stream_params {
    pub hdr: sof_ipc_hdr,
    pub buffer: sof_ipc_host_buffer,
    pub /: *mut *mut *mut uint32_t direction; /< enum sof_ipc_stream_direction,
    pub /: *mut *mut *mut uint32_t frame_fmt; /< enum sof_ipc_frame,
    pub /: *mut *mut *mut uint32_t buffer_fmt; /< enum sof_ipc_buffer_format,
    pub rate: u32,
    pub stream_tag: u16,
    pub channels: u16,
    pub sample_valid_bytes: u16,
    pub sample_container_bytes: u16,
    pub host_period_bytes: u32,
    pub /: *mut *mut *mut uint16_t no_stream_position; /< 1 means don't send stream position,
    pub /: *mut *mut *mut uint8_t cont_update_posn; /< 1 means continuous update stream position,
    pub reserved0: u8,
    pub /: *mut *mut *mut int16_t ext_data_length; /< 0, means no extended data,
    pub reserved: [u8; 2],
    pub /: *mut *mut *mut uint16_t chmap[SOF_IPC_MAX_CHANNELS]; /< channel map - SOF_CHMAP_,
    pub /: *mut *mut *mut uint8_t ext_data[]; /< extended data,
    pub __packed: },
// PCM params info - SOF_IPC_STREAM_PCM_PARAMS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_pcm_params {
    pub hdr: sof_ipc_cmd_hdr,
    pub comp_id: u32,
    pub /: *mut *mut *mut uint32_t flags; /< generic PCM flags - SOF_PCM_FLAG_,
    pub reserved: [u32; 2],
    pub params: sof_ipc_stream_params,
    pub __packed: },
// PCM params info reply - SOF_IPC_STREAM_PCM_PARAMS_REPLY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_pcm_params_reply {
    pub rhdr: sof_ipc_reply,
    pub comp_id: u32,
    pub posn_offset: u32,
    pub __packed: },
// free stream - SOF_IPC_STREAM_PCM_PARAMS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_stream {
    pub hdr: sof_ipc_cmd_hdr,
    pub comp_id: u32,
    pub __packed: },
// flags indicating which time stamps are in sync with each other

// flags indicating which time stamps are valid

// flags indicating time stamps are 64bit else 3use low 32bit

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_stream_posn {
    pub rhdr: sof_ipc_reply,
    pub /: *mut *mut *mut uint32_t comp_id; /< host component ID,
    pub /: *mut *mut *mut uint32_t flags; /< SOF_TIME_,
    pub /: *mut *mut *mut uint32_t wallclock_hz; /< frequency of wallclock in Hz,
    pub /: *mut *mut *mut uint32_t timestamp_ns; /< resolution of timestamp in ns,
    pub /: *mut *mut *mut uint64_t host_posn; /< host DMA position in bytes,
    pub /: *mut *mut *mut uint64_t dai_posn; /< DAI DMA position in bytes,
    pub /: *mut *mut *mut uint64_t comp_posn; /< comp position in bytes,
    pub /: *mut *mut *mut uint64_t wallclock; /< audio wall clock,
    pub /: *mut *mut *mut uint64_t timestamp; /< system time stamp,
    pub /: *mut *mut *mut uint32_t xrun_comp_id; /< comp ID of XRUN component,
    pub /: *mut *mut *mut int32_t xrun_size; /< XRUN size in bytes,
    pub __packed: },
