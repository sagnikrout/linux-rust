//! Automatically rewritten from C Header to Rust Module
//! Source: net/batman-adv/gateway_common.h
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
// Copyright (C) B.A.T.M.A.N. contributors:
//
// Marek Lindner
//

//
// enum batadv_bandwidth_units - bandwidth unit types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum batadv_bandwidth_units {
// @BATADV_BW_UNIT_KBIT: unit type kbit
    BATADV_BW_UNIT_KBIT,

// @BATADV_BW_UNIT_MBIT: unit type mbit
    BATADV_BW_UNIT_MBIT,
}

extern "C" {
    pub fn batadv_gw_tvlv_container_update(bat_priv: *mut batadv_priv);
}
extern "C" {
    pub fn batadv_gw_init(bat_priv: *mut batadv_priv);
}
extern "C" {
    pub fn batadv_gw_free(bat_priv: *mut batadv_priv);
}
