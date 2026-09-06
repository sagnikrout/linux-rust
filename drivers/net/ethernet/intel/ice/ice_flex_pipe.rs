//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_flex_pipe.h
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
// Copyright (c) 2019, Intel Corporation.

pub const ICE_FDIR_REG_SET_SIZE: c_int = 4;
extern "C" {
    pub fn ice_release_change_lock(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_set_dvm_boost_entries(hw: *mut ice_hw) -> c_int;
}
// Rx parser PTYPE functions
extern "C" {
    pub fn ice_hw_ptype_ena(hw: *mut ice_hw, ptype: u16) -> bool;
}
// XLT2/VSI group functions
extern "C" {
    pub fn ice_init_pkg(hw: *mut ice_hw, buff: *mut u8, len: u32) -> ice_ddp_state;
}
extern "C" {
    pub fn ice_is_init_pkg_successful(state: ice_ddp_state) -> bool;
}
extern "C" {
    pub fn ice_init_hw_tbls(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_free_seg(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_fill_blk_tbls(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_clear_hw_tbls(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_free_hw_tbls(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_rem_prof(hw: *mut ice_hw, blk: ice_block, id: u64) -> c_int;
}
extern "C" {
    pub fn ice_pkg_buf_free(hw: *mut ice_hw, bld: *mut ice_buf_build);
}
