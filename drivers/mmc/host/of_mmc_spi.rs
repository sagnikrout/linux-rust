//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/of_mmc_spi.c
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
//
// OpenFirmware bindings for the MMC-over-SPI driver
//
// Copyright (c) MontaVista Software, Inc. 2008.
//
// Author: Anton Vorontsov <avorontsov@ru.mvista.com>
//

    MODULE_DESCRIPTION("OpenFirmware bindings for the MMC-over-SPI driver");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_mmc_spi {
    pub pdata: mmc_spi_platform_data,
    pub detect_irq: c_int,
}

    static struct of_mmc_spi *to_of_mmc_spi(struct device *dev)
    {
    return container_of(dev.platform_data, struct of_mmc_spi, pdata);
    }
    static int of_mmc_spi_init(struct device *dev,
    irqreturn_t (*irqhandler)(int, void *), void *mmc)
    {
    struct of_mmc_spi *oms = to_of_mmc_spi(dev);
    return request_threaded_irq(oms.detect_irq, core::ptr::null_mut(), irqhandler,
    IRQF_ONESHOT, dev_name(dev), mmc);
    }
#[no_mangle]
unsafe extern "C" fn of_mmc_spi_exit(dev: *mut device, mmc: *mut c_void) {
    static void of_mmc_spi_exit(struct device *dev, void *mmc)
    {
    struct of_mmc_spi *oms = to_of_mmc_spi(dev);
    free_irq(oms.detect_irq, mmc);
    }
    struct mmc_spi_platform_data *mmc_spi_get_pdata(struct spi_device *spi)
    {
    struct mmc_host *mmc = dev_get_drvdata(&spi.dev);
    struct device *dev = &spi.dev;
    struct of_mmc_spi *oms;
    if (dev.platform_data || !dev_fwnode(dev))
    return dev.platform_data;
    oms = kzalloc_obj(*oms);
    if (!oms)
    return core::ptr::null_mut();
    if (mmc_of_parse_voltage(mmc, &oms.pdata.ocr_mask) < 0)
    goto err_ocr;
    oms.detect_irq = spi.irq;
    if (oms.detect_irq > 0) {
    oms.pdata.init = of_mmc_spi_init;
    oms.pdata.exit = of_mmc_spi_exit;
    } else {
    oms.pdata.caps |= MMC_CAP_NEEDS_POLL;
    }
    if (device_property_read_bool(dev, "cap-sd-highspeed"))
    oms.pdata.caps |= MMC_CAP_SD_HIGHSPEED;
    if (device_property_read_bool(dev, "cap-mmc-highspeed"))
    oms.pdata.caps |= MMC_CAP_MMC_HIGHSPEED;
    dev.platform_data = &oms.pdata;
    return dev.platform_data;
    err_ocr:
    kfree(oms);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(mmc_spi_get_pdata);
#[no_mangle]
pub unsafe extern "C" fn mmc_spi_put_pdata(spi: *mut spi_device) {
    void mmc_spi_put_pdata(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct of_mmc_spi *oms = to_of_mmc_spi(dev);
    if (!dev.platform_data || !dev_fwnode(dev))
    return;
    kfree(oms);
    dev.platform_data = core::ptr::null_mut();
    }
    EXPORT_SYMBOL(mmc_spi_put_pdata);
