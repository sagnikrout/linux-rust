//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/pvrusb2/pvrusb2-ctrl.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvr2_ctl_type {
    pvr2_ctl_int = 0,
    pvr2_ctl_enum = 1,
    pvr2_ctl_bitmask = 2,
    pvr2_ctl_bool = 3,
}

// Set the given control.
extern "C" {
    pub fn pvr2_ctrl_set_value(: *mut pvr2_ctrl, val: c_int) -> c_int;
}
// Set/clear specific bits of the given control.
extern "C" {
    pub fn pvr2_ctrl_set_mask_value(: *mut pvr2_ctrl, mask: c_int, val: c_int) -> c_int;
}
// Get the current value of the given control.
extern "C" {
    pub fn pvr2_ctrl_get_value(: *mut pvr2_ctrl, valptr: *mut c_int) -> c_int;
}
// Retrieve control's type
extern "C" {
    pub fn pvr2_ctrl_get_type(: *mut pvr2_ctrl) -> pvr2_ctl_type;
}
// Retrieve control's maximum value (int type)
extern "C" {
    pub fn pvr2_ctrl_get_max(: *mut pvr2_ctrl) -> c_int;
}
// Retrieve control's minimum value (int type)
extern "C" {
    pub fn pvr2_ctrl_get_min(: *mut pvr2_ctrl) -> c_int;
}
// Retrieve control's default value (any type)
extern "C" {
    pub fn pvr2_ctrl_get_def(: *mut pvr2_ctrl, valptr: *mut c_int) -> c_int;
}
// Retrieve control's enumeration count (enum only)
extern "C" {
    pub fn pvr2_ctrl_get_cnt(: *mut pvr2_ctrl) -> c_int;
}
// Retrieve control's valid mask bits (bit mask only)
extern "C" {
    pub fn pvr2_ctrl_get_mask(: *mut pvr2_ctrl) -> c_int;
}
// Retrieve the control's name
// Retrieve the control's desc
// Retrieve a control enumeration or bit mask value
// Return true if control is writable
extern "C" {
    pub fn pvr2_ctrl_is_writable(: *mut pvr2_ctrl) -> c_int;
}
// Return V4L flags value for control (or zero if there is no v4l control
extern "C" {
    pub fn pvr2_ctrl_get_v4lflags(: *mut pvr2_ctrl) -> c_uint;
}
// Return V4L ID for this control or zero if none
extern "C" {
    pub fn pvr2_ctrl_get_v4lid(: *mut pvr2_ctrl) -> c_int;
}
// Return true if control has custom symbolic representation
extern "C" {
    pub fn pvr2_ctrl_has_custom_symbols(: *mut pvr2_ctrl) -> c_int;
}
// Convert a given mask/val to a custom symbolic value
// Convert a symbolic value to a mask/value pair
// Convert a given mask/val to a symbolic value
// Convert a symbolic value to a mask/value pair
// Convert a given mask/val to a symbolic value - must already be
