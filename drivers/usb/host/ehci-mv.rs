//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ehci-mv.c
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
// Copyright (C) 2011 Marvell International Ltd. All rights reserved.
// Author: Chao Xie <chao.xie@marvell.com>
// Neil Zhang <zhangwm@marvell.com>
//

// registers
pub const U2x_CAPREGS_OFFSET: c_uint = 0x100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_hcd_mv {
// Which mode does this ehci running OTG/Host ?
    pub mode: c_int,
    pub base: *mut void __iomem,
    pub cap_regs: *mut void __iomem,
    pub op_regs: *mut void __iomem,
    pub otg: *mut usb_phy,
    pub clk: *mut clk,
    pub phy: *mut phy,
    pub vbus): *mut *mut int (set_vbus)(unsigned int,
}

#[no_mangle]
unsafe extern "C" fn mv_ehci_enable(ehci_mv: *mut ehci_hcd_mv) -> c_int {
    static int mv_ehci_enable(struct ehci_hcd_mv *ehci_mv)
    {
    int retval;
    retval = clk_prepare_enable(ehci_mv.clk);
    if (retval)
    return retval;
    retval = phy_init(ehci_mv.phy);
    if (retval)
    clk_disable_unprepare(ehci_mv.clk);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn mv_ehci_disable(ehci_mv: *mut ehci_hcd_mv) {
    static void mv_ehci_disable(struct ehci_hcd_mv *ehci_mv)
    {
    phy_exit(ehci_mv.phy);
    clk_disable_unprepare(ehci_mv.clk);
    }
#[no_mangle]
unsafe extern "C" fn mv_ehci_reset(hcd: *mut usb_hcd) -> c_int {
    static int mv_ehci_reset(struct usb_hcd *hcd)
    {
    struct device *dev = hcd.self.controller;
    struct ehci_hcd_mv *ehci_mv = hcd_to_ehci_hcd_mv(hcd);
    struct ehci_hcd *ehci = hcd_to_ehci(hcd);
    u32 status;
    int retval;
    if (ehci_mv == core::ptr::null_mut()) {
    dev_err(dev, "Can not find private ehci data\n");
    return -ENODEV;
    }
    hcd.has_tt = 1;
    retval = ehci_setup(hcd);
    if (retval)
    dev_err(dev, "ehci_setup failed %d\n", retval);
    if (of_usb_get_phy_mode(dev.of_node) == USBPHY_INTERFACE_MODE_HSIC) {
    status = ehci_readl(ehci, &ehci.regs.port_status[0]);
    status |= PORT_TEST_FORCE;
    ehci_writel(ehci, status, &ehci.regs.port_status[0]);
    status &= ~PORT_TEST_FORCE;
    ehci_writel(ehci, status, &ehci.regs.port_status[0]);
    }
    return retval;
    }
    static struct hc_driver __read_mostly ehci_platform_hc_driver;
    static const struct ehci_driver_overrides platform_overrides __initconst = {
    .reset =		mv_ehci_reset,
    .extra_priv_size =	sizeof(struct ehci_hcd_mv),
    };
#[no_mangle]
unsafe extern "C" fn mv_ehci_probe(pdev: *mut platform_device) -> c_int {
    static int mv_ehci_probe(struct platform_device *pdev)
    {
    struct mv_usb_platform_data *pdata = dev_get_platdata(&pdev.dev);
    struct usb_hcd *hcd;
    struct ehci_hcd *ehci;
    struct ehci_hcd_mv *ehci_mv;
    struct resource *r;
    int retval;
    u32 offset;
    u32 status;
    if (usb_disabled())
    return -ENODEV;
    hcd = usb_create_hcd(&ehci_platform_hc_driver, &pdev.dev, dev_name(&pdev.dev));
    if (!hcd)
    return -ENOMEM;
    platform_set_drvdata(pdev, hcd);
    ehci_mv = hcd_to_ehci_hcd_mv(hcd);
    ehci_mv.mode = MV_USB_MODE_HOST;
    if (pdata) {
    ehci_mv.mode = pdata.mode;
    ehci_mv.set_vbus = pdata.set_vbus;
    }
    ehci_mv.phy = devm_phy_optional_get(&pdev.dev, "usb");
    if (IS_ERR(ehci_mv.phy)) {
    retval = PTR_ERR(ehci_mv.phy);
    if (retval != -EPROBE_DEFER)
    dev_err(&pdev.dev, "Failed to get phy.\n");
    goto err_put_hcd;
    }
    ehci_mv.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(ehci_mv.clk)) {
    dev_err(&pdev.dev, "error getting clock\n");
    retval = PTR_ERR(ehci_mv.clk);
    goto err_put_hcd;
    }
    ehci_mv.base = devm_platform_get_and_ioremap_resource(pdev, 0, &r);
    if (IS_ERR(ehci_mv.base)) {
    retval = PTR_ERR(ehci_mv.base);
    goto err_put_hcd;
    }
    retval = mv_ehci_enable(ehci_mv);
    if (retval) {
    dev_err(&pdev.dev, "init phy error %d\n", retval);
    goto err_put_hcd;
    }
    ehci_mv.cap_regs =
    (void __iomem *) ((unsigned long) ehci_mv.base + U2x_CAPREGS_OFFSET);
    offset = readl(ehci_mv.cap_regs) & CAPLENGTH_MASK;
    ehci_mv.op_regs =
    (void __iomem *) ((unsigned long) ehci_mv.cap_regs + offset);
    hcd.rsrc_start = r.start;
    hcd.rsrc_len = resource_size(r);
    hcd.regs = ehci_mv.op_regs;
    retval = platform_get_irq(pdev, 0);
    if (retval < 0)
    goto err_disable_clk;
    hcd.irq = retval;
    ehci = hcd_to_ehci(hcd);
    ehci.caps = (struct ehci_caps __iomem *) ehci_mv.cap_regs;
    if (ehci_mv.mode == MV_USB_MODE_OTG) {
    ehci_mv.otg = devm_usb_get_phy(&pdev.dev, USB_PHY_TYPE_USB2);
    if (IS_ERR(ehci_mv.otg)) {
    retval = PTR_ERR(ehci_mv.otg);
    if (retval == -ENXIO)
    dev_info(&pdev.dev, "MV_USB_MODE_OTG "
    "must have CONFIG_USB_PHY enabled\n");
    else
    dev_err(&pdev.dev,
    "unable to find transceiver\n");
    goto err_disable_clk;
    }
    retval = otg_set_host(ehci_mv.otg.otg, &hcd.self);
    if (retval < 0) {
    dev_err(&pdev.dev,
    "unable to register with transceiver\n");
    retval = -ENODEV;
    goto err_disable_clk;
    }
// otg will enable clock before use as host
    mv_ehci_disable(ehci_mv);
    } else {
    if (ehci_mv.set_vbus)
    ehci_mv.set_vbus(1);
    retval = usb_add_hcd(hcd, hcd.irq, IRQF_SHARED);
    if (retval) {
    dev_err(&pdev.dev,
    "failed to add hcd with err %d\n", retval);
    goto err_set_vbus;
    }
    device_wakeup_enable(hcd.self.controller);
    }
    if (of_usb_get_phy_mode(pdev.dev.of_node) == USBPHY_INTERFACE_MODE_HSIC) {
    status = ehci_readl(ehci, &ehci.regs.port_status[0]);
// These "reserved" bits actually enable HSIC mode.
    status |= BIT(25);
    status &= ~GENMASK(31, 30);
    ehci_writel(ehci, status, &ehci.regs.port_status[0]);
    }
    dev_info(&pdev.dev,
    "successful find EHCI device with regs 0x%p irq %d"
    " working in %s mode\n", hcd.regs, hcd.irq,
    ehci_mv.mode == MV_USB_MODE_OTG ? "OTG" : "Host");
    return 0;
    err_set_vbus:
    if (ehci_mv.set_vbus)
    ehci_mv.set_vbus(0);
    err_disable_clk:
    mv_ehci_disable(ehci_mv);
    err_put_hcd:
    usb_put_hcd(hcd);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn mv_ehci_remove(pdev: *mut platform_device) {
    static void mv_ehci_remove(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    struct ehci_hcd_mv *ehci_mv = hcd_to_ehci_hcd_mv(hcd);
    if (hcd.rh_registered)
    usb_remove_hcd(hcd);
    if (!IS_ERR_OR_NULL(ehci_mv.otg))
    otg_set_host(ehci_mv.otg.otg, core::ptr::null_mut());
    if (ehci_mv.mode == MV_USB_MODE_HOST) {
    if (ehci_mv.set_vbus)
    ehci_mv.set_vbus(0);
    mv_ehci_disable(ehci_mv);
    }
    usb_put_hcd(hcd);
    }
    static const struct platform_device_id ehci_id_table[] = {
    {"pxa-u2oehci", 0},
    {"pxa-sph", 0},
    {},
    };
#[no_mangle]
unsafe extern "C" fn mv_ehci_shutdown(pdev: *mut platform_device) {
    static void mv_ehci_shutdown(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    if (!hcd.rh_registered)
    return;
    if (hcd.driver.shutdown)
    hcd.driver.shutdown(hcd);
    }
    static const struct of_device_id ehci_mv_dt_ids[] = {
    { .compatible = "marvell,pxau2o-ehci", },
    {},
    };
    static struct platform_driver ehci_mv_driver = {
    .probe = mv_ehci_probe,
    .remove = mv_ehci_remove,
    .shutdown = mv_ehci_shutdown,
    .driver = {
    .name = "mv-ehci",
    .bus = &platform_bus_type,
    .of_match_table = ehci_mv_dt_ids,
    },
    .id_table = ehci_id_table,
    };
#[no_mangle]
unsafe extern "C" fn ehci_platform_init() -> int __init {
    static int __init ehci_platform_init(void)
    {
    if (usb_disabled())
    return -ENODEV;
    ehci_init_driver(&ehci_platform_hc_driver, &platform_overrides);
    return platform_driver_register(&ehci_mv_driver);
    }
    module_init(ehci_platform_init);
#[no_mangle]
unsafe extern "C" fn ehci_platform_cleanup() -> void __exit {
    static void __exit ehci_platform_cleanup(void)
    {
    platform_driver_unregister(&ehci_mv_driver);
    }
    module_exit(ehci_platform_cleanup);
    MODULE_DESCRIPTION("Marvell EHCI driver");
    MODULE_AUTHOR("Chao Xie <chao.xie@marvell.com>");
    MODULE_AUTHOR("Neil Zhang <zhangwm@marvell.com>");
    MODULE_ALIAS("mv-ehci");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(of, ehci_mv_dt_ids);
