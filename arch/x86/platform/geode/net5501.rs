//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/geode/net5501.c
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
// System Specific setup for Soekris net5501
// At the moment this means setup of GPIO control of LEDs and buttons
// on net5501 boards.
//
// Copyright (C) 2008-2009 Tower Technologies
// Written by Alessandro Zummo <a.zummo@towertech.it>
//
// Copyright (C) 2008 Constantin Baranov <const@mimas.ru>
// Copyright (C) 2011 Ed Wildgoose <kernel@wildgooses.com>
// and Philip Prindeville <philipp@redfish-solutions.com>
//

pub const BIOS_REGION_BASE: c_uint = 0xffff0000;
pub const BIOS_REGION_SIZE: c_uint = 0x00010000;
    static const struct geode_led net5501_leds[] __initconst = {
    { 6, true },
    };
#[no_mangle]
unsafe extern "C" fn register_net5501() -> void __init {
    static void __init register_net5501(void)
    {
    geode_create_restart_key(24);
    geode_create_leds("net5501", net5501_leds, ARRAY_SIZE(net5501_leds));
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net5501_board {
    pub offset: u16,
    pub len: u16,
    pub sig: *mut c_char,
}

    static struct net5501_board __initdata boards[] = {
    { 0xb7b, 7, "net5501" },	/* net5501 v1.33/1.33c */
    { 0xb1f, 7, "net5501" },	/* net5501 v1.32i */
    };
#[no_mangle]
unsafe extern "C" fn net5501_present() -> bool __init {
    static bool __init net5501_present(void)
    {
    int i;
    unsigned char *rombase, *bios;
    let mut found: bool = false;
    rombase = ioremap(BIOS_REGION_BASE, BIOS_REGION_SIZE - 1);
    if (!rombase) {
    printk(KERN_ERR "%s: failed to get rombase\n", KBUILD_MODNAME);
    return found;
    }
    bios = rombase + 0x20;	/* null terminated */
    if (memcmp(bios, "comBIOS", 7))
    goto unmap;
    for (i = 0; i < ARRAY_SIZE(boards); i++) {
    unsigned char *model = rombase + boards[i].offset;
    if (!memcmp(model, boards[i].sig, boards[i].len)) {
    printk(KERN_INFO "%s: system is recognized as \"%s\"\n",
    KBUILD_MODNAME, model);
    found = true;
    break;
    }
    }
    unmap:
    iounmap(rombase);
    return found;
    }
#[no_mangle]
unsafe extern "C" fn net5501_init() -> int __init {
    static int __init net5501_init(void)
    {
    if (!is_geode())
    return 0;
    if (!net5501_present())
    return 0;
    register_net5501();
    return 0;
    }
    device_initcall(net5501_init);
