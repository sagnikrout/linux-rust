//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-ipq4019.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2015, The Linux Foundation. All rights reserved.
// Copyright (c) 2020 Sartura Ltd.

pub const MDIO_MODE_REG: c_uint = 0x40;

// 0 = Clause 22, 1 = Clause 45

pub const MDIO_MODE_DIV_1: c_uint = 0x0;
pub const MDIO_MODE_DIV_2: c_uint = 0x1;
pub const MDIO_MODE_DIV_4: c_uint = 0x3;
pub const MDIO_MODE_DIV_8: c_uint = 0x7;
pub const MDIO_MODE_DIV_16: c_uint = 0xf;
pub const MDIO_MODE_DIV_32: c_uint = 0x1f;
pub const MDIO_MODE_DIV_64: c_uint = 0x3f;
pub const MDIO_MODE_DIV_128: c_uint = 0x7f;
pub const MDIO_MODE_DIV_256: c_uint = 0xff;
pub const MDIO_ADDR_REG: c_uint = 0x44;
pub const MDIO_DATA_WRITE_REG: c_uint = 0x48;
pub const MDIO_DATA_READ_REG: c_uint = 0x4c;
pub const MDIO_CMD_REG: c_uint = 0x50;

pub const MDIO_CMD_ACCESS_CODE_READ: c_int = 0;
pub const MDIO_CMD_ACCESS_CODE_WRITE: c_int = 1;
pub const MDIO_CMD_ACCESS_CODE_C45_ADDR: c_int = 0;
pub const MDIO_CMD_ACCESS_CODE_C45_WRITE: c_int = 1;
pub const MDIO_CMD_ACCESS_CODE_C45_READ: c_int = 2;
pub const IPQ4019_MDIO_TIMEOUT: c_int = 10000;
pub const IPQ4019_MDIO_SLEEP: c_int = 10;
// MDIO clock source frequency is fixed to 100M
pub const IPQ_MDIO_CLK_RATE: c_int = 100000000;
pub const IPQ_PHY_SET_DELAY_US: c_int = 100000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipq4019_mdio_data {
    pub membase: *mut void __iomem,
    pub eth_ldo_rdy: *mut void __iomem,
    pub mdio_clk: *mut clk,
    pub mdc_rate: c_uint,
}

