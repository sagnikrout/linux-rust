//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_chnl_cfg.h
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
// Copyright (C) 2020-21 Intel Corporation
//

// Number of TDs on the trace channel
pub const IPC_MEM_TDS_TRC: c_int = 32;
// Trace channel TD buffer size.
pub const IPC_MEM_MAX_DL_TRC_BUF_SIZE: c_int = 8192;
// Channel ID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_channel_id {
    IPC_MEM_IP_CHL_ID_0 = 0,
    IPC_MEM_CTRL_CHL_ID_1,
    IPC_MEM_CTRL_CHL_ID_2,
    IPC_MEM_CTRL_CHL_ID_3,
    IPC_MEM_CTRL_CHL_ID_4,
    IPC_MEM_CTRL_CHL_ID_5,
    IPC_MEM_CTRL_CHL_ID_6,
    IPC_MEM_CTRL_CHL_ID_7,
}

//
// struct ipc_chnl_cfg - IPC channel configuration structure
// @id:				Interface ID
// @ul_pipe:			Uplink datastream
// @dl_pipe:			Downlink datastream
// @ul_nr_of_entries:		Number of Transfer descriptor uplink pipe
// @dl_nr_of_entries:		Number of Transfer descriptor downlink pipe
// @dl_buf_size:		Downlink buffer size
// @wwan_port_type:		Wwan subsystem port type
// @accumulation_backoff:	Time in usec for data accumalation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_chnl_cfg {
    pub id: u32,
    pub ul_pipe: u32,
    pub dl_pipe: u32,
    pub ul_nr_of_entries: u32,
    pub dl_nr_of_entries: u32,
    pub dl_buf_size: u32,
    pub wwan_port_type: u32,
    pub accumulation_backoff: u32,
}

//
// ipc_chnl_cfg_get - Get pipe configuration.
// @chnl_cfg:		Array of ipc_chnl_cfg struct
// @index:		Channel index (up to MAX_CHANNELS)
//
// Return: 0 on success and failure value on error
//
extern "C" {
    pub fn ipc_chnl_cfg_get(chnl_cfg: *mut ipc_chnl_cfg, index: c_int) -> c_int;
}
