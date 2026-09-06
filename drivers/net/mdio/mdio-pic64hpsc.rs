//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-pic64hpsc.c
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
// Microchip PIC64-HPSC/HX MDIO controller driver
//
// Copyright (c) 2026 Microchip Technology Inc. and its subsidiaries.
//

pub const MDIO_REG_PRESCALER: c_uint = 0x20;

pub const MDIO_REG_FRAME_CFG_1: c_uint = 0x24;

pub const MDIO_REG_FRAME_CFG_2: c_uint = 0x28;

// Possible value of MDIO_OPERATION_MASK

pub const MDIO_REG_FRAME_STATUS: c_uint = 0x2C;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic64hpsc_mdio_dev {
    pub regs: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn pic64hpsc_mdio_wait_trigger(bus: *mut mii_bus) -> c_int {
    static int pic64hpsc_mdio_wait_trigger(struct mii_bus *bus)
    {
    struct pic64hpsc_mdio_dev *priv = bus.priv;
    u32 val;
    int ret;
// The MDIO_TRIGGER bit returns 0 when a transaction has completed.
    ret = readl_poll_timeout(priv.regs + MDIO_REG_FRAME_CFG_2, val,
    !(val & MDIO_TRIGGER_BIT), 50, 10000);
    if (ret < 0)
    dev_dbg(&bus.dev, "TRIGGER bit timeout: %x\n", val);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pic64hpsc_mdio_c22_read(bus: *mut mii_bus, mii_id: c_int, regnum: c_int) -> c_int {
    static int pic64hpsc_mdio_c22_read(struct mii_bus *bus, int mii_id, int regnum)
    {
    struct pic64hpsc_mdio_dev *priv = bus.priv;
    u32 val;
    int ret;
    ret = pic64hpsc_mdio_wait_trigger(bus);
    if (ret)
    return ret;
    writel(MDIO_TRIGGER_BIT | FIELD_PREP(MDIO_REG_DEV_ADDR_MASK, regnum) |
    FIELD_PREP(MDIO_PHY_PRT_ADDR_MASK, mii_id) |
    FIELD_PREP(MDIO_OPERATION_MASK, MDIO_OPERATION_READ) |
    FIELD_PREP(MDIO_START_OF_FRAME_MASK, 1),
    priv.regs + MDIO_REG_FRAME_CFG_2);
    ret = pic64hpsc_mdio_wait_trigger(bus);
    if (ret)
    return ret;
    val = readl(priv.regs + MDIO_REG_FRAME_STATUS);
// The MDIO_READOK is a 1-bit value reflecting the inverse of the MDIO
// bus value captured during the 2nd TA cycle. A PHY/Port should drive
// the MDIO bus with a logic 0 on the 2nd TA cycle, however, the
// PHY/Port could optionally drive a logic 1, to communicate a read
// failure. This feature is optional, not defined by the 802.3 standard
// and not supported in standard external PHYs.
//
    if (!(bus.phy_ignore_ta_mask & 1 << mii_id) &&
    !FIELD_GET(MDIO_READOK_BIT, val)) {
    dev_dbg(&bus.dev, "READOK bit cleared\n");
    return -EIO;
    }
    return FIELD_GET(MDIO_RDATA_MASK, val);
    }
    static int pic64hpsc_mdio_c22_write(struct mii_bus *bus, int mii_id, int regnum,
    u16 value)
    {
    struct pic64hpsc_mdio_dev *priv = bus.priv;
    int ret;
    ret = pic64hpsc_mdio_wait_trigger(bus);
    if (ret < 0)
    return ret;
    writel(FIELD_PREP(MDIO_WDATA_MASK, value),
    priv.regs + MDIO_REG_FRAME_CFG_1);
    writel(MDIO_TRIGGER_BIT | FIELD_PREP(MDIO_REG_DEV_ADDR_MASK, regnum) |
    FIELD_PREP(MDIO_PHY_PRT_ADDR_MASK, mii_id) |
    FIELD_PREP(MDIO_OPERATION_MASK, MDIO_OPERATION_WRITE) |
    FIELD_PREP(MDIO_START_OF_FRAME_MASK, 1),
    priv.regs + MDIO_REG_FRAME_CFG_2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic64hpsc_mdio_probe(pdev: *mut platform_device) -> c_int {
    static int pic64hpsc_mdio_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct pic64hpsc_mdio_dev *priv;
    struct mii_bus *bus;
    unsigned long rate;
    struct clk *clk;
    u32 bus_freq;
    u32 div;
    int ret;
    bus = devm_mdiobus_alloc_size(dev, sizeof(*priv));
    if (!bus)
    return -ENOMEM;
    priv = bus.priv;
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return PTR_ERR(priv.regs);
    bus.name = KBUILD_MODNAME;
    bus.read = pic64hpsc_mdio_c22_read;
    bus.write = pic64hpsc_mdio_c22_write;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s", dev_name(dev));
    bus.parent = dev;
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    if (of_property_read_u32(np, "clock-frequency", &bus_freq))
    bus_freq = 2500000;
    rate = clk_get_rate(clk);
    div = DIV_ROUND_UP(rate, 2 * bus_freq) - 1;
    if (div == 0 || div & ~MDIO_CFG_PRESCALE_MASK) {
    dev_err(dev, "MDIO clock-frequency out of range\n");
    return -EINVAL;
    }
    dev_dbg(dev, "rate=%lu bus_freq=%u real_bus_freq=%lu div=%u\n", rate,
    bus_freq, rate / (2 * (1 + div)), div);
    writel(div, priv.regs + MDIO_REG_PRESCALER);
    ret = devm_of_mdiobus_register(dev, bus, np);
    if (ret) {
    dev_err(dev, "Cannot register MDIO bus (%d)\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct of_device_id pic64hpsc_mdio_match[] = {
    { .compatible = "microchip,pic64hpsc-mdio" },
    {}
    };
    MODULE_DEVICE_TABLE(of, pic64hpsc_mdio_match);
    static struct platform_driver pic64hpsc_mdio_driver = {
    .probe = pic64hpsc_mdio_probe,
    .driver = {
    .name = KBUILD_MODNAME,
    .of_match_table = pic64hpsc_mdio_match,
    },
    };
    module_platform_driver(pic64hpsc_mdio_driver);
    MODULE_AUTHOR("Charles Perry <charles.perry@microchip.com>");
    MODULE_DESCRIPTION("Microchip PIC64-HPSC/HX MDIO driver");
    MODULE_LICENSE("GPL");
