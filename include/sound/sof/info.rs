//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/info.h
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
// Firmware boot and version
//
pub const SOF_IPC_MAX_ELEMS: c_int = 16;
//
// Firmware boot info flag bits (64-bit)
//

// extended data types that can be appended onto end of sof_ipc_fw_ready
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_ext_data {
    SOF_IPC_EXT_UNUSED		= 0,
    SOF_IPC_EXT_WINDOW		= 1,
    SOF_IPC_EXT_CC_INFO		= 2,
    SOF_IPC_EXT_PROBE_INFO		= 3,
    SOF_IPC_EXT_USER_ABI_INFO	= 4,
}

// Build u32 number in format MMmmmppp

// FW version - SOF_IPC_GLB_VERSION
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_fw_version {
    pub hdr: sof_ipc_hdr,
    pub major: u16,
    pub minor: u16,
    pub micro: u16,
    pub build: u16,
    pub date: [u8; 12],
    pub time: [u8; 10],
    pub tag: [u8; 6],
    pub abi_version: u32,
// used to check FW and ldc file compatibility, reproducible value
    pub src_hash: u32,
// reserved for future use
    pub reserved: [u32; 3],
    pub __packed: },
// FW ready Message - sent by firmware when boot has completed
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_fw_ready {
    pub hdr: sof_ipc_cmd_hdr,
    pub /: *mut *mut uint32_t dspbox_offset; / dsp initiated IPC mailbox,
    pub /: *mut *mut uint32_t hostbox_offset; / host initiated IPC mailbox,
    pub dspbox_size: u32,
    pub hostbox_size: u32,
    pub version: sof_ipc_fw_version,
// Miscellaneous flags
    pub flags: u64,
// reserved for future use
    pub reserved: [u32; 4],
    pub __packed: },
//
// Extended Firmware data. All optional, depends on platform/arch.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_region {
    SOF_IPC_REGION_DOWNBOX	= 0,
    SOF_IPC_REGION_UPBOX,
    SOF_IPC_REGION_TRACE,
    SOF_IPC_REGION_DEBUG,
    SOF_IPC_REGION_STREAM,
    SOF_IPC_REGION_REGS,
    SOF_IPC_REGION_EXCEPTION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_ext_data_hdr {
    pub hdr: sof_ipc_cmd_hdr,
    pub /: *mut *mut *mut uint32_t type; /< SOF_IPC_EXT_,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_window_elem {
    pub hdr: sof_ipc_hdr,
    pub /: *mut *mut *mut uint32_t type; /< SOF_IPC_REGION_,
    pub /: *mut *mut *mut uint32_t id; /< platform specific - used to map to host memory,
    pub /: *mut *mut *mut uint32_t flags; /< R, W, RW, etc - to define,
    pub /: *mut *mut *mut uint32_t size; /< size of region in bytes,
// offset in window region as windows can be partitioned
    pub offset: u32,
    pub __packed: },
// extended data memory windows for IPC, trace and debug
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_window {
    pub ext_hdr: sof_ipc_ext_data_hdr,
    pub num_windows: u32,
    pub window: [sof_ipc_window_elem; SOF_IPC_MAX_ELEMS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_cc_version {
    pub ext_hdr: sof_ipc_ext_data_hdr,
    pub major: u32,
    pub minor: u32,
    pub micro: u32,
// reserved for future use
    pub reserved: [u32; 4],
    pub /: *mut *mut uint8_t name[16]; / null terminated compiler name,
    pub /: *mut *mut uint8_t optim[4]; / null terminated compiler -O flag value,
    pub /: *mut *mut uint8_t desc[32]; / null terminated compiler description,
    pub __packed: },
// extended data: Probe setup
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_probe_support {
    pub ext_hdr: sof_ipc_ext_data_hdr,
    pub probe_points_max: u32,
    pub injection_dmas_max: u32,
// reserved for future use
    pub reserved: [u32; 2],
    pub __packed: },
// extended data: user abi version(s)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_user_abi_version {
    pub ext_hdr: sof_ipc_ext_data_hdr,
    pub abi_dbg_version: u32,
    pub __packed: },
