//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-aspeed.c
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
// Copyright (C) 2019 IBM Corp.

pub const ASPEED_MDIO_CTRL: c_uint = 0x0;

pub const ASPEED_MDIO_CTRL_ST_C45: c_int = 0;
pub const ASPEED_MDIO_CTRL_ST_C22: c_int = 1;

pub const ASPEED_MDIO_DATA: c_uint = 0x4;

pub const ASPEED_MDIO_INTERVAL_US: c_int = 100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_mdio {
    pub base: *mut void __iomem,
    pub reset: *mut reset_control,
}

    static int aspeed_mdio_op(struct mii_bus *bus, u8 st, u8 op, u8 phyad, u8 regad,
    u16 data)
    {
    struct aspeed_mdio *ctx = bus.priv;
    u32 ctrl;
    dev_dbg(&bus.dev, "%s: st: %u op: %u, phyad: %u, regad: %u, data: %u\n",
    __func__, st, op, phyad, regad, data);
    ctrl = ASPEED_MDIO_CTRL_FIRE
    | FIELD_PREP(ASPEED_MDIO_CTRL_ST, st)
    | FIELD_PREP(ASPEED_MDIO_CTRL_OP, op)
    | FIELD_PREP(ASPEED_MDIO_CTRL_PHYAD, phyad)
    | FIELD_PREP(ASPEED_MDIO_CTRL_REGAD, regad)
    | FIELD_PREP(ASPEED_MDIO_DATA_MIIRDATA, data);
    iowrite32(ctrl, ctx.base + ASPEED_MDIO_CTRL);
// Workaround for read-after-write issue.
// The controller may return stale data if a read follows immediately
// after a write. A dummy read forces the hardware to update its
// internal state, ensuring that the next real read returns correct data.
//
    ioread32(ctx.base + ASPEED_MDIO_CTRL);
    return readl_poll_timeout(ctx.base + ASPEED_MDIO_CTRL, ctrl,
    !(ctrl & ASPEED_MDIO_CTRL_FIRE),
    ASPEED_MDIO_INTERVAL_US,
    ASPEED_MDIO_TIMEOUT_US);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_mdio_get_data(bus: *mut mii_bus) -> c_int {
    static int aspeed_mdio_get_data(struct mii_bus *bus)
    {
    struct aspeed_mdio *ctx = bus.priv;
    u32 data;
    int rc;
    rc = readl_poll_timeout(ctx.base + ASPEED_MDIO_DATA, data,
    data & ASPEED_MDIO_DATA_IDLE,
    ASPEED_MDIO_INTERVAL_US,
    ASPEED_MDIO_TIMEOUT_US);
    if (rc < 0)
    return rc;
    return FIELD_GET(ASPEED_MDIO_DATA_MIIRDATA, data);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_mdio_read_c22(bus: *mut mii_bus, addr: c_int, regnum: c_int) -> c_int {
    static int aspeed_mdio_read_c22(struct mii_bus *bus, int addr, int regnum)
    {
    int rc;
    rc = aspeed_mdio_op(bus, ASPEED_MDIO_CTRL_ST_C22, MDIO_C22_OP_READ,
    addr, regnum, 0);
    if (rc < 0)
    return rc;
    return aspeed_mdio_get_data(bus);
    }
    static int aspeed_mdio_write_c22(struct mii_bus *bus, int addr, int regnum,
    u16 val)
    {
    return aspeed_mdio_op(bus, ASPEED_MDIO_CTRL_ST_C22, MDIO_C22_OP_WRITE,
    addr, regnum, val);
    }
    static int aspeed_mdio_read_c45(struct mii_bus *bus, int addr, int devad,
    int regnum)
    {
    int rc;
    rc = aspeed_mdio_op(bus, ASPEED_MDIO_CTRL_ST_C45, MDIO_C45_OP_ADDR,
    addr, devad, regnum);
    if (rc < 0)
    return rc;
    rc = aspeed_mdio_op(bus, ASPEED_MDIO_CTRL_ST_C45, MDIO_C45_OP_READ,
    addr, devad, 0);
    if (rc < 0)
    return rc;
    return aspeed_mdio_get_data(bus);
    }
    static int aspeed_mdio_write_c45(struct mii_bus *bus, int addr, int devad,
    int regnum, u16 val)
    {
    int rc;
    rc = aspeed_mdio_op(bus, ASPEED_MDIO_CTRL_ST_C45, MDIO_C45_OP_ADDR,
    addr, devad, regnum);
    if (rc < 0)
    return rc;
    return aspeed_mdio_op(bus, ASPEED_MDIO_CTRL_ST_C45, MDIO_C45_OP_WRITE,
    addr, devad, val);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_mdio_probe(pdev: *mut platform_device) -> c_int {
    static int aspeed_mdio_probe(struct platform_device *pdev)
    {
    struct aspeed_mdio *ctx;
    struct mii_bus *bus;
    int rc;
    bus = devm_mdiobus_alloc_size(&pdev.dev, sizeof(*ctx));
    if (!bus)
    return -ENOMEM;
    ctx = bus.priv;
    ctx.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctx.base))
    return PTR_ERR(ctx.base);
    ctx.reset = devm_reset_control_get_optional_shared(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(ctx.reset))
    return PTR_ERR(ctx.reset);
    reset_control_deassert(ctx.reset);
    bus.name = DRV_NAME;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s%d", pdev.name, pdev.id);
    bus.parent = &pdev.dev;
    bus.read = aspeed_mdio_read_c22;
    bus.write = aspeed_mdio_write_c22;
    bus.read_c45 = aspeed_mdio_read_c45;
    bus.write_c45 = aspeed_mdio_write_c45;
    rc = of_mdiobus_register(bus, pdev.dev.of_node);
    if (rc) {
    dev_err(&pdev.dev, "Cannot register MDIO bus!\n");
    reset_control_assert(ctx.reset);
    return rc;
    }
    platform_set_drvdata(pdev, bus);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_mdio_remove(pdev: *mut platform_device) {
    static void aspeed_mdio_remove(struct platform_device *pdev)
    {
    struct mii_bus *bus = (struct mii_bus *)platform_get_drvdata(pdev);
    struct aspeed_mdio *ctx = bus.priv;
    reset_control_assert(ctx.reset);
    mdiobus_unregister(bus);
    }
    static const struct of_device_id aspeed_mdio_of_match[] = {
    { .compatible = "aspeed,ast2600-mdio", },
    { },
    };
    MODULE_DEVICE_TABLE(of, aspeed_mdio_of_match);
    static struct platform_driver aspeed_mdio_driver = {
    .driver = {
    .name = DRV_NAME,
    .of_match_table = aspeed_mdio_of_match,
    },
    .probe = aspeed_mdio_probe,
    .remove = aspeed_mdio_remove,
    };
    module_platform_driver(aspeed_mdio_driver);
    MODULE_AUTHOR("Andrew Jeffery <andrew@aj.id.au>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ASPEED MDIO bus controller");
