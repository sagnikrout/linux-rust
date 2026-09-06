//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/plat-ram.c
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
// drivers/mtd/maps/plat-ram.c
//
// (c) 2004-2005 Simtec Electronics
// http://www.simtec.co.uk/products/SWLINUX
// Ben Dooks <ben@simtec.co.uk>
//
// Generic platform device based RAM map
//

// private structure for each mtd platform ram device created
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platram_info {
    pub dev: *mut device,
    pub mtd: *mut mtd_info,
    pub map: map_info,
    pub pdata: *mut platdata_mtd_ram,
}

// to_platram_info()
//
// device private data to struct platram_info conversion
//
    static inline struct platram_info *to_platram_info(struct platform_device *dev)
    {
    return platform_get_drvdata(dev);
    }
// platram_setrw
//
// call the platform device's set rw/ro control
//
// to = 0 => read-only
// = 1 => read-write
//
#[no_mangle]
pub unsafe extern "C" fn platram_setrw(info: *mut platram_info, to: c_int) {
    static inline void platram_setrw(struct platram_info *info, int to)
    {
    if (info.pdata == core::ptr::null_mut())
    return;
    if (info.pdata.set_rw != core::ptr::null_mut())
    (info.pdata.set_rw)(info.dev, to);
    }
// platram_remove
//
// called to remove the device from the driver's control
//
#[no_mangle]
unsafe extern "C" fn platram_remove(pdev: *mut platform_device) {
    static void platram_remove(struct platform_device *pdev)
    {
    struct platram_info *info = to_platram_info(pdev);
    dev_dbg(&pdev.dev, "removing device\n");
    if (info == core::ptr::null_mut())
    return;
    if (info.mtd) {
    mtd_device_unregister(info.mtd);
    map_destroy(info.mtd);
    }
// ensure ram is left read-only
    platram_setrw(info, PLATRAM_RO);
    kfree(info);
    }
// platram_probe
//
// called from device drive system when a device matching our
// driver is found.
//
#[no_mangle]
unsafe extern "C" fn platram_probe(pdev: *mut platform_device) -> c_int {
    static int platram_probe(struct platform_device *pdev)
    {
    struct platdata_mtd_ram	*pdata;
    struct platram_info *info;
    struct resource *res;
    let mut err: c_int = 0;
    dev_dbg(&pdev.dev, "probe entered\n");
    if (dev_get_platdata(&pdev.dev) == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "no platform data supplied\n");
    err = -ENOENT;
    goto exit_error;
    }
    pdata = dev_get_platdata(&pdev.dev);
    info = kzalloc_obj(*info);
    if (info == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto exit_error;
    }
    platform_set_drvdata(pdev, info);
    info.dev = &pdev.dev;
    info.pdata = pdata;
// get the resource for the memory mapping
    info.map.virt = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(info.map.virt)) {
    err = PTR_ERR(info.map.virt);
    goto exit_free;
    }
    dev_dbg(&pdev.dev, "got platform resource %p (0x%llx)\n", res,
    (unsigned long long)res.start);
// setup map parameters
    info.map.phys = res.start;
    info.map.size = resource_size(res);
    info.map.name = pdata.mapname != core::ptr::null_mut() ?
    (char *)pdata.mapname : (char *)pdev.name;
    info.map.bankwidth = pdata.bankwidth;
    dev_dbg(&pdev.dev, "virt %p, %lu bytes\n", info.map.virt, info.map.size);
    simple_map_init(&info.map);
    dev_dbg(&pdev.dev, "initialised map, probing for mtd\n");
// probe for the right mtd map driver
// supplied by the platform_data struct
    if (pdata.map_probes) {
    const char * const *map_probes = pdata.map_probes;
    for ( ; !info.mtd && *map_probes; map_probes++)
    info.mtd = do_map_probe(*map_probes , &info.map);
    }
// fallback to map_ram
    else
    info.mtd = do_map_probe("map_ram", &info.map);
    if (info.mtd == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "failed to probe for map_ram\n");
    err = -ENOMEM;
    goto exit_free;
    }
    info.mtd.dev.parent = &pdev.dev;
    platram_setrw(info, PLATRAM_RW);
// check to see if there are any available partitions, or whether
// to add this device whole
    err = mtd_device_parse_register(info.mtd, pdata.probes, core::ptr::null_mut(),
    pdata.partitions,
    pdata.nr_partitions);
    if (err) {
    dev_err(&pdev.dev, "failed to register mtd device\n");
    goto exit_free;
    }
    dev_info(&pdev.dev, "registered mtd device\n");
    if (pdata.nr_partitions) {
// add the whole device.
    err = mtd_device_register(info.mtd, core::ptr::null_mut(), 0);
    if (err) {
    dev_err(&pdev.dev,
    "failed to register the entire device\n");
    goto exit_free;
    }
    }
    return 0;
    exit_free:
    platram_remove(pdev);
    exit_error:
    return err;
    }
// device driver info
// work with hotplug and coldplug
    MODULE_ALIAS("platform:mtd-ram");
    static struct platform_driver platram_driver = {
    .probe		= platram_probe,
    .remove		= platram_remove,
    .driver		= {
    .name	= "mtd-ram",
    },
    };
    module_platform_driver(platram_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Ben Dooks <ben@simtec.co.uk>");
    MODULE_DESCRIPTION("MTD platform RAM map driver");
