//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/v4l2-core/v4l2-ctrls-priv.h
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
// V4L2 controls framework private header.
//
// Copyright (C) 2010-2021  Hans Verkuil <hverkuil@kernel.org>
//

//
// Small helper function to determine if the autocluster is set to manual
// mode.
//
// Small helper function to determine if the autocluster will be set to manual
// mode.
//
// v4l2-ctrls-core.c
extern "C" {
    pub fn cur_to_new(ctrl: *mut v4l2_ctrl);
}
extern "C" {
    pub fn cur_to_req(ref: *mut v4l2_ctrl_ref);
}
extern "C" {
    pub fn new_to_cur(fh: *mut v4l2_fh, ctrl: *mut v4l2_ctrl, ch_flags: u32);
}
extern "C" {
    pub fn new_to_req(ref: *mut v4l2_ctrl_ref);
}
extern "C" {
    pub fn req_to_new(ref: *mut v4l2_ctrl_ref) -> c_int;
}
extern "C" {
    pub fn send_initial_event(fh: *mut v4l2_fh, ctrl: *mut v4l2_ctrl);
}
extern "C" {
    pub fn send_event(fh: *mut v4l2_fh, ctrl: *mut v4l2_ctrl, changes: u32);
}
extern "C" {
    pub fn update_from_auto_cluster(master: *mut v4l2_ctrl);
}
// v4l2-ctrls-api.c
// v4l2-ctrls-request.c
extern "C" {
    pub fn v4l2_ctrl_handler_init_request(hdl: *mut v4l2_ctrl_handler);
}
extern "C" {
    pub fn v4l2_ctrl_handler_free_request(hdl: *mut v4l2_ctrl_handler);
}
