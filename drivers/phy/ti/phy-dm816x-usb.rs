//! Automatically rewritten from C to Rust
//! Source: drivers/phy/ti/phy-dm816x-usb.c
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


// SPDX-License-Identifier: GPL-2.0-only

//
// TRM has two sets of USB_CTRL registers.. The correct register bits
// are in TRM section 24.9.8.2 USB_CTRL Register. The TRM documents the
// phy as being SR70LX Synopsys USB 2.0 OTG nanoPHY. It also seems at
// least dm816x rev c ignores writes to USB_CTRL register, but the TI
// kernel is writing to those so it's possible that later revisions
// have worknig USB_CTRL register.
//
// Also note that At least USB_CTRL register seems to be dm816x specific
// according to the TRM. It's possible that USBPHY_CTRL is more generic,
// but that would have to be checked against the SR70LX documentation
// which does not seem to be publicly available.
//
// Finally, the phy on dm814x and am335x is different from dm816x.
//

pub const DM816X_USBPHY_CTRL_TXRISETUNE: c_int = 1;
pub const DM816X_USBPHY_CTRL_TXVREFTUNE: c_uint = 0xc;
pub const DM816X_USBPHY_CTRL_TXPREEMTUNE: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm816x_usb_phy {
    pub syscon: *mut regmap,
    pub dev: *mut device,
    pub instance: c_uint,
    pub refclk: *mut clk,
    pub phy: usb_phy,
    pub /: *mut *mut unsigned int usb_ctrl; / Shared between phy0 and phy1,
    pub usbphy_ctrl: c_uint,
}

