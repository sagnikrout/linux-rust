//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-ipq8064.c
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
// Qualcomm IPQ8064 MDIO interface driver
//
// Copyright (C) 2019 Christian Lamparter <chunkeey@gmail.com>
// Copyright (C) 2020 Ansuel Smith <ansuelsmth@gmail.com>
//

// MII address register definitions
pub const MII_ADDR_REG_ADDR: c_uint = 0x10;

pub const MII_REG_SHIFT: c_int = 6;

pub const MII_ADDR_SHIFT: c_int = 11;

pub const MII_DATA_REG_ADDR: c_uint = 0x14;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipq8064_mdio {
    pub /: *mut *mut *mut regmap base; / NSS_GMAC0_BASE,
}

    static int
    ipq8064_mdio_wait_busy(struct ipq8064_mdio *priv)
    {
    u32 busy;
    return regmap_read_poll_timeout(priv.base, MII_ADDR_REG_ADDR, busy,
    !(busy & MII_BUSY), MII_MDIO_DELAY_USEC,
    MII_MDIO_RETRY_MSEC * USEC_PER_MSEC);
    }
    static int
    ipq8064_mdio_read(struct mii_bus *bus, int phy_addr, int reg_offset)
    {
    let mut miiaddr: u32 = MII_BUSY | MII_CLKRANGE_250_300M;
    struct ipq8064_mdio *priv = bus.priv;
    u32 ret_val;
    int err;
    miiaddr |= ((phy_addr << MII_ADDR_SHIFT) & MII_ADDR_MASK) |
    ((reg_offset << MII_REG_SHIFT) & MII_REG_MASK);
    regmap_write(priv.base, MII_ADDR_REG_ADDR, miiaddr);
    usleep_range(10, 13);
    err = ipq8064_mdio_wait_busy(priv);
    if (err)
    return err;
    regmap_read(priv.base, MII_DATA_REG_ADDR, &ret_val);
    return (int)ret_val;
    }
    static int
    ipq8064_mdio_write(struct mii_bus *bus, int phy_addr, int reg_offset, u16 data)
    {
    let mut miiaddr: u32 = MII_WRITE | MII_BUSY | MII_CLKRANGE_250_300M;
    struct ipq8064_mdio *priv = bus.priv;
    regmap_write(priv.base, MII_DATA_REG_ADDR, data);
    miiaddr |= ((phy_addr << MII_ADDR_SHIFT) & MII_ADDR_MASK) |
    ((reg_offset << MII_REG_SHIFT) & MII_REG_MASK);
    regmap_write(priv.base, MII_ADDR_REG_ADDR, miiaddr);
// For the specific reg 31 extra time is needed or the next
// read will produce garbage data.
//
    if (reg_offset == 31)
    usleep_range(30, 43);
    else
    usleep_range(10, 13);
    return ipq8064_mdio_wait_busy(priv);
    }
    static const struct regmap_config ipq8064_mdio_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .can_multi_write = false,
// the mdio lock is used by any user of this mdio driver
    .disable_locking = true,
    .cache_type = REGCACHE_NONE,
    };
    static int
    ipq8064_mdio_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct ipq8064_mdio *priv;
    struct resource res;
    struct mii_bus *bus;
    void __iomem *base;
    int ret;
    if (of_address_to_resource(np, 0, &res))
    return -ENOMEM;
    base = devm_ioremap(&pdev.dev, res.start, resource_size(&res));
    if (!base)
    return -ENOMEM;
    bus = devm_mdiobus_alloc_size(&pdev.dev, sizeof(*priv));
    if (!bus)
    return -ENOMEM;
    bus.name = "ipq8064_mdio_bus";
    bus.read = ipq8064_mdio_read;
    bus.write = ipq8064_mdio_write;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s-mii", dev_name(&pdev.dev));
    bus.parent = &pdev.dev;
    priv = bus.priv;
    priv.base = devm_regmap_init_mmio(&pdev.dev, base,
    &ipq8064_mdio_regmap_config);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    ret = of_mdiobus_register(bus, np);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, bus);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipq8064_mdio_remove(pdev: *mut platform_device) {
    static void ipq8064_mdio_remove(struct platform_device *pdev)
    {
    struct mii_bus *bus = platform_get_drvdata(pdev);
    mdiobus_unregister(bus);
    }
    static const struct of_device_id ipq8064_mdio_dt_ids[] = {
    { .compatible = "qcom,ipq8064-mdio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ipq8064_mdio_dt_ids);
    static struct platform_driver ipq8064_mdio_driver = {
    .probe = ipq8064_mdio_probe,
    .remove = ipq8064_mdio_remove,
    .driver = {
    .name = "ipq8064-mdio",
    .of_match_table = ipq8064_mdio_dt_ids,
    },
    };
    module_platform_driver(ipq8064_mdio_driver);
    MODULE_DESCRIPTION("Qualcomm IPQ8064 MDIO interface driver");
    MODULE_AUTHOR("Christian Lamparter <chunkeey@gmail.com>");
    MODULE_AUTHOR("Ansuel Smith <ansuelsmth@gmail.com>");
    MODULE_LICENSE("GPL");
