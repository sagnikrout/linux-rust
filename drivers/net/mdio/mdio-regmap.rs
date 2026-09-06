//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-regmap.c
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
// Driver for MMIO-Mapped MDIO devices. Some IPs expose internal PHYs or PCS
// within the MMIO-mapped area
//
// Copyright (C) 2023 Maxime Chevallier <maxime.chevallier@bootlin.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_regmap_priv {
    pub regmap: *mut regmap,
    pub valid_addr: u8,
}

#[no_mangle]
unsafe extern "C" fn mdio_regmap_read_c22(bus: *mut mii_bus, addr: c_int, regnum: c_int) -> c_int {
    static int mdio_regmap_read_c22(struct mii_bus *bus, int addr, int regnum)
    {
    struct mdio_regmap_priv *ctx = bus.priv;
    unsigned int val;
    int ret;
    if (ctx.valid_addr != addr)
    return -ENODEV;
    ret = regmap_read(ctx.regmap, regnum, &val);
    if (ret < 0)
    return ret;
    return val;
    }
    static int mdio_regmap_write_c22(struct mii_bus *bus, int addr, int regnum,
    u16 val)
    {
    struct mdio_regmap_priv *ctx = bus.priv;
    if (ctx.valid_addr != addr)
    return -ENODEV;
    return regmap_write(ctx.regmap, regnum, val);
    }
    struct mii_bus *devm_mdio_regmap_register(struct device *dev,
    const struct mdio_regmap_config *config)
    {
    struct mdio_regmap_priv *mr;
    struct mii_bus *mii;
    int rc;
    if (!config.parent)
    return ERR_PTR(-EINVAL);
    mii = devm_mdiobus_alloc_size(config.parent, sizeof(*mr));
    if (!mii)
    return ERR_PTR(-ENOMEM);
    mr = mii.priv;
    mr.regmap = config.regmap;
    mr.valid_addr = config.valid_addr;
    mii.name = DRV_NAME;
    strscpy(mii.id, config.name, MII_BUS_ID_SIZE);
    mii.parent = config.parent;
    mii.read = mdio_regmap_read_c22;
    mii.write = mdio_regmap_write_c22;
    if (config.autoscan)
    mii.phy_mask = ~BIT(config.valid_addr);
    else
    mii.phy_mask = ~0;
    rc = devm_mdiobus_register(dev, mii);
    if (rc) {
    dev_err(config.parent, "Cannot register MDIO bus![%s] (%d)\n", mii.id, rc);
    return ERR_PTR(rc);
    }
    return mii;
    }
    EXPORT_SYMBOL_GPL(devm_mdio_regmap_register);
    MODULE_DESCRIPTION("MDIO API over regmap");
    MODULE_AUTHOR("Maxime Chevallier <maxime.chevallier@bootlin.com>");
    MODULE_LICENSE("GPL");
