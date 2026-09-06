//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/dai-amd.h
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
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc.. All rights reserved.
//

// ACP Configuration Request - SOF_IPC_DAI_AMD_CONFIG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dai_acp_params {
    pub hdr: sof_ipc_hdr,
    pub /: *mut *mut uint32_t fsync_rate; / FSYNC frequency in Hz,
    pub tdm_slots: u32,
    pub tdm_mode: u32,
    pub format: u32,
    pub __packed: },
// ACPDMIC Configuration Request - SOF_IPC_DAI_AMD_CONFIG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dai_acpdmic_params {
    pub pdm_rate: u32,
    pub pdm_ch: u32,
    pub __packed: },
// ACP_SDW Configuration Request - SOF_IPC_DAI_AMD_SDW_CONFIG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dai_acp_sdw_params {
    pub hdr: sof_ipc_hdr,
    pub rate: u32,
    pub channels: u32,
    pub __packed: },
