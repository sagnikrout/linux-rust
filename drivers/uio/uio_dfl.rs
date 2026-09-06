//! Automatically rewritten from C to Rust
//! Source: drivers/uio/uio_dfl.c
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
// Generic DFL driver for Userspace I/O devicess
//
// Copyright (C) 2021 Intel Corporation, Inc.
//

#[no_mangle]
unsafe extern "C" fn uio_dfl_probe(ddev: *mut dfl_device) -> c_int {
    static int uio_dfl_probe(struct dfl_device *ddev)
    {
    struct resource *r = &ddev.mmio_res;
    struct device *dev = &ddev.dev;
    struct uio_info *uioinfo;
    struct uio_mem *uiomem;
    int ret;
    uioinfo = devm_kzalloc(dev, sizeof(struct uio_info), GFP_KERNEL);
    if (!uioinfo)
    return -ENOMEM;
    uioinfo.name = DRIVER_NAME;
    uioinfo.version = "0";
    uiomem = &uioinfo.mem[0];
    uiomem.memtype = UIO_MEM_PHYS;
    uiomem.addr = r.start & PAGE_MASK;
    uiomem.offs = r.start & ~PAGE_MASK;
    uiomem.size = (uiomem.offs + resource_size(r)
    + PAGE_SIZE - 1) & PAGE_MASK;
    uiomem.name = r.name;
// Irq is yet to be supported
    uioinfo.irq = UIO_IRQ_NONE;
    ret = devm_uio_register_device(dev, uioinfo);
    if (ret)
    dev_err(dev, "unable to register uio device\n");
    return ret;
    }
pub const FME_FEATURE_ID_ETH_GROUP: c_uint = 0x10;
pub const FME_FEATURE_ID_HSSI_SUBSYS: c_uint = 0x15;
pub const FME_FEATURE_ID_VENDOR_SPECIFIC: c_uint = 0x23;
pub const PORT_FEATURE_ID_IOPLL_USRCLK: c_uint = 0x14;
    static const struct dfl_device_id uio_dfl_ids[] = {
    { FME_ID, FME_FEATURE_ID_ETH_GROUP },
    { FME_ID, FME_FEATURE_ID_HSSI_SUBSYS },
    { FME_ID, FME_FEATURE_ID_VENDOR_SPECIFIC },
    { PORT_ID, PORT_FEATURE_ID_IOPLL_USRCLK },
    { }
    };
    MODULE_DEVICE_TABLE(dfl, uio_dfl_ids);
    static struct dfl_driver uio_dfl_driver = {
    .drv = {
    .name = DRIVER_NAME,
    },
    .id_table	= uio_dfl_ids,
    .probe		= uio_dfl_probe,
    };
    module_dfl_driver(uio_dfl_driver);
    MODULE_DESCRIPTION("Generic DFL driver for Userspace I/O devices");
    MODULE_AUTHOR("Intel Corporation");
    MODULE_LICENSE("GPL v2");
