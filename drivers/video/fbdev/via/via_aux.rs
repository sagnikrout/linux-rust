//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/via_aux.h
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
// Copyright 2011 Florian Tobias Schandinat <FlorianSchandinat@gmx.de>
//
// infrastructure for devices connected via I2C
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct via_aux_bus {
    pub /: *mut *mut *mut i2c_adapter adap; / the I2C device to access the bus,
    pub /: *mut *mut list_head drivers; / drivers for devices on this bus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct via_aux_drv {
    pub /: *mut *mut list_head chain; / chain to support multiple drivers,
    pub /: *mut *mut *mut via_aux_bus bus; / the I2C bus used,
    pub /: *mut *mut u8 addr; / the I2C target address,
    pub /: *const *const *const char name; / human readable name of the driver,
    pub /: *mut *mut *mut void data; / private data of this driver,
    pub drv): *mut *mut void (cleanup)(struct via_aux_drv,
    pub drv): *mut (struct via_aux_drv,
}

extern "C" {
    pub fn via_aux_free(bus: *mut via_aux_bus);
}
// data = *drv;
// probe functions of existing drivers - should only be called in via_aux.c
extern "C" {
    pub fn via_aux_ch7301_probe(bus: *mut via_aux_bus);
}
extern "C" {
    pub fn via_aux_edid_probe(bus: *mut via_aux_bus);
}
extern "C" {
    pub fn via_aux_sii164_probe(bus: *mut via_aux_bus);
}
extern "C" {
    pub fn via_aux_vt1636_probe(bus: *mut via_aux_bus);
}
extern "C" {
    pub fn via_aux_vt1632_probe(bus: *mut via_aux_bus);
}
extern "C" {
    pub fn via_aux_vt1631_probe(bus: *mut via_aux_bus);
}
extern "C" {
    pub fn via_aux_vt1625_probe(bus: *mut via_aux_bus);
}
extern "C" {
    pub fn via_aux_vt1622_probe(bus: *mut via_aux_bus);
}
extern "C" {
    pub fn via_aux_vt1621_probe(bus: *mut via_aux_bus);
}
