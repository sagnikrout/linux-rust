//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ipack/devices/ipoctal.h
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
// driver for the IPOCTAL boards
//
// Copyright (C) 2009-2012 CERN (www.cern.ch)
// Author: Nicolas Serafini, EIC2 SA
// Author: Samuel Iglesias Gonsalvez <siglesias@igalia.com>
//
pub const NR_CHANNELS: c_int = 8;
pub const IPOCTAL_MAX_BOARDS: c_int = 16;

//
// struct ipoctal_stats -- Stats since last reset
//
// @tx: Number of transmitted bytes
// @rx: Number of received bytes
// @overrun: Number of overrun errors
// @parity_err: Number of parity errors
// @framing_err: Number of framing errors
// @rcv_break: Number of break received
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoctal_stats {
    pub tx: c_ulong,
    pub rx: c_ulong,
    pub overrun_err: c_ulong,
    pub parity_err: c_ulong,
    pub framing_err: c_ulong,
    pub rcv_break: c_ulong,
}
