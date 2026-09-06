//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/input/mt.h
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
// Input Multitouch Library
//
// Copyright (c) 2010 Henrik Rydberg
//

pub const TRKID_MAX: c_uint = 0xffff;
pub const INPUT_MT_POINTER: c_uint = 0x0001	/* pointer device, e.g. trackpad */;
pub const INPUT_MT_DIRECT: c_uint = 0x0002	/* direct device, e.g. touchscreen */;
pub const INPUT_MT_DROP_UNUSED: c_uint = 0x0004	/* drop contacts not seen in frame */;
pub const INPUT_MT_TRACK: c_uint = 0x0008	/* use in-kernel tracking */;
pub const INPUT_MT_SEMI_MT: c_uint = 0x0010	/* semi-mt device, finger count handled manually */;
pub const INPUT_MT_TOTAL_FORCE: c_uint = 0x0020	/* calculate total force from slots pressure */;
//
// struct input_mt_slot - represents the state of an input MT slot
// @abs: holds current values of ABS_MT axes for this slot
// @frame: last frame at which input_mt_report_slot_state() was called
// @key: optional driver designation of this slot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_mt_slot {
    pub 1]: int abs[ABS_MT_LAST - ABS_MT_FIRST +,
    pub frame: c_uint,
    pub key: c_uint,
}

//
// struct input_mt - state of tracked contacts
// @trkid: stores MT tracking ID for the next contact
// @num_slots: number of MT slots the device uses
// @slot: MT slot currently being transmitted
// @flags: input_mt operation flags
// @frame: increases every time input_mt_sync_frame() is called
// @red: reduced cost matrix for in-kernel tracking
// @slots: array of slots holding current values of tracked contacts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_mt {
    pub trkid: c_int,
    pub num_slots: c_int,
    pub slot: c_int,
    pub flags: c_uint,
    pub frame: c_uint,
    pub red: *mut c_int,
    pub __counted_by(num_slots): input_mt_slot slots[],
}

extern "C" {
    pub fn input_mt_destroy_slots(dev: *mut input_dev);
}
extern "C" {
    pub fn input_mt_report_finger_count(dev: *mut input_dev, count: c_int);
}
extern "C" {
    pub fn input_mt_report_pointer_emulation(dev: *mut input_dev, use_count: bool);
}
extern "C" {
    pub fn input_mt_drop_unused(dev: *mut input_dev);
}
extern "C" {
    pub fn input_mt_sync_frame(dev: *mut input_dev);
}
//
// struct input_mt_pos - contact position
// @x: horizontal coordinate
// @y: vertical coordinate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_mt_pos {
    pub y: s16 x,,
}

extern "C" {
    pub fn input_mt_get_slot_by_key(dev: *mut input_dev, key: c_int) -> c_int;
}
