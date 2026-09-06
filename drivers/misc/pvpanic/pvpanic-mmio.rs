//! Automatically rewritten from C to Rust
//! Source: drivers/misc/pvpanic/pvpanic-mmio.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Pvpanic MMIO Device Support
//
// Copyright (C) 2013 Fujitsu.
// Copyright (C) 2018 ZTE.
// Copyright (C) 2021 Oracle.
//

    MODULE_AUTHOR("Hu Tao <hutao@cn.fujitsu.com>");
    MODULE_DESCRIPTION("pvpanic-mmio device driver");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn pvpanic_mmio_probe(pdev: *mut platform_device) -> c_int {
    static int pvpanic_mmio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct resource *res;
    void __iomem *base;
    res = platform_get_mem_or_io(pdev, 0);
    if (!res)
    return -EINVAL;
    switch (resource_type(res)) {
    case IORESOURCE_IO:
    base = devm_ioport_map(dev, res.start, resource_size(res));
    if (!base)
    return -ENOMEM;
    break;
    case IORESOURCE_MEM:
    base = devm_ioremap_resource(dev, res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    break;
    default:
    return -EINVAL;
    }
    return devm_pvpanic_probe(dev, base);
    }
    static const struct of_device_id pvpanic_mmio_match[] = {
    { .compatible = "qemu,pvpanic-mmio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, pvpanic_mmio_match);
    static const struct acpi_device_id pvpanic_device_ids[] = {
    { .id = "QEMU0001" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, pvpanic_device_ids);
    static struct platform_driver pvpanic_mmio_driver = {
    .driver = {
    .name = "pvpanic-mmio",
    .of_match_table = pvpanic_mmio_match,
    .acpi_match_table = pvpanic_device_ids,
    .dev_groups = pvpanic_dev_groups,
    },
    .probe = pvpanic_mmio_probe,
    };
    module_platform_driver(pvpanic_mmio_driver);
