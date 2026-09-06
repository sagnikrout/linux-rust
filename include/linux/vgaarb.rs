//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vgaarb.h
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


// SPDX-License-Identifier: MIT
//
// The VGA aribiter manages VGA space routing and VGA resource decode to
// allow multiple VGA devices to be used in a system in a safe way.
//
// (C) Copyright 2005 Benjamin Herrenschmidt <benh@kernel.crashing.org>
// (C) Copyright 2007 Paulo R. Zanoni <przanoni@gmail.com>
// (C) Copyright 2007, 2009 Tiago Vignatti <vignatti@freedesktop.org>
//

// Legacy VGA regions
pub const VGA_RSRC_NONE: c_uint = 0x00;
pub const VGA_RSRC_LEGACY_IO: c_uint = 0x01;
pub const VGA_RSRC_LEGACY_MEM: c_uint = 0x02;

// Non-legacy access
pub const VGA_RSRC_NORMAL_IO: c_uint = 0x04;
pub const VGA_RSRC_NORMAL_MEM: c_uint = 0x08;

extern "C" {
    pub fn vga_set_legacy_decoding(pdev: *mut pci_dev, decodes: c_uint);
}
extern "C" {
    pub fn vga_get(pdev: *mut pci_dev, rsrc: c_uint, interruptible: c_int) -> c_int;
}
extern "C" {
    pub fn vga_put(pdev: *mut pci_dev, rsrc: c_uint);
}
extern "C" {
    pub fn vga_set_default_device(pdev: *mut pci_dev);
}
extern "C" {
    pub fn vga_remove_vgacon(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn int(pdev: *mut *mut set_decode)(struct pci_dev, state): bool) -> unsigned;
}

//
// vga_get_interruptible
// @pdev: pci device of the VGA card or NULL for the system default
// @rsrc: bit mask of resources to acquire and lock
//
// Shortcut to vga_get with interruptible set to true.
//
// On success, release the VGA resource again with vga_put().
//
extern "C" {
    pub fn vga_get(_arg: pdev, _arg: rsrc, _arg: 1) -> return;
}
//
// vga_get_uninterruptible - shortcut to vga_get()
// @pdev: pci device of the VGA card or NULL for the system default
// @rsrc: bit mask of resources to acquire and lock
//
// Shortcut to vga_get with interruptible set to false.
//
// On success, release the VGA resource again with vga_put().
//
extern "C" {
    pub fn vga_get(_arg: pdev, _arg: rsrc, _arg: 0) -> return;
}
