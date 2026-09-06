//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/rk8xx-spi.c
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
// Rockchip RK806 Core (SPI) driver
//
// Copyright (c) 2021 Rockchip Electronics Co., Ltd.
// Copyright (c) 2023 Collabora Ltd.
//
// Author: Xu Shengfei <xsf@rock-chips.com>
// Author: Sebastian Reichel <sebastian.reichel@collabora.com>
//

pub const RK806_ADDR_SIZE: c_int = 2;

    (RK806_CMD_##CMD | RK806_CMD_CRC_DIS | (VALUE_BYTES - 1))
    static const struct regmap_range rk806_volatile_ranges[] = {
    regmap_reg_range(RK806_POWER_EN0, RK806_POWER_EN5),
    regmap_reg_range(RK806_DVS_START_CTRL, RK806_INT_MSK1),
    };
    static const struct regmap_access_table rk806_volatile_table = {
    .yes_ranges = rk806_volatile_ranges,
    .n_yes_ranges = ARRAY_SIZE(rk806_volatile_ranges),
    };
    static const struct regmap_config rk806_regmap_config_spi = {
    .reg_bits = 16,
    .val_bits = 8,
    .max_register = RK806_BUCK_RSERVE_REG5,
    .cache_type = REGCACHE_MAPLE,
    .volatile_table = &rk806_volatile_table,
    };
#[no_mangle]
unsafe extern "C" fn rk806_spi_bus_write(context: *mut c_void, vdata: *const c_void, count: usize) -> c_int {
    static int rk806_spi_bus_write(void *context, const void *vdata, size_t count)
    {
    struct device *dev = context;
    struct spi_device *spi = to_spi_device(dev);
    struct spi_transfer xfer[2] = { 0 };
// data and thus count includes the register address
    let mut val_size: usize = count - RK806_ADDR_SIZE;
    char cmd;
    if (val_size < 1 || val_size > (RK806_CMD_LEN_MSK + 1))
    return -EINVAL;
    cmd = RK806_CMD_WITH_SIZE(WRITE, val_size);
    xfer[0].tx_buf = &cmd;
    xfer[0].len = sizeof(cmd);
    xfer[1].tx_buf = vdata;
    xfer[1].len = count;
    return spi_sync_transfer(spi, xfer, ARRAY_SIZE(xfer));
    }
    static int rk806_spi_bus_read(void *context, const void *vreg, size_t reg_size,
    void *val, size_t val_size)
    {
    struct device *dev = context;
    struct spi_device *spi = to_spi_device(dev);
    char txbuf[3] = { 0 };
    if (reg_size != RK806_ADDR_SIZE ||
    val_size < 1 || val_size > (RK806_CMD_LEN_MSK + 1))
    return -EINVAL;
// TX buffer contains command byte followed by two address bytes
    txbuf[0] = RK806_CMD_WITH_SIZE(READ, val_size);
    memcpy(txbuf+1, vreg, reg_size);
    return spi_write_then_read(spi, txbuf, sizeof(txbuf), val, val_size);
    }
    static const struct regmap_bus rk806_regmap_bus_spi = {
    .write = rk806_spi_bus_write,
    .read = rk806_spi_bus_read,
    .reg_format_endian_default = REGMAP_ENDIAN_LITTLE,
    };
#[no_mangle]
unsafe extern "C" fn rk8xx_spi_probe(spi: *mut spi_device) -> c_int {
    static int rk8xx_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init(&spi.dev, &rk806_regmap_bus_spi,
    &spi.dev, &rk806_regmap_config_spi);
    if (IS_ERR(regmap))
    return dev_err_probe(&spi.dev, PTR_ERR(regmap),
    "Failed to init regmap\n");
    return rk8xx_probe(&spi.dev, RK806_ID, spi.irq, regmap);
    }
    static const struct of_device_id rk8xx_spi_of_match[] = {
    { .compatible = "rockchip,rk806", },
    { }
    };
    MODULE_DEVICE_TABLE(of, rk8xx_spi_of_match);
    static const struct spi_device_id rk8xx_spi_id_table[] = {
    { .name = "rk806" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, rk8xx_spi_id_table);
    static struct spi_driver rk8xx_spi_driver = {
    .driver		= {
    .name	= "rk8xx-spi",
    .of_match_table = rk8xx_spi_of_match,
    },
    .probe		= rk8xx_spi_probe,
    .id_table	= rk8xx_spi_id_table,
    };
    module_spi_driver(rk8xx_spi_driver);
    MODULE_AUTHOR("Xu Shengfei <xsf@rock-chips.com>");
    MODULE_DESCRIPTION("RK8xx SPI PMIC driver");
    MODULE_LICENSE("GPL");
