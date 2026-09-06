//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixd/ixd.h
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
// Copyright (C) 2025 Intel Corporation

//
// struct ixd_adapter - Data structure representing a CPF
// @cp_ctx: Control plane communication context
// @init_task: Delayed initialization after reset
// @init_task.init_work: Delayed initialization work
// @init_task.reset_retries: How many times to check, whether reset is completed
// @init_task.vc_retries: Number of retries to establish mailbox communication
// @init_task.success: init_work completion status
// @mbx_task: Control queue Rx handling
// @xnm: virtchnl transaction manager
// @asq: Send control queue info
// @arq: Receive control queue info
// @vc_ver: Negotiated virtchnl version
// @vc_ver.major: Negotiated major virtchnl version
// @vc_ver.minor: Negotiated minor virtchnl version
// @caps: Negotiated virtchnl capabilities
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixd_adapter {
    pub cp_ctx: libie_ctlq_ctx,
    pub init_work: delayed_work,
    pub reset_retries: u8,
    pub vc_retries: u8,
    pub success: bool,
    pub init_task: },
    pub mbx_task: delayed_work,
    pub xnm: *mut libie_ctlq_xn_manager,
    pub asq: *mut libie_ctlq_info,
    pub arq: *mut libie_ctlq_info,
    pub major: u32,
    pub minor: u32,
    pub vc_ver: },
    pub caps: virtchnl2_get_capabilities,
}

//
// ixd_to_dev - Get the corresponding device struct from an adapter
// @adapter: PCI device driver-specific private data
//
// Return: struct device corresponding to the given adapter
//
extern "C" {
    pub fn ixd_trigger_reset(adapter: *mut ixd_adapter);
}
extern "C" {
    pub fn ixd_check_reset_complete(adapter: *mut ixd_adapter) -> bool;
}
extern "C" {
    pub fn ixd_init_task(work: *mut work_struct);
}
extern "C" {
    pub fn ixd_init_dflt_mbx(adapter: *mut ixd_adapter) -> c_int;
}
extern "C" {
    pub fn ixd_deinit_dflt_mbx(adapter: *mut ixd_adapter);
}
