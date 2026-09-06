//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/zoran/zoran_device.h
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
// Zoran zr36057/zr36067 PCI controller driver, for the
// Pinnacle/Miro DC10/DC10+/DC30/DC30+, Iomega Buz, Linux
// Media Labs LML33/LML33R10.
//
// This part handles card-specific data and detection
//
// Copyright (C) 2000 Serguei Miridonov <mirsev@cicese.mx>
//
// general purpose I/O
extern "C" {
    pub fn GPIO(zr: *mut zoran, bit: c_int, value: c_uint);
}
// codec (or actually: guest bus) access
extern "C" {
    pub fn post_office_wait(zr: *mut zoran) -> c_int;
}
extern "C" {
    pub fn post_office_read(zr: *mut zoran, guest: c_uint, reg: c_uint) -> c_int;
}
extern "C" {
    pub fn jpeg_codec_sleep(zr: *mut zoran, sleep: c_int);
}
extern "C" {
    pub fn jpeg_codec_reset(zr: *mut zoran) -> c_int;
}
// zr360x7 access to raw capture
extern "C" {
    pub fn zr36057_set_memgrab(zr: *mut zoran, mode: c_int);
}
extern "C" {
    pub fn wait_grab_pending(zr: *mut zoran) -> c_int;
}
// interrupts
extern "C" {
    pub fn print_interrupts(zr: *mut zoran);
}
extern "C" {
    pub fn clear_interrupt_counters(zr: *mut zoran);
}
extern "C" {
    pub fn zoran_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
// JPEG codec access
extern "C" {
    pub fn jpeg_start(zr: *mut zoran);
}
extern "C" {
    pub fn zr36057_enable_jpg(zr: *mut zoran, mode: zoran_codec_mode);
}
extern "C" {
    pub fn zoran_feed_stat_com(zr: *mut zoran);
}
// general
extern "C" {
    pub fn zoran_set_pci_master(zr: *mut zoran, set_master: c_int);
}
extern "C" {
    pub fn zoran_init_hardware(zr: *mut zoran);
}
extern "C" {
    pub fn zr36057_restart(zr: *mut zoran);
}
// i2c

