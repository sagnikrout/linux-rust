//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/geode/alix.c
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
// System Specific setup for PCEngines ALIX.
// At the moment this means setup of GPIO control of LEDs
// on Alix.2/3/6 boards.
//
// Copyright (C) 2008 Constantin Baranov <const@mimas.ru>
// Copyright (C) 2011 Ed Wildgoose <kernel@wildgooses.com>
// and Philip Prindeville <philipp@redfish-solutions.com>
//

pub const BIOS_SIGNATURE_TINYBIOS: c_uint = 0xf0000;
pub const BIOS_SIGNATURE_COREBOOT: c_uint = 0x500;
pub const BIOS_REGION_SIZE: c_uint = 0x10000;
//
// This driver is not modular, but to keep back compatibility
// with existing use cases, continuing with module_param is
// the easiest way forward.
//
    let mut force: static bool = 0;
    module_param(force, bool, 0444);
// FIXME: Award bios is not automatically detected as Alix platform
    MODULE_PARM_DESC(force, "Force detection as ALIX.2/ALIX.3 platform");
    static const struct geode_led alix_leds[] __initconst = {
    { 6, true },
    { 25, false },
    { 27, false },
    };
#[no_mangle]
unsafe extern "C" fn register_alix() -> void __init {
    static void __init register_alix(void)
    {
    geode_create_restart_key(24);
    geode_create_leds("alix", alix_leds, ARRAY_SIZE(alix_leds));
    }
    static bool __init alix_present(unsigned long bios_phys,
    const char *alix_sig,
    size_t alix_sig_len)
    {
    let mut bios_len: usize = BIOS_REGION_SIZE;
    const char *bios_virt;
    const char *scan_end;
    const char *p;
    char name[64];
    if (force) {
    printk(KERN_NOTICE "%s: forced to skip BIOS test, "
    "assume system is ALIX.2/ALIX.3\n",
    KBUILD_MODNAME);
    return true;
    }
    bios_virt = phys_to_virt(bios_phys);
    scan_end = bios_virt + bios_len - (alix_sig_len + 2);
    for (p = bios_virt; p < scan_end; p++) {
    const char *tail;
    char *a;
    if (memcmp(p, alix_sig, alix_sig_len) != 0)
    continue;
    memcpy(name, p, sizeof(name));
// remove the first \0 character from string
    a = strchr(name, '\0');
    if (a)
// a = ' ';
// cut the string at a newline
    a = strchr(name, '\r');
    if (a)
// a = '\0';
    tail = p + alix_sig_len;
    if ((tail[0] == '2' || tail[0] == '3' || tail[0] == '6')) {
    printk(KERN_INFO
    "%s: system is recognized as \"%s\"\n",
    KBUILD_MODNAME, name);
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn alix_present_dmi() -> bool __init {
    static bool __init alix_present_dmi(void)
    {
    const char *vendor, *product;
    vendor = dmi_get_system_info(DMI_SYS_VENDOR);
    if (!vendor || strcmp(vendor, "PC Engines"))
    return false;
    product = dmi_get_system_info(DMI_PRODUCT_NAME);
    if (!product || (strcmp(product, "ALIX.2D") && strcmp(product, "ALIX.6")))
    return false;
    printk(KERN_INFO "%s: system is recognized as \"%s %s\"\n",
    KBUILD_MODNAME, vendor, product);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn alix_init() -> int __init {
    static int __init alix_init(void)
    {
    const char tinybios_sig[] = "PC Engines ALIX.";
    const char coreboot_sig[] = "PC Engines\0ALIX.";
    if (!is_geode())
    return 0;
    if (alix_present(BIOS_SIGNATURE_TINYBIOS, tinybios_sig, sizeof(tinybios_sig) - 1) ||
    alix_present(BIOS_SIGNATURE_COREBOOT, coreboot_sig, sizeof(coreboot_sig) - 1) ||
    alix_present_dmi())
    register_alix();
    return 0;
    }
    device_initcall(alix_init);
