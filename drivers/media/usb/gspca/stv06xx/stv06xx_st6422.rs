//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/stv06xx/stv06xx_st6422.h
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
// Support for the sensor part which is integrated (I think) into the
// st6422 stv06xx alike bridge, as its integrated there are no i2c writes
// but instead direct bridge writes.
//
// Copyright (c) 2009 Hans de Goede <hdegoede@redhat.com>
//
// Strongly based on qc-usb-messenger, which is:
// Copyright (c) 2001 Jean-Fredric Clere, Nikolas Zimmermann, Georg Acher
// Mark Cave-Ayland, Carlo E Prelz, Dick Streefland
// Copyright (c) 2002, 2003 Tuukka Toivonen
//

extern "C" {
    pub fn st6422_probe(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn st6422_start(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn st6422_init(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn st6422_init_controls(sd: *mut sd) -> static int;
}
extern "C" {
    pub fn st6422_stop(sd: *mut sd) -> static int;
}
// No known way to lower framerate in case of less bandwidth
