//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/microchip/ksz_spi.c
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
// Microchip ksz series register access through SPI
//
// Copyright (C) 2017-2024 Microchip Technology Inc.
// Tristram Ha <Tristram.Ha@microchip.com>
//

pub const KSZ8463_SPI_ADDR_SHIFT: c_int = 13;
pub const KSZ8463_SPI_ADDR_ALIGN: c_int = 3;
pub const KSZ8463_SPI_TURNAROUND_SHIFT: c_int = 2;
pub const KSZ8795_SPI_ADDR_SHIFT: c_int = 12;
pub const KSZ8795_SPI_ADDR_ALIGN: c_int = 3;
pub const KSZ8795_SPI_TURNAROUND_SHIFT: c_int = 1;
pub const KSZ8863_SPI_ADDR_SHIFT: c_int = 8;
pub const KSZ8863_SPI_ADDR_ALIGN: c_int = 8;
pub const KSZ8863_SPI_TURNAROUND_SHIFT: c_int = 0;
pub const KSZ9477_SPI_ADDR_SHIFT: c_int = 24;
pub const KSZ9477_SPI_ADDR_ALIGN: c_int = 3;
pub const KSZ9477_SPI_TURNAROUND_SHIFT: c_int = 5;
    KSZ_REGMAP_TABLE(ksz8795, 16, KSZ8795_SPI_ADDR_SHIFT,
    KSZ8795_SPI_TURNAROUND_SHIFT, KSZ8795_SPI_ADDR_ALIGN);
    KSZ_REGMAP_TABLE(ksz8863, 16, KSZ8863_SPI_ADDR_SHIFT,
    KSZ8863_SPI_TURNAROUND_SHIFT, KSZ8863_SPI_ADDR_ALIGN);
    KSZ_REGMAP_TABLE(ksz9477, 32, KSZ9477_SPI_ADDR_SHIFT,
    KSZ9477_SPI_TURNAROUND_SHIFT, KSZ9477_SPI_ADDR_ALIGN);
