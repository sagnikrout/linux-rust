//! Automatically rewritten from C to Rust
//! Source: drivers/phy/ti/phy-omap-usb2.c
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
// omap-usb2.c - USB PHY, talking to USB controller on TI SoCs.
//
// Copyright (C) 2012-2020 Texas Instruments Incorporated - http://www.ti.com
// Author: Kishon Vijay Abraham I <kishon@ti.com>
//

pub const USB2PHY_ANA_CONFIG1: c_uint = 0x4c;

pub const USB2PHY_CHRG_DET: c_uint = 0x14;

// SoC Specific USB2_OTG register definitions

// Driver Flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_usb {
    pub phy: usb_phy,
    pub comparator: *mut phy_companion,
    pub pll_ctrl_base: *mut void __iomem,
    pub phy_base: *mut void __iomem,
    pub dev: *mut device,
    pub control_dev: *mut device,
    pub wkupclk: *mut clk,
    pub optclk: *mut clk,
    pub flags: u8,
    pub /: *mut *mut *mut regmap syscon_phy_power; / ctrl. reg. acces,
    pub /: *mut *mut unsigned int power_reg; / power reg. index within syscon,
    pub mask: u32,
    pub power_on: u32,
    pub power_off: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_phy_data {
    pub label: *const c_char,
    pub flags: u8,
    pub mask: u32,
    pub power_on: u32,
    pub power_off: u32,
}

