//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/qcom/qdsp6/q6adm.h
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
pub const ADM_PATH_PLAYBACK: c_uint = 0x1;
pub const ADM_PATH_LIVE_REC: c_uint = 0x2;
pub const MAX_COPPS_PER_PORT: c_int = 8;
pub const NULL_COPP_TOPOLOGY: c_uint = 0x00010312;
// multiple copp per stream.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct route_payload {
    pub num_copps: c_int,
    pub session_id: c_int,
    pub copp_idx: [c_int; MAX_COPPS_PER_PORT],
    pub port_id: [c_int; MAX_COPPS_PER_PORT],
}

extern "C" {
    pub fn q6adm_close(dev: *mut device, copp: *mut q6copp) -> c_int;
}
extern "C" {
    pub fn q6adm_get_copp_id(copp: *mut q6copp) -> c_int;
}
