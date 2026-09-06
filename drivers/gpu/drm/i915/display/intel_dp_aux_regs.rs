//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dp_aux_regs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

//
// The aux channel provides a way to talk to the signal sink for DDC etc. Max
// packet size supported is 20 bytes in each direction, hence the 5 fixed data
// registers
//
// Wrapper macro to convert from aux_ch to the index used in some of the
// registers.
//

pub const _DPA_AUX_CH_CTL: c_uint = 0x64010;
pub const _DPB_AUX_CH_CTL: c_uint = 0x64110;

pub const _PCH_DPB_AUX_CH_CTL: c_uint = 0xe4110;
pub const _PCH_DPC_AUX_CH_CTL: c_uint = 0xe4210;

pub const _XELPDP_USBC1_AUX_CH_CTL: c_uint = 0x16f210;
pub const _XELPDP_USBC2_AUX_CH_CTL: c_uint = 0x16f410;

pub const _DPA_AUX_CH_DATA1: c_uint = 0x64014;
pub const _DPB_AUX_CH_DATA1: c_uint = 0x64114;

pub const _PCH_DPB_AUX_CH_DATA1: c_uint = 0xe4114;
pub const _PCH_DPC_AUX_CH_DATA1: c_uint = 0xe4214;

pub const _XELPDP_USBC1_AUX_CH_DATA1: c_uint = 0x16f214;
pub const _XELPDP_USBC2_AUX_CH_DATA1: c_uint = 0x16f414;

// PICA Power Well Control

