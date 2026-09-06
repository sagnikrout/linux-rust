//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-loongson-plat.c
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
// Platform driver for Loongson SPI Support
// Copyright (C) 2023 Loongson Technology Corporation Limited

#[no_mangle]
unsafe extern "C" fn loongson_spi_platform_probe(pdev: *mut platform_device) -> c_int {
    static int loongson_spi_platform_probe(struct platform_device *pdev)
    {
    int ret;
    void __iomem *reg_base;
    struct device *dev = &pdev.dev;
    reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg_base))
    return PTR_ERR(reg_base);
    ret = loongson_spi_init_controller(dev, reg_base);
    if (ret)
    return dev_err_probe(dev, ret, "failed to initialize controller\n");
    return 0;
    }
    static const struct of_device_id loongson_spi_id_table[] = {
    { .compatible = "loongson,ls2k1000-spi" },
    { }
    };
    MODULE_DEVICE_TABLE(of, loongson_spi_id_table);
    static struct platform_driver loongson_spi_plat_driver = {
    .probe = loongson_spi_platform_probe,
    .driver	= {
    .name	= "loongson-spi",
    .bus = &platform_bus_type,
    .pm = pm_sleep_ptr(&loongson_spi_dev_pm_ops),
    .of_match_table = loongson_spi_id_table,
    },
    };
    module_platform_driver(loongson_spi_plat_driver);
    MODULE_DESCRIPTION("Loongson spi platform driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("SPI_LOONGSON_CORE");
