//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_hfi_gen2.h
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

//
// struct iris_inst_hfi_gen2 - holds per video instance parameters for hfi_gen2
//
// @inst: pointer to iris_instance structure
// @packet: HFI packet
// @ipsc_properties_set: boolean to set ipsc properties to fw
// @opsc_properties_set: boolean to set opsc properties to fw
// @hfi_frame_info: structure of frame info
// @src_subcr_params: subscription params to fw on input port
// @dst_subcr_params: subscription params to fw on output port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_inst_hfi_gen2 {
    pub inst: iris_inst,
    pub packet: *mut iris_hfi_header,
    pub ipsc_properties_set: bool,
    pub opsc_properties_set: bool,
    pub hfi_frame_info: iris_hfi_frame_info,
    pub src_subcr_params: hfi_subscription_params,
    pub dst_subcr_params: hfi_subscription_params,
}

extern "C" {
    pub fn iris_hfi_gen2_sys_ops_init(core: *mut iris_core);
}
extern "C" {
    pub fn iris_hfi_gen2_response_handler(core: *mut iris_core);
}