#[no_mangle]
unsafe extern "C" fn ipq4019_mdio_wait_busy(bus: *mut mii_bus) -> c_int {
    static int ipq4019_mdio_wait_busy(struct mii_bus *bus)
    {
    struct ipq4019_mdio_data *priv = bus.priv;
    unsigned int busy;
    return readl_poll_timeout(priv.membase + MDIO_CMD_REG, busy,
    (busy & MDIO_CMD_ACCESS_BUSY) == 0,
    IPQ4019_MDIO_SLEEP, IPQ4019_MDIO_TIMEOUT);
    }
    static int ipq4019_mdio_read_c45(struct mii_bus *bus, int mii_id, int mmd,
    int reg)
    {
    struct ipq4019_mdio_data *priv = bus.priv;
    unsigned int data;
    unsigned int cmd;
    if (ipq4019_mdio_wait_busy(bus))
    return -ETIMEDOUT;
    data = readl(priv.membase + MDIO_MODE_REG);
    data |= MDIO_MODE_C45;
    writel(data, priv.membase + MDIO_MODE_REG);
// issue the phy address and mmd
    writel((mii_id << 8) | mmd, priv.membase + MDIO_ADDR_REG);
// issue reg
    writel(reg, priv.membase + MDIO_DATA_WRITE_REG);
    cmd = MDIO_CMD_ACCESS_START | MDIO_CMD_ACCESS_CODE_C45_ADDR;
// issue read command
    writel(cmd, priv.membase + MDIO_CMD_REG);
// Wait read complete
    if (ipq4019_mdio_wait_busy(bus))
    return -ETIMEDOUT;
    cmd = MDIO_CMD_ACCESS_START | MDIO_CMD_ACCESS_CODE_C45_READ;
    writel(cmd, priv.membase + MDIO_CMD_REG);
    if (ipq4019_mdio_wait_busy(bus))
    return -ETIMEDOUT;
// Read and return data
    return readl(priv.membase + MDIO_DATA_READ_REG);
    }
#[no_mangle]
unsafe extern "C" fn ipq4019_mdio_read_c22(bus: *mut mii_bus, mii_id: c_int, regnum: c_int) -> c_int {
    static int ipq4019_mdio_read_c22(struct mii_bus *bus, int mii_id, int regnum)
    {
    struct ipq4019_mdio_data *priv = bus.priv;
    unsigned int data;
    unsigned int cmd;
    if (ipq4019_mdio_wait_busy(bus))
    return -ETIMEDOUT;
    data = readl(priv.membase + MDIO_MODE_REG);
    data &= ~MDIO_MODE_C45;
    writel(data, priv.membase + MDIO_MODE_REG);
// issue the phy address and reg
    writel((mii_id << 8) | regnum, priv.membase + MDIO_ADDR_REG);
    cmd = MDIO_CMD_ACCESS_START | MDIO_CMD_ACCESS_CODE_READ;
// issue read command
    writel(cmd, priv.membase + MDIO_CMD_REG);
// Wait read complete
    if (ipq4019_mdio_wait_busy(bus))
    return -ETIMEDOUT;
// Read and return data
    return readl(priv.membase + MDIO_DATA_READ_REG);
    }
    static int ipq4019_mdio_write_c45(struct mii_bus *bus, int mii_id, int mmd,
    int reg, u16 value)
    {
    struct ipq4019_mdio_data *priv = bus.priv;
    unsigned int data;
    unsigned int cmd;
    if (ipq4019_mdio_wait_busy(bus))
    return -ETIMEDOUT;
    data = readl(priv.membase + MDIO_MODE_REG);
    data |= MDIO_MODE_C45;
    writel(data, priv.membase + MDIO_MODE_REG);
// issue the phy address and mmd
    writel((mii_id << 8) | mmd, priv.membase + MDIO_ADDR_REG);
// issue reg
    writel(reg, priv.membase + MDIO_DATA_WRITE_REG);
    cmd = MDIO_CMD_ACCESS_START | MDIO_CMD_ACCESS_CODE_C45_ADDR;
    writel(cmd, priv.membase + MDIO_CMD_REG);
    if (ipq4019_mdio_wait_busy(bus))
    return -ETIMEDOUT;
// issue write data
    writel(value, priv.membase + MDIO_DATA_WRITE_REG);
    cmd = MDIO_CMD_ACCESS_START | MDIO_CMD_ACCESS_CODE_C45_WRITE;
    writel(cmd, priv.membase + MDIO_CMD_REG);
// Wait write complete
    if (ipq4019_mdio_wait_busy(bus))
    return -ETIMEDOUT;
    return 0;
    }
    static int ipq4019_mdio_write_c22(struct mii_bus *bus, int mii_id, int regnum,
    u16 value)
    {
    struct ipq4019_mdio_data *priv = bus.priv;
    unsigned int data;
    unsigned int cmd;
    if (ipq4019_mdio_wait_busy(bus))
    return -ETIMEDOUT;
// Enter Clause 22 mode
    data = readl(priv.membase + MDIO_MODE_REG);
    data &= ~MDIO_MODE_C45;
    writel(data, priv.membase + MDIO_MODE_REG);
// issue the phy address and reg
    writel((mii_id << 8) | regnum, priv.membase + MDIO_ADDR_REG);
// issue write data
    writel(value, priv.membase + MDIO_DATA_WRITE_REG);
// issue write command
    cmd = MDIO_CMD_ACCESS_START | MDIO_CMD_ACCESS_CODE_WRITE;
    writel(cmd, priv.membase + MDIO_CMD_REG);
// Wait write complete
    if (ipq4019_mdio_wait_busy(bus))
    return -ETIMEDOUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipq4019_mdio_set_div(priv: *mut ipq4019_mdio_data) -> c_int {
    static int ipq4019_mdio_set_div(struct ipq4019_mdio_data *priv)
    {
    unsigned long ahb_rate;
    int div;
    u32 val;
// If we don't have a clock for AHB use the fixed value
    ahb_rate = IPQ_MDIO_CLK_RATE;
    if (priv.mdio_clk)
    ahb_rate = clk_get_rate(priv.mdio_clk);
// MDC rate is ahb_rate/(MDIO_MODE_DIV + 1)
// While supported, internal documentation doesn't
// assure correct functionality of the MDIO bus
// with divider of 1, 2 or 4.
//
    for (div = 8; div <= 256; div *= 2) {
// The requested rate is supported by the div
    if (priv.mdc_rate == DIV_ROUND_UP(ahb_rate, div)) {
    val = readl(priv.membase + MDIO_MODE_REG);
    val &= ~MDIO_MODE_DIV_MASK;
    val |= MDIO_MODE_DIV(div);
    writel(val, priv.membase + MDIO_MODE_REG);
    return 0;
    }
    }
// The requested rate is not supported
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn ipq_mdio_reset(bus: *mut mii_bus) -> c_int {
    static int ipq_mdio_reset(struct mii_bus *bus)
    {
    struct ipq4019_mdio_data *priv = bus.priv;
    u32 val;
    int ret;
// To indicate CMN_PLL that ethernet_ldo has been ready if platform resource 1
// is specified in the device tree.
//
    if (priv.eth_ldo_rdy) {
    val = readl(priv.eth_ldo_rdy);
    val |= BIT(0);
    writel(val, priv.eth_ldo_rdy);
    fsleep(IPQ_PHY_SET_DELAY_US);
    }
// Configure MDIO clock source frequency if clock is specified in the device tree
    ret = clk_set_rate(priv.mdio_clk, IPQ_MDIO_CLK_RATE);
    if (ret)
    return ret;
    ret = clk_prepare_enable(priv.mdio_clk);
    if (ret)
    return ret;
    mdelay(10);
// Restore MDC rate
    return ipq4019_mdio_set_div(priv);
    }
    static void ipq4019_mdio_select_mdc_rate(struct platform_device *pdev,
    struct ipq4019_mdio_data *priv)
    {
    unsigned long ahb_rate;
    int div;
    u32 val;
// MDC rate defined in DT, we don't have to decide a default value
    if (!of_property_read_u32(pdev.dev.of_node, "clock-frequency",
    &priv.mdc_rate))
    return;
// If we don't have a clock for AHB use the fixed value
    ahb_rate = IPQ_MDIO_CLK_RATE;
    if (priv.mdio_clk)
    ahb_rate = clk_get_rate(priv.mdio_clk);
// Check what is the current div set
    val = readl(priv.membase + MDIO_MODE_REG);
    div = FIELD_GET(MDIO_MODE_DIV_MASK, val);
// div is not set to the default value of /256
// Probably someone changed that (bootloader, other drivers)
// Keep this and don't overwrite it.
//
    if (div != MDIO_MODE_DIV_256) {
    priv.mdc_rate = DIV_ROUND_UP(ahb_rate, div + 1);
    return;
    }
// If div is /256 assume nobody have set this value and
// try to find one MDC rate that is close the 802.3 spec of
// 2.5MHz
//
    for (div = 256; div >= 8; div /= 2) {
// Stop as soon as we found a divider that
// reached the closest value to 2.5MHz
//
    if (DIV_ROUND_UP(ahb_rate, div) > 2500000)
    break;
    priv.mdc_rate = DIV_ROUND_UP(ahb_rate, div);
    }
    }
#[no_mangle]
unsafe extern "C" fn ipq4019_mdio_probe(pdev: *mut platform_device) -> c_int {
    static int ipq4019_mdio_probe(struct platform_device *pdev)
    {
    struct ipq4019_mdio_data *priv;
    struct mii_bus *bus;
    struct resource *res;
    int ret;
    bus = devm_mdiobus_alloc_size(&pdev.dev, sizeof(*priv));
    if (!bus)
    return -ENOMEM;
    priv = bus.priv;
    priv.membase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.membase))
    return PTR_ERR(priv.membase);
    priv.mdio_clk = devm_clk_get_optional(&pdev.dev, "gcc_mdio_ahb_clk");
    if (IS_ERR(priv.mdio_clk))
    return PTR_ERR(priv.mdio_clk);
    ipq4019_mdio_select_mdc_rate(pdev, priv);
    ret = ipq4019_mdio_set_div(priv);
    if (ret)
    return ret;
// The platform resource is provided on the chipset IPQ5018
// This resource is optional
    res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if (res) {
    priv.eth_ldo_rdy = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(priv.eth_ldo_rdy))
    return PTR_ERR(priv.eth_ldo_rdy);
    }
    bus.name = "ipq4019_mdio";
    bus.read = ipq4019_mdio_read_c22;
    bus.write = ipq4019_mdio_write_c22;
    bus.read_c45 = ipq4019_mdio_read_c45;
    bus.write_c45 = ipq4019_mdio_write_c45;
    bus.reset = ipq_mdio_reset;
    bus.parent = &pdev.dev;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s%d", pdev.name, pdev.id);
    ret = of_mdiobus_register(bus, pdev.dev.of_node);
    if (ret) {
    dev_err(&pdev.dev, "Cannot register MDIO bus!\n");
    return ret;
    }
    platform_set_drvdata(pdev, bus);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipq4019_mdio_remove(pdev: *mut platform_device) {
    static void ipq4019_mdio_remove(struct platform_device *pdev)
    {
    struct mii_bus *bus = platform_get_drvdata(pdev);
    mdiobus_unregister(bus);
    }
    static const struct of_device_id ipq4019_mdio_dt_ids[] = {
    { .compatible = "qcom,ipq4019-mdio" },
    { .compatible = "qcom,ipq5018-mdio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ipq4019_mdio_dt_ids);
    static struct platform_driver ipq4019_mdio_driver = {
    .probe = ipq4019_mdio_probe,
    .remove = ipq4019_mdio_remove,
    .driver = {
    .name = "ipq4019-mdio",
    .of_match_table = ipq4019_mdio_dt_ids,
    },
    };
    module_platform_driver(ipq4019_mdio_driver);
    MODULE_DESCRIPTION("ipq4019 MDIO interface driver");
    MODULE_AUTHOR("Qualcomm Atheros");
    MODULE_LICENSE("Dual BSD/GPL");
