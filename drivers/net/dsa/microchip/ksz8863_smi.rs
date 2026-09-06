//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/microchip/ksz8863_smi.c
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
// Microchip KSZ8863 series register access through SMI
//
// Copyright (C) 2019 Pengutronix, Michael Grzeschik <kernel@pengutronix.de>
//

// Serial Management Interface (SMI) uses the following frame format:
//
// preamble|start|Read/Write|  PHY   |  REG  |TA|   Data bits      | Idle
// |frame| OP code  |address |address|  |                  |
// read | 32x1´s | 01  |    00    | 1xRRR  | RRRRR |Z0| 00000000DDDDDDDD |  Z
// write| 32x1´s | 01  |    00    | 0xRRR  | RRRRR |10| xxxxxxxxDDDDDDDD |  Z
//

    static int ksz8863_mdio_read(void *ctx, const void *reg_buf, size_t reg_len,
    void *val_buf, size_t val_len)
    {
    struct ksz_device *dev = ctx;
    struct mdio_device *mdev;
    let mut reg: u8 = *(u8 *)reg_buf;
    u8 *val = val_buf;
    int i, ret = 0;
    mdev = dev.priv;
    mutex_lock_nested(&mdev.bus.mdio_lock, MDIO_MUTEX_NESTED);
    for (i = 0; i < val_len; i++) {
    let mut tmp: c_int = reg + i;
    ret = __mdiobus_read(mdev.bus, ((tmp & 0xE0) >> 5) |
    SMI_KSZ88XX_READ_PHY, tmp);
    if (ret < 0)
    goto out;
    val[i] = ret;
    }
    ret = 0;
    out:
    mutex_unlock(&mdev.bus.mdio_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ksz8863_mdio_write(ctx: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int ksz8863_mdio_write(void *ctx, const void *data, size_t count)
    {
    struct ksz_device *dev = ctx;
    struct mdio_device *mdev;
    int i, ret = 0;
    u32 reg;
    u8 *val;
    mdev = dev.priv;
    val = (u8 *)(data + 4);
    reg = *(u32 *)data;
    mutex_lock_nested(&mdev.bus.mdio_lock, MDIO_MUTEX_NESTED);
    for (i = 0; i < (count - 4); i++) {
    let mut tmp: c_int = reg + i;
    ret = __mdiobus_write(mdev.bus, ((tmp & 0xE0) >> 5),
    tmp, val[i]);
    if (ret < 0)
    goto out;
    }
    out:
    mutex_unlock(&mdev.bus.mdio_lock);
    return ret;
    }
    static const struct regmap_bus regmap_smi[] = {
    {
    .read = ksz8863_mdio_read,
    .write = ksz8863_mdio_write,
    },
    {
    .read = ksz8863_mdio_read,
    .write = ksz8863_mdio_write,
    .val_format_endian_default = REGMAP_ENDIAN_BIG,
    },
    {
    .read = ksz8863_mdio_read,
    .write = ksz8863_mdio_write,
    .val_format_endian_default = REGMAP_ENDIAN_BIG,
    }
    };
    static const struct regmap_config ksz8863_regmap_config[] = {
    {
    .name = "#8",
    .reg_bits = 8,
    .pad_bits = 24,
    .val_bits = 8,
    .cache_type = REGCACHE_NONE,
    .lock = ksz_regmap_lock,
    .unlock = ksz_regmap_unlock,
    .max_register = U8_MAX,
    },
    {
    .name = "#16",
    .reg_bits = 8,
    .pad_bits = 24,
    .val_bits = 16,
    .cache_type = REGCACHE_NONE,
    .lock = ksz_regmap_lock,
    .unlock = ksz_regmap_unlock,
    .max_register = U8_MAX,
    },
    {
    .name = "#32",
    .reg_bits = 8,
    .pad_bits = 24,
    .val_bits = 32,
    .cache_type = REGCACHE_NONE,
    .lock = ksz_regmap_lock,
    .unlock = ksz_regmap_unlock,
    .max_register = U8_MAX,
    }
    };
#[no_mangle]
unsafe extern "C" fn ksz8863_smi_probe(mdiodev: *mut mdio_device) -> c_int {
    static int ksz8863_smi_probe(struct mdio_device *mdiodev)
    {
    struct device *ddev = &mdiodev.dev;
    const struct ksz_chip_data *chip;
    struct regmap_config rc;
    struct ksz_device *dev;
    int ret;
    int i;
    chip = device_get_match_data(ddev);
    if (!chip)
    return -EINVAL;
    dev = ksz_switch_alloc(&mdiodev.dev, chip, mdiodev);
    if (!dev)
    return -ENOMEM;
    for (i = 0; i < __KSZ_NUM_REGMAPS; i++) {
    rc = ksz8863_regmap_config[i];
    rc.lock_arg = &dev.regmap_mutex;
    rc.wr_table = chip.wr_table;
    rc.rd_table = chip.rd_table;
    dev.regmap[i] = devm_regmap_init(&mdiodev.dev,
    &regmap_smi[i], dev,
    &rc);
    if (IS_ERR(dev.regmap[i])) {
    return dev_err_probe(&mdiodev.dev,
    PTR_ERR(dev.regmap[i]),
    "Failed to initialize regmap%i\n",
    ksz8863_regmap_config[i].val_bits);
    }
    }
    if (mdiodev.dev.platform_data)
    dev.pdata = mdiodev.dev.platform_data;
    ret = ksz_switch_register(dev);
// Main DSA driver may not be started yet.
    if (ret)
    return ret;
    dev_set_drvdata(&mdiodev.dev, dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ksz8863_smi_remove(mdiodev: *mut mdio_device) {
    static void ksz8863_smi_remove(struct mdio_device *mdiodev)
    {
    struct ksz_device *dev = dev_get_drvdata(&mdiodev.dev);
    if (dev)
    ksz_switch_remove(dev);
    }
#[no_mangle]
unsafe extern "C" fn ksz8863_smi_shutdown(mdiodev: *mut mdio_device) {
    static void ksz8863_smi_shutdown(struct mdio_device *mdiodev)
    {
    struct ksz_device *dev = dev_get_drvdata(&mdiodev.dev);
    if (dev)
    dsa_switch_shutdown(dev.ds);
    dev_set_drvdata(&mdiodev.dev, core::ptr::null_mut());
    }
    static const struct of_device_id ksz8863_dt_ids[] = {
    {
    .compatible = "microchip,ksz8863",
    .data = &ksz_switch_chips[KSZ88X3]
    },
    {
    .compatible = "microchip,ksz8873",
    .data = &ksz_switch_chips[KSZ88X3]
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, ksz8863_dt_ids);
    static struct mdio_driver ksz8863_driver = {
    .probe	= ksz8863_smi_probe,
    .remove	= ksz8863_smi_remove,
    .shutdown = ksz8863_smi_shutdown,
    .mdiodrv.driver = {
    .name	= "ksz8863-switch",
    .of_match_table = ksz8863_dt_ids,
    },
    };
    mdio_module_driver(ksz8863_driver);
    MODULE_AUTHOR("Michael Grzeschik <m.grzeschik@pengutronix.de>");
    MODULE_DESCRIPTION("Microchip KSZ8863 SMI Switch driver");
    MODULE_LICENSE("GPL v2");
