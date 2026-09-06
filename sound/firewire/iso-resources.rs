//! Automatically rewritten from C Header to Rust Module
//! Source: sound/firewire/iso-resources.h
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

// Macro flag: #define SOUND_FIREWIRE_ISO_RESOURCES_H_INCLUDED

//
// struct fw_iso_resources - manages channel/bandwidth allocation
// @channels_mask: if the device does not support all channel numbers, set this
// bit mask to something else than the default (all ones)
//
// This structure manages (de)allocation of isochronous resources (channel and
// bandwidth) for one isochronous stream.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_iso_resources {
    pub channels_mask: u64,
// private:
    pub unit: *mut fw_unit,
    pub mutex: mutex,
    pub channel: c_uint,
    pub /: *mut *mut unsigned int bandwidth; / in bandwidth units, without overhead,
    pub bandwidth_overhead: c_uint,
    pub /: *mut *mut int generation; / in which allocation is valid,
    pub allocated: bool,
}

extern "C" {
    pub fn fw_iso_resources_destroy(r: *mut fw_iso_resources);
}
extern "C" {
    pub fn fw_iso_resources_update(r: *mut fw_iso_resources) -> c_int;
}
extern "C" {
    pub fn fw_iso_resources_free(r: *mut fw_iso_resources);
}
