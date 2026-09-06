//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_vf_mbx.h
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
// Copyright (c) 2018, Intel Corporation.

// Defining the mailbox message threshold as 63 asynchronous
// pending messages. Normal VF functionality does not require
// sending more than 63 asynchronous pending message.
//
pub const ICE_ASYNC_VF_MSG_THRESHOLD: c_int = 63;

extern "C" {
    pub fn ice_conv_link_speed_to_virtchnl(adv_link_support: bool, link_speed: u16) -> u32;
}
extern "C" {
    pub fn ice_mbx_vf_clear_cnt_e830(hw: *const ice_hw, vf_id: u16);
}
extern "C" {
    pub fn ice_mbx_clear_malvf(vf_info: *mut ice_mbx_vf_info);
}
extern "C" {
    pub fn ice_mbx_init_vf_info(hw: *mut ice_hw, vf_info: *mut ice_mbx_vf_info);
}
extern "C" {
    pub fn ice_mbx_init_snapshot(hw: *mut ice_hw);
}

