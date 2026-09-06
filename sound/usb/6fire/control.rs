//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/6fire/control.h
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
// Linux driver for TerraTec DMX 6Fire USB
//
// Author:	Torsten Schenk <torsten.schenk@zoho.com>
// Created:	Jan 01, 2011
// Copyright:	(C) Torsten Schenk
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_runtime {
    pub rt): *mut *mut int (update_streaming)(struct control_runtime,
    pub rate): *mut *mut *mut int (set_rate)(struct control_runtime rt, int,
    pub spdif_in): int n_analog_in, bool spdif_out, bool,
    pub chip: *mut sfire_chip,
    pub element: [*mut snd_kcontrol; CONTROL_MAX_ELEMENTS],
    pub opt_coax_switch: bool,
    pub line_phono_switch: bool,
    pub digital_thru_switch: bool,
    pub usb_streaming: bool,
    pub output_vol: [u8; 6],
    pub ovol_updated: u8,
    pub output_mute: u8,
    pub input_vol: [i8; 2],
    pub ivol_updated: u8,
}

extern "C" {
    pub fn usb6fire_control_init(chip: *mut sfire_chip) -> c_int;
}
extern "C" {
    pub fn usb6fire_control_abort(chip: *mut sfire_chip);
}
extern "C" {
    pub fn usb6fire_control_destroy(chip: *mut sfire_chip);
}
