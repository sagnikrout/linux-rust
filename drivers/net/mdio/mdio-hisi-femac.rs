//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-hisi-femac.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Hisilicon Fast Ethernet MDIO Bus Driver
//
// Copyright (c) 2016 HiSilicon Technologies Co., Ltd.
//

pub const MDIO_RWCTRL: c_uint = 0x00;
pub const MDIO_RO_DATA: c_uint = 0x04;

pub const BIT_PHY_ADDR_OFFSET: c_int = 8;
pub const BIT_WR_DATA_OFFSET: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_femac_mdio_data {
    pub clk: *mut clk,
    pub membase: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn hisi_femac_mdio_wait_ready(data: *mut hisi_femac_mdio_data) -> c_int {
    static int hisi_femac_mdio_wait_ready(struct hisi_femac_mdio_data *data)
    {
    u32 val;
    return readl_poll_timeout(data.membase + MDIO_RWCTRL,
    val, val & MDIO_RW_FINISH, 20, 10000);
    }
#[no_mangle]
unsafe extern "C" fn hisi_femac_mdio_read(bus: *mut mii_bus, mii_id: c_int, regnum: c_int) -> c_int {
    static int hisi_femac_mdio_read(struct mii_bus *bus, int mii_id, int regnum)
    {
    struct hisi_femac_mdio_data *data = bus.priv;
    int ret;
    ret = hisi_femac_mdio_wait_ready(data);
    if (ret)
    return ret;
    writel((mii_id << BIT_PHY_ADDR_OFFSET) | regnum,
    data.membase + MDIO_RWCTRL);
    ret = hisi_femac_mdio_wait_ready(data);
    if (ret)
    return ret;
    return readl(data.membase + MDIO_RO_DATA) & 0xFFFF;
    }
    static int hisi_femac_mdio_write(struct mii_bus *bus, int mii_id, int regnum,
    u16 value)
    {
    struct hisi_femac_mdio_data *data = bus.priv;
    int ret;
    ret = hisi_femac_mdio_wait_ready(data);
    if (ret)
    return ret;
    writel(MDIO_WRITE | (value << BIT_WR_DATA_OFFSET) |
    (mii_id << BIT_PHY_ADDR_OFFSET) | regnum,
    data.membase + MDIO_RWCTRL);
    return hisi_femac_mdio_wait_ready(data);
    }
#[no_mangle]
unsafe extern "C" fn hisi_femac_mdio_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_femac_mdio_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct mii_bus *bus;
    struct hisi_femac_mdio_data *data;
    int ret;
    bus = mdiobus_alloc_size(sizeof(*data));
    if (!bus)
    return -ENOMEM;
    bus.name = "hisi_femac_mii_bus";
    bus.read = &hisi_femac_mdio_read;
    bus.write = &hisi_femac_mdio_write;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s", pdev.name);
    bus.parent = &pdev.dev;
    data = bus.priv;
    data.membase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(data.membase)) {
    ret = PTR_ERR(data.membase);
    goto err_out_free_mdiobus;
    }
    data.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(data.clk)) {
    ret = PTR_ERR(data.clk);
    goto err_out_free_mdiobus;
    }
    ret = clk_prepare_enable(data.clk);
    if (ret)
    goto err_out_free_mdiobus;
    ret = of_mdiobus_register(bus, np);
    if (ret)
    goto err_out_disable_clk;
    platform_set_drvdata(pdev, bus);
    return 0;
    err_out_disable_clk:
    clk_disable_unprepare(data.clk);
    err_out_free_mdiobus:
    mdiobus_free(bus);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_femac_mdio_remove(pdev: *mut platform_device) {
    static void hisi_femac_mdio_remove(struct platform_device *pdev)
    {
    struct mii_bus *bus = platform_get_drvdata(pdev);
    struct hisi_femac_mdio_data *data = bus.priv;
    mdiobus_unregister(bus);
    clk_disable_unprepare(data.clk);
    mdiobus_free(bus);
    }
    static const struct of_device_id hisi_femac_mdio_dt_ids[] = {
    { .compatible = "hisilicon,hisi-femac-mdio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, hisi_femac_mdio_dt_ids);
    static struct platform_driver hisi_femac_mdio_driver = {
    .probe = hisi_femac_mdio_probe,
    .remove = hisi_femac_mdio_remove,
    .driver = {
    .name = "hisi-femac-mdio",
    .of_match_table = hisi_femac_mdio_dt_ids,
    },
    };
    module_platform_driver(hisi_femac_mdio_driver);
    MODULE_DESCRIPTION("Hisilicon Fast Ethernet MAC MDIO interface driver");
    MODULE_AUTHOR("Dongpo Li <lidongpo@hisilicon.com>");
    MODULE_LICENSE("GPL");
