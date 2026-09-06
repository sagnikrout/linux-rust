//! Automatically rewritten from C to Rust
//! Source: drivers/pnp/system.c
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
// system.c - a driver for reserving pnp system resources
//
// Some code is based on pnpbios_core.c
// Copyright 2002 Adam Belay <ambx1@neo.rr.com>
// (c) Copyright 2007 Hewlett-Packard Development Company, L.P.
// Bjorn Helgaas <bjorn.helgaas@hp.com>
//

    static const struct pnp_device_id pnp_dev_table[] = {
// General ID for reserving resources
    { .id = "PNP0c02" },
// memory controller
    { .id = "PNP0c01" },
    { }
    };
#[no_mangle]
unsafe extern "C" fn reserve_range(dev: *mut pnp_dev, r: *mut resource, port: c_int) {
    static void reserve_range(struct pnp_dev *dev, struct resource *r, int port)
    {
    char *regionid;
    const char *pnpid = dev_name(&dev.dev);
    let mut start: resource_size_t = r.start, end = r.end;
    struct resource *res;
    regionid = kmalloc(16, GFP_KERNEL);
    if (!regionid)
    return;
    snprintf(regionid, 16, "pnp %s", pnpid);
    if (port)
    res = request_region(start, end - start + 1, regionid);
    else
    res = request_mem_region(start, end - start + 1, regionid);
    if (res)
    res.flags &= ~IORESOURCE_BUSY;
    else
    kfree(regionid);
//
// Failures at this point are usually harmless. pci quirks for
// example do reserve stuff they know about too, so we may well
// have double reservations.
//
    dev_info(&dev.dev, "%pR %s reserved\n", r,
    res ? "has been" : "could not be");
    }
#[no_mangle]
unsafe extern "C" fn reserve_resources_of_dev(dev: *mut pnp_dev) {
    static void reserve_resources_of_dev(struct pnp_dev *dev)
    {
    struct resource *res;
    int i;
    for (i = 0; (res = pnp_get_resource(dev, IORESOURCE_IO, i)); i++) {
    if (res.flags & IORESOURCE_DISABLED)
    continue;
    if (res.start == 0)
    continue;	/* disabled */
    if (res.start < 0x100)
//
// Below 0x100 is only standard PC hardware
// (pics, kbd, timer, dma, ...)
// We should not get resource conflicts there,
// and the kernel reserves these anyway
// (see arch/i386/kernel/setup.c).
// So, do nothing
//
    continue;
    if (res.end < res.start)
    continue;	/* invalid */
    reserve_range(dev, res, 1);
    }
    for (i = 0; (res = pnp_get_resource(dev, IORESOURCE_MEM, i)); i++) {
    if (res.flags & IORESOURCE_DISABLED)
    continue;
    reserve_range(dev, res, 0);
    }
    }
    static int system_pnp_probe(struct pnp_dev *dev,
    const struct pnp_device_id *dev_id)
    {
    reserve_resources_of_dev(dev);
    return 0;
    }
    static struct pnp_driver system_pnp_driver = {
    .name     = "system",
    .id_table = pnp_dev_table,
    .flags    = PNP_DRIVER_RES_DO_NOT_CHANGE,
    .probe    = system_pnp_probe,
    };
#[no_mangle]
unsafe extern "C" fn pnp_system_init() -> int __init {
    static int __init pnp_system_init(void)
    {
    return pnp_register_driver(&system_pnp_driver);
    }
//
// Reserve motherboard resources after PCI claim BARs,
// but before PCI assign resources for uninitialized PCI devices
//
    fs_initcall(pnp_system_init);
