//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ivtv/ivtv-mailbox.h
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
pub const IVTV_MBOX_DMA_END: c_int = 8;
pub const IVTV_MBOX_DMA: c_int = 9;
extern "C" {
    pub fn ivtv_api(itv: *mut ivtv, cmd: c_int, args: c_int, data[]: u32) -> c_int;
}
extern "C" {
    pub fn ivtv_vapi_result(itv: *mut ivtv, data[CX2341X_MBOX_MAX_DATA]: u32, cmd: c_int, args: c_int, ...) -> c_int;
}
extern "C" {
    pub fn ivtv_vapi(itv: *mut ivtv, cmd: c_int, args: c_int, ...) -> c_int;
}
extern "C" {
    pub fn ivtv_api_func(priv: *mut c_void, cmd: u32, in: c_int, out: c_int, data[CX2341X_MBOX_MAX_DATA]: u32) -> c_int;
}
extern "C" {
    pub fn ivtv_mailbox_cache_invalidate(itv: *mut ivtv);
}
