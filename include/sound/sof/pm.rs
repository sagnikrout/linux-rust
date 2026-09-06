//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/pm.h
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
// PM
//
// PM context element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_pm_ctx_elem {
    pub hdr: sof_ipc_hdr,
    pub type: u32,
    pub size: u32,
    pub addr: u64,
    pub __packed: },
//
// PM context - SOF_IPC_PM_CTX_SAVE, SOF_IPC_PM_CTX_RESTORE,
// SOF_IPC_PM_CTX_SIZE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_pm_ctx {
    pub hdr: sof_ipc_cmd_hdr,
    pub buffer: sof_ipc_host_buffer,
    pub num_elems: u32,
    pub size: u32,
// reserved for future use
    pub reserved: [u32; 8],
    pub elems: [sof_ipc_pm_ctx_elem; ],
    pub __packed: },
// enable or disable cores - SOF_IPC_PM_CORE_ENABLE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_pm_core_config {
    pub hdr: sof_ipc_cmd_hdr,
    pub enable_mask: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_pm_gate {
    pub hdr: sof_ipc_cmd_hdr,
    pub /: *mut *mut uint32_t flags; / platform specific,
// reserved for future use
    pub reserved: [u32; 5],
    pub __packed: },
