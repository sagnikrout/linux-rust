//! Automatically rewritten from C Header to Rust Module
//! Source: net/batman-adv/log.h
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
// Marek Lindner, Simon Wunderlich
//

extern "C" {
    pub fn batadv_debug_log_setup(bat_priv: *mut batadv_priv) -> c_int;
}
extern "C" {
    pub fn batadv_debug_log_cleanup(bat_priv: *mut batadv_priv);
}

//
// enum batadv_dbg_level - available log levels
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum batadv_dbg_level {
// @BATADV_DBG_BATMAN: OGM and TQ computations related messages
    BATADV_DBG_BATMAN	= BIT(0),

// @BATADV_DBG_ROUTES: route added / changed / deleted
    BATADV_DBG_ROUTES	= BIT(1),

// @BATADV_DBG_TT: translation table messages
    BATADV_DBG_TT		= BIT(2),

// @BATADV_DBG_BLA: bridge loop avoidance messages
    BATADV_DBG_BLA		= BIT(3),

// @BATADV_DBG_DAT: ARP snooping and DAT related messages
    BATADV_DBG_DAT		= BIT(4),

// @BATADV_DBG_MCAST: multicast related messages
    BATADV_DBG_MCAST	= BIT(6),

// @BATADV_DBG_TP_METER: throughput meter messages
    BATADV_DBG_TP_METER	= BIT(7),

// @BATADV_DBG_ALL: the union of all the above log levels
    BATADV_DBG_ALL		= 255,
}

//
// _batadv_dbg() - Store debug output with(out) rate limiting
// @type: type of debug message
// @bat_priv: the bat priv with all the mesh interface information
// @ratelimited: whether output should be rate limited
// @fmt: format string
// @arg: variable arguments
//

//
// batadv_dbg() - Store debug output without rate limiting
// @type: type of debug message
// @bat_priv: the bat priv with all the mesh interface information
// @arg: format string and variable arguments
//

//
// batadv_dbg_ratelimited() - Store debug output with rate limiting
// @type: type of debug message
// @bat_priv: the bat priv with all the mesh interface information
// @arg: format string and variable arguments
//

//
// batadv_info() - Store message in debug buffer and print it to kmsg buffer
// @net_dev: the mesh interface net device
// @fmt: format string
// @arg: variable arguments
//

//
// batadv_err() - Store error in debug buffer and print it to kmsg buffer
// @net_dev: the mesh interface net device
// @fmt: format string
// @arg: variable arguments
//

