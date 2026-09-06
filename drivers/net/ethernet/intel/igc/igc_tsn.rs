//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igc/igc_tsn.h
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
// Copyright (c)  2020 Intel Corporation

pub const IGC_RX_MIN_FRAG_SIZE: c_int = 60;
pub const SMD_FRAME_SIZE: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_txd_popts_type {
    SMD_V = 0x01,
    SMD_R = 0x02,
}

extern "C" {
    pub fn igc_fpe_init(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_fpe_clear_preempt_queue(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_fpe_get_supported_frag_size(frag_size: u32) -> u32;
}
extern "C" {
    pub fn igc_tsn_offload_apply(adapter: *mut igc_adapter) -> c_int;
}
extern "C" {
    pub fn igc_tsn_reset(adapter: *mut igc_adapter) -> c_int;
}
extern "C" {
    pub fn igc_tsn_adjust_txtime_offset(adapter: *mut igc_adapter);
}
extern "C" {
    pub fn igc_tsn_is_taprio_activated_by_user(adapter: *mut igc_adapter) -> bool;
}
