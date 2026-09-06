//! Automatically rewritten from C to Rust
//! Source: drivers/misc/lis3lv02d/lis3lv02d_spi.c
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
// lis3lv02d_spi - SPI glue layer for lis3lv02d
//
// Copyright (c) 2009 Daniel Mack <daniel@caiaq.de>
//

pub const LIS3_SPI_READ: c_uint = 0x80;
#[no_mangle]
unsafe extern "C" fn lis3_spi_read(lis3: *mut lis3lv02d, reg: c_int, v: *mut u8) -> c_int {
    static int lis3_spi_read(struct lis3lv02d *lis3, int reg, u8 *v)
    {
    struct spi_device *spi = lis3.bus_priv;
    let mut ret: c_int = spi_w8r8(spi, reg | LIS3_SPI_READ);
    if (ret < 0)
    return -EINVAL;
// v = (u8) ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lis3_spi_write(lis3: *mut lis3lv02d, reg: c_int, val: u8) -> c_int {
    static int lis3_spi_write(struct lis3lv02d *lis3, int reg, u8 val)
    {
    u8 tmp[2] = { reg, val };
    struct spi_device *spi = lis3.bus_priv;
    return spi_write(spi, tmp, sizeof(tmp));
    }
#[no_mangle]
unsafe extern "C" fn lis3_spi_init(lis3: *mut lis3lv02d) -> c_int {
    static int lis3_spi_init(struct lis3lv02d *lis3)
    {
    u8 reg;
    int ret;
// power up the device
    ret = lis3.read(lis3, CTRL_REG1, &reg);
    if (ret < 0)
    return ret;
    reg |= CTRL1_PD0 | CTRL1_Xen | CTRL1_Yen | CTRL1_Zen;
    return lis3.write(lis3, CTRL_REG1, reg);
    }
    static union axis_conversion lis3lv02d_axis_normal =
    { .as_array = { 1, 2, 3 } };

    static const struct of_device_id lis302dl_spi_dt_ids[] = {
    { .compatible = "st,lis302dl-spi" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lis302dl_spi_dt_ids);

#[no_mangle]
unsafe extern "C" fn lis302dl_spi_probe(spi: *mut spi_device) -> c_int {
    static int lis302dl_spi_probe(struct spi_device *spi)
    {
    int ret;
    spi.bits_per_word = 8;
    spi.mode = SPI_MODE_0;
    ret = spi_setup(spi);
    if (ret < 0)
    return ret;
    lis3_dev.bus_priv	= spi;
    lis3_dev.init		= lis3_spi_init;
    lis3_dev.read		= lis3_spi_read;
    lis3_dev.write		= lis3_spi_write;
    lis3_dev.irq		= spi.irq;
    lis3_dev.ac		= lis3lv02d_axis_normal;
    lis3_dev.pdata		= spi.dev.platform_data;

    if (of_match_device(lis302dl_spi_dt_ids, &spi.dev)) {
    lis3_dev.of_node = spi.dev.of_node;
    ret = lis3lv02d_init_dt(&lis3_dev);
    if (ret)
    return ret;
    }

    spi_set_drvdata(spi, &lis3_dev);
    return lis3lv02d_init_device(&lis3_dev);
    }
#[no_mangle]
unsafe extern "C" fn lis302dl_spi_remove(spi: *mut spi_device) {
    static void lis302dl_spi_remove(struct spi_device *spi)
    {
    struct lis3lv02d *lis3 = spi_get_drvdata(spi);
    lis3lv02d_joystick_disable(lis3);
    lis3lv02d_poweroff(lis3);
    lis3lv02d_remove_fs(&lis3_dev);
    }

#[no_mangle]
unsafe extern "C" fn lis3lv02d_spi_suspend(dev: *mut device) -> c_int {
    static int lis3lv02d_spi_suspend(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct lis3lv02d *lis3 = spi_get_drvdata(spi);
    if (!lis3.pdata || !lis3.pdata.wakeup_flags)
    lis3lv02d_poweroff(&lis3_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lis3lv02d_spi_resume(dev: *mut device) -> c_int {
    static int lis3lv02d_spi_resume(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct lis3lv02d *lis3 = spi_get_drvdata(spi);
    if (!lis3.pdata || !lis3.pdata.wakeup_flags)
    lis3lv02d_poweron(lis3);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(lis3lv02d_spi_pm, lis3lv02d_spi_suspend,
    lis3lv02d_spi_resume);
    static struct spi_driver lis302dl_spi_driver = {
    .driver	 = {
    .name   = DRV_NAME,
    .pm	= &lis3lv02d_spi_pm,
    .of_match_table = of_match_ptr(lis302dl_spi_dt_ids),
    },
    .probe	= lis302dl_spi_probe,
    .remove	= lis302dl_spi_remove,
    };
    module_spi_driver(lis302dl_spi_driver);
    MODULE_AUTHOR("Daniel Mack <daniel@caiaq.de>");
    MODULE_DESCRIPTION("lis3lv02d SPI glue layer");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("spi:" DRV_NAME);
