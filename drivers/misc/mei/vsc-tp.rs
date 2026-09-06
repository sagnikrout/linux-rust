//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/mei/vsc-tp.h
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
// Copyright (c) 2023, Intel Corporation.
// Intel Visual Sensing Controller Transport Layer Linux driver
//

pub const VSC_TP_CMD_WRITE: c_uint = 0x01;
pub const VSC_TP_CMD_READ: c_uint = 0x02;
pub const VSC_TP_CMD_ACK: c_uint = 0x10;
pub const VSC_TP_CMD_NACK: c_uint = 0x11;
pub const VSC_TP_CMD_BUSY: c_uint = 0x12;
//
// typedef vsc_event_cb_t - event callback function signature
// @context: the execution context of who registered this callback
//
// The callback function is called in interrupt context and the data
// payload is only valid during the call. If the user needs access
// the data payload later, it must copy the payload.
//
extern "C" {
    pub fn void(context: *mut *mut vsc_tp_event_cb_t)(void) -> typedef;
}
extern "C" {
    pub fn vsc_tp_intr_enable(tp: *mut vsc_tp);
}
extern "C" {
    pub fn vsc_tp_intr_disable(tp: *mut vsc_tp);
}
extern "C" {
    pub fn vsc_tp_intr_synchronize(tp: *mut vsc_tp);
}
extern "C" {
    pub fn vsc_tp_reset(tp: *mut vsc_tp);
}
extern "C" {
    pub fn vsc_tp_need_read(tp: *mut vsc_tp) -> bool;
}
extern "C" {
    pub fn vsc_tp_init(tp: *mut vsc_tp, dev: *mut device) -> c_int;
}