#[no_mangle]
unsafe extern "C" fn dm816x_usb_phy_set_host(otg: *mut usb_otg, host: *mut usb_bus) -> c_int {
    static int dm816x_usb_phy_set_host(struct usb_otg *otg, struct usb_bus *host)
    {
    otg.host = host;
    if (!host)
    otg.state = OTG_STATE_UNDEFINED;
    return 0;
    }
    static int dm816x_usb_phy_set_peripheral(struct usb_otg *otg,
    struct usb_gadget *gadget)
    {
    otg.gadget = gadget;
    if (!gadget)
    otg.state = OTG_STATE_UNDEFINED;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dm816x_usb_phy_init(x: *mut phy) -> c_int {
    static int dm816x_usb_phy_init(struct phy *x)
    {
    struct dm816x_usb_phy *phy = phy_get_drvdata(x);
    unsigned int val;
    if (clk_get_rate(phy.refclk) != 24000000)
    dev_warn(phy.dev, "nonstandard phy refclk\n");
// Set PLL ref clock and put phys to sleep
    regmap_update_bits(phy.syscon, phy.usb_ctrl,
    DM816X_USB_CTRL_PHYCLKSRC |
    DM816X_USB_CTRL_PHYSLEEP1 |
    DM816X_USB_CTRL_PHYSLEEP0,
    0);
    regmap_read(phy.syscon, phy.usb_ctrl, &val);
    if ((val & 3) != 0)
    dev_info(phy.dev,
    "Working dm816x USB_CTRL! (0x%08x)\n",
    val);
//
// TI kernel sets these values for "symmetrical eye diagram and
// better signal quality" so let's assume somebody checked the
// values with a scope and set them here too.
//
    regmap_read(phy.syscon, phy.usbphy_ctrl, &val);
    val |= DM816X_USBPHY_CTRL_TXRISETUNE |
    DM816X_USBPHY_CTRL_TXVREFTUNE |
    DM816X_USBPHY_CTRL_TXPREEMTUNE;
    regmap_write(phy.syscon, phy.usbphy_ctrl, val);
    return 0;
    }
    static const struct phy_ops ops = {
    .init		= dm816x_usb_phy_init,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn dm816x_usb_phy_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused dm816x_usb_phy_runtime_suspend(struct device *dev)
    {
    struct dm816x_usb_phy *phy = dev_get_drvdata(dev);
    unsigned int mask, val;
    let mut error: c_int = 0;
    mask = BIT(phy.instance);
    val = ~BIT(phy.instance);
    error = regmap_update_bits(phy.syscon, phy.usb_ctrl,
    mask, val);
    if (error)
    dev_err(phy.dev, "phy%i failed to power off\n",
    phy.instance);
    clk_disable(phy.refclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dm816x_usb_phy_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused dm816x_usb_phy_runtime_resume(struct device *dev)
    {
    struct dm816x_usb_phy *phy = dev_get_drvdata(dev);
    unsigned int mask, val;
    int error;
    error = clk_enable(phy.refclk);
    if (error)
    return error;
//
// Note that at least dm816x rev c does not seem to do
// anything with the USB_CTRL register. But let's follow
// what the TI tree is doing in case later revisions use
// USB_CTRL.
//
    mask = BIT(phy.instance);
    val = BIT(phy.instance);
    error = regmap_update_bits(phy.syscon, phy.usb_ctrl,
    mask, val);
    if (error) {
    dev_err(phy.dev, "phy%i failed to power on\n",
    phy.instance);
    clk_disable(phy.refclk);
    return error;
    }
    return 0;
    }
    static UNIVERSAL_DEV_PM_OPS(dm816x_usb_phy_pm_ops,
    dm816x_usb_phy_runtime_suspend,
    dm816x_usb_phy_runtime_resume,
    core::ptr::null_mut());
    static const struct of_device_id dm816x_usb_phy_id_table[] = {
    {
    .compatible = "ti,dm8168-usb-phy",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, dm816x_usb_phy_id_table);
#[no_mangle]
unsafe extern "C" fn dm816x_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int dm816x_usb_phy_probe(struct platform_device *pdev)
    {
    struct dm816x_usb_phy *phy;
    struct resource *res;
    struct phy *generic_phy;
    struct phy_provider *phy_provider;
    struct usb_otg *otg;
    int error;
    phy = devm_kzalloc(&pdev.dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENOENT;
    phy.syscon = syscon_regmap_lookup_by_phandle(pdev.dev.of_node,
    "syscon");
    if (IS_ERR(phy.syscon))
    return PTR_ERR(phy.syscon);
//
// According to sprs614e.pdf, the first usb_ctrl is shared and
// the second instance for usb_ctrl is reserved.. Also the
// register bits are different from earlier TRMs.
//
    phy.usb_ctrl = 0x20;
    phy.usbphy_ctrl = (res.start & 0xff) + 4;
    if (phy.usbphy_ctrl == 0x2c)
    phy.instance = 1;
    otg = devm_kzalloc(&pdev.dev, sizeof(*otg), GFP_KERNEL);
    if (!otg)
    return -ENOMEM;
    phy.dev = &pdev.dev;
    phy.phy.dev = phy.dev;
    phy.phy.label = "dm8168_usb_phy";
    phy.phy.otg = otg;
    phy.phy.type = USB_PHY_TYPE_USB2;
    otg.set_host = dm816x_usb_phy_set_host;
    otg.set_peripheral = dm816x_usb_phy_set_peripheral;
    otg.usb_phy = &phy.phy;
    platform_set_drvdata(pdev, phy);
    phy.refclk = devm_clk_get(phy.dev, "refclk");
    if (IS_ERR(phy.refclk))
    return PTR_ERR(phy.refclk);
    error = clk_prepare(phy.refclk);
    if (error)
    return error;
    pm_runtime_enable(phy.dev);
    generic_phy = devm_phy_create(phy.dev, core::ptr::null_mut(), &ops);
    if (IS_ERR(generic_phy)) {
    error = PTR_ERR(generic_phy);
    goto clk_unprepare;
    }
    phy_set_drvdata(generic_phy, phy);
    phy_provider = devm_of_phy_provider_register(phy.dev,
    of_phy_simple_xlate);
    if (IS_ERR(phy_provider)) {
    error = PTR_ERR(phy_provider);
    goto clk_unprepare;
    }
    usb_add_phy_dev(&phy.phy);
    return 0;
    clk_unprepare:
    pm_runtime_disable(phy.dev);
    clk_unprepare(phy.refclk);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn dm816x_usb_phy_remove(pdev: *mut platform_device) {
    static void dm816x_usb_phy_remove(struct platform_device *pdev)
    {
    struct dm816x_usb_phy *phy = platform_get_drvdata(pdev);
    usb_remove_phy(&phy.phy);
    pm_runtime_disable(phy.dev);
    clk_unprepare(phy.refclk);
    }
    static struct platform_driver dm816x_usb_phy_driver = {
    .probe		= dm816x_usb_phy_probe,
    .remove		= dm816x_usb_phy_remove,
    .driver		= {
    .name	= "dm816x-usb-phy",
    .pm	= &dm816x_usb_phy_pm_ops,
    .of_match_table = dm816x_usb_phy_id_table,
    },
    };
    module_platform_driver(dm816x_usb_phy_driver);
    MODULE_AUTHOR("Tony Lindgren <tony@atomide.com>");
    MODULE_DESCRIPTION("dm816x usb phy driver");
    MODULE_LICENSE("GPL v2");
