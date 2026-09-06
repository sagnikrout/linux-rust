//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/platform_sst_audio.h
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
// platform_sst_audio.h:  sst audio platform data header file
//
// Copyright (C) 2012-14 Intel Corporation
// Author: Jeeja KP <jeeja.kp@intel.com>
// Omair Mohammed Abdullah <omair.m.abdullah@intel.com>
// Vinod Koul ,vinod.koul@intel.com>
//
pub const MAX_NUM_STREAMS_MRFLD: c_int = 25;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_audio_task_id_mrfld {
    SST_TASK_ID_NONE = 0,
    SST_TASK_ID_SBA = 1,
    SST_TASK_ID_MEDIA = 3,
    SST_TASK_ID_MAX = SST_TASK_ID_MEDIA,
}

// Device IDs for Merrifield are Pipe IDs,
// ref: DSP spec v0.75
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_audio_device_id_mrfld {
// Output pipeline IDs
    PIPE_ID_OUT_START = 0x0,
    PIPE_CODEC_OUT0 = 0x2,
    PIPE_CODEC_OUT1 = 0x3,
    PIPE_SPROT_LOOP_OUT = 0x4,
    PIPE_MEDIA_LOOP1_OUT = 0x5,
    PIPE_MEDIA_LOOP2_OUT = 0x6,
    PIPE_VOIP_OUT = 0xC,
    PIPE_PCM0_OUT = 0xD,
    PIPE_PCM1_OUT = 0xE,
    PIPE_PCM2_OUT = 0xF,
    PIPE_MEDIA0_OUT = 0x12,
    PIPE_MEDIA1_OUT = 0x13,
// Input Pipeline IDs
    PIPE_ID_IN_START = 0x80,
    PIPE_CODEC_IN0 = 0x82,
    PIPE_CODEC_IN1 = 0x83,
    PIPE_SPROT_LOOP_IN = 0x84,
    PIPE_MEDIA_LOOP1_IN = 0x85,
    PIPE_MEDIA_LOOP2_IN = 0x86,
    PIPE_VOIP_IN = 0x8C,
    PIPE_PCM0_IN = 0x8D,
    PIPE_PCM1_IN = 0x8E,
    PIPE_MEDIA0_IN = 0x8F,
    PIPE_MEDIA1_IN = 0x90,
    PIPE_MEDIA2_IN = 0x91,
    PIPE_MEDIA3_IN = 0x9C,
    PIPE_RSVD = 0xFF,
}

// The stream map for each platform consists of an array of the below
// stream map structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_dev_stream_map {
    pub /: *mut *mut u8 dev_num; / device id,
    pub /: *mut *mut u8 subdev_num; / substream,
    pub direction: u8,
    pub /: *mut *mut u8 device_id; / fw id,
    pub /: *mut *mut u8 task_id; / fw task,
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_platform_data {
// Intel software platform id
    pub pdev_strm_map: *mut sst_dev_stream_map,
    pub strm_map_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_info {
    pub iram_start: u32,
    pub iram_end: u32,
    pub iram_use: bool,
    pub dram_start: u32,
    pub dram_end: u32,
    pub dram_use: bool,
    pub imr_start: u32,
    pub imr_end: u32,
    pub imr_use: bool,
    pub mailbox_start: u32,
    pub use_elf: bool,
    pub lpe_viewpt_rqd: bool,
    pub max_streams: c_uint,
    pub dma_max_len: u32,
    pub num_probes: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_lib_dnld_info {
    pub mod_base: c_uint,
    pub mod_end: c_uint,
    pub mod_table_offset: c_uint,
    pub mod_table_size: c_uint,
    pub mod_ddr_dnld: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_res_info {
    pub shim_offset: c_uint,
    pub shim_size: c_uint,
    pub shim_phy_addr: c_uint,
    pub ssp0_offset: c_uint,
    pub ssp0_size: c_uint,
    pub dma0_offset: c_uint,
    pub dma0_size: c_uint,
    pub dma1_offset: c_uint,
    pub dma1_size: c_uint,
    pub iram_offset: c_uint,
    pub iram_size: c_uint,
    pub dram_offset: c_uint,
    pub dram_size: c_uint,
    pub mbox_offset: c_uint,
    pub mbox_size: c_uint,
    pub acpi_lpe_res_index: c_uint,
    pub acpi_ddr_index: c_uint,
    pub acpi_ipc_irq_index: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_ipc_info {
    pub ipc_offset: c_int,
    pub mbox_recv_off: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_platform_info {
    pub probe_data: *const sst_info,
    pub ipc_info: *const sst_ipc_info,
    pub res_info: *const sst_res_info,
    pub lib_info: *const sst_lib_dnld_info,
    pub platform: *const c_char,
    pub streams_lost_on_suspend: bool,
}

extern "C" {
    pub fn add_sst_platform_device() -> c_int;
}
