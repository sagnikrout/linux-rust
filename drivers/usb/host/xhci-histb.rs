//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/xhci-histb.c
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
// xHCI host controller driver for HiSilicon STB SoCs
//
// Copyright (C) 2017-2018 HiSilicon Co., Ltd. http://www.hisilicon.com
//
// Authors: Jianguo Sun <sunjianguo1@huawei.com>
//

pub const GTXTHRCFG: c_uint = 0xc108;
pub const GRXTHRCFG: c_uint = 0xc10c;
pub const REG_GUSB2PHYCFG0: c_uint = 0xc200;

pub const REG_GUSB3PIPECTL0: c_uint = 0xc2c0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_hcd_histb {
    pub dev: *mut device,
    pub hcd: *mut usb_hcd,
    pub ctrl: *mut void __iomem,
    pub bus_clk: *mut clk,
    pub utmi_clk: *mut clk,
    pub pipe_clk: *mut clk,
    pub suspend_clk: *mut clk,
    pub soft_reset: *mut reset_control,
}

    static inline struct xhci_hcd_histb *hcd_to_histb(struct usb_hcd *hcd)
    {
    return dev_get_drvdata(hcd.self.controller);
    }
#[no_mangle]
unsafe extern "C" fn xhci_histb_config(histb: *mut xhci_hcd_histb) -> c_int {
    static int xhci_histb_config(struct xhci_hcd_histb *histb)
    {
    struct device_node *np = histb.dev.of_node;
    u32 regval;
    if (of_property_match_string(np, "phys-names", "inno") >= 0) {
// USB2 PHY chose ulpi 8bit interface
    regval = readl(histb.ctrl + REG_GUSB2PHYCFG0);
    regval &= ~BIT_UTMI_ULPI;
    regval &= ~(BIT_UTMI_8_16);
    regval &= ~BIT_FREECLK_EXIST;
    writel(regval, histb.ctrl + REG_GUSB2PHYCFG0);
    }
    if (of_property_match_string(np, "phys-names", "combo") >= 0) {
//
// write 0x010c0012 to GUSB3PIPECTL0
// GUSB3PIPECTL0[5:3] = 010 : Tx Margin = 900mV ,
// decrease TX voltage
// GUSB3PIPECTL0[2:1] = 01 : Tx Deemphasis = -3.5dB,
// refer to xHCI spec
//
    regval = readl(histb.ctrl + REG_GUSB3PIPECTL0);
    regval &= ~USB3_DEEMPHASIS_MASK;
    regval |= USB3_DEEMPHASIS0;
    regval |= USB3_TX_MARGIN1;
    writel(regval, histb.ctrl + REG_GUSB3PIPECTL0);
    }
    writel(0x23100000, histb.ctrl + GTXTHRCFG);
    writel(0x23100000, histb.ctrl + GRXTHRCFG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xhci_histb_clks_get(histb: *mut xhci_hcd_histb) -> c_int {
    static int xhci_histb_clks_get(struct xhci_hcd_histb *histb)
    {
    struct device *dev = histb.dev;
    histb.bus_clk = devm_clk_get(dev, "bus");
    if (IS_ERR(histb.bus_clk)) {
    dev_err(dev, "fail to get bus clk\n");
    return PTR_ERR(histb.bus_clk);
    }
    histb.utmi_clk = devm_clk_get(dev, "utmi");
    if (IS_ERR(histb.utmi_clk)) {
    dev_err(dev, "fail to get utmi clk\n");
    return PTR_ERR(histb.utmi_clk);
    }
    histb.pipe_clk = devm_clk_get(dev, "pipe");
    if (IS_ERR(histb.pipe_clk)) {
    dev_err(dev, "fail to get pipe clk\n");
    return PTR_ERR(histb.pipe_clk);
    }
    histb.suspend_clk = devm_clk_get(dev, "suspend");
    if (IS_ERR(histb.suspend_clk)) {
    dev_err(dev, "fail to get suspend clk\n");
    return PTR_ERR(histb.suspend_clk);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xhci_histb_host_enable(histb: *mut xhci_hcd_histb) -> c_int {
    static int xhci_histb_host_enable(struct xhci_hcd_histb *histb)
    {
    int ret;
    ret = clk_prepare_enable(histb.bus_clk);
    if (ret) {
    dev_err(histb.dev, "failed to enable bus clk\n");
    return ret;
    }
    ret = clk_prepare_enable(histb.utmi_clk);
    if (ret) {
    dev_err(histb.dev, "failed to enable utmi clk\n");
    goto err_utmi_clk;
    }
    ret = clk_prepare_enable(histb.pipe_clk);
    if (ret) {
    dev_err(histb.dev, "failed to enable pipe clk\n");
    goto err_pipe_clk;
    }
    ret = clk_prepare_enable(histb.suspend_clk);
    if (ret) {
    dev_err(histb.dev, "failed to enable suspend clk\n");
    goto err_suspend_clk;
    }
    reset_control_deassert(histb.soft_reset);
    return 0;
    err_suspend_clk:
    clk_disable_unprepare(histb.pipe_clk);
    err_pipe_clk:
    clk_disable_unprepare(histb.utmi_clk);
    err_utmi_clk:
    clk_disable_unprepare(histb.bus_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xhci_histb_host_disable(histb: *mut xhci_hcd_histb) {
    static void xhci_histb_host_disable(struct xhci_hcd_histb *histb)
    {
    reset_control_assert(histb.soft_reset);
    clk_disable_unprepare(histb.suspend_clk);
    clk_disable_unprepare(histb.pipe_clk);
    clk_disable_unprepare(histb.utmi_clk);
    clk_disable_unprepare(histb.bus_clk);
    }
// called during probe() after chip reset completes
#[no_mangle]
unsafe extern "C" fn xhci_histb_setup(hcd: *mut usb_hcd) -> c_int {
    static int xhci_histb_setup(struct usb_hcd *hcd)
    {
    struct xhci_hcd_histb *histb = hcd_to_histb(hcd);
    int ret;
    if (usb_hcd_is_primary_hcd(hcd)) {
    ret = xhci_histb_config(histb);
    if (ret)
    return ret;
    }
    return xhci_gen_setup(hcd, core::ptr::null_mut());
    }
    static const struct xhci_driver_overrides xhci_histb_overrides __initconst = {
    .reset = xhci_histb_setup,
    };
    static struct hc_driver __read_mostly xhci_histb_hc_driver;
#[no_mangle]
unsafe extern "C" fn xhci_histb_probe(pdev: *mut platform_device) -> c_int {
    static int xhci_histb_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct xhci_hcd_histb *histb;
    const struct hc_driver *driver;
    struct usb_hcd *hcd;
    struct xhci_hcd *xhci;
    struct resource *res;
    int irq;
    let mut ret: c_int = -ENODEV;
    if (usb_disabled())
    return -ENODEV;
    driver = &xhci_histb_hc_driver;
    histb = devm_kzalloc(dev, sizeof(*histb), GFP_KERNEL);
    if (!histb)
    return -ENOMEM;
    histb.dev = dev;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    histb.ctrl = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(histb.ctrl))
    return PTR_ERR(histb.ctrl);
    ret = xhci_histb_clks_get(histb);
    if (ret)
    return ret;
    histb.soft_reset = devm_reset_control_get(dev, "soft");
    if (IS_ERR(histb.soft_reset)) {
    dev_err(dev, "failed to get soft reset\n");
    return PTR_ERR(histb.soft_reset);
    }
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
    device_enable_async_suspend(dev);
// Initialize dma_mask and coherent_dma_mask to 32-bits
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32));
    if (ret)
    goto disable_pm;
    hcd = usb_create_hcd(driver, dev, dev_name(dev));
    if (!hcd) {
    ret = -ENOMEM;
    goto disable_pm;
    }
    hcd.regs = histb.ctrl;
    hcd.rsrc_start = res.start;
    hcd.rsrc_len = resource_size(res);
    histb.hcd = hcd;
    dev_set_drvdata(hcd.self.controller, histb);
    ret = xhci_histb_host_enable(histb);
    if (ret)
    goto put_hcd;
    xhci = hcd_to_xhci(hcd);
    device_wakeup_enable(hcd.self.controller);
    xhci.main_hcd = hcd;
    xhci.shared_hcd = usb_create_shared_hcd(driver, dev, dev_name(dev),
    hcd);
    if (!xhci.shared_hcd) {
    ret = -ENOMEM;
    goto disable_host;
    }
    if (device_property_read_bool(dev, "usb2-lpm-disable"))
    xhci.quirks |= XHCI_HW_LPM_DISABLE;
    if (device_property_read_bool(dev, "usb3-lpm-capable"))
    xhci.quirks |= XHCI_LPM_SUPPORT;
// imod_interval is the interrupt moderation value in nanoseconds.
    xhci.imod_interval = 40000;
    device_property_read_u32(dev, "imod-interval-ns",
    &xhci.imod_interval);
    ret = usb_add_hcd(hcd, irq, IRQF_SHARED);
    if (ret)
    goto put_usb3_hcd;
    if (GET_MAX_PSA_SIZE(xhci.hcc_params) >= 4)
    xhci.shared_hcd.can_do_streams = 1;
    ret = usb_add_hcd(xhci.shared_hcd, irq, IRQF_SHARED);
    if (ret)
    goto dealloc_usb2_hcd;
    device_enable_async_suspend(dev);
    pm_runtime_put_noidle(dev);
//
// Prevent runtime pm from being on as default, users should enable
// runtime pm using power/control in sysfs.
//
    pm_runtime_forbid(dev);
    return 0;
    dealloc_usb2_hcd:
    usb_remove_hcd(hcd);
    put_usb3_hcd:
    usb_put_hcd(xhci.shared_hcd);
    disable_host:
    xhci_histb_host_disable(histb);
    put_hcd:
    usb_put_hcd(hcd);
    disable_pm:
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xhci_histb_remove(dev: *mut platform_device) {
    static void xhci_histb_remove(struct platform_device *dev)
    {
    struct xhci_hcd_histb *histb = platform_get_drvdata(dev);
    struct usb_hcd *hcd = histb.hcd;
    struct xhci_hcd	*xhci = hcd_to_xhci(hcd);
    struct usb_hcd *shared_hcd = xhci.shared_hcd;
    xhci.xhc_state |= XHCI_STATE_REMOVING;
    usb_remove_hcd(shared_hcd);
    xhci.shared_hcd = core::ptr::null_mut();
    device_wakeup_disable(&dev.dev);
    usb_remove_hcd(hcd);
    usb_put_hcd(shared_hcd);
    xhci_histb_host_disable(histb);
    usb_put_hcd(hcd);
    pm_runtime_put_sync(&dev.dev);
    pm_runtime_disable(&dev.dev);
    }
#[no_mangle]
unsafe extern "C" fn xhci_histb_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused xhci_histb_suspend(struct device *dev)
    {
    struct xhci_hcd_histb *histb = dev_get_drvdata(dev);
    struct usb_hcd *hcd = histb.hcd;
    struct xhci_hcd	*xhci = hcd_to_xhci(hcd);
    int ret;
    ret = xhci_suspend(xhci, device_may_wakeup(dev));
    if (!device_may_wakeup(dev))
    xhci_histb_host_disable(histb);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xhci_histb_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused xhci_histb_resume(struct device *dev)
    {
    struct xhci_hcd_histb *histb = dev_get_drvdata(dev);
    struct usb_hcd *hcd = histb.hcd;
    struct xhci_hcd *xhci = hcd_to_xhci(hcd);
    if (!device_may_wakeup(dev))
    xhci_histb_host_enable(histb);
    return xhci_resume(xhci, false, false);
    }
    static const struct dev_pm_ops xhci_histb_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(xhci_histb_suspend, xhci_histb_resume)
    };

    static const struct of_device_id histb_xhci_of_match[] = {
    { .compatible = "hisilicon,hi3798cv200-xhci"},
    { },
    };
    MODULE_DEVICE_TABLE(of, histb_xhci_of_match);

    static struct platform_driver histb_xhci_driver = {
    .probe	= xhci_histb_probe,
    .remove = xhci_histb_remove,
    .driver	= {
    .name = "xhci-histb",
    .pm = DEV_PM_OPS,
    .of_match_table = of_match_ptr(histb_xhci_of_match),
    },
    };
    MODULE_ALIAS("platform:xhci-histb");
#[no_mangle]
unsafe extern "C" fn xhci_histb_init() -> int __init {
    static int __init xhci_histb_init(void)
    {
    xhci_init_driver(&xhci_histb_hc_driver, &xhci_histb_overrides);
    return platform_driver_register(&histb_xhci_driver);
    }
    module_init(xhci_histb_init);
#[no_mangle]
unsafe extern "C" fn xhci_histb_exit() -> void __exit {
    static void __exit xhci_histb_exit(void)
    {
    platform_driver_unregister(&histb_xhci_driver);
    }
    module_exit(xhci_histb_exit);
    MODULE_DESCRIPTION("HiSilicon STB xHCI Host Controller Driver");
    MODULE_LICENSE("GPL v2");
