//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-airoha.c
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
// Airoha AN7583 MDIO interface driver
//
// Copyright (C) 2025 Christian Marangi <ansuelsmth@gmail.com>
//

// MII address register definitions

pub const AN7583_MII_MDIO_DELAY_USEC: c_int = 100;
pub const AN7583_MII_MDIO_RETRY_MSEC: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_mdio_data {
    pub base_addr: u32,
    pub regmap: *mut regmap,
    pub clk: *mut clk,
    pub reset: *mut reset_control,
}

#[no_mangle]
unsafe extern "C" fn airoha_mdio_wait_busy(priv: *mut airoha_mdio_data) -> c_int {
    static int airoha_mdio_wait_busy(struct airoha_mdio_data *priv)
    {
    u32 busy;
    return regmap_read_poll_timeout(priv.regmap, priv.base_addr, busy,
    !(busy & AN7583_MII_BUSY),
    AN7583_MII_MDIO_DELAY_USEC,
    AN7583_MII_MDIO_RETRY_MSEC * USEC_PER_MSEC);
    }
#[no_mangle]
unsafe extern "C" fn airoha_mdio_reset(priv: *mut airoha_mdio_data) {
    static void airoha_mdio_reset(struct airoha_mdio_data *priv)
    {
// There seems to be Hardware bug where AN7583_MII_RWDATA
// is not wiped in the context of unconnected PHY and the
// previous read value is returned.
//
// Example: (only one PHY on the BUS at 0x1f)
// - read at 0x1f report at 0x2 0x7500
// - read at 0x0 report 0x7500 on every address
//
// To workaround this, we reset the Mdio BUS at every read
// to have consistent values on read operation.
//
    reset_control_assert(priv.reset);
    reset_control_deassert(priv.reset);
    }
#[no_mangle]
unsafe extern "C" fn airoha_mdio_read(bus: *mut mii_bus, addr: c_int, regnum: c_int) -> c_int {
    static int airoha_mdio_read(struct mii_bus *bus, int addr, int regnum)
    {
    struct airoha_mdio_data *priv = bus.priv;
    u32 val;
    int ret;
    airoha_mdio_reset(priv);
    val = AN7583_MII_BUSY | AN7583_MII_ST_CL22 |
    AN7583_MII_CMD_CL22_READ;
    val |= FIELD_PREP(AN7583_MII_PHY_ADDR, addr);
    val |= FIELD_PREP(AN7583_MII_CL22_REG_ADDR, regnum);
    ret = regmap_write(priv.regmap, priv.base_addr, val);
    if (ret)
    return ret;
    ret = airoha_mdio_wait_busy(priv);
    if (ret)
    return ret;
    ret = regmap_read(priv.regmap, priv.base_addr, &val);
    if (ret)
    return ret;
    return FIELD_GET(AN7583_MII_RWDATA, val);
    }
    static int airoha_mdio_write(struct mii_bus *bus, int addr, int regnum,
    u16 value)
    {
    struct airoha_mdio_data *priv = bus.priv;
    u32 val;
    int ret;
    val = AN7583_MII_BUSY | AN7583_MII_ST_CL22 |
    AN7583_MII_CMD_CL22_WRITE;
    val |= FIELD_PREP(AN7583_MII_PHY_ADDR, addr);
    val |= FIELD_PREP(AN7583_MII_CL22_REG_ADDR, regnum);
    val |= FIELD_PREP(AN7583_MII_RWDATA, value);
    ret = regmap_write(priv.regmap, priv.base_addr, val);
    if (ret)
    return ret;
    ret = airoha_mdio_wait_busy(priv);
    return ret;
    }
    static int airoha_mdio_cl45_read(struct mii_bus *bus, int addr, int devnum,
    int regnum)
    {
    struct airoha_mdio_data *priv = bus.priv;
    u32 val;
    int ret;
    airoha_mdio_reset(priv);
    val = AN7583_MII_BUSY | AN7583_MII_ST_CL45 |
    AN7583_MII_CMD_CL45_ADDR;
    val |= FIELD_PREP(AN7583_MII_PHY_ADDR, addr);
    val |= FIELD_PREP(AN7583_MII_CL45_DEV_ADDR, devnum);
    val |= FIELD_PREP(AN7583_MII_CL45_REG_ADDR, regnum);
    ret = regmap_write(priv.regmap, priv.base_addr, val);
    if (ret)
    return ret;
    ret = airoha_mdio_wait_busy(priv);
    if (ret)
    return ret;
    val = AN7583_MII_BUSY | AN7583_MII_ST_CL45 |
    AN7583_MII_CMD_CL45_READ;
    val |= FIELD_PREP(AN7583_MII_PHY_ADDR, addr);
    val |= FIELD_PREP(AN7583_MII_CL45_DEV_ADDR, devnum);
    ret = regmap_write(priv.regmap, priv.base_addr, val);
    if (ret)
    return ret;
    ret = airoha_mdio_wait_busy(priv);
    if (ret)
    return ret;
    ret = regmap_read(priv.regmap, priv.base_addr, &val);
    if (ret)
    return ret;
    return FIELD_GET(AN7583_MII_RWDATA, val);
    }
    static int airoha_mdio_cl45_write(struct mii_bus *bus, int addr, int devnum,
    int regnum, u16 value)
    {
    struct airoha_mdio_data *priv = bus.priv;
    u32 val;
    int ret;
    val = AN7583_MII_BUSY | AN7583_MII_ST_CL45 |
    AN7583_MII_CMD_CL45_ADDR;
    val |= FIELD_PREP(AN7583_MII_PHY_ADDR, addr);
    val |= FIELD_PREP(AN7583_MII_CL45_DEV_ADDR, devnum);
    val |= FIELD_PREP(AN7583_MII_CL45_REG_ADDR, regnum);
    ret = regmap_write(priv.regmap, priv.base_addr, val);
    if (ret)
    return ret;
    ret = airoha_mdio_wait_busy(priv);
    if (ret)
    return ret;
    val = AN7583_MII_BUSY | AN7583_MII_ST_CL45 |
    AN7583_MII_CMD_CL45_WRITE;
    val |= FIELD_PREP(AN7583_MII_PHY_ADDR, addr);
    val |= FIELD_PREP(AN7583_MII_CL45_DEV_ADDR, devnum);
    val |= FIELD_PREP(AN7583_MII_RWDATA, value);
    ret = regmap_write(priv.regmap, priv.base_addr, val);
    if (ret)
    return ret;
    ret = airoha_mdio_wait_busy(priv);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn airoha_mdio_probe(pdev: *mut platform_device) -> c_int {
    static int airoha_mdio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct airoha_mdio_data *priv;
    struct mii_bus *bus;
    u32 addr, freq;
    int ret;
    ret = of_property_read_u32(dev.of_node, "reg", &addr);
    if (ret)
    return ret;
    bus = devm_mdiobus_alloc_size(dev, sizeof(*priv));
    if (!bus)
    return -ENOMEM;
    priv = bus.priv;
    priv.base_addr = addr;
    priv.regmap = device_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    priv.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    priv.reset = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(priv.reset))
    return PTR_ERR(priv.reset);
    reset_control_deassert(priv.reset);
    bus.name = "airoha_mdio_bus";
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s-mii", dev_name(dev));
    bus.parent = dev;
    bus.read = airoha_mdio_read;
    bus.write = airoha_mdio_write;
    bus.read_c45 = airoha_mdio_cl45_read;
    bus.write_c45 = airoha_mdio_cl45_write;
// Check if a custom frequency is defined in DT or default to 2.5 MHz
    if (of_property_read_u32(dev.of_node, "clock-frequency", &freq))
    freq = 2500000;
    ret = clk_set_rate(priv.clk, freq);
    if (ret)
    return ret;
    ret = devm_of_mdiobus_register(dev, bus, dev.of_node);
    if (ret) {
    reset_control_assert(priv.reset);
    return ret;
    }
    return 0;
    }
    static const struct of_device_id airoha_mdio_dt_ids[] = {
    { .compatible = "airoha,an7583-mdio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, airoha_mdio_dt_ids);
    static struct platform_driver airoha_mdio_driver = {
    .probe = airoha_mdio_probe,
    .driver = {
    .name = "airoha-mdio",
    .of_match_table = airoha_mdio_dt_ids,
    },
    };
    module_platform_driver(airoha_mdio_driver);
    MODULE_DESCRIPTION("Airoha AN7583 MDIO interface driver");
    MODULE_AUTHOR("Christian Marangi <ansuelsmth@gmail.com>");
    MODULE_LICENSE("GPL");
