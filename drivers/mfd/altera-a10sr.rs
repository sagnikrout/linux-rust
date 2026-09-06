//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/altera-a10sr.c
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
// Altera Arria10 DevKit System Resource MFD Driver
//
// Author: Thor Thayer <tthayer@opensource.altera.com>
//
// Copyright Intel Corporation (C) 2014-2016. All Rights Reserved
//
// SPI access for Altera Arria10 MAX5 System Resource Chip
//
// Adapted from DA9052
//

    static const struct mfd_cell altr_a10sr_subdev_info[] = {
    {
    .name = "altr_a10sr_gpio",
    .of_compatible = "altr,a10sr-gpio",
    },
    {
    .name = "altr_a10sr_reset",
    .of_compatible = "altr,a10sr-reset",
    },
    };
#[no_mangle]
unsafe extern "C" fn altr_a10sr_reg_readable(dev: *mut device, reg: c_uint) -> bool {
    static bool altr_a10sr_reg_readable(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case ALTR_A10SR_VERSION_READ:
    case ALTR_A10SR_LED_REG:
    case ALTR_A10SR_PBDSW_REG:
    case ALTR_A10SR_PBDSW_IRQ_REG:
    case ALTR_A10SR_PWR_GOOD1_REG:
    case ALTR_A10SR_PWR_GOOD2_REG:
    case ALTR_A10SR_PWR_GOOD3_REG:
    case ALTR_A10SR_FMCAB_REG:
    case ALTR_A10SR_HPS_RST_REG:
    case ALTR_A10SR_USB_QSPI_REG:
    case ALTR_A10SR_SFPA_REG:
    case ALTR_A10SR_SFPB_REG:
    case ALTR_A10SR_I2C_M_REG:
    case ALTR_A10SR_WARM_RST_REG:
    case ALTR_A10SR_WR_KEY_REG:
    case ALTR_A10SR_PMBUS_REG:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn altr_a10sr_reg_writeable(dev: *mut device, reg: c_uint) -> bool {
    static bool altr_a10sr_reg_writeable(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case ALTR_A10SR_LED_REG:
    case ALTR_A10SR_PBDSW_IRQ_REG:
    case ALTR_A10SR_FMCAB_REG:
    case ALTR_A10SR_HPS_RST_REG:
    case ALTR_A10SR_USB_QSPI_REG:
    case ALTR_A10SR_SFPA_REG:
    case ALTR_A10SR_SFPB_REG:
    case ALTR_A10SR_WARM_RST_REG:
    case ALTR_A10SR_WR_KEY_REG:
    case ALTR_A10SR_PMBUS_REG:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn altr_a10sr_reg_volatile(dev: *mut device, reg: c_uint) -> bool {
    static bool altr_a10sr_reg_volatile(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case ALTR_A10SR_PBDSW_REG:
    case ALTR_A10SR_PBDSW_IRQ_REG:
    case ALTR_A10SR_PWR_GOOD1_REG:
    case ALTR_A10SR_PWR_GOOD2_REG:
    case ALTR_A10SR_PWR_GOOD3_REG:
    case ALTR_A10SR_HPS_RST_REG:
    case ALTR_A10SR_I2C_M_REG:
    case ALTR_A10SR_WARM_RST_REG:
    case ALTR_A10SR_WR_KEY_REG:
    case ALTR_A10SR_PMBUS_REG:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config altr_a10sr_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .cache_type = REGCACHE_NONE,
    .use_single_read = true,
    .use_single_write = true,
    .read_flag_mask = 1,
    .write_flag_mask = 0,
    .max_register = ALTR_A10SR_WR_KEY_REG,
    .readable_reg = altr_a10sr_reg_readable,
    .writeable_reg = altr_a10sr_reg_writeable,
    .volatile_reg = altr_a10sr_reg_volatile,
    };
#[no_mangle]
unsafe extern "C" fn altr_a10sr_spi_probe(spi: *mut spi_device) -> c_int {
    static int altr_a10sr_spi_probe(struct spi_device *spi)
    {
    int ret;
    struct altr_a10sr *a10sr;
    a10sr = devm_kzalloc(&spi.dev, sizeof(*a10sr),
    GFP_KERNEL);
    if (!a10sr)
    return -ENOMEM;
    spi.mode = SPI_MODE_3;
    spi.bits_per_word = 8;
    spi_setup(spi);
    a10sr.dev = &spi.dev;
    spi_set_drvdata(spi, a10sr);
    a10sr.regmap = devm_regmap_init_spi(spi, &altr_a10sr_regmap_config);
    if (IS_ERR(a10sr.regmap)) {
    ret = PTR_ERR(a10sr.regmap);
    dev_err(&spi.dev, "Failed to allocate register map: %d\n",
    ret);
    return ret;
    }
    ret = devm_mfd_add_devices(a10sr.dev, PLATFORM_DEVID_AUTO,
    altr_a10sr_subdev_info,
    ARRAY_SIZE(altr_a10sr_subdev_info),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    dev_err(a10sr.dev, "Failed to register sub-devices: %d\n",
    ret);
    return ret;
    }
    static const struct of_device_id altr_a10sr_spi_of_match[] = {
    { .compatible = "altr,a10sr" },
    { },
    };
    MODULE_DEVICE_TABLE(of, altr_a10sr_spi_of_match);
    static const struct spi_device_id altr_a10sr_spi_ids[] = {
    { .name = "a10sr" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, altr_a10sr_spi_ids);
    static struct spi_driver altr_a10sr_spi_driver = {
    .probe = altr_a10sr_spi_probe,
    .driver = {
    .name = "altr_a10sr",
    .of_match_table = altr_a10sr_spi_of_match,
    },
    .id_table = altr_a10sr_spi_ids,
    };
    builtin_driver(altr_a10sr_spi_driver, spi_register_driver)