#[no_mangle]
unsafe extern "C" fn ksz8463_reg(reg: u16, size: usize) -> u16 {
    static u16 ksz8463_reg(u16 reg, size_t size)
    {
    switch (size) {
    case 1:
    reg = ((reg >> 2) << 4) | (1 << (reg & 3));
    break;
    case 2:
    reg = ((reg >> 2) << 4) | (reg & 2 ? 0x0c : 0x03);
    break;
    default:
    reg = ((reg >> 2) << 4) | 0xf;
    break;
    }
    reg <<= KSZ8463_SPI_TURNAROUND_SHIFT;
    return reg;
    }
    static int ksz8463_spi_read(void *context,
    const void *reg, size_t reg_size,
    void *val, size_t val_size)
    {
    struct device *dev = context;
    struct spi_device *spi = to_spi_device(dev);
    u8 bytes[2];
    u16 cmd;
    int rc;
    if (reg_size > 2 || val_size > 4)
    return -EINVAL;
    memcpy(&cmd, reg, sizeof(u16));
    cmd = ksz8463_reg(cmd, val_size);
// SPI command uses big-endian format.
    put_unaligned_be16(cmd, bytes);
    rc = spi_write_then_read(spi, bytes, reg_size, val, val_size);

// Register value uses little-endian format so need to convert when
// running in big-endian system.
//
    if (!rc && val_size > 1) {
    if (val_size == 2) {
    let mut v: u16 = get_unaligned_le16(val);
    memcpy(val, &v, sizeof(v));
    } else if (val_size == 4) {
    let mut v: u32 = get_unaligned_le32(val);
    memcpy(val, &v, sizeof(v));
    }
    }

    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ksz8463_spi_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int ksz8463_spi_write(void *context, const void *data, size_t count)
    {
    struct device *dev = context;
    struct spi_device *spi = to_spi_device(dev);
    let mut val_size: usize = count - 2;
    u8 bytes[6];
    u16 cmd;
    if (count <= 2 || count > 6)
    return -EINVAL;
    memcpy(bytes, data, count);
    memcpy(&cmd, data, sizeof(u16));
    cmd = ksz8463_reg(cmd, val_size);
    cmd |= (1 << (KSZ8463_SPI_ADDR_SHIFT + KSZ8463_SPI_TURNAROUND_SHIFT));
// SPI command uses big-endian format.
    put_unaligned_be16(cmd, bytes);

// Register value uses little-endian format so need to convert when
// running in big-endian system.
//
    if (val_size == 2) {
    u8 *val = &bytes[2];
    u16 v;
    memcpy(&v, val, sizeof(v));
    put_unaligned_le16(v, val);
    } else if (val_size == 4) {
    u8 *val = &bytes[2];
    u32 v;
    memcpy(&v, val, sizeof(v));
    put_unaligned_le32(v, val);
    }

    return spi_write(spi, bytes, count);
    }
    KSZ8463_REGMAP_TABLE(ksz8463, KSZ8463_SPI_ADDR_SHIFT, 0,
    KSZ8463_SPI_ADDR_ALIGN);
#[no_mangle]
unsafe extern "C" fn ksz_spi_probe(spi: *mut spi_device) -> c_int {
    static int ksz_spi_probe(struct spi_device *spi)
    {
    const struct regmap_config *regmap_config;
    const struct ksz_chip_data *chip;
    struct device *ddev = &spi.dev;
    struct regmap_config rc;
    struct ksz_device *dev;
    int i, ret = 0;
    chip = device_get_match_data(ddev);
    if (!chip)
    return -EINVAL;
    dev = ksz_switch_alloc(&spi.dev, chip, spi);
    if (!dev)
    return -ENOMEM;
// Save chip id to do special initialization when probing.
    dev.chip_id = chip.chip_id;
    if (chip.chip_id == KSZ88X3_CHIP_ID)
    regmap_config = ksz8863_regmap_config;
#[no_mangle]
pub unsafe extern "C" fn if(KSZ8463_CHIP_ID: chip->chip_id ==) -> else {
    else if (chip.chip_id == KSZ8463_CHIP_ID)
    regmap_config = ksz8463_regmap_config;
    else if (chip.chip_id == KSZ8795_CHIP_ID ||
    chip.chip_id == KSZ8794_CHIP_ID ||
    chip.chip_id == KSZ8765_CHIP_ID)
    regmap_config = ksz8795_regmap_config;
    else if (chip.chip_id == KSZ8895_CHIP_ID ||
    chip.chip_id == KSZ8864_CHIP_ID)
    regmap_config = ksz8863_regmap_config;
    else
    regmap_config = ksz9477_regmap_config;
    for (i = 0; i < __KSZ_NUM_REGMAPS; i++) {
    rc = regmap_config[i];
    rc.lock_arg = &dev.regmap_mutex;
    rc.wr_table = chip.wr_table;
    rc.rd_table = chip.rd_table;
    dev.regmap[i] = devm_regmap_init_spi(spi, &rc);
    if (IS_ERR(dev.regmap[i])) {
    return dev_err_probe(&spi.dev, PTR_ERR(dev.regmap[i]),
    "Failed to initialize regmap%i\n",
    regmap_config[i].val_bits);
    }
    }
    if (spi.dev.platform_data)
    dev.pdata = spi.dev.platform_data;
// setup spi
    spi.mode = SPI_MODE_3;
    ret = spi_setup(spi);
    if (ret)
    return ret;
    dev.irq = spi.irq;
    ret = ksz_switch_register(dev);
// Main DSA driver may not be started yet.
    if (ret)
    return ret;
    spi_set_drvdata(spi, dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ksz_spi_remove(spi: *mut spi_device) {
    static void ksz_spi_remove(struct spi_device *spi)
    {
    struct ksz_device *dev = spi_get_drvdata(spi);
    if (dev)
    ksz_switch_remove(dev);
    }
#[no_mangle]
unsafe extern "C" fn ksz_spi_shutdown(spi: *mut spi_device) {
    static void ksz_spi_shutdown(struct spi_device *spi)
    {
    struct ksz_device *dev = spi_get_drvdata(spi);
    if (!dev)
    return;
    ksz_switch_shutdown(dev);
    spi_set_drvdata(spi, core::ptr::null_mut());
    }
    static const struct of_device_id ksz_dt_ids[] = {
    {
    .compatible = "microchip,ksz8463",
    .data = &ksz_switch_chips[KSZ8463]
    },
    {
    .compatible = "microchip,ksz8765",
    .data = &ksz_switch_chips[KSZ8765]
    },
    {
    .compatible = "microchip,ksz8794",
    .data = &ksz_switch_chips[KSZ8794]
    },
    {
    .compatible = "microchip,ksz8795",
    .data = &ksz_switch_chips[KSZ8795]
    },
    {
    .compatible = "microchip,ksz8863",
    .data = &ksz_switch_chips[KSZ88X3]
    },
    {
    .compatible = "microchip,ksz8864",
    .data = &ksz_switch_chips[KSZ8864]
    },
    {
    .compatible = "microchip,ksz8873",
    .data = &ksz_switch_chips[KSZ88X3]
    },
    {
    .compatible = "microchip,ksz8895",
    .data = &ksz_switch_chips[KSZ8895]
    },
    {
    .compatible = "microchip,ksz9477",
    .data = &ksz_switch_chips[KSZ9477]
    },
    {
    .compatible = "microchip,ksz9896",
    .data = &ksz_switch_chips[KSZ9896]
    },
    {
    .compatible = "microchip,ksz9897",
    .data = &ksz_switch_chips[KSZ9897]
    },
    {
    .compatible = "microchip,ksz9893",
    .data = &ksz_switch_chips[KSZ9893]
    },
    {
    .compatible = "microchip,ksz9563",
    .data = &ksz_switch_chips[KSZ9563]
    },
    {
    .compatible = "microchip,ksz8563",
    .data = &ksz_switch_chips[KSZ8563]
    },
    {
    .compatible = "microchip,ksz8567",
    .data = &ksz_switch_chips[KSZ8567]
    },
    {
    .compatible = "microchip,ksz9567",
    .data = &ksz_switch_chips[KSZ9567]
    },
    {
    .compatible = "microchip,lan9370",
    .data = &ksz_switch_chips[LAN9370]
    },
    {
    .compatible = "microchip,lan9371",
    .data = &ksz_switch_chips[LAN9371]
    },
    {
    .compatible = "microchip,lan9372",
    .data = &ksz_switch_chips[LAN9372]
    },
    {
    .compatible = "microchip,lan9373",
    .data = &ksz_switch_chips[LAN9373]
    },
    {
    .compatible = "microchip,lan9374",
    .data = &ksz_switch_chips[LAN9374]
    },
    {
    .compatible = "microchip,lan9646",
    .data = &ksz_switch_chips[LAN9646]
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, ksz_dt_ids);
    static const struct spi_device_id ksz_spi_ids[] = {
    { "ksz8463" },
    { "ksz8765" },
    { "ksz8794" },
    { "ksz8795" },
    { "ksz8863" },
    { "ksz8864" },
    { "ksz8873" },
    { "ksz8895" },
    { "ksz9477" },
    { "ksz9896" },
    { "ksz9897" },
    { "ksz9893" },
    { "ksz9563" },
    { "ksz8563" },
    { "ksz8567" },
    { "ksz9567" },
    { "lan9370" },
    { "lan9371" },
    { "lan9372" },
    { "lan9373" },
    { "lan9374" },
    { "lan9646" },
    { },
    };
    MODULE_DEVICE_TABLE(spi, ksz_spi_ids);
    static DEFINE_SIMPLE_DEV_PM_OPS(ksz_spi_pm_ops,
    ksz_switch_suspend, ksz_switch_resume);
    static struct spi_driver ksz_spi_driver = {
    .driver = {
    .name	= "ksz-switch",
    .of_match_table = ksz_dt_ids,
    .pm = &ksz_spi_pm_ops,
    },
    .id_table = ksz_spi_ids,
    .probe	= ksz_spi_probe,
    .remove	= ksz_spi_remove,
    .shutdown = ksz_spi_shutdown,
    };
    module_spi_driver(ksz_spi_driver);
    MODULE_ALIAS("spi:lan937x");
    MODULE_AUTHOR("Tristram Ha <Tristram.Ha@microchip.com>");
    MODULE_DESCRIPTION("Microchip ksz Series Switch SPI Driver");
    MODULE_LICENSE("GPL");
