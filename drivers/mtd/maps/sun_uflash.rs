//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/sun_uflash.c
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
// sun_uflash.c - Driver for user-programmable flash on
// Sun Microsystems SME boardsets.
//
// This driver does NOT provide access to the OBP-flash for
// safety reasons-- use <linux>/drivers/sbus/char/flash.c instead.
//
// Copyright (c) 2001 Eric Brower (ebrower@usa.net)
//

pub const UFLASH_WINDOW_SIZE: c_uint = 0x200000;

    MODULE_AUTHOR("Eric Brower <ebrower@usa.net>");
    MODULE_DESCRIPTION("User-programmable flash device on Sun Microsystems boardsets");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("2.1");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uflash_dev {
    pub /: *const *const *const char name; / device name,
    pub /: *mut *mut map_info map; / mtd map info,
    pub /: *mut *mut *mut mtd_info mtd; / mtd info,
}

    struct map_info uflash_map_templ = {
    .name =		"SUNW,???-????",
    .size =		UFLASH_WINDOW_SIZE,
    .bankwidth =	UFLASH_BUSWIDTH,
    };
#[no_mangle]
unsafe extern "C" fn uflash_devinit(op: *mut platform_device, dp: *mut device_node) -> c_int {
    static int uflash_devinit(struct platform_device *op, struct device_node *dp)
    {
    struct uflash_dev *up;
    if (op.resource[1].flags) {
// Non-CFI userflash device-- once I find one we
// can work on supporting it.
//
    printk(KERN_ERR PFX "Unsupported device at %pOF, 0x%llx\n",
    dp, (unsigned long long)op.resource[0].start);
    return -ENODEV;
    }
    up = kzalloc_obj(struct uflash_dev);
    if (!up)
    return -ENOMEM;
// copy defaults and tweak parameters
    memcpy(&up.map, &uflash_map_templ, sizeof(uflash_map_templ));
    up.map.size = resource_size(&op.resource[0]);
    up.name = of_get_property(dp, "model", core::ptr::null_mut());
    if (up.name && 0 < strlen(up.name))
    up.map.name = up.name;
    up.map.phys = op.resource[0].start;
    up.map.virt = of_ioremap(&op.resource[0], 0, up.map.size,
    DRIVER_NAME);
    if (!up.map.virt) {
    printk(KERN_ERR PFX "Failed to map device.\n");
    kfree(up);
    return -EINVAL;
    }
    simple_map_init(&up.map);
// MTD registration
    up.mtd = do_map_probe("cfi_probe", &up.map);
    if (!up.mtd) {
    of_iounmap(&op.resource[0], up.map.virt, up.map.size);
    kfree(up);
    return -ENXIO;
    }
    up.mtd.owner = THIS_MODULE;
    mtd_device_register(up.mtd, core::ptr::null_mut(), 0);
    dev_set_drvdata(&op.dev, up);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uflash_probe(op: *mut platform_device) -> c_int {
    static int uflash_probe(struct platform_device *op)
    {
    struct device_node *dp = op.dev.of_node;
// Flashprom must have the "user" property in order to
// be used by this driver.
//
    if (!of_property_read_bool(dp, "user"))
    return -ENODEV;
    return uflash_devinit(op, dp);
    }
#[no_mangle]
unsafe extern "C" fn uflash_remove(op: *mut platform_device) {
    static void uflash_remove(struct platform_device *op)
    {
    struct uflash_dev *up = dev_get_drvdata(&op.dev);
    if (up.mtd) {
    mtd_device_unregister(up.mtd);
    map_destroy(up.mtd);
    }
    if (up.map.virt) {
    of_iounmap(&op.resource[0], up.map.virt, up.map.size);
    up.map.virt = core::ptr::null_mut();
    }
    kfree(up);
    }
    static const struct of_device_id uflash_match[] = {
    {
    .name = UFLASH_OBPNAME,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, uflash_match);
    static struct platform_driver uflash_driver = {
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = uflash_match,
    },
    .probe		= uflash_probe,
    .remove		= uflash_remove,
    };
    module_platform_driver(uflash_driver);
