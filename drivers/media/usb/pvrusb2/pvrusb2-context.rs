//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/pvrusb2/pvrusb2-context.h
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
// Copyright (C) 2005 Mike Isely <isely@pobox.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_context_stream {
    pub user: *mut pvr2_channel,
    pub stream: *mut pvr2_stream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_context {
    pub mc_first: *mut pvr2_channel,
    pub mc_last: *mut pvr2_channel,
    pub exist_next: *mut pvr2_context,
    pub exist_prev: *mut pvr2_context,
    pub notify_next: *mut pvr2_context,
    pub notify_prev: *mut pvr2_context,
    pub hdw: *mut pvr2_hdw,
    pub video_stream: pvr2_context_stream,
    pub mutex: mutex,
    pub notify_flag: c_int,
    pub initialized_flag: c_int,
    pub disconnect_flag: c_int,
// Called after pvr2_context initialization is complete
    pub ): *mut *mut void (setup_func)(struct pvr2_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_channel {
    pub mc_head: *mut pvr2_context,
    pub mc_next: *mut pvr2_channel,
    pub mc_prev: *mut pvr2_channel,
    pub stream: *mut pvr2_context_stream,
    pub hdw: *mut pvr2_hdw,
    pub input_mask: c_uint,
    pub ): *mut *mut void (check_func)(struct pvr2_channel,
}

extern "C" {
    pub fn pvr2_context_disconnect(: *mut pvr2_context);
}
extern "C" {
    pub fn pvr2_channel_init(: *mut pvr2_channel, : *mut pvr2_context);
}
extern "C" {
    pub fn pvr2_channel_done(: *mut pvr2_channel);
}
extern "C" {
    pub fn pvr2_channel_limit_inputs(: *mut pvr2_channel, int: unsigned) -> c_int;
}
extern "C" {
    pub fn pvr2_channel_get_limited_inputs(: *mut pvr2_channel) -> c_uint;
}
extern "C" {
    pub fn pvr2_context_global_init() -> c_int;
}
extern "C" {
    pub fn pvr2_context_global_done();
}
