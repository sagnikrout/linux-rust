//! Automatically rewritten from C Header to Rust Module
//! Source: sound/aoa/aoa.h
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
// Apple Onboard Audio definitions
//
// Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
//

pub const MAX_CODEC_NAME_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aoa_codec {
    pub name: [c_char; MAX_CODEC_NAME_LEN],
    pub owner: *mut module,
// called when the fabric wants to init this codec.
// Do alsa card manipulations from here.
    pub codec): *mut *mut int (init)(struct aoa_codec,
// called when the fabric is done with the codec.
// The alsa card will be cleaned up so don't bother.
    pub codec): *mut *mut void (exit)(struct aoa_codec,
// May be NULL, but can be used by the fabric.
// Refcounting is the codec driver's responsibility
    pub node: *mut device_node,
// assigned by fabric before init() is called, points
// to the soundbus device. Cannot be NULL.
    pub soundbus_dev: *mut soundbus_dev,
// assigned by the fabric before init() is called, points
// to the fabric's gpio runtime record for the relevant
// device.
    pub gpio: *mut gpio_runtime,
// assigned by the fabric before init() is called, contains
// a codec specific bitmask of what outputs and inputs are
// actually connected
    pub connected: u32,
// data the fabric can associate with this structure
    pub fabric_data: *const c_void,
// private!
    pub list: list_head,
    pub fabric: *mut aoa_fabric,
}

// return 0 on success
pub const MAX_LAYOUT_NAME_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aoa_fabric {
    pub name: [c_char; MAX_LAYOUT_NAME_LEN],
    pub owner: *mut module,
// once codecs register, they are passed here after.
// They are of course not initialised, since the
// fabric is responsible for initialising some fields
// in the codec structure!
    pub codec): *mut *mut int (found_codec)(struct aoa_codec,
// called for each codec when it is removed,
// also in the case that aoa_fabric_unregister
// is called and all codecs are removed
// from this fabric.
// Also called if found_codec returned 0 but
// the codec couldn't initialise.
    pub codec): *mut *mut void (remove_codec)(struct aoa_codec,
// If found_codec returned 0, and the codec
// could be initialised, this is called.
    pub codec): *mut *mut void (attached_codec)(struct aoa_codec,
}

// return 0 on success, -EEXIST if another fabric is
// registered, -EALREADY if the same fabric is registered.
// Passing NULL can be used to test for the presence
// of another fabric, if -EALREADY is returned there is
// no other fabric present.
// In the case that the function returns -EALREADY
// and the fabric passed is not NULL, all codecs
// that are not assigned yet are passed to the fabric
// again for reconsideration.
// it is vital to call this when the fabric exits!
// When calling, the remove_codec will be called
// for all codecs, unless it is NULL.
// if for some reason you want to get rid of a codec
// before the fabric is removed, use this.
// Note that remove_codec is called for it!
// alsa help methods
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aoa_card {
    pub alsa_card: *mut snd_card,
}

extern "C" {
    pub fn aoa_snd_ctl_add(control: *mut *mut snd_kcontrol) -> c_int;
}
// GPIO stuff
// extern struct gpio_methods *map_gpio_methods;
