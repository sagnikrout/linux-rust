//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ehci-st.c
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
// ST EHCI driver
//
// Copyright (C) 2014 STMicroelectronics – All Rights Reserved
//
// Author: Peter Griffin <peter.griffin@linaro.org>
//
// Derived from ehci-platform.c
//

pub const USB_MAX_CLKS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_ehci_platform_priv {
    pub clks: [*mut clk; USB_MAX_CLKS],
    pub clk48: *mut clk,
    pub rst: *mut reset_control,
    pub pwr: *mut reset_control,
    pub phy: *mut phy,
}

    ((struct st_ehci_platform_priv *)hcd_to_ehci(h).priv)
pub const EHCI_CAPS_SIZE: c_uint = 0x10;

#[no_mangle]
unsafe extern "C" fn st_ehci_platform_reset(hcd: *mut usb_hcd) -> c_int {
    static int st_ehci_platform_reset(struct usb_hcd *hcd)
    {
    struct platform_device *pdev = to_platform_device(hcd.self.controller);
    struct usb_ehci_pdata *pdata = dev_get_platdata(&pdev.dev);
    struct ehci_hcd *ehci = hcd_to_ehci(hcd);
    u32 threshold;
// Set EHCI packet buffer IN/OUT threshold to 128 bytes
    threshold = 128 | (128 << 16);
    writel(threshold, hcd.regs + AHB2STBUS_INSREG01);
    ehci.caps = hcd.regs + pdata.caps_offset;
    return ehci_setup(hcd);
    }
#[no_mangle]
unsafe extern "C" fn st_ehci_platform_power_on(dev: *mut platform_device) -> c_int {
    static int st_ehci_platform_power_on(struct platform_device *dev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(dev);
    struct st_ehci_platform_priv *priv = hcd_to_ehci_priv(hcd);
    int clk, ret;
    ret = reset_control_deassert(priv.pwr);
    if (ret)
    return ret;
    ret = reset_control_deassert(priv.rst);
    if (ret)
    goto err_assert_power;
// some SoCs don't have a dedicated 48Mhz clock, but those that do
    need the rate to be explicitly set */
    if (priv.clk48) {
    ret = clk_set_rate(priv.clk48, 48000000);
    if (ret)
    goto err_assert_reset;
    }
    for (clk = 0; clk < USB_MAX_CLKS && priv.clks[clk]; clk++) {
    ret = clk_prepare_enable(priv.clks[clk]);
    if (ret)
    goto err_disable_clks;
    }
    ret = phy_init(priv.phy);
    if (ret)
    goto err_disable_clks;
    ret = phy_power_on(priv.phy);
    if (ret)
    goto err_exit_phy;
    return 0;
    err_exit_phy:
    phy_exit(priv.phy);
    err_disable_clks:
    while (--clk >= 0)
    clk_disable_unprepare(priv.clks[clk]);
    err_assert_reset:
    reset_control_assert(priv.rst);
    err_assert_power:
    reset_control_assert(priv.pwr);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn st_ehci_platform_power_off(dev: *mut platform_device) {
    static void st_ehci_platform_power_off(struct platform_device *dev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(dev);
    struct st_ehci_platform_priv *priv = hcd_to_ehci_priv(hcd);
    int clk;
    reset_control_assert(priv.pwr);
    reset_control_assert(priv.rst);
    phy_power_off(priv.phy);
    phy_exit(priv.phy);
    for (clk = USB_MAX_CLKS - 1; clk >= 0; clk--)
    if (priv.clks[clk])
    clk_disable_unprepare(priv.clks[clk]);
    }
    static struct hc_driver __read_mostly ehci_platform_hc_driver;
    static const struct ehci_driver_overrides platform_overrides __initconst = {
    .reset =		st_ehci_platform_reset,
    .extra_priv_size =	sizeof(struct st_ehci_platform_priv),
    };
    static struct usb_ehci_pdata ehci_platform_defaults = {
    .power_on =		st_ehci_platform_power_on,
    .power_suspend =	st_ehci_platform_power_off,
    .power_off =		st_ehci_platform_power_off,
    };
#[no_mangle]
unsafe extern "C" fn st_ehci_platform_probe(dev: *mut platform_device) -> c_int {
    static int st_ehci_platform_probe(struct platform_device *dev)
    {
    struct usb_hcd *hcd;
    struct resource *res_mem;
    struct usb_ehci_pdata *pdata = &ehci_platform_defaults;
    struct st_ehci_platform_priv *priv;
    int err, irq, clk = 0;
    if (usb_disabled())
    return -ENODEV;
    irq = platform_get_irq(dev, 0);
    if (irq < 0)
    return irq;
    hcd = usb_create_hcd(&ehci_platform_hc_driver, &dev.dev,
    dev_name(&dev.dev));
    if (!hcd)
    return -ENOMEM;
    platform_set_drvdata(dev, hcd);
    dev.dev.platform_data = pdata;
    priv = hcd_to_ehci_priv(hcd);
    priv.phy = devm_phy_get(&dev.dev, "usb");
    if (IS_ERR(priv.phy)) {
    err = PTR_ERR(priv.phy);
    goto err_put_hcd;
    }
    for (clk = 0; clk < USB_MAX_CLKS; clk++) {
    priv.clks[clk] = of_clk_get(dev.dev.of_node, clk);
    if (IS_ERR(priv.clks[clk])) {
    err = PTR_ERR(priv.clks[clk]);
    if (err == -EPROBE_DEFER)
    goto err_put_clks;
    priv.clks[clk] = core::ptr::null_mut();
    break;
    }
    }
// some SoCs don't have a dedicated 48Mhz clock, but those that
    do need the rate to be explicitly set */
    priv.clk48 = devm_clk_get(&dev.dev, "clk48");
    if (IS_ERR(priv.clk48)) {
    dev_info(&dev.dev, "48MHz clk not found\n");
    priv.clk48 = core::ptr::null_mut();
    }
    priv.pwr =
    devm_reset_control_get_optional_shared(&dev.dev, "power");
    if (IS_ERR(priv.pwr)) {
    err = PTR_ERR(priv.pwr);
    if (err == -EPROBE_DEFER)
    goto err_put_clks;
    priv.pwr = core::ptr::null_mut();
    }
    priv.rst =
    devm_reset_control_get_optional_shared(&dev.dev, "softreset");
    if (IS_ERR(priv.rst)) {
    err = PTR_ERR(priv.rst);
    if (err == -EPROBE_DEFER)
    goto err_put_clks;
    priv.rst = core::ptr::null_mut();
    }
    if (pdata.power_on) {
    err = pdata.power_on(dev);
    if (err < 0)
    goto err_put_clks;
    }
    hcd.regs = devm_platform_get_and_ioremap_resource(dev, 0, &res_mem);
    if (IS_ERR(hcd.regs)) {
    err = PTR_ERR(hcd.regs);
    goto err_put_clks;
    }
    hcd.rsrc_start = res_mem.start;
    hcd.rsrc_len = resource_size(res_mem);
    err = usb_add_hcd(hcd, irq, IRQF_SHARED);
    if (err)
    goto err_put_clks;
    device_wakeup_enable(hcd.self.controller);
    platform_set_drvdata(dev, hcd);
    return err;
    err_put_clks:
    while (--clk >= 0)
    clk_put(priv.clks[clk]);
    err_put_hcd:
    if (pdata == &ehci_platform_defaults)
    dev.dev.platform_data = core::ptr::null_mut();
    usb_put_hcd(hcd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn st_ehci_platform_remove(dev: *mut platform_device) {
    static void st_ehci_platform_remove(struct platform_device *dev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(dev);
    struct usb_ehci_pdata *pdata = dev_get_platdata(&dev.dev);
    struct st_ehci_platform_priv *priv = hcd_to_ehci_priv(hcd);
    int clk;
    usb_remove_hcd(hcd);
    if (pdata.power_off)
    pdata.power_off(dev);
    for (clk = 0; clk < USB_MAX_CLKS && priv.clks[clk]; clk++)
    clk_put(priv.clks[clk]);
    usb_put_hcd(hcd);
    if (pdata == &ehci_platform_defaults)
    dev.dev.platform_data = core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn st_ehci_suspend(dev: *mut device) -> c_int {
    static int st_ehci_suspend(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    struct usb_ehci_pdata *pdata = dev_get_platdata(dev);
    struct platform_device *pdev = to_platform_device(dev);
    let mut do_wakeup: bool = device_may_wakeup(dev);
    int ret;
    ret = ehci_suspend(hcd, do_wakeup);
    if (ret)
    return ret;
    if (pdata.power_suspend)
    pdata.power_suspend(pdev);
    pinctrl_pm_select_sleep_state(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn st_ehci_resume(dev: *mut device) -> c_int {
    static int st_ehci_resume(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    struct usb_ehci_pdata *pdata = dev_get_platdata(dev);
    struct platform_device *pdev = to_platform_device(dev);
    int err;
    pinctrl_pm_select_default_state(dev);
    if (pdata.power_on) {
    err = pdata.power_on(pdev);
    if (err < 0)
    return err;
    }
    ehci_resume(hcd, false);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(st_ehci_pm_ops, st_ehci_suspend, st_ehci_resume);

    static const struct of_device_id st_ehci_ids[] = {
    { .compatible = "st,st-ehci-300x", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, st_ehci_ids);
    static struct platform_driver ehci_platform_driver = {
    .probe		= st_ehci_platform_probe,
    .remove		= st_ehci_platform_remove,
    .shutdown	= usb_hcd_platform_shutdown,
    .driver		= {
    .name	= "st-ehci",

    .pm	= &st_ehci_pm_ops,

    .of_match_table = st_ehci_ids,
    }
    };
#[no_mangle]
unsafe extern "C" fn ehci_platform_init() -> int __init {
    static int __init ehci_platform_init(void)
    {
    if (usb_disabled())
    return -ENODEV;
    ehci_init_driver(&ehci_platform_hc_driver, &platform_overrides);
    return platform_driver_register(&ehci_platform_driver);
    }
    module_init(ehci_platform_init);
#[no_mangle]
unsafe extern "C" fn ehci_platform_cleanup() -> void __exit {
    static void __exit ehci_platform_cleanup(void)
    {
    platform_driver_unregister(&ehci_platform_driver);
    }
    module_exit(ehci_platform_cleanup);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_AUTHOR("Peter Griffin <peter.griffin@linaro.org>");
    MODULE_LICENSE("GPL");
