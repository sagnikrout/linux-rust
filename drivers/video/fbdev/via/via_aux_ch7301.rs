//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/via/via_aux_ch7301.c
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
// driver for Chrontel CH7301 DVI Transmitter
//

    static const char *name = "CH7301 DVI Transmitter";
#[no_mangle]
unsafe extern "C" fn probe(bus: *mut via_aux_bus, addr: u8) {
    static void probe(struct via_aux_bus *bus, u8 addr)
    {
    struct via_aux_drv drv = {
    .bus	=	bus,
    .addr	=	addr,
    .name	=	name};
    u8 tmp;
    if (!via_aux_read(&drv, 0x4B, &tmp, 1) || tmp != 0x17)
    return;
    printk(KERN_INFO "viafb: Found %s at address 0x%x\n", name, addr);
    via_aux_add(&drv);
    }
#[no_mangle]
pub unsafe extern "C" fn via_aux_ch7301_probe(bus: *mut via_aux_bus) {
    void via_aux_ch7301_probe(struct via_aux_bus *bus)
    {
    probe(bus, 0x75);
    probe(bus, 0x76);
    }
