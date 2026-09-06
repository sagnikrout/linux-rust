//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ac97/codec.h
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

//
// struct ac97_id - matches a codec device and driver on an ac97 bus
// @id: The significant bits if the codec vendor ID1 and ID2
// @mask: Bitmask specifying which bits of the id field are significant when
// matching. A driver binds to a device when :
// ((vendorID1 << 8 | vendorID2) & (mask_id1 << 8 | mask_id2)) == id.
// @data: Private data used by the driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ac97_id {
    pub id: c_uint,
    pub mask: c_uint,
    pub data: *mut c_void,
}

//
// struct ac97_codec_device - a ac97 codec
// @dev: the core device
// @vendor_id: the vendor_id of the codec, as sensed on the AC-link
// @num: the codec number, 0 is primary, 1 is first slave, etc ...
// @clk: the clock BIT_CLK provided by the codec
// @ac97_ctrl: ac97 digital controller on the same AC-link
//
// This is the device instantiated for each codec living on a AC-link. There are
// normally 0 to 4 codec devices per AC-link, and all of them are controlled by
// an AC97 digital controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ac97_codec_device {
    pub dev: device,
    pub vendor_id: c_uint,
    pub num: c_uint,
    pub clk: *mut clk,
    pub ac97_ctrl: *mut ac97_controller,
}

//
// struct ac97_codec_driver - a ac97 codec driver
// @driver: the device driver structure
// @probe: the function called when a ac97_codec_device is matched
// @remove: the function called when the device is unbound/removed
// @shutdown: shutdown function (might be NULL)
// @id_table: ac97 vendor_id match table, { } member terminated
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ac97_codec_driver {
    pub driver: device_driver,
    pub ): *mut *mut int (probe)(struct ac97_codec_device,
    pub dev): *mut *mut void (remove)(struct ac97_codec_device,
    pub ): *mut *mut void (shutdown)(struct ac97_codec_device,
    pub id_table: *const ac97_id,
}

extern "C" {
    pub fn container_of(_arg: d, ac97_codec_device: struct, _arg: dev) -> return;
}

extern "C" {
    pub fn snd_ac97_codec_driver_register(drv: *mut ac97_codec_driver) -> c_int;
}
extern "C" {
    pub fn snd_ac97_codec_driver_unregister(drv: *mut ac97_codec_driver);
}

extern "C" {
    pub fn dev_get_drvdata(_arg: ac97_codec_dev2dev(adev)) -> return;
}
