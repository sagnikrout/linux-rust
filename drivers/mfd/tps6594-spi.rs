//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/tps6594-spi.c
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
// SPI access driver for the following TI PMICs:
// - LP8764
// - TPS65224
// - TPS652G1
// - TPS6593
// - TPS6594
//
// Copyright (C) 2023 BayLibre Incorporated - https://www.baylibre.com
//

pub const TPS6594_SPI_PAGE_SHIFT: c_int = 5;

    static bool enable_crc;
    module_param(enable_crc, bool, 0444);
    MODULE_PARM_DESC(enable_crc, "Enable CRC feature for SPI interface");
    DECLARE_CRC8_TABLE(tps6594_spi_crc_table);
#[no_mangle]
unsafe extern "C" fn tps6594_spi_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int tps6594_spi_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct spi_device *spi = context;
    struct tps6594 *tps = spi_get_drvdata(spi);
    u8 buf[4] = { 0 };
    let mut count_rx: usize = 1;
    int ret;
    buf[0] = reg;
    buf[1] = TPS6594_REG_TO_PAGE(reg) << TPS6594_SPI_PAGE_SHIFT | TPS6594_SPI_READ_BIT;
    if (tps.use_crc)
    count_rx++;
    ret = spi_write_then_read(spi, buf, 2, buf + 2, count_rx);
    if (ret < 0)
    return ret;
    if (tps.use_crc && buf[3] != crc8(tps6594_spi_crc_table, buf, 3, CRC8_INIT_VALUE))
    return -EIO;
// val = buf[2];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps6594_spi_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int tps6594_spi_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    struct spi_device *spi = context;
    struct tps6594 *tps = spi_get_drvdata(spi);
    u8 buf[4] = { 0 };
    let mut count: usize = 3;
    buf[0] = reg;
    buf[1] = TPS6594_REG_TO_PAGE(reg) << TPS6594_SPI_PAGE_SHIFT;
    buf[2] = val;
    if (tps.use_crc)
    buf[3] = crc8(tps6594_spi_crc_table, buf, count++, CRC8_INIT_VALUE);
    return spi_write(spi, buf, count);
    }
    static struct regmap_config tps6594_spi_regmap_config = {
    .reg_bits = 16,
    .val_bits = 8,
    .max_register = TPS6594_REG_DWD_FAIL_CNT_REG,
    .volatile_table = &tps6594_volatile_table,
    .reg_read = tps6594_spi_reg_read,
    .reg_write = tps6594_spi_reg_write,
    .use_single_read = true,
    .use_single_write = true,
    };
    static const struct of_device_id tps6594_spi_of_match_table[] = {
    { .compatible = "ti,tps6594-q1", .data = (void *)TPS6594, },
    { .compatible = "ti,tps6593-q1", .data = (void *)TPS6593, },
    { .compatible = "ti,lp8764-q1",  .data = (void *)LP8764,  },
    { .compatible = "ti,tps65224-q1", .data = (void *)TPS65224, },
    { .compatible = "ti,tps652g1", .data = (void *)TPS652G1, },
    {}
    };
    MODULE_DEVICE_TABLE(of, tps6594_spi_of_match_table);
#[no_mangle]
unsafe extern "C" fn tps6594_spi_probe(spi: *mut spi_device) -> c_int {
    static int tps6594_spi_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct tps6594 *tps;
    const struct of_device_id *match;
    tps = devm_kzalloc(dev, sizeof(*tps), GFP_KERNEL);
    if (!tps)
    return -ENOMEM;
    spi_set_drvdata(spi, tps);
    tps.dev = dev;
    tps.reg = spi_get_chipselect(spi, 0);
    tps.irq = spi.irq;
    match = of_match_device(tps6594_spi_of_match_table, dev);
    if (!match)
    return dev_err_probe(dev, -EINVAL, "Failed to find matching chip ID\n");
    tps.chip_id = (unsigned long)match.data;
    if (tps.chip_id == TPS65224 || tps.chip_id == TPS652G1)
    tps6594_spi_regmap_config.volatile_table = &tps65224_volatile_table;
    tps.regmap = devm_regmap_init(dev, core::ptr::null_mut(), spi, &tps6594_spi_regmap_config);
    if (IS_ERR(tps.regmap))
    return dev_err_probe(dev, PTR_ERR(tps.regmap), "Failed to init regmap\n");
    crc8_populate_msb(tps6594_spi_crc_table, TPS6594_CRC8_POLYNOMIAL);
    return tps6594_device_init(tps, enable_crc);
    }
    static struct spi_driver tps6594_spi_driver = {
    .driver	= {
    .name = "tps6594",
    .of_match_table = tps6594_spi_of_match_table,
    },
    .probe = tps6594_spi_probe,
    };
    module_spi_driver(tps6594_spi_driver);
    MODULE_AUTHOR("Julien Panis <jpanis@baylibre.com>");
    MODULE_DESCRIPTION("SPI Interface Driver for TPS65224, TPS6594/3, and LP8764");
    MODULE_LICENSE("GPL");
