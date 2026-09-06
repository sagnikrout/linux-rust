//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/via/via_aux_vt1631.c
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
// driver for VIA VT1631 LVDS Transmitter
//

    static const char *name = "VT1631 LVDS Transmitter";
#[no_mangle]
pub unsafe extern "C" fn via_aux_vt1631_probe(bus: *mut via_aux_bus) {
    void via_aux_vt1631_probe(struct via_aux_bus *bus)
    {
    struct via_aux_drv drv = {
    .bus	=	bus,
    .addr	=	0x38,
    .name	=	name};
// check vendor id and device id
    const u8 id[] = {0x06, 0x11, 0x91, 0x31}, len = ARRAY_SIZE(id);
    u8 tmp[ARRAY_SIZE(id)];
    if (!via_aux_read(&drv, 0x00, tmp, len) || memcmp(id, tmp, len))
    return;
    printk(KERN_INFO "viafb: Found %s\n", name);
    via_aux_add(&drv);
    }
