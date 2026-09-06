//! Automatically rewritten from C Header to Rust Module
//! Source: sound/firewire/fcp.h
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

// Macro flag: #define SOUND_FIREWIRE_FCP_H_INCLUDED
pub const AVC_PLUG_INFO_BUF_BYTES: c_int = 4;
//
// AV/C Digital Interface Command Set General Specification 4.2
// (Sep 2004, 1394TA)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avc_general_plug_dir {
    AVC_GENERAL_PLUG_DIR_IN		= 0,
    AVC_GENERAL_PLUG_DIR_OUT	= 1,
    AVC_GENERAL_PLUG_DIR_COUNT
}

extern "C" {
    pub fn fcp_bus_reset(unit: *mut fw_unit);
}
