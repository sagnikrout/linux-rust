//! Automatically rewritten from C to Rust
//! Source: drivers/phy/renesas/r8a779f0-ether-serdes.c
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
// Renesas Ethernet SERDES device driver
//
// Copyright (C) 2022-2025 Renesas Electronics Corporation
//

pub const R8A779F0_ETH_SERDES_NUM: c_int = 3;
pub const R8A779F0_ETH_SERDES_OFFSET: c_uint = 0x0400;
pub const R8A779F0_ETH_SERDES_BANK_SELECT: c_uint = 0x03fc;
pub const R8A779F0_ETH_SERDES_TIMEOUT_US: c_int = 100000;
pub const R8A779F0_ETH_SERDES_NUM_RETRY_LINKUP: c_int = 3;
    struct r8a779f0_eth_serdes_drv_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a779f0_eth_serdes_channel {
    pub dd: *mut r8a779f0_eth_serdes_drv_data,
    pub phy: *mut phy,
    pub addr: *mut void __iomem,
    pub phy_interface: phy_interface_t,
    pub speed: c_int,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r8a779f0_eth_serdes_drv_data {
    pub addr: *mut void __iomem,
    pub pdev: *mut platform_device,
    pub reset: *mut reset_control,
    pub channel: [r8a779f0_eth_serdes_channel; R8A779F0_ETH_SERDES_NUM],
    pub initialized: bool,
}

//
// The datasheet describes initialization procedure without any information
// about registers' name/bits. So, this is all black magic to initialize
// the hardware.
//
#[no_mangle]
unsafe extern "C" fn r8a779f0_eth_serdes_write32(addr: *mut void __iomem, offs: u32, bank: u32, data: u32) {
    static void r8a779f0_eth_serdes_write32(void __iomem *addr, u32 offs, u32 bank, u32 data)
    {
    iowrite32(bank, addr + R8A779F0_ETH_SERDES_BANK_SELECT);
    iowrite32(data, addr + offs);
    }
#[no_mangle]
unsafe extern "C" fn r8a779f0_eth_serdes_read32(addr: *mut void __iomem, offs: u32, bank: u32) -> u32 {
    static u32 r8a779f0_eth_serdes_read32(void __iomem *addr, u32 offs,  u32 bank)
    {
    iowrite32(bank, addr + R8A779F0_ETH_SERDES_BANK_SELECT);
    return ioread32(addr + offs);
    }
    static int
    r8a779f0_eth_serdes_reg_wait(struct r8a779f0_eth_serdes_channel *channel,
    u32 offs, u32 bank, u32 mask, u32 expected)
    {
    int ret;
    u32 val;
    iowrite32(bank, channel.addr + R8A779F0_ETH_SERDES_BANK_SELECT);
    ret = readl_poll_timeout_atomic(channel.addr + offs, val,
    (val & mask) == expected,
    1, R8A779F0_ETH_SERDES_TIMEOUT_US);
    if (ret)
    dev_dbg(&channel.phy.dev,
    "%s: index %d, offs %x, bank %x, mask %x, expected %x\n",
    __func__, channel.index, offs, bank, mask, expected);
    return ret;
    }
    static int
    r8a779f0_eth_serdes_common_init_ram(struct r8a779f0_eth_serdes_drv_data *dd)
    {
    struct r8a779f0_eth_serdes_channel *channel;
    int i, ret;
    for (i = 0; i < R8A779F0_ETH_SERDES_NUM; i++) {
    channel = &dd.channel[i];
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x026c, 0x180, BIT(0), 0x01);
    if (ret)
    return ret;
    }
    r8a779f0_eth_serdes_write32(dd.addr, 0x026c, 0x180, 0x03);
    return ret;
    }
    static int
    r8a779f0_eth_serdes_common_setting(struct r8a779f0_eth_serdes_channel *channel)
    {
    struct r8a779f0_eth_serdes_drv_data *dd = channel.dd;
// Set combination mode
    r8a779f0_eth_serdes_write32(dd.addr, 0x0244, 0x180, 0x00d7);
    r8a779f0_eth_serdes_write32(dd.addr, 0x01cc, 0x180, 0xc200);
    r8a779f0_eth_serdes_write32(dd.addr, 0x01c4, 0x180, 0x0042);
    r8a779f0_eth_serdes_write32(dd.addr, 0x01c8, 0x180, 0x0000);
    r8a779f0_eth_serdes_write32(dd.addr, 0x01dc, 0x180, 0x002f);
    r8a779f0_eth_serdes_write32(dd.addr, 0x01d0, 0x180, 0x0060);
    r8a779f0_eth_serdes_write32(dd.addr, 0x01d8, 0x180, 0x2200);
    r8a779f0_eth_serdes_write32(dd.addr, 0x01d4, 0x180, 0x0000);
    r8a779f0_eth_serdes_write32(dd.addr, 0x01e0, 0x180, 0x003d);
    return 0;
    }
    static int
    r8a779f0_eth_serdes_chan_setting(struct r8a779f0_eth_serdes_channel *channel)
    {
    int ret;
    switch (channel.phy_interface) {
    case PHY_INTERFACE_MODE_SGMII:
    r8a779f0_eth_serdes_write32(channel.addr, 0x0000, 0x380, 0x2000);
    r8a779f0_eth_serdes_write32(channel.addr, 0x01c0, 0x180, 0x0011);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0248, 0x180, 0x0540);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0258, 0x180, 0x0015);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0144, 0x180, 0x0100);
    r8a779f0_eth_serdes_write32(channel.addr, 0x01a0, 0x180, 0x0000);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00d0, 0x180, 0x0002);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0150, 0x180, 0x0003);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00c8, 0x180, 0x0100);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0148, 0x180, 0x0100);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0174, 0x180, 0x0000);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0160, 0x180, 0x0007);
    r8a779f0_eth_serdes_write32(channel.addr, 0x01ac, 0x180, 0x0000);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00c4, 0x180, 0x0310);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00c8, 0x180, 0x0101);
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x00c8, 0x0180, BIT(0), 0);
    if (ret)
    return ret;
    r8a779f0_eth_serdes_write32(channel.addr, 0x0148, 0x180, 0x0101);
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x0148, 0x0180, BIT(0), 0);
    if (ret)
    return ret;
    r8a779f0_eth_serdes_write32(channel.addr, 0x00c4, 0x180, 0x1310);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00d8, 0x180, 0x1800);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00dc, 0x180, 0x0000);
    r8a779f0_eth_serdes_write32(channel.addr, 0x001c, 0x300, 0x0001);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0000, 0x380, 0x2100);
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x0000, 0x0380, BIT(8), 0);
    if (ret)
    return ret;
    if (channel.speed == 1000)
    r8a779f0_eth_serdes_write32(channel.addr, 0x0000, 0x1f00, 0x0140);
