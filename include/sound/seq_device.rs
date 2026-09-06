//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/seq_device.h
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
// ALSA sequencer device management
// Copyright (c) 1999 by Takashi Iwai <tiwai@suse.de>
//
// registered device information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_device {
// device info
    pub /: *mut *mut *mut snd_card card; / sound card,
    pub /: *mut *mut int device; / device number,
    pub /: *const *const *const char id; / driver id,
    pub /: *mut *mut char name[80]; / device name,
    pub /: *mut *mut int argsize; / size of the argument,
    pub /: *mut *mut *mut void driver_data; / private data for driver,
    pub /: *mut *mut *mut void private_data; / private data for the caller,
    pub device): *mut *mut void (private_free)(struct snd_seq_device,
    pub dev: device,
    pub /: *mut *mut unsigned char args[]; / driver-specific argument,
}

// sequencer driver
// driver operators
// probe:
// Initialize the device with given parameters.
// Typically,
// 1. call snd_hwdep_new
// 2. allocate private data and initialize it
// 3. call snd_hwdep_register
// 4. store the instance to dev->driver_data pointer.
//
// remove:
// Release the private data.
// Typically, call snd_device_free(dev->card, dev->driver_data)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_driver {
    pub dev): *mut *mut int (probe)(struct snd_seq_device,
    pub dev): *mut *mut void (remove)(struct snd_seq_device,
    pub driver: device_driver,
    pub id: *mut c_char,
    pub argsize: c_int,
}

//
// prototypes
//

extern "C" {
    pub fn snd_seq_device_load_drivers();
}

// Macro flag: #define snd_seq_device_load_drivers()

extern "C" {
    pub fn snd_seq_driver_unregister(drv: *mut snd_seq_driver);
}

//
// id strings for generic devices
//

