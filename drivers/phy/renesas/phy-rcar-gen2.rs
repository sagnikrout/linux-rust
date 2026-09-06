//! Automatically rewritten from C to Rust
//! Source: drivers/phy/renesas/phy-rcar-gen2.c
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
// Renesas R-Car Gen2 PHY driver
//
// Copyright (C) 2014 Renesas Solutions Corp.
// Copyright (C) 2014 Cogent Embedded, Inc.
// Copyright (C) 2019 Renesas Electronics Corp.
//

pub const USBHS_LPSTS: c_uint = 0x02;
pub const USBHS_UGCTRL: c_uint = 0x80;
pub const USBHS_UGCTRL2: c_uint = 0x84;
pub const USBHS_UGSTS: c_uint = 0x88	/* From technical update */;
// Low Power Status register (LPSTS)
pub const USBHS_LPSTS_SUSPM: c_uint = 0x4000;
// USB General control register (UGCTRL)
pub const USBHS_UGCTRL_CONNECT: c_uint = 0x00000004;
pub const USBHS_UGCTRL_PLLRESET: c_uint = 0x00000001;
// USB General control register 2 (UGCTRL2)
pub const USBHS_UGCTRL2_USB2SEL: c_uint = 0x80000000;
pub const USBHS_UGCTRL2_USB2SEL_PCI: c_uint = 0x00000000;
pub const USBHS_UGCTRL2_USB2SEL_USB30: c_uint = 0x80000000;
pub const USBHS_UGCTRL2_USB0SEL: c_uint = 0x00000030;
pub const USBHS_UGCTRL2_USB0SEL_PCI: c_uint = 0x00000010;
pub const USBHS_UGCTRL2_USB0SEL_HS_USB: c_uint = 0x00000030;
pub const USBHS_UGCTRL2_USB0SEL_USB20: c_uint = 0x00000010;
pub const USBHS_UGCTRL2_USB0SEL_HS_USB20: c_uint = 0x00000020;
// USB General status register (UGSTS)
pub const USBHS_UGSTS_LOCK: c_uint = 0x00000100 /* From technical update */;
pub const PHYS_PER_CHANNEL: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_gen2_phy {
    pub phy: *mut phy,
    pub channel: *mut rcar_gen2_channel,
    pub number: c_int,
    pub select_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_gen2_channel {
    pub of_node: *mut device_node,
    pub drv: *mut rcar_gen2_phy_driver,
    pub phys: [rcar_gen2_phy; PHYS_PER_CHANNEL],
    pub selected_phy: c_int,
    pub select_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_gen2_phy_driver {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub lock: spinlock_t,
    pub num_channels: c_int,
    pub channels: *mut rcar_gen2_channel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_gen2_phy_data {
    pub gen2_phy_ops: *const phy_ops,
    pub (*select_value)[PHYS_PER_CHANNEL]: *const u32,
    pub num_channels: u32,
}

#[no_mangle]
unsafe extern "C" fn rcar_gen2_phy_init(p: *mut phy) -> c_int {
    static int rcar_gen2_phy_init(struct phy *p)
    {
    struct rcar_gen2_phy *phy = phy_get_drvdata(p);
    struct rcar_gen2_channel *channel = phy.channel;
    struct rcar_gen2_phy_driver *drv = channel.drv;
    unsigned long flags;
    u32 ugctrl2;
//
// Try to acquire exclusive access to PHY.  The first driver calling
// phy_init()  on a given channel wins, and all attempts  to use another
// PHY on this channel will fail until phy_exit() is called by the first
// driver.   Achieving this with cmpxchg() should be SMP-safe.
//
    if (cmpxchg(&channel.selected_phy, -1, phy.number) != -1)
    return -EBUSY;
    clk_prepare_enable(drv.clk);
    spin_lock_irqsave(&drv.lock, flags);
    ugctrl2 = readl(drv.base + USBHS_UGCTRL2);
    ugctrl2 &= ~channel.select_mask;
    ugctrl2 |= phy.select_value;
    writel(ugctrl2, drv.base + USBHS_UGCTRL2);
    spin_unlock_irqrestore(&drv.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcar_gen2_phy_exit(p: *mut phy) -> c_int {
    static int rcar_gen2_phy_exit(struct phy *p)
    {
    struct rcar_gen2_phy *phy = phy_get_drvdata(p);
    struct rcar_gen2_channel *channel = phy.channel;
    clk_disable_unprepare(channel.drv.clk);
    channel.selected_phy = -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcar_gen2_phy_power_on(p: *mut phy) -> c_int {
    static int rcar_gen2_phy_power_on(struct phy *p)
    {
    struct rcar_gen2_phy *phy = phy_get_drvdata(p);
    struct rcar_gen2_phy_driver *drv = phy.channel.drv;
    void __iomem *base = drv.base;
    unsigned long flags;
    u32 value;
    let mut err: c_int = 0, i;
// Skip if it's not USBHS
    if (phy.select_value != USBHS_UGCTRL2_USB0SEL_HS_USB)
    return 0;
    spin_lock_irqsave(&drv.lock, flags);
// Power on USBHS PHY
    value = readl(base + USBHS_UGCTRL);
    value &= ~USBHS_UGCTRL_PLLRESET;
    writel(value, base + USBHS_UGCTRL);
    value = readw(base + USBHS_LPSTS);
    value |= USBHS_LPSTS_SUSPM;
    writew(value, base + USBHS_LPSTS);
    for (i = 0; i < 20; i++) {
    value = readl(base + USBHS_UGSTS);
    if ((value & USBHS_UGSTS_LOCK) == USBHS_UGSTS_LOCK) {
    value = readl(base + USBHS_UGCTRL);
    value |= USBHS_UGCTRL_CONNECT;
    writel(value, base + USBHS_UGCTRL);
    goto out;
    }
    udelay(1);
    }
// Timed out waiting for the PLL lock
    err = -ETIMEDOUT;
    out:
    spin_unlock_irqrestore(&drv.lock, flags);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rcar_gen2_phy_power_off(p: *mut phy) -> c_int {
    static int rcar_gen2_phy_power_off(struct phy *p)
    {
    struct rcar_gen2_phy *phy = phy_get_drvdata(p);
    struct rcar_gen2_phy_driver *drv = phy.channel.drv;
    void __iomem *base = drv.base;
    unsigned long flags;
    u32 value;
// Skip if it's not USBHS
    if (phy.select_value != USBHS_UGCTRL2_USB0SEL_HS_USB)
    return 0;
    spin_lock_irqsave(&drv.lock, flags);
// Power off USBHS PHY
    value = readl(base + USBHS_UGCTRL);
    value &= ~USBHS_UGCTRL_CONNECT;
    writel(value, base + USBHS_UGCTRL);
    value = readw(base + USBHS_LPSTS);
    value &= ~USBHS_LPSTS_SUSPM;
    writew(value, base + USBHS_LPSTS);
    value = readl(base + USBHS_UGCTRL);
    value |= USBHS_UGCTRL_PLLRESET;
    writel(value, base + USBHS_UGCTRL);
    spin_unlock_irqrestore(&drv.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rz_g1c_phy_power_on(p: *mut phy) -> c_int {
    static int rz_g1c_phy_power_on(struct phy *p)
    {
    struct rcar_gen2_phy *phy = phy_get_drvdata(p);
    struct rcar_gen2_phy_driver *drv = phy.channel.drv;
    void __iomem *base = drv.base;
    unsigned long flags;
    u32 value;
    spin_lock_irqsave(&drv.lock, flags);
// Power on USBHS PHY
    value = readl(base + USBHS_UGCTRL);
    value &= ~USBHS_UGCTRL_PLLRESET;
    writel(value, base + USBHS_UGCTRL);
// As per the data sheet wait 340 micro sec for power stable
    udelay(340);
    if (phy.select_value == USBHS_UGCTRL2_USB0SEL_HS_USB20) {
    value = readw(base + USBHS_LPSTS);
    value |= USBHS_LPSTS_SUSPM;
    writew(value, base + USBHS_LPSTS);
    }
    spin_unlock_irqrestore(&drv.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rz_g1c_phy_power_off(p: *mut phy) -> c_int {
    static int rz_g1c_phy_power_off(struct phy *p)
    {
    struct rcar_gen2_phy *phy = phy_get_drvdata(p);
    struct rcar_gen2_phy_driver *drv = phy.channel.drv;
    void __iomem *base = drv.base;
    unsigned long flags;
    u32 value;
    spin_lock_irqsave(&drv.lock, flags);
// Power off USBHS PHY
    if (phy.select_value == USBHS_UGCTRL2_USB0SEL_HS_USB20) {
    value = readw(base + USBHS_LPSTS);
    value &= ~USBHS_LPSTS_SUSPM;
    writew(value, base + USBHS_LPSTS);
    }
    value = readl(base + USBHS_UGCTRL);
    value |= USBHS_UGCTRL_PLLRESET;
    writel(value, base + USBHS_UGCTRL);
    spin_unlock_irqrestore(&drv.lock, flags);
    return 0;
    }
    static const struct phy_ops rcar_gen2_phy_ops = {
    .init		= rcar_gen2_phy_init,
    .exit		= rcar_gen2_phy_exit,
    .power_on	= rcar_gen2_phy_power_on,
    .power_off	= rcar_gen2_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static const struct phy_ops rz_g1c_phy_ops = {
    .init		= rcar_gen2_phy_init,
    .exit		= rcar_gen2_phy_exit,
    .power_on	= rz_g1c_phy_power_on,
    .power_off	= rz_g1c_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static const u32 pci_select_value[][PHYS_PER_CHANNEL] = {
    [0]	= { USBHS_UGCTRL2_USB0SEL_PCI, USBHS_UGCTRL2_USB0SEL_HS_USB },
    [2]	= { USBHS_UGCTRL2_USB2SEL_PCI, USBHS_UGCTRL2_USB2SEL_USB30 },
    };
    static const u32 usb20_select_value[][PHYS_PER_CHANNEL] = {
    { USBHS_UGCTRL2_USB0SEL_USB20, USBHS_UGCTRL2_USB0SEL_HS_USB20 },
    };
    static const struct rcar_gen2_phy_data rcar_gen2_usb_phy_data = {
    .gen2_phy_ops = &rcar_gen2_phy_ops,
    .select_value = pci_select_value,
    .num_channels = ARRAY_SIZE(pci_select_value),
    };
    static const struct rcar_gen2_phy_data rz_g1c_usb_phy_data = {
    .gen2_phy_ops = &rz_g1c_phy_ops,
    .select_value = usb20_select_value,
    .num_channels = ARRAY_SIZE(usb20_select_value),
    };
    static const struct of_device_id rcar_gen2_phy_match_table[] = {
    {
    .compatible = "renesas,usb-phy-r8a77470",
    .data = &rz_g1c_usb_phy_data,
    },
    {
    .compatible = "renesas,usb-phy-r8a7790",
    .data = &rcar_gen2_usb_phy_data,
    },
    {
    .compatible = "renesas,usb-phy-r8a7791",
    .data = &rcar_gen2_usb_phy_data,
    },
    {
    .compatible = "renesas,usb-phy-r8a7794",
    .data = &rcar_gen2_usb_phy_data,
    },
    {
    .compatible = "renesas,rcar-gen2-usb-phy",
    .data = &rcar_gen2_usb_phy_data,
    },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, rcar_gen2_phy_match_table);
    static struct phy *rcar_gen2_phy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct rcar_gen2_phy_driver *drv;
    struct device_node *np = args.np;
    int i;
    drv = dev_get_drvdata(dev);
    if (!drv)
    return ERR_PTR(-EINVAL);
    for (i = 0; i < drv.num_channels; i++) {
    if (np == drv.channels[i].of_node)
    break;
    }
    if (i >= drv.num_channels || args.args[0] >= 2)
    return ERR_PTR(-ENODEV);
    return drv.channels[i].phys[args.args[0]].phy;
    }
    static const u32 select_mask[] = {
    [0]	= USBHS_UGCTRL2_USB0SEL,
    [2]	= USBHS_UGCTRL2_USB2SEL,
    };
#[no_mangle]
unsafe extern "C" fn rcar_gen2_phy_probe(pdev: *mut platform_device) -> c_int {
    static int rcar_gen2_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rcar_gen2_phy_driver *drv;
    struct phy_provider *provider;
    void __iomem *base;
    struct clk *clk;
    const struct rcar_gen2_phy_data *data;
    let mut i: c_int = 0;
    if (!dev.of_node) {
    dev_err(dev,
    "This driver is required to be instantiated from device tree\n");
    return -EINVAL;
    }
    clk = devm_clk_get(dev, "usbhs");
    if (IS_ERR(clk)) {
    dev_err(dev, "Can't get USBHS clock\n");
    return PTR_ERR(clk);
    }
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    drv = devm_kzalloc(dev, sizeof(*drv), GFP_KERNEL);
    if (!drv)
    return -ENOMEM;
    spin_lock_init(&drv.lock);
    drv.clk = clk;
    drv.base = base;
    data = of_device_get_match_data(dev);
    if (!data)
    return -EINVAL;
    drv.num_channels = of_get_child_count(dev.of_node);
    drv.channels = devm_kcalloc(dev, drv.num_channels,
    sizeof(struct rcar_gen2_channel),
    GFP_KERNEL);
    if (!drv.channels)
    return -ENOMEM;
    for_each_child_of_node_scoped(dev.of_node, np) {
    struct rcar_gen2_channel *channel = drv.channels + i;
    u32 channel_num;
    int error, n;
    channel.of_node = np;
    channel.drv = drv;
    channel.selected_phy = -1;
    error = of_property_read_u32(np, "reg", &channel_num);
    if (error || channel_num >= data.num_channels) {
    dev_err(dev, "Invalid \"reg\" property\n");
    return error;
    }
    channel.select_mask = select_mask[channel_num];
    for (n = 0; n < PHYS_PER_CHANNEL; n++) {
    struct rcar_gen2_phy *phy = &channel.phys[n];
    phy.channel = channel;
    phy.number = n;
    phy.select_value = data.select_value[channel_num][n];
    phy.phy = devm_phy_create(dev, core::ptr::null_mut(),
    data.gen2_phy_ops);
    if (IS_ERR(phy.phy)) {
    dev_err(dev, "Failed to create PHY\n");
    return PTR_ERR(phy.phy);
    }
    phy_set_drvdata(phy.phy, phy);
    }
    i++;
    }
    provider = devm_of_phy_provider_register(dev, rcar_gen2_phy_xlate);
    if (IS_ERR(provider)) {
    dev_err(dev, "Failed to register PHY provider\n");
    return PTR_ERR(provider);
    }
    dev_set_drvdata(dev, drv);
    return 0;
    }
    static struct platform_driver rcar_gen2_phy_driver = {
    .driver = {
    .name		= "phy_rcar_gen2",
    .of_match_table	= rcar_gen2_phy_match_table,
    },
    .probe	= rcar_gen2_phy_probe,
    };
    module_platform_driver(rcar_gen2_phy_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Renesas R-Car Gen2 PHY");
    MODULE_AUTHOR("Sergei Shtylyov <sergei.shtylyov@cogentembedded.com>");