#[no_mangle]
pub unsafe extern "C" fn if(100: channel->speed ==) -> else {
    else if (channel.speed == 100)
    r8a779f0_eth_serdes_write32(channel.addr, 0x0000, 0x1f00, 0x2100);
// For AN_ON
    r8a779f0_eth_serdes_write32(channel.addr, 0x0004, 0x1f80, 0x0005);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0028, 0x1f80, 0x07a1);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0000, 0x1f80, 0x0208);
    break;
    case PHY_INTERFACE_MODE_USXGMII:
    r8a779f0_eth_serdes_write32(channel.addr, 0x001c, 0x300, 0x0000);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0014, 0x380, 0x0050);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0000, 0x380, 0x2200);
    r8a779f0_eth_serdes_write32(channel.addr, 0x001c, 0x380, 0x0400);
    r8a779f0_eth_serdes_write32(channel.addr, 0x01c0, 0x180, 0x0001);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0248, 0x180, 0x056a);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0258, 0x180, 0x0015);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0144, 0x180, 0x1100);
    r8a779f0_eth_serdes_write32(channel.addr, 0x01a0, 0x180, 0x0001);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00d0, 0x180, 0x0001);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0150, 0x180, 0x0001);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00c8, 0x180, 0x0300);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0148, 0x180, 0x0300);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0174, 0x180, 0x0000);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0160, 0x180, 0x0004);
    r8a779f0_eth_serdes_write32(channel.addr, 0x01ac, 0x180, 0x0000);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00c4, 0x180, 0x0310);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00c8, 0x180, 0x0301);
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x00c8, 0x180, BIT(0), 0);
    if (ret)
    return ret;
    r8a779f0_eth_serdes_write32(channel.addr, 0x0148, 0x180, 0x0301);
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x0148, 0x180, BIT(0), 0);
    if (ret)
    return ret;
    r8a779f0_eth_serdes_write32(channel.addr, 0x00c4, 0x180, 0x1310);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00d8, 0x180, 0x1800);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00dc, 0x180, 0x0000);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0000, 0x380, 0x2300);
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x0000, 0x380, BIT(8), 0);
    if (ret)
    return ret;
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static int
    r8a779f0_eth_serdes_chan_speed(struct r8a779f0_eth_serdes_channel *channel)
    {
    int ret;
    switch (channel.phy_interface) {
    case PHY_INTERFACE_MODE_SGMII:
// For AN_ON
    if (channel.speed == 1000)
    r8a779f0_eth_serdes_write32(channel.addr, 0x0000, 0x1f00, 0x1140);
#[no_mangle]
pub unsafe extern "C" fn if(100: channel->speed ==) -> else {
    else if (channel.speed == 100)
    r8a779f0_eth_serdes_write32(channel.addr, 0x0000, 0x1f00, 0x3100);
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x0008, 0x1f80, BIT(0), 1);
    if (ret)
    return ret;
    r8a779f0_eth_serdes_write32(channel.addr, 0x0008, 0x1f80, 0x0000);
    break;
    case PHY_INTERFACE_MODE_USXGMII:
    r8a779f0_eth_serdes_write32(channel.addr, 0x0000, 0x1f00, 0x0120);
    usleep_range(10, 20);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0000, 0x380, 0x2600);
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x0000, 0x380, BIT(10), 0);
    if (ret)
    return ret;
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn r8a779f0_eth_serdes_monitor_linkup(channel: *mut r8a779f0_eth_serdes_channel) -> c_int {
    static int r8a779f0_eth_serdes_monitor_linkup(struct r8a779f0_eth_serdes_channel *channel)
    {
    int i, ret;
    for (i = 0; i < R8A779F0_ETH_SERDES_NUM_RETRY_LINKUP; i++) {
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x0004, 0x300,
    BIT(2), BIT(2));
    if (!ret)
    break;
// restart
    r8a779f0_eth_serdes_write32(channel.addr, 0x0144, 0x180, 0x0100);
    udelay(1);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0144, 0x180, 0x0000);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn r8a779f0_eth_serdes_hw_init(channel: *mut r8a779f0_eth_serdes_channel) -> c_int {
    static int r8a779f0_eth_serdes_hw_init(struct r8a779f0_eth_serdes_channel *channel)
    {
    struct r8a779f0_eth_serdes_drv_data *dd = channel.dd;
    int i, ret;
    if (dd.initialized)
    return 0;
    reset_control_reset(dd.reset);
    usleep_range(1000, 2000);
    ret = r8a779f0_eth_serdes_common_init_ram(dd);
    if (ret)
    return ret;
    for (i = 0; i < R8A779F0_ETH_SERDES_NUM; i++) {
    ret = r8a779f0_eth_serdes_reg_wait(&dd.channel[i], 0x0000,
    0x300, BIT(15), 0);
    if (ret)
    return ret;
    }
    for (i = 0; i < R8A779F0_ETH_SERDES_NUM; i++)
    r8a779f0_eth_serdes_write32(dd.channel[i].addr, 0x03d4, 0x380, 0x0443);
    ret = r8a779f0_eth_serdes_common_setting(channel);
    if (ret)
    return ret;
    for (i = 0; i < R8A779F0_ETH_SERDES_NUM; i++)
    r8a779f0_eth_serdes_write32(dd.channel[i].addr, 0x03d0, 0x380, 0x0001);
    r8a779f0_eth_serdes_write32(dd.addr, 0x0000, 0x380, 0x8000);
    ret = r8a779f0_eth_serdes_common_init_ram(dd);
    if (ret)
    return ret;
    return r8a779f0_eth_serdes_reg_wait(&dd.channel[0], 0x0000, 0x380, BIT(15), 0);
    }
#[no_mangle]
unsafe extern "C" fn r8a779f0_eth_serdes_init(p: *mut phy) -> c_int {
    static int r8a779f0_eth_serdes_init(struct phy *p)
    {
    struct r8a779f0_eth_serdes_channel *channel = phy_get_drvdata(p);
    int ret;
    ret = r8a779f0_eth_serdes_hw_init(channel);
    if (!ret)
    channel.dd.initialized = true;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn r8a779f0_eth_serdes_exit(p: *mut phy) -> c_int {
    static int r8a779f0_eth_serdes_exit(struct phy *p)
    {
    struct r8a779f0_eth_serdes_channel *channel = phy_get_drvdata(p);
    channel.dd.initialized = false;
    return 0;
    }
    static int r8a779f0_eth_serdes_hw_init_late(struct r8a779f0_eth_serdes_channel
// channel)
    {
    int ret;
    u32 val;
    ret = r8a779f0_eth_serdes_chan_setting(channel);
    if (ret)
    return ret;
    ret = r8a779f0_eth_serdes_chan_speed(channel);
    if (ret)
    return ret;
    r8a779f0_eth_serdes_write32(channel.addr, 0x03c0, 0x380, 0x0000);
    r8a779f0_eth_serdes_write32(channel.addr, 0x03d0, 0x380, 0x0000);
    val = r8a779f0_eth_serdes_read32(channel.addr, 0x00c0, 0x180);
    r8a779f0_eth_serdes_write32(channel.addr, 0x00c0, 0x180, val | BIT(8));
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x0100, 0x180, BIT(0), 1);
    if (ret)
    return ret;
    r8a779f0_eth_serdes_write32(channel.addr, 0x00c0, 0x180, val & ~BIT(8));
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x0100, 0x180, BIT(0), 0);
    if (ret)
    return ret;
    val = r8a779f0_eth_serdes_read32(channel.addr, 0x0144, 0x180);
    r8a779f0_eth_serdes_write32(channel.addr, 0x0144, 0x180, val | BIT(4));
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x0180, 0x180, BIT(0), 1);
    if (ret)
    return ret;
    r8a779f0_eth_serdes_write32(channel.addr, 0x0144, 0x180, val & ~BIT(4));
    ret = r8a779f0_eth_serdes_reg_wait(channel, 0x0180, 0x180, BIT(0), 0);
    if (ret)
    return ret;
    return r8a779f0_eth_serdes_monitor_linkup(channel);
    }
#[no_mangle]
unsafe extern "C" fn r8a779f0_eth_serdes_power_on(p: *mut phy) -> c_int {
    static int r8a779f0_eth_serdes_power_on(struct phy *p)
    {
    struct r8a779f0_eth_serdes_channel *channel = phy_get_drvdata(p);
    return r8a779f0_eth_serdes_hw_init_late(channel);
    }
    static int r8a779f0_eth_serdes_set_mode(struct phy *p, enum phy_mode mode,
    int submode)
    {
    struct r8a779f0_eth_serdes_channel *channel = phy_get_drvdata(p);
    if (mode != PHY_MODE_ETHERNET)
    return -EOPNOTSUPP;
    switch (submode) {
    case PHY_INTERFACE_MODE_GMII:
    case PHY_INTERFACE_MODE_SGMII:
    case PHY_INTERFACE_MODE_USXGMII:
    channel.phy_interface = submode;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn r8a779f0_eth_serdes_set_speed(p: *mut phy, speed: c_int) -> c_int {
    static int r8a779f0_eth_serdes_set_speed(struct phy *p, int speed)
    {
    struct r8a779f0_eth_serdes_channel *channel = phy_get_drvdata(p);
    channel.speed = speed;
    return 0;
    }
    static const struct phy_ops r8a779f0_eth_serdes_ops = {
    .init		= r8a779f0_eth_serdes_init,
    .exit		= r8a779f0_eth_serdes_exit,
    .power_on	= r8a779f0_eth_serdes_power_on,
    .set_mode	= r8a779f0_eth_serdes_set_mode,
    .set_speed	= r8a779f0_eth_serdes_set_speed,
    };
    static struct phy *r8a779f0_eth_serdes_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct r8a779f0_eth_serdes_drv_data *dd = dev_get_drvdata(dev);
    if (args.args[0] >= R8A779F0_ETH_SERDES_NUM)
    return ERR_PTR(-ENODEV);
    return dd.channel[args.args[0]].phy;
    }
    static const struct of_device_id r8a779f0_eth_serdes_of_table[] = {
    { .compatible = "renesas,r8a779f0-ether-serdes", },
    { }
    };
    MODULE_DEVICE_TABLE(of, r8a779f0_eth_serdes_of_table);
#[no_mangle]
unsafe extern "C" fn r8a779f0_eth_serdes_probe(pdev: *mut platform_device) -> c_int {
    static int r8a779f0_eth_serdes_probe(struct platform_device *pdev)
    {
    struct r8a779f0_eth_serdes_drv_data *dd;
    struct phy_provider *provider;
    int i;
    dd = devm_kzalloc(&pdev.dev, sizeof(*dd), GFP_KERNEL);
    if (!dd)
    return -ENOMEM;
    platform_set_drvdata(pdev, dd);
    dd.pdev = pdev;
    dd.addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(dd.addr))
    return PTR_ERR(dd.addr);
    dd.reset = devm_reset_control_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(dd.reset))
    return PTR_ERR(dd.reset);
    for (i = 0; i < R8A779F0_ETH_SERDES_NUM; i++) {
    struct r8a779f0_eth_serdes_channel *channel = &dd.channel[i];
    channel.phy = devm_phy_create(&pdev.dev, core::ptr::null_mut(),
    &r8a779f0_eth_serdes_ops);
    if (IS_ERR(channel.phy))
    return PTR_ERR(channel.phy);
    channel.addr = dd.addr + R8A779F0_ETH_SERDES_OFFSET * i;
    channel.dd = dd;
    channel.index = i;
    phy_set_drvdata(channel.phy, channel);
    }
    provider = devm_of_phy_provider_register(&pdev.dev,
    r8a779f0_eth_serdes_xlate);
    if (IS_ERR(provider))
    return PTR_ERR(provider);
    pm_runtime_enable(&pdev.dev);
    pm_runtime_get_sync(&pdev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn r8a779f0_eth_serdes_remove(pdev: *mut platform_device) {
    static void r8a779f0_eth_serdes_remove(struct platform_device *pdev)
    {
    pm_runtime_put(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    }
    static struct platform_driver r8a779f0_eth_serdes_driver_platform = {
    .probe = r8a779f0_eth_serdes_probe,
    .remove = r8a779f0_eth_serdes_remove,
    .driver = {
    .name = "r8a779f0_eth_serdes",
    .of_match_table = r8a779f0_eth_serdes_of_table,
    }
    };
    module_platform_driver(r8a779f0_eth_serdes_driver_platform);
    MODULE_AUTHOR("Yoshihiro Shimoda");
    MODULE_DESCRIPTION("Renesas Ethernet SERDES device driver");
    MODULE_LICENSE("GPL");
