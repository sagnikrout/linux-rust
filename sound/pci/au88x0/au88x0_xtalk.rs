//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/au88x0/au88x0_xtalk.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// au88x0_cxtalk.h
//
// Wed Nov 19 19:07:17 2003
// Copyright  2003  mjander
// mjander@users.sourceforge.org
//
// The crosstalk canceler supports 5 stereo input channels. The result is

pub const XTDLINE_SZ: c_int = 32;
pub const XTGAINS_SZ: c_int = 10;
pub const XTINST_SZ: c_int = 4;
pub const XT_HEADPHONE: c_int = 1;
pub const XT_SPEAKER0: c_int = 2;
pub const XT_SPEAKER1: c_int = 3;
pub const XT_DIAMOND: c_int = 4;
extern "C" {
    pub fn vortex_XtalkHw_SetGainsAllChan(vortex: *mut *mut vortex_t) -> static void;
}
extern "C" {
    pub fn vortex_XtalkHw_SetSampleRate(vortex: *mut *mut vortex_t, sr: u32) -> static void;
}
extern "C" {
    pub fn vortex_XtalkHw_ProgramPipe(vortex: *mut *mut vortex_t) -> static void;
}
extern "C" {
    pub fn vortex_XtalkHw_ProgramPipe(vortex: *mut *mut vortex_t) -> static void;
}
extern "C" {
    pub fn vortex_XtalkHw_ProgramXtalkWide(vortex: *mut *mut vortex_t) -> static void;
}
extern "C" {
    pub fn vortex_XtalkHw_ProgramXtalkNarrow(vortex: *mut *mut vortex_t) -> static void;
}
extern "C" {
    pub fn vortex_XtalkHw_ProgramDiamondXtalk(vortex: *mut *mut vortex_t) -> static void;
}
extern "C" {
    pub fn vortex_XtalkHw_Enable(vortex: *mut *mut vortex_t) -> static void;
}
extern "C" {
    pub fn vortex_XtalkHw_Disable(vortex: *mut *mut vortex_t) -> static void;
}
extern "C" {
    pub fn vortex_XtalkHw_init(vortex: *mut *mut vortex_t) -> static void;
}
