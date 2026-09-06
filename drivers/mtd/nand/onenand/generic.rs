//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/onenand/generic.c
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
// Copyright (c) 2005 Samsung Electronics
// Kyungmin Park <kyungmin.park@samsung.com>
//
// Overview:
// This is a device driver for the OneNAND flash for generic boards.
//

//
// Note: Driver name and platform data format have been updated!
//
// This version of the driver is named "onenand-flash" and takes struct
// onenand_platform_data as platform data. The old ARM-specific version
// with the name "onenand" used to take struct flash_platform_data.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct onenand_info {
    pub mtd: mtd_info,
    pub onenand: onenand_chip,
}

#[no_mangle]
unsafe extern "C" fn generic_onenand_probe(pdev: *mut platform_device) -> c_int {
    static int generic_onenand_probe(struct platform_device *pdev)
    {
    struct onenand_info *info;
    struct onenand_platform_data *pdata = dev_get_platdata(&pdev.dev);
    struct resource *res = pdev.resource;
    let mut size: c_ulong = resource_size(res);
    int err;
    info = kzalloc_obj(struct onenand_info);
    if (!info)
    return -ENOMEM;
    if (!request_mem_region(res.start, size, dev_name(&pdev.dev))) {
    err = -EBUSY;
    goto out_free_info;
    }
    info.onenand.base = ioremap(res.start, size);
    if (!info.onenand.base) {
    err = -ENOMEM;
    goto out_release_mem_region;
    }
    info.onenand.mmcontrol = pdata ? pdata.mmcontrol : core::ptr::null_mut();
    err = platform_get_irq(pdev, 0);
    if (err < 0)
    goto out_iounmap;
    info.onenand.irq = err;
    info.mtd.dev.parent = &pdev.dev;
    info.mtd.priv = &info.onenand;
    if (onenand_scan(&info.mtd, 1)) {
    err = -ENXIO;
    goto out_iounmap;
    }
    err = mtd_device_register(&info.mtd, pdata ? pdata.parts : core::ptr::null_mut(),
    pdata ? pdata.nr_parts : 0);
    platform_set_drvdata(pdev, info);
    return 0;
    out_iounmap:
    iounmap(info.onenand.base);
    out_release_mem_region:
    release_mem_region(res.start, size);
    out_free_info:
    kfree(info);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn generic_onenand_remove(pdev: *mut platform_device) {
    static void generic_onenand_remove(struct platform_device *pdev)
    {
    struct onenand_info *info = platform_get_drvdata(pdev);
    struct resource *res = pdev.resource;
    let mut size: c_ulong = resource_size(res);
    if (info) {
    onenand_release(&info.mtd);
    release_mem_region(res.start, size);
    iounmap(info.onenand.base);
    kfree(info);
    }
    }
    static struct platform_driver generic_onenand_driver = {
    .driver = {
    .name		= DRIVER_NAME,
    },
    .probe		= generic_onenand_probe,
    .remove		= generic_onenand_remove,
    };
    module_platform_driver(generic_onenand_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Kyungmin Park <kyungmin.park@samsung.com>");
    MODULE_DESCRIPTION("Glue layer for OneNAND flash on generic boards");
    MODULE_ALIAS("platform:" DRIVER_NAME);
