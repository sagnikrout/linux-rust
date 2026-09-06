//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/radio/radio-isa.h
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
// Framework for ISA radio drivers.
// This takes care of all the V4L2 scaffolding, allowing the ISA drivers
// to concentrate on the actual hardware operation.
//
// Copyright (C) 2012 Hans Verkuil <hverkuil@kernel.org>
//

// Core structure for radio ISA cards
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radio_isa_card {
    pub drv: *const radio_isa_driver,
    pub v4l2_dev: v4l2_device,
    pub hdl: v4l2_ctrl_handler,
    pub vdev: video_device,
    pub lock: mutex,
    pub ops: *const radio_isa_ops,
    pub mute: *mut v4l2_ctrl,
    pub volume: *mut v4l2_ctrl,
}

// I/O port
// Card is in stereo audio mode
// Current frequency
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radio_isa_ops {
// Allocate and initialize a radio_isa_card struct
    pub (*alloc)(void): *mut radio_isa_card,
// Probe whether a card is present at the given port
    pub io): *mut *mut *mut bool (probe)(struct radio_isa_card isa, int,
// Special card initialization can be done here, this is called after
// the standard controls are registered, but before they are setup,
// thus allowing drivers to add their own controls here.
    pub isa): *mut *mut int (init)(struct radio_isa_card,
// Set mute and volume.
    pub volume): *mut *mut *mut int (s_mute_volume)(struct radio_isa_card isa, bool mute, int,
// Set frequency
    pub freq): *mut *mut *mut int (s_frequency)(struct radio_isa_card isa, u32,
// Set stereo/mono audio mode
    pub stereo): *mut *mut *mut int (s_stereo)(struct radio_isa_card isa, bool,
// Get rxsubchans value for VIDIOC_G_TUNER
    pub isa): *mut *mut u32 (g_rxsubchans)(struct radio_isa_card,
// Get the signal strength for VIDIOC_G_TUNER
    pub isa): *mut *mut u32 (g_signal)(struct radio_isa_card,
}

// Top level structure needed to instantiate the cards
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radio_isa_driver {
    pub driver: isa_driver,

    pub pnp_driver: pnp_driver,

    pub ops: *const radio_isa_ops,
// The module_param_array with the specified I/O ports
    pub io_params: *mut c_int,
// The module_param_array with the radio_nr values
    pub radio_nr_params: *mut c_int,
// Whether we should probe for possible cards
    pub probe: bool,
// The list of possible I/O ports
    pub io_ports: *const c_int,
// The size of that list
    pub num_of_io_ports: c_int,
// The region size to request
    pub region_size: unsigned,
// The name of the card
    pub card: *const c_char,
// Card can capture stereo audio
    pub has_stereo: bool,
// The maximum volume for the volume control. If 0, then there
    pub max_volume: c_int,
}

extern "C" {
    pub fn radio_isa_match(pdev: *mut device, dev: c_uint) -> c_int;
}
extern "C" {
    pub fn radio_isa_probe(pdev: *mut device, dev: c_uint) -> c_int;
}
extern "C" {
    pub fn radio_isa_remove(pdev: *mut device, dev: c_uint);
}

extern "C" {
    pub fn radio_isa_pnp_remove(dev: *mut pnp_dev);
}

