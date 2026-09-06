//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ehci-exynos.c
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
// Samsung Exynos USB HOST EHCI Controller
//
// Copyright (C) 2011 Samsung Electronics Co.Ltd
// Author: Jingoo Han <jg1.han@samsung.com>
// Author: Joonyoung Shim <jy0922.shim@samsung.com>
//

    (EHCI_INSNREG00_ENA_INCR16 | EHCI_INSNREG00_ENA_INCR8 |	\
    EHCI_INSNREG00_ENA_INCR4 | EHCI_INSNREG00_ENA_INCRX_ALIGN)
    static struct hc_driver __read_mostly exynos_ehci_hc_driver;
pub const PHY_NUMBER: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_ehci_hcd {
    pub clk: *mut clk,
    pub of_node: *mut device_node,
    pub phy: [*mut phy; PHY_NUMBER],
    pub legacy_phy: bool,
}

    static int exynos_ehci_get_phy(struct device *dev,
    struct exynos_ehci_hcd *exynos_ehci)
    {
    struct phy *phy;
    int phy_number, num_phys;
    int ret;
// Get PHYs for the controller
    num_phys = of_count_phandle_with_args(dev.of_node, "phys",
    "#phy-cells");
    for (phy_number = 0; phy_number < num_phys; phy_number++) {
    phy = devm_of_phy_get_by_index(dev, dev.of_node, phy_number);
    if (IS_ERR(phy))
    return PTR_ERR(phy);
    exynos_ehci.phy[phy_number] = phy;
    }
    if (num_phys > 0)
    return 0;
// Get PHYs using legacy bindings
    for_each_available_child_of_node_scoped(dev.of_node, child) {
    ret = of_property_read_u32(child, "reg", &phy_number);
    if (ret) {
    dev_err(dev, "Failed to parse device tree\n");
    return ret;
    }
    if (phy_number >= PHY_NUMBER) {
    dev_err(dev, "Invalid number of PHYs\n");
    return -EINVAL;
    }
    phy = devm_of_phy_optional_get(dev, child, core::ptr::null_mut());
    exynos_ehci.phy[phy_number] = phy;
    if (IS_ERR(phy))
    return PTR_ERR(phy);
    }
    exynos_ehci.legacy_phy = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_ehci_phy_enable(dev: *mut device) -> c_int {
    static int exynos_ehci_phy_enable(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    struct exynos_ehci_hcd *exynos_ehci = to_exynos_ehci(hcd);
    int i;
    let mut ret: c_int = 0;
    for (i = 0; ret == 0 && i < PHY_NUMBER; i++)
    ret = phy_power_on(exynos_ehci.phy[i]);
    if (ret)
    for (i--; i >= 0; i--)
    phy_power_off(exynos_ehci.phy[i]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos_ehci_phy_disable(dev: *mut device) {
    static void exynos_ehci_phy_disable(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    struct exynos_ehci_hcd *exynos_ehci = to_exynos_ehci(hcd);
    int i;
    for (i = 0; i < PHY_NUMBER; i++)
    phy_power_off(exynos_ehci.phy[i]);
    }
#[no_mangle]
unsafe extern "C" fn exynos_setup_vbus_gpio(dev: *mut device) {
    static void exynos_setup_vbus_gpio(struct device *dev)
    {
    struct gpio_desc *gpio;
    int err;
    gpio = devm_gpiod_get_optional(dev, "samsung,vbus", GPIOD_OUT_HIGH);
    err = PTR_ERR_OR_ZERO(gpio);
    if (err)
    dev_err(dev, "can't request ehci vbus gpio: %d\n", err);
    }
#[no_mangle]
unsafe extern "C" fn exynos_ehci_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_ehci_probe(struct platform_device *pdev)
    {
    struct exynos_ehci_hcd *exynos_ehci;
    struct usb_hcd *hcd;
    struct ehci_hcd *ehci;
    struct resource *res;
    int irq;
    int err;
//
// Right now device-tree probed devices don't get dma_mask set.
// Since shared usb code relies on it, set it here for now.
// Once we move to full device tree support this will vanish off.
//
    err = dma_coerce_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(32));
    if (err)
    return err;
    exynos_setup_vbus_gpio(&pdev.dev);
    hcd = usb_create_hcd(&exynos_ehci_hc_driver,
    &pdev.dev, dev_name(&pdev.dev));
    if (!hcd) {
    dev_err(&pdev.dev, "Unable to create HCD\n");
    return -ENOMEM;
    }
    exynos_ehci = to_exynos_ehci(hcd);
    err = exynos_ehci_get_phy(&pdev.dev, exynos_ehci);
    if (err)
    goto fail_io;
    exynos_ehci.clk = devm_clk_get_enabled(&pdev.dev, "usbhost");
    if (IS_ERR(exynos_ehci.clk)) {
    dev_err(&pdev.dev, "Failed to get usbhost clock\n");
    err = PTR_ERR(exynos_ehci.clk);
    goto fail_io;
    }
    hcd.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(hcd.regs)) {
    err = PTR_ERR(hcd.regs);
    goto fail_io;
    }
    hcd.rsrc_start = res.start;
    hcd.rsrc_len = resource_size(res);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    err = irq;
    goto fail_io;
    }
    err = exynos_ehci_phy_enable(&pdev.dev);
    if (err) {
    dev_err(&pdev.dev, "Failed to enable USB phy\n");
    goto fail_io;
    }
    ehci = hcd_to_ehci(hcd);
    ehci.caps = hcd.regs;
//
// Workaround: reset of_node pointer to avoid conflict between legacy
// Exynos EHCI port subnodes and generic USB device bindings
//
    exynos_ehci.of_node = pdev.dev.of_node;
    if (exynos_ehci.legacy_phy)
    pdev.dev.of_node = core::ptr::null_mut();
// DMA burst Enable
    writel(EHCI_INSNREG00_ENABLE_DMA_BURST, EHCI_INSNREG00(hcd.regs));
    err = usb_add_hcd(hcd, irq, IRQF_SHARED);
    if (err) {
    dev_err(&pdev.dev, "Failed to add USB HCD\n");
    goto fail_add_hcd;
    }
    device_wakeup_enable(hcd.self.controller);
    platform_set_drvdata(pdev, hcd);
    return 0;
    fail_add_hcd:
    exynos_ehci_phy_disable(&pdev.dev);
    pdev.dev.of_node = exynos_ehci.of_node;
    fail_io:
    usb_put_hcd(hcd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn exynos_ehci_remove(pdev: *mut platform_device) {
    static void exynos_ehci_remove(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    struct exynos_ehci_hcd *exynos_ehci = to_exynos_ehci(hcd);
    pdev.dev.of_node = exynos_ehci.of_node;
    usb_remove_hcd(hcd);
    exynos_ehci_phy_disable(&pdev.dev);
    usb_put_hcd(hcd);
    }
#[no_mangle]
unsafe extern "C" fn exynos_ehci_suspend(dev: *mut device) -> c_int {
    static int exynos_ehci_suspend(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    struct exynos_ehci_hcd *exynos_ehci = to_exynos_ehci(hcd);
    let mut do_wakeup: bool = device_may_wakeup(dev);
    int rc;
    rc = ehci_suspend(hcd, do_wakeup);
    if (rc)
    return rc;
    exynos_ehci_phy_disable(dev);
    clk_disable_unprepare(exynos_ehci.clk);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn exynos_ehci_resume(dev: *mut device) -> c_int {
    static int exynos_ehci_resume(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    struct exynos_ehci_hcd *exynos_ehci = to_exynos_ehci(hcd);
    int ret;
    ret = clk_prepare_enable(exynos_ehci.clk);
    if (ret)
    return ret;
    ret = exynos_ehci_phy_enable(dev);
    if (ret) {
    dev_err(dev, "Failed to enable USB phy\n");
    clk_disable_unprepare(exynos_ehci.clk);
    return ret;
    }
// DMA burst Enable
    writel(EHCI_INSNREG00_ENABLE_DMA_BURST, EHCI_INSNREG00(hcd.regs));
    ehci_resume(hcd, false);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(exynos_ehci_pm_ops,
    exynos_ehci_suspend, exynos_ehci_resume);

    static const struct of_device_id exynos_ehci_match[] = {
    { .compatible = "samsung,exynos4210-ehci" },
    {},
    };
    MODULE_DEVICE_TABLE(of, exynos_ehci_match);

    static struct platform_driver exynos_ehci_driver = {
    .probe		= exynos_ehci_probe,
    .remove		= exynos_ehci_remove,
    .shutdown	= usb_hcd_platform_shutdown,
    .driver = {
    .name	= "exynos-ehci",
    .pm	= pm_ptr(&exynos_ehci_pm_ops),
    .of_match_table = of_match_ptr(exynos_ehci_match),
    }
    };
    static const struct ehci_driver_overrides exynos_overrides __initconst = {
    .extra_priv_size = sizeof(struct exynos_ehci_hcd),
    };
#[no_mangle]
unsafe extern "C" fn ehci_exynos_init() -> int __init {
    static int __init ehci_exynos_init(void)
    {
    if (usb_disabled())
    return -ENODEV;
    ehci_init_driver(&exynos_ehci_hc_driver, &exynos_overrides);
    return platform_driver_register(&exynos_ehci_driver);
    }
    module_init(ehci_exynos_init);
#[no_mangle]
unsafe extern "C" fn ehci_exynos_cleanup() -> void __exit {
    static void __exit ehci_exynos_cleanup(void)
    {
    platform_driver_unregister(&exynos_ehci_driver);
    }
    module_exit(ehci_exynos_cleanup);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_ALIAS("platform:exynos-ehci");
    MODULE_AUTHOR("Jingoo Han");
    MODULE_AUTHOR("Joonyoung Shim");
    MODULE_LICENSE("GPL v2");