#[no_mangle]
pub unsafe extern "C" fn omap_usb_readl(addr: *mut void __iomem, offset: c_uint) -> u32 {
    static inline u32 omap_usb_readl(void __iomem *addr, unsigned int offset)
    {
    return __raw_readl(addr + offset);
    }
    static inline void omap_usb_writel(void __iomem *addr, unsigned int offset,
    u32 data)
    {
    __raw_writel(data, addr + offset);
    }
//
// omap_usb2_set_comparator() - links the comparator present in the system with this phy
//
// @comparator:  the companion phy(comparator) for this phy
//
// The phy companion driver should call this API passing the phy_companion
// filled with set_vbus and start_srp to be used by usb phy.
//
// For use by phy companion driver
//
#[no_mangle]
pub unsafe extern "C" fn omap_usb2_set_comparator(comparator: *mut phy_companion) -> c_int {
    int omap_usb2_set_comparator(struct phy_companion *comparator)
    {
    struct omap_usb	*phy;
    struct usb_phy	*x = usb_get_phy(USB_PHY_TYPE_USB2);
    if (IS_ERR(x))
    return -ENODEV;
    phy = phy_to_omapusb(x);
    phy.comparator = comparator;
    return 0;
    }
    EXPORT_SYMBOL_GPL(omap_usb2_set_comparator);
#[no_mangle]
unsafe extern "C" fn omap_usb_set_vbus(otg: *mut usb_otg, enabled: bool) -> c_int {
    static int omap_usb_set_vbus(struct usb_otg *otg, bool enabled)
    {
    struct omap_usb *phy = phy_to_omapusb(otg.usb_phy);
    if (!phy.comparator || !phy.comparator.set_vbus)
    return -ENODEV;
    return phy.comparator.set_vbus(phy.comparator, enabled);
    }
#[no_mangle]
unsafe extern "C" fn omap_usb_start_srp(otg: *mut usb_otg) -> c_int {
    static int omap_usb_start_srp(struct usb_otg *otg)
    {
    struct omap_usb *phy = phy_to_omapusb(otg.usb_phy);
    if (!phy.comparator || !phy.comparator.start_srp)
    return -ENODEV;
    return phy.comparator.start_srp(phy.comparator);
    }
#[no_mangle]
unsafe extern "C" fn omap_usb_set_host(otg: *mut usb_otg, host: *mut usb_bus) -> c_int {
    static int omap_usb_set_host(struct usb_otg *otg, struct usb_bus *host)
    {
    otg.host = host;
    if (!host)
    otg.state = OTG_STATE_UNDEFINED;
    return 0;
    }
    static int omap_usb_set_peripheral(struct usb_otg *otg,
    struct usb_gadget *gadget)
    {
    otg.gadget = gadget;
    if (!gadget)
    otg.state = OTG_STATE_UNDEFINED;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_usb_phy_power(phy: *mut omap_usb, on: c_int) -> c_int {
    static int omap_usb_phy_power(struct omap_usb *phy, int on)
    {
    u32 val;
    int ret;
    if (!phy.syscon_phy_power) {
    omap_control_phy_power(phy.control_dev, on);
    return 0;
    }
    if (on)
    val = phy.power_on;
    else
    val = phy.power_off;
    ret = regmap_update_bits(phy.syscon_phy_power, phy.power_reg,
    phy.mask, val);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn omap_usb_power_off(x: *mut phy) -> c_int {
    static int omap_usb_power_off(struct phy *x)
    {
    struct omap_usb *phy = phy_get_drvdata(x);
    return omap_usb_phy_power(phy, false);
    }
#[no_mangle]
unsafe extern "C" fn omap_usb_power_on(x: *mut phy) -> c_int {
    static int omap_usb_power_on(struct phy *x)
    {
    struct omap_usb *phy = phy_get_drvdata(x);
    return omap_usb_phy_power(phy, true);
    }
#[no_mangle]
unsafe extern "C" fn omap_usb2_disable_clocks(phy: *mut omap_usb) -> c_int {
    static int omap_usb2_disable_clocks(struct omap_usb *phy)
    {
    clk_disable_unprepare(phy.wkupclk);
    if (!IS_ERR(phy.optclk))
    clk_disable_unprepare(phy.optclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_usb2_enable_clocks(phy: *mut omap_usb) -> c_int {
    static int omap_usb2_enable_clocks(struct omap_usb *phy)
    {
    int ret;
    ret = clk_prepare_enable(phy.wkupclk);
    if (ret < 0) {
    dev_err(phy.dev, "Failed to enable wkupclk %d\n", ret);
    goto err0;
    }
    if (!IS_ERR(phy.optclk)) {
    ret = clk_prepare_enable(phy.optclk);
    if (ret < 0) {
    dev_err(phy.dev, "Failed to enable optclk %d\n", ret);
    goto err1;
    }
    }
    return 0;
    err1:
    clk_disable_unprepare(phy.wkupclk);
    err0:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn omap_usb_init(x: *mut phy) -> c_int {
    static int omap_usb_init(struct phy *x)
    {
    struct omap_usb *phy = phy_get_drvdata(x);
    u32 val;
    omap_usb2_enable_clocks(phy);
    if (phy.flags & OMAP_USB2_CALIBRATE_FALSE_DISCONNECT) {
//
// Reduce the sensitivity of internal PHY by enabling the
// DISCON_BYP_LATCH of the USB2PHY_ANA_CONFIG1 register. This
// resolves issues with certain devices which can otherwise
// be prone to false disconnects.
//
    val = omap_usb_readl(phy.phy_base, USB2PHY_ANA_CONFIG1);
    val |= USB2PHY_DISCON_BYP_LATCH;
    omap_usb_writel(phy.phy_base, USB2PHY_ANA_CONFIG1, val);
    }
    if (phy.flags & OMAP_USB2_DISABLE_CHRG_DET) {
    val = omap_usb_readl(phy.phy_base, USB2PHY_CHRG_DET);
    val |= USB2PHY_CHRG_DET_USE_CHG_DET_REG |
    USB2PHY_CHRG_DET_DIS_CHG_DET;
    omap_usb_writel(phy.phy_base, USB2PHY_CHRG_DET, val);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_usb_exit(x: *mut phy) -> c_int {
    static int omap_usb_exit(struct phy *x)
    {
    struct omap_usb *phy = phy_get_drvdata(x);
    return omap_usb2_disable_clocks(phy);
    }
    static const struct phy_ops ops = {
    .init		= omap_usb_init,
    .exit		= omap_usb_exit,
    .power_on	= omap_usb_power_on,
    .power_off	= omap_usb_power_off,
    .owner		= THIS_MODULE,
    };
    static const struct usb_phy_data omap_usb2_data = {
    .label = "omap_usb2",
    .flags = OMAP_USB2_HAS_START_SRP | OMAP_USB2_HAS_SET_VBUS,
    .mask = OMAP_DEV_PHY_PD,
    .power_off = OMAP_DEV_PHY_PD,
    };
    static const struct usb_phy_data omap5_usb2_data = {
    .label = "omap5_usb2",
    .flags = 0,
    .mask = OMAP_DEV_PHY_PD,
    .power_off = OMAP_DEV_PHY_PD,
    };
    static const struct usb_phy_data dra7x_usb2_data = {
    .label = "dra7x_usb2",
    .flags = OMAP_USB2_CALIBRATE_FALSE_DISCONNECT,
    .mask = OMAP_DEV_PHY_PD,
    .power_off = OMAP_DEV_PHY_PD,
    };
    static const struct usb_phy_data dra7x_usb2_phy2_data = {
    .label = "dra7x_usb2_phy2",
    .flags = OMAP_USB2_CALIBRATE_FALSE_DISCONNECT,
    .mask = OMAP_USB2_PHY_PD,
    .power_off = OMAP_USB2_PHY_PD,
    };
    static const struct usb_phy_data am437x_usb2_data = {
    .label = "am437x_usb2",
    .flags =  0,
    .mask = AM437X_USB2_PHY_PD | AM437X_USB2_OTG_PD |
    AM437X_USB2_OTGVDET_EN | AM437X_USB2_OTGSESSEND_EN,
    .power_on = AM437X_USB2_OTGVDET_EN | AM437X_USB2_OTGSESSEND_EN,
    .power_off = AM437X_USB2_PHY_PD | AM437X_USB2_OTG_PD,
    };
    static const struct usb_phy_data am654_usb2_data = {
    .label = "am654_usb2",
    .flags = OMAP_USB2_CALIBRATE_FALSE_DISCONNECT,
    .mask = AM654_USB2_OTG_PD | AM654_USB2_VBUS_DET_EN |
    AM654_USB2_VBUSVALID_DET_EN,
    .power_on = AM654_USB2_VBUS_DET_EN | AM654_USB2_VBUSVALID_DET_EN,
    .power_off = AM654_USB2_OTG_PD,
    };
    static const struct of_device_id omap_usb2_id_table[] = {
    {
    .compatible = "ti,omap-usb2",
    .data = &omap_usb2_data,
    },
    {
    .compatible = "ti,omap5-usb2",
    .data = &omap5_usb2_data,
    },
    {
    .compatible = "ti,dra7x-usb2",
    .data = &dra7x_usb2_data,
    },
    {
    .compatible = "ti,dra7x-usb2-phy2",
    .data = &dra7x_usb2_phy2_data,
    },
    {
    .compatible = "ti,am437x-usb2",
    .data = &am437x_usb2_data,
    },
    {
    .compatible = "ti,am654-usb2",
    .data = &am654_usb2_data,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, omap_usb2_id_table);
#[no_mangle]
unsafe extern "C" fn omap_usb2_init_errata(phy: *mut omap_usb) {
    static void omap_usb2_init_errata(struct omap_usb *phy)
    {
    static const struct soc_device_attribute am65x_sr10_soc_devices[] = {
    { .family = "AM65X", .revision = "SR1.0" },
    { /* sentinel */ }
    };
//
// Errata i2075: USB2PHY: USB2PHY Charger Detect is Enabled by
// Default Without VBUS Presence.
//
// AM654x SR1.0 has a silicon bug due to which D+ is pulled high after
// POR, which could cause enumeration failure with some USB hubs.
// Disabling the USB2_PHY Charger Detect function will put D+
// into the normal state.
//
    if (soc_device_match(am65x_sr10_soc_devices))
    phy.flags |= OMAP_USB2_DISABLE_CHRG_DET;
    }
#[no_mangle]
unsafe extern "C" fn omap_usb2_put_device(_dev: *mut c_void) {
    static void omap_usb2_put_device(void *_dev)
    {
    struct device *dev = _dev;
    put_device(dev);
    }
#[no_mangle]
unsafe extern "C" fn omap_usb2_probe(pdev: *mut platform_device) -> c_int {
    static int omap_usb2_probe(struct platform_device *pdev)
    {
    struct omap_usb	*phy;
    struct phy *generic_phy;
    struct phy_provider *phy_provider;
    struct usb_otg *otg;
    struct device_node *node = pdev.dev.of_node;
    struct device_node *control_node;
    struct platform_device *control_pdev;
    const struct usb_phy_data *phy_data;
    int ret;
    phy_data = device_get_match_data(&pdev.dev);
    if (!phy_data)
    return -EINVAL;
    phy = devm_kzalloc(&pdev.dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    otg = devm_kzalloc(&pdev.dev, sizeof(*otg), GFP_KERNEL);
    if (!otg)
    return -ENOMEM;
    phy.dev		= &pdev.dev;
    phy.phy.dev		= phy.dev;
    phy.phy.label		= phy_data.label;
    phy.phy.otg		= otg;
    phy.phy.type		= USB_PHY_TYPE_USB2;
    phy.mask		= phy_data.mask;
    phy.power_on		= phy_data.power_on;
    phy.power_off		= phy_data.power_off;
    phy.flags		= phy_data.flags;
    omap_usb2_init_errata(phy);
    phy.phy_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(phy.phy_base))
    return PTR_ERR(phy.phy_base);
    phy.syscon_phy_power = syscon_regmap_lookup_by_phandle(node,
    "syscon-phy-power");
    if (IS_ERR(phy.syscon_phy_power)) {
    dev_dbg(&pdev.dev,
    "can't get syscon-phy-power, using control device\n");
    phy.syscon_phy_power = core::ptr::null_mut();
    control_node = of_parse_phandle(node, "ctrl-module", 0);
    if (!control_node) {
    dev_err(&pdev.dev,
    "Failed to get control device phandle\n");
    return -EINVAL;
    }
    control_pdev = of_find_device_by_node(control_node);
    if (!control_pdev) {
    dev_err(&pdev.dev, "Failed to get control device\n");
    return -EINVAL;
    }
    phy.control_dev = &control_pdev.dev;
    ret = devm_add_action_or_reset(&pdev.dev, omap_usb2_put_device,
    phy.control_dev);
    if (ret)
    return ret;
    } else {
    if (of_property_read_u32_index(node,
    "syscon-phy-power", 1,
    &phy.power_reg)) {
    dev_err(&pdev.dev,
    "couldn't get power reg. offset\n");
    return -EINVAL;
    }
    }
    phy.wkupclk = devm_clk_get(phy.dev, "wkupclk");
    if (IS_ERR(phy.wkupclk)) {
    if (PTR_ERR(phy.wkupclk) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    dev_warn(&pdev.dev, "unable to get wkupclk %ld, trying old name\n",
    PTR_ERR(phy.wkupclk));
    phy.wkupclk = devm_clk_get(phy.dev, "usb_phy_cm_clk32k");
    if (IS_ERR(phy.wkupclk))
    return dev_err_probe(&pdev.dev, PTR_ERR(phy.wkupclk),
    "unable to get usb_phy_cm_clk32k\n");
    dev_warn(&pdev.dev,
    "found usb_phy_cm_clk32k, please fix DTS\n");
    }
    phy.optclk = devm_clk_get(phy.dev, "refclk");
    if (IS_ERR(phy.optclk)) {
    if (PTR_ERR(phy.optclk) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    dev_dbg(&pdev.dev, "unable to get refclk, trying old name\n");
    phy.optclk = devm_clk_get(phy.dev, "usb_otg_ss_refclk960m");
    if (IS_ERR(phy.optclk)) {
    if (PTR_ERR(phy.optclk) != -EPROBE_DEFER) {
    dev_dbg(&pdev.dev,
    "unable to get usb_otg_ss_refclk960m\n");
    }
    } else {
    dev_warn(&pdev.dev,
    "found usb_otg_ss_refclk960m, please fix DTS\n");
    }
    }
    otg.set_host = omap_usb_set_host;
    otg.set_peripheral = omap_usb_set_peripheral;
    if (phy_data.flags & OMAP_USB2_HAS_SET_VBUS)
    otg.set_vbus = omap_usb_set_vbus;
    if (phy_data.flags & OMAP_USB2_HAS_START_SRP)
    otg.start_srp = omap_usb_start_srp;
    otg.usb_phy = &phy.phy;
    platform_set_drvdata(pdev, phy);
    pm_runtime_enable(phy.dev);
    generic_phy = devm_phy_create(phy.dev, core::ptr::null_mut(), &ops);
    if (IS_ERR(generic_phy)) {
    pm_runtime_disable(phy.dev);
    return PTR_ERR(generic_phy);
    }
    phy_set_drvdata(generic_phy, phy);
    omap_usb_power_off(generic_phy);
    phy_provider = devm_of_phy_provider_register(phy.dev,
    of_phy_simple_xlate);
    if (IS_ERR(phy_provider)) {
    pm_runtime_disable(phy.dev);
    return PTR_ERR(phy_provider);
    }
    usb_add_phy_dev(&phy.phy);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_usb2_remove(pdev: *mut platform_device) {
    static void omap_usb2_remove(struct platform_device *pdev)
    {
    struct omap_usb	*phy = platform_get_drvdata(pdev);
    usb_remove_phy(&phy.phy);
    pm_runtime_disable(phy.dev);
    }
    static struct platform_driver omap_usb2_driver = {
    .probe		= omap_usb2_probe,
    .remove		= omap_usb2_remove,
    .driver		= {
    .name	= "omap-usb2",
    .of_match_table = omap_usb2_id_table,
    },
    };
    module_platform_driver(omap_usb2_driver);
    MODULE_AUTHOR("Texas Instruments Inc.");
    MODULE_DESCRIPTION("OMAP USB2 phy driver");
    MODULE_LICENSE("GPL v2");
