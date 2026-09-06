//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/pxa2xx-flash.c
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
// Map driver for Intel XScale PXA2xx platforms.
//
// Author:	Nicolas Pitre
// Copyright:	(C) 2001 MontaVista Software Inc.
//

pub const CACHELINESIZE: c_int = 32;
    static void pxa2xx_map_inval_cache(struct map_info *map, unsigned long from,
    ssize_t len)
    {
    let mut start: c_ulong = (unsigned long)map.cached + from;
    let mut end: c_ulong = start + len;
    start &= ~(CACHELINESIZE - 1);
    while (start < end) {
// invalidate D cache line
    asm volatile ("mcr p15, 0, %0, c7, c6, 1" : : "r" (start));
    start += CACHELINESIZE;
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa2xx_flash_info {
    pub mtd: *mut mtd_info,
    pub map: map_info,
}

    static const char * const probes[] = { "RedBoot", "cmdlinepart", core::ptr::null_mut() };
#[no_mangle]
unsafe extern "C" fn pxa2xx_flash_probe(pdev: *mut platform_device) -> c_int {
    static int pxa2xx_flash_probe(struct platform_device *pdev)
    {
    struct flash_platform_data *flash = dev_get_platdata(&pdev.dev);
    struct pxa2xx_flash_info *info;
    struct resource *res;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENODEV;
    info = kzalloc_obj(struct pxa2xx_flash_info);
    if (!info)
    return -ENOMEM;
    info.map.name = flash.name;
    info.map.bankwidth = flash.width;
    info.map.phys = res.start;
    info.map.size = resource_size(res);
    info.map.virt = ioremap(info.map.phys, info.map.size);
    if (!info.map.virt) {
    printk(KERN_WARNING "Failed to ioremap %s\n",
    info.map.name);
    kfree(info);
    return -ENOMEM;
    }
    info.map.cached = ioremap_cache(info.map.phys, info.map.size);
    if (!info.map.cached)
    printk(KERN_WARNING "Failed to ioremap cached %s\n",
    info.map.name);
    info.map.inval_cache = pxa2xx_map_inval_cache;
    simple_map_init(&info.map);
    printk(KERN_NOTICE
    "Probing %s at physical address 0x%08lx"
    " (%d-bit bankwidth)\n",
    info.map.name, (unsigned long)info.map.phys,
    info.map.bankwidth * 8);
    info.mtd = do_map_probe(flash.map_name, &info.map);
    if (!info.mtd) {
    iounmap((void *)info.map.virt);
    if (info.map.cached)
    iounmap(info.map.cached);
    kfree(info);
    return -EIO;
    }
    info.mtd.dev.parent = &pdev.dev;
    mtd_device_parse_register(info.mtd, probes, core::ptr::null_mut(), flash.parts,
    flash.nr_parts);
    platform_set_drvdata(pdev, info);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa2xx_flash_remove(dev: *mut platform_device) {
    static void pxa2xx_flash_remove(struct platform_device *dev)
    {
    struct pxa2xx_flash_info *info = platform_get_drvdata(dev);
    mtd_device_unregister(info.mtd);
    map_destroy(info.mtd);
    iounmap(info.map.virt);
    if (info.map.cached)
    iounmap(info.map.cached);
    kfree(info);
    }

#[no_mangle]
unsafe extern "C" fn pxa2xx_flash_shutdown(dev: *mut platform_device) {
    static void pxa2xx_flash_shutdown(struct platform_device *dev)
    {
    struct pxa2xx_flash_info *info = platform_get_drvdata(dev);
    if (info && mtd_suspend(info.mtd) == 0)
    mtd_resume(info.mtd);
    }

    static struct platform_driver pxa2xx_flash_driver = {
    .driver = {
    .name		= "pxa2xx-flash",
    },
    .probe		= pxa2xx_flash_probe,
    .remove		= pxa2xx_flash_remove,
    .shutdown	= pxa2xx_flash_shutdown,
    };
    module_platform_driver(pxa2xx_flash_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Nicolas Pitre <nico@fluxnic.net>");
    MODULE_DESCRIPTION("MTD map driver for Intel XScale PXA2xx");
