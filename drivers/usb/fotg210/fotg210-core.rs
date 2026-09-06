//! Automatically rewritten from C to Rust
//! Source: drivers/usb/fotg210/fotg210-core.c
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
// Central probing code for the FOTG210 dual role driver
// We register one driver for the hardware and then we decide
// whether to proceed with probing the host or the peripheral
// driver.
//

// Role Register 0x80
pub const FOTG210_RR: c_uint = 0x80;

//
// Gemini-specific initialization function, only executed on the
// Gemini SoC using the global misc control register.
//
// The gemini USB blocks are connected to either Mini-A (host mode) or
// Mini-B (peripheral mode) plugs. There is no role switch support on the
// Gemini SoC, just either-or.
//
pub const GEMINI_GLOBAL_MISC_CTRL: c_uint = 0x30;

    static int fotg210_gemini_init(struct fotg210 *fotg, struct resource *res,
    enum usb_dr_mode mode)
    {
    struct device *dev = fotg.dev;
    struct device_node *np = dev.of_node;
    struct regmap *map;
    bool wakeup;
    u32 mask, val;
    int ret;
    map = syscon_regmap_lookup_by_phandle(np, "syscon");
    if (IS_ERR(map))
    return dev_err_probe(dev, PTR_ERR(map), "no syscon\n");
    fotg.map = map;
    wakeup = of_property_read_bool(np, "wakeup-source");
//
// Figure out if this is USB0 or USB1 by simply checking the
// physical base address.
//
    mask = 0;
    if (res.start == 0x69000000) {
    fotg.port = GEMINI_PORT_1;
    mask = GEMINI_MISC_USB1_VBUS_ON | GEMINI_MISC_USB1_MINI_B |
    GEMINI_MISC_USB1_WAKEUP;
    if (mode == USB_DR_MODE_HOST)
    val = GEMINI_MISC_USB1_VBUS_ON;
    else
    val = GEMINI_MISC_USB1_MINI_B;
    if (wakeup)
    val |= GEMINI_MISC_USB1_WAKEUP;
    } else {
    fotg.port = GEMINI_PORT_0;
    mask = GEMINI_MISC_USB0_VBUS_ON | GEMINI_MISC_USB0_MINI_B |
    GEMINI_MISC_USB0_WAKEUP;
    if (mode == USB_DR_MODE_HOST)
    val = GEMINI_MISC_USB0_VBUS_ON;
    else
    val = GEMINI_MISC_USB0_MINI_B;
    if (wakeup)
    val |= GEMINI_MISC_USB0_WAKEUP;
    }
    ret = regmap_update_bits(map, GEMINI_GLOBAL_MISC_CTRL, mask, val);
    if (ret) {
    dev_err(dev, "failed to initialize Gemini PHY\n");
    return ret;
    }
    dev_info(dev, "initialized Gemini PHY in %s mode\n",
    (mode == USB_DR_MODE_HOST) ? "host" : "gadget");
    return 0;
    }
//
// fotg210_vbus() - Called by gadget driver to enable/disable VBUS
// @fotg: pointer to a private fotg210 object
// @enable: true to enable VBUS, false to disable VBUS
//
#[no_mangle]
pub unsafe extern "C" fn fotg210_vbus(fotg: *mut fotg210, enable: bool) {
    void fotg210_vbus(struct fotg210 *fotg, bool enable)
    {
    u32 mask;
    u32 val;
    int ret;
    switch (fotg.port) {
    case GEMINI_PORT_0:
    mask = GEMINI_MISC_USB0_VBUS_ON;
    val = enable ? GEMINI_MISC_USB0_VBUS_ON : 0;
    break;
    case GEMINI_PORT_1:
    mask = GEMINI_MISC_USB1_VBUS_ON;
    val = enable ? GEMINI_MISC_USB1_VBUS_ON : 0;
    break;
    default:
    return;
    }
    ret = regmap_update_bits(fotg.map, GEMINI_GLOBAL_MISC_CTRL, mask, val);
    if (ret)
    dev_err(fotg.dev, "failed to %s VBUS\n",
    str_enable_disable(enable));
    dev_info(fotg.dev, "%s: %s VBUS\n", __func__, str_enable_disable(enable));
    }
#[no_mangle]
unsafe extern "C" fn fotg210_probe(pdev: *mut platform_device) -> c_int {
    static int fotg210_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    enum usb_dr_mode mode;
    struct fotg210 *fotg;
    u32 val;
    int ret;
    fotg = devm_kzalloc(dev, sizeof(*fotg), GFP_KERNEL);
    if (!fotg)
    return -ENOMEM;
    fotg.dev = dev;
    fotg.base = devm_platform_get_and_ioremap_resource(pdev, 0, &fotg.res);
    if (IS_ERR(fotg.base))
    return PTR_ERR(fotg.base);
    fotg.pclk = devm_clk_get_optional_enabled(dev, "PCLK");
    if (IS_ERR(fotg.pclk))
    return PTR_ERR(fotg.pclk);
    mode = usb_get_dr_mode(dev);
    if (of_device_is_compatible(dev.of_node, "cortina,gemini-usb")) {
    ret = fotg210_gemini_init(fotg, fotg.res, mode);
    if (ret)
    return ret;
    }
    val = readl(fotg.base + FOTG210_RR);
    if (mode == USB_DR_MODE_PERIPHERAL) {
    if (!(val & FOTG210_RR_CROLE))
    dev_err(dev, "block not in device role\n");
    ret = fotg210_udc_probe(pdev, fotg);
    } else {
    if (val & FOTG210_RR_CROLE)
    dev_err(dev, "block not in host role\n");
    ret = fotg210_hcd_probe(pdev, fotg);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fotg210_remove(pdev: *mut platform_device) {
    static void fotg210_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    enum usb_dr_mode mode;
    mode = usb_get_dr_mode(dev);
    if (mode == USB_DR_MODE_PERIPHERAL)
    fotg210_udc_remove(pdev);
    else
    fotg210_hcd_remove(pdev);
    }

    static const struct of_device_id fotg210_of_match[] = {
    { .compatible = "faraday,fotg200" },
    { .compatible = "faraday,fotg210" },
// TODO: can we also handle FUSB220?
    {},
    };
    MODULE_DEVICE_TABLE(of, fotg210_of_match);

    static struct platform_driver fotg210_driver = {
    .driver = {
    .name   = "fotg210",
    .of_match_table = of_match_ptr(fotg210_of_match),
    },
    .probe  = fotg210_probe,
    .remove = fotg210_remove,
    };
#[no_mangle]
unsafe extern "C" fn fotg210_init() -> int __init {
    static int __init fotg210_init(void)
    {
    if (IS_ENABLED(CONFIG_USB_FOTG210_HCD) && !usb_disabled())
    fotg210_hcd_init();
    return platform_driver_register(&fotg210_driver);
    }
    module_init(fotg210_init);
#[no_mangle]
unsafe extern "C" fn fotg210_cleanup() -> void __exit {
    static void __exit fotg210_cleanup(void)
    {
    platform_driver_unregister(&fotg210_driver);
    if (IS_ENABLED(CONFIG_USB_FOTG210_HCD))
    fotg210_hcd_cleanup();
    }
    module_exit(fotg210_cleanup);
    MODULE_AUTHOR("Yuan-Hsin Chen, Feng-Hsin Chiang");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("FOTG210 Dual Role Controller Driver");
