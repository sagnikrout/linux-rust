//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ac97/controller.h
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
//
// Copyright (C) 2016 Robert Jarzmik <robert.jarzmik@free.fr>
//

pub const AC97_BUS_MAX_CODECS: c_int = 4;
pub const AC97_SLOTS_AVAILABLE_ALL: c_uint = 0xf;
//
// struct ac97_controller - The AC97 controller of the AC-Link
// @ops:		the AC97 operations.
// @controllers:	linked list of all existing controllers.
// @adap:		the shell device ac97-%d, ie. ac97 adapter
// @nr:			the number of the shell device
// @slots_available:	the mask of accessible/scanable codecs.
// @parent:		the device providing the AC97 controller.
// @codecs:		the 4 possible AC97 codecs (NULL if none found).
// @codecs_pdata:	platform_data for each codec (NULL if no pdata).
//
// This structure is internal to AC97 bus, and should not be used by the
// controllers themselves, excepting for using @dev.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ac97_controller {
    pub ops: *const ac97_controller_ops,
    pub controllers: list_head,
    pub adap: device,
    pub nr: c_int,
    pub slots_available: c_ushort,
    pub parent: *mut device,
    pub codecs: [*mut ac97_codec_device; AC97_BUS_MAX_CODECS],
    pub codecs_pdata: [*mut c_void; AC97_BUS_MAX_CODECS],
}

//
// struct ac97_controller_ops - The AC97 operations
// @reset:	Cold reset of the AC97 AC-Link.
// @warm_reset:	Warm reset of the AC97 AC-Link.
// @read:	Read of a single AC97 register.
// Returns the register value or a negative error code.
// @write:	Write of a single AC97 register.
//
// These are the basic operation an AC97 controller must provide for an AC97
// access functions. Amongst these, all but the last 2 are mandatory.
// The slot number is also known as the AC97 codec number, between 0 and 3.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ac97_controller_ops {
    pub adrv): *mut *mut void (reset)(struct ac97_controller,
    pub adrv): *mut *mut void (warm_reset)(struct ac97_controller,
    pub val): unsigned short reg, unsigned short,
    pub reg): *mut *mut *mut int (read)(struct ac97_controller adrv, int slot, unsigned short,
}

extern "C" {
    pub fn snd_ac97_controller_unregister(ac97_ctrl: *mut ac97_controller);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

