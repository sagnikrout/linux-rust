//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ata/sata_gemini.h
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
// Header for the Gemini SATA bridge
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gemini_muxmode {
    GEMINI_MUXMODE_0 = 0,
    GEMINI_MUXMODE_1,
    GEMINI_MUXMODE_2,
    GEMINI_MUXMODE_3,
}

extern "C" {
    pub fn gemini_sata_bridge_enabled(sg: *mut sata_gemini, is_ata1: bool) -> bool;
}
extern "C" {
    pub fn gemini_sata_get_muxmode(sg: *mut sata_gemini) -> gemini_muxmode;
}
extern "C" {
    pub fn gemini_sata_start_bridge(sg: *mut sata_gemini, bridge: c_uint) -> c_int;
}
extern "C" {
    pub fn gemini_sata_stop_bridge(sg: *mut sata_gemini, bridge: c_uint);
}
