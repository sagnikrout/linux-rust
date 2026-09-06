//! Automatically rewritten from C to Rust
//! Source: drivers/soc/nuvoton/wpcm450-soc.c
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
// Nuvoton WPCM450 SoC Identification
//
// Copyright (C) 2022 Jonathan Neuschäfer
//

pub const GCR_PDID: c_int = 0;

pub const CHIP_WPCM450: c_uint = 0x926450;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct revision {
    pub number: u8,
    pub name: *const c_char,
}

    static const struct revision revisions[] __initconst = {
    { 0x00, "Z1" },
    { 0x03, "Z2" },
    { 0x04, "Z21" },
    { 0x08, "A1" },
    { 0x09, "A2" },
    { 0x0a, "A3" },
    {}
    };
#[no_mangle]
unsafe extern "C" fn get_revision(rev: c_uint) -> *const char  __init {
    static const char * __init get_revision(unsigned int rev)
    {
    int i;
    for (i = 0; revisions[i].name; i++)
    if (revisions[i].number == rev)
    return revisions[i].name;
    return core::ptr::null_mut();
    }
    static struct soc_device_attribute *wpcm450_attr;
    static struct soc_device *wpcm450_soc;
#[no_mangle]
unsafe extern "C" fn wpcm450_soc_init() -> int __init {
    static int __init wpcm450_soc_init(void)
    {
    struct soc_device_attribute *attr;
    struct soc_device *soc;
    const char *revision;
    struct regmap *gcr;
    u32 pdid;
    int ret;
    if (!of_machine_is_compatible("nuvoton,wpcm450"))
    return 0;
    gcr = syscon_regmap_lookup_by_compatible("nuvoton,wpcm450-gcr");
    if (IS_ERR(gcr))
    return PTR_ERR(gcr);
    ret = regmap_read(gcr, GCR_PDID, &pdid);
    if (ret)
    return ret;
    if (PDID_CHIP(pdid) != CHIP_WPCM450) {
    pr_warn("Unknown chip ID in GCR.PDID: 0x%06x\n", PDID_CHIP(pdid));
    return -ENODEV;
    }
    revision = get_revision(PDID_REV(pdid));
    if (!revision) {
    pr_warn("Unknown chip revision in GCR.PDID: 0x%02x\n", PDID_REV(pdid));
    return -ENODEV;
    }
    attr = kzalloc_obj(*attr);
    if (!attr)
    return -ENOMEM;
    attr.family = "Nuvoton NPCM";
    attr.soc_id = "WPCM450";
    attr.revision = revision;
    soc = soc_device_register(attr);
    if (IS_ERR(soc)) {
    kfree(attr);
    pr_warn("Could not register SoC device\n");
    return PTR_ERR(soc);
    }
    wpcm450_soc = soc;
    wpcm450_attr = attr;
    return 0;
    }
    module_init(wpcm450_soc_init);
#[no_mangle]
unsafe extern "C" fn wpcm450_soc_exit() -> void __exit {
    static void __exit wpcm450_soc_exit(void)
    {
    if (wpcm450_soc) {
    soc_device_unregister(wpcm450_soc);
    wpcm450_soc = core::ptr::null_mut();
    kfree(wpcm450_attr);
    }
    }
    module_exit(wpcm450_soc_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jonathan Neuschäfer");
    MODULE_DESCRIPTION("Nuvoton WPCM450 SoC Identification driver");
