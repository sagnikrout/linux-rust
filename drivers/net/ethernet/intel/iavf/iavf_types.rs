//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/iavf/iavf_types.h
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
// Copyright(c) 2024 Intel Corporation.

// structure used to queue PTP commands for processing
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_ptp_aq_cmd {
    pub list: list_head,
    pub v_opcode:16: virtchnl_ops,
    pub msglen: u16,
    pub __counted_by(msglen): u8 msg[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_ptp {
    pub phc_time_waitqueue: wait_queue_head_t,
    pub hw_caps: virtchnl_ptp_caps,
    pub info: ptp_clock_info,
    pub clock: *mut ptp_clock,
    pub aq_cmds: list_head,
    pub cached_phc_time: u64,
    pub cached_phc_updated: c_ulong,
// Lock protecting access to the AQ command list
    pub aq_cmd_lock: mutex,
    pub hwtstamp_config: kernel_hwtstamp_config,
    pub phc_time_ready:1: bool,
}
