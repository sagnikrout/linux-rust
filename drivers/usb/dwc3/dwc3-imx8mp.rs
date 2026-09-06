//! Automatically rewritten from C to Rust
//! Source: drivers/usb/dwc3/dwc3-imx8mp.c
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
// dwc3-imx8mp.c - NXP imx8mp Specific Glue layer
//
// Copyright (c) 2020 NXP.
//

// USB wakeup registers
pub const USB_WAKEUP_CTRL: c_uint = 0x00;
// Global wakeup interrupt enable, also used to clear interrupt

// Wakeup from connect or disconnect, only for superspeed

// 0 select vbus_valid, 1 select sessvld

// Enable signal for wake up from u3 state

// Enable signal for wake up from id change

// Enable signal for wake up from vbus change

// Enable signal for wake up from dp/dm change

// USB glue registers
pub const USB_CTRL0: c_uint = 0x00;
pub const USB_CTRL1: c_uint = 0x04;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_imx8mp {
    pub dev: *mut device,
    pub dwc3_pdev: *mut platform_device,
    pub hsio_blk_base: *mut void __iomem,
    pub glue_base: *mut void __iomem,
    pub hsio_clk: *mut clk,
    pub suspend_clk: *mut clk,
    pub irq: c_int,
    pub pm_suspended: bool,
    pub wakeup_pending: bool,
}

#[no_mangle]
unsafe extern "C" fn imx8mp_configure_glue(dwc3_imx: *mut dwc3_imx8mp) {
    static void imx8mp_configure_glue(struct dwc3_imx8mp *dwc3_imx)
    {
    struct device *dev = dwc3_imx.dev;
    u32 value;
    if (!dwc3_imx.glue_base)
    return;
    value = readl(dwc3_imx.glue_base + USB_CTRL0);
    if (device_property_read_bool(dev, "fsl,permanently-attached"))
    value |= (USB_CTRL0_USB2_FIXED | USB_CTRL0_USB3_FIXED);
    else
    value &= ~(USB_CTRL0_USB2_FIXED | USB_CTRL0_USB3_FIXED);
    if (device_property_read_bool(dev, "fsl,disable-port-power-control"))
    value &= ~(USB_CTRL0_PORTPWR_EN);
    else
    value |= USB_CTRL0_PORTPWR_EN;
    writel(value, dwc3_imx.glue_base + USB_CTRL0);
    value = readl(dwc3_imx.glue_base + USB_CTRL1);
    if (device_property_read_bool(dev, "fsl,over-current-active-low"))
    value |= USB_CTRL1_OC_POLARITY;
    else
    value &= ~USB_CTRL1_OC_POLARITY;
    if (device_property_read_bool(dev, "fsl,power-active-low"))
    value |= USB_CTRL1_PWR_POLARITY;
    else
    value &= ~USB_CTRL1_PWR_POLARITY;
    writel(value, dwc3_imx.glue_base + USB_CTRL1);
    }
    static void dwc3_imx8mp_wakeup_enable(struct dwc3_imx8mp *dwc3_imx,
    pm_message_t msg)
    {
    struct dwc3	*dwc3 = platform_get_drvdata(dwc3_imx.dwc3_pdev);
    u32		val;
    if (!dwc3)
    return;
    val = readl(dwc3_imx.hsio_blk_base + USB_WAKEUP_CTRL);
    if ((dwc3.current_dr_role == DWC3_GCTL_PRTCAP_HOST) && dwc3.xhci) {
    val |= USB_WAKEUP_EN | USB_WAKEUP_DPDM_EN;
    if (PMSG_IS_AUTO(msg))
    val |= USB_WAKEUP_SS_CONN | USB_WAKEUP_U3_EN;
    } else {
    val |= USB_WAKEUP_EN | USB_WAKEUP_VBUS_EN |
    USB_WAKEUP_VBUS_SRC_SESS_VAL;
    }
    writel(val, dwc3_imx.hsio_blk_base + USB_WAKEUP_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx8mp_wakeup_disable(dwc3_imx: *mut dwc3_imx8mp) {
    static void dwc3_imx8mp_wakeup_disable(struct dwc3_imx8mp *dwc3_imx)
    {
    u32 val;
    val = readl(dwc3_imx.hsio_blk_base + USB_WAKEUP_CTRL);
    val &= ~(USB_WAKEUP_EN | USB_WAKEUP_EN_MASK);
    writel(val, dwc3_imx.hsio_blk_base + USB_WAKEUP_CTRL);
    }
    static const struct property_entry dwc3_imx8mp_properties[] = {
    PROPERTY_ENTRY_BOOL("xhci-missing-cas-quirk"),
    PROPERTY_ENTRY_BOOL("xhci-skip-phy-init-quirk"),
    {},
    };
    static const struct software_node dwc3_imx8mp_swnode = {
    .properties = dwc3_imx8mp_properties,
    };
#[no_mangle]
unsafe extern "C" fn dwc3_imx8mp_interrupt(irq: c_int, _dwc3_imx: *mut c_void) -> irqreturn_t {
    static irqreturn_t dwc3_imx8mp_interrupt(int irq, void *_dwc3_imx)
    {
    struct dwc3_imx8mp	*dwc3_imx = _dwc3_imx;
    struct dwc3		*dwc = platform_get_drvdata(dwc3_imx.dwc3_pdev);
    if (!dwc3_imx.pm_suspended)
    return IRQ_HANDLED;
    disable_irq_nosync(dwc3_imx.irq);
    dwc3_imx.wakeup_pending = true;
    if ((dwc.current_dr_role == DWC3_GCTL_PRTCAP_HOST) && dwc.xhci)
    pm_runtime_resume(&dwc.xhci.dev);
#[no_mangle]
pub unsafe extern "C" fn if(DWC3_GCTL_PRTCAP_DEVICE: dwc->current_dr_role ==) -> else {
    else if (dwc.current_dr_role == DWC3_GCTL_PRTCAP_DEVICE)
    pm_runtime_get(dwc.dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_pre_set_role(dwc: *mut dwc3, role: enum usb_role) {
    static void dwc3_imx_pre_set_role(struct dwc3 *dwc, enum usb_role role)
    {
    if (role == USB_ROLE_HOST)
//
// For xhci host, we need disable dwc core auto
// suspend, because during this auto suspend delay(5s),
// xhci host RUN_STOP is cleared and wakeup is not
// enabled, if device is inserted, xhci host can't
// response the connection.
//
    pm_runtime_dont_use_autosuspend(dwc.dev);
    else
    pm_runtime_use_autosuspend(dwc.dev);
    }
    struct dwc3_glue_ops dwc3_imx_glue_ops = {
    .pre_set_role   = dwc3_imx_pre_set_role,
    };
#[no_mangle]
unsafe extern "C" fn dwc3_imx8mp_probe(pdev: *mut platform_device) -> c_int {
    static int dwc3_imx8mp_probe(struct platform_device *pdev)
    {
    struct device		*dev = &pdev.dev;
    struct device_node	*node = dev.of_node;
    struct dwc3_imx8mp	*dwc3_imx;
    struct dwc3		*dwc3;
    struct resource		*res;
    int			err, irq;
    if (!node) {
    dev_err(dev, "device node not found\n");
    return -EINVAL;
    }
    dwc3_imx = devm_kzalloc(dev, sizeof(*dwc3_imx), GFP_KERNEL);
    if (!dwc3_imx)
    return -ENOMEM;
    platform_set_drvdata(pdev, dwc3_imx);
    dwc3_imx.dev = dev;
    dwc3_imx.hsio_blk_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(dwc3_imx.hsio_blk_base))
    return PTR_ERR(dwc3_imx.hsio_blk_base);
    res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if (!res) {
    dev_warn(dev, "Base address for glue layer missing. Continuing without, some features are missing though.");
    } else {
    dwc3_imx.glue_base = devm_ioremap_resource(dev, res);
    if (IS_ERR(dwc3_imx.glue_base))
    return PTR_ERR(dwc3_imx.glue_base);
    }
    dwc3_imx.hsio_clk = devm_clk_get_enabled(dev, "hsio");
    if (IS_ERR(dwc3_imx.hsio_clk))
    return dev_err_probe(dev, PTR_ERR(dwc3_imx.hsio_clk),
    "Failed to get hsio clk\n");
    dwc3_imx.suspend_clk = devm_clk_get_enabled(dev, "suspend");
    if (IS_ERR(dwc3_imx.suspend_clk))
    return dev_err_probe(dev, PTR_ERR(dwc3_imx.suspend_clk),
    "Failed to get suspend clk\n");
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    dwc3_imx.irq = irq;
    struct device_node *dwc3_np __free(device_node) = of_get_compatible_child(node,
    "snps,dwc3");
    if (!dwc3_np)
    return dev_err_probe(dev, -ENODEV, "failed to find dwc3 core child\n");
    imx8mp_configure_glue(dwc3_imx);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    err = pm_runtime_get_sync(dev);
    if (err < 0)
    goto disable_rpm;
    err = device_add_software_node(dev, &dwc3_imx8mp_swnode);
    if (err) {
    err = -ENODEV;
    dev_err(dev, "failed to add software node\n");
    goto disable_rpm;
    }
    err = of_platform_populate(node, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    if (err) {
    dev_err(&pdev.dev, "failed to create dwc3 core\n");
    goto remove_swnode;
    }
    dwc3_imx.dwc3_pdev = of_find_device_by_node(dwc3_np);
    if (!dwc3_imx.dwc3_pdev) {
    dev_err(dev, "failed to get dwc3 platform device\n");
    err = -ENODEV;
    goto depopulate;
    }
    dwc3 = platform_get_drvdata(dwc3_imx.dwc3_pdev);
    if (!dwc3) {
    err = dev_err_probe(dev, -EPROBE_DEFER, "failed to get dwc3 platform data\n");
    goto put_dwc3;
    }
    dwc3.glue_ops = &dwc3_imx_glue_ops;
    if (dwc3.dr_mode == USB_DR_MODE_HOST)
    pm_runtime_dont_use_autosuspend(dwc3.dev);
    err = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), dwc3_imx8mp_interrupt,
    IRQF_ONESHOT, dev_name(dev), dwc3_imx);
    if (err) {
    dev_err(dev, "failed to request IRQ #%d -. %d\n", irq, err);
    goto put_dwc3;
    }
    device_set_wakeup_capable(dev, true);
    pm_runtime_put(dev);
    return 0;
    put_dwc3:
    put_device(&dwc3_imx.dwc3_pdev.dev);
    depopulate:
    of_platform_depopulate(dev);
    remove_swnode:
    device_remove_software_node(dev);
    disable_rpm:
    pm_runtime_disable(dev);
    pm_runtime_put_noidle(dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx8mp_remove(pdev: *mut platform_device) {
    static void dwc3_imx8mp_remove(struct platform_device *pdev)
    {
    struct dwc3_imx8mp *dwc3_imx = platform_get_drvdata(pdev);
    struct device *dev = &pdev.dev;
    put_device(&dwc3_imx.dwc3_pdev.dev);
    pm_runtime_get_sync(dev);
    of_platform_depopulate(dev);
    device_remove_software_node(dev);
    pm_runtime_disable(dev);
    pm_runtime_put_noidle(dev);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx8mp_suspend(dwc3_imx: *mut dwc3_imx8mp, msg: pm_message_t) -> c_int {
    static int dwc3_imx8mp_suspend(struct dwc3_imx8mp *dwc3_imx, pm_message_t msg)
    {
    if (dwc3_imx.pm_suspended)
    return 0;
// Wakeup enable
    if (PMSG_IS_AUTO(msg) || device_may_wakeup(dwc3_imx.dev))
    dwc3_imx8mp_wakeup_enable(dwc3_imx, msg);
    dwc3_imx.pm_suspended = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx8mp_resume(dwc3_imx: *mut dwc3_imx8mp, msg: pm_message_t) -> c_int {
    static int dwc3_imx8mp_resume(struct dwc3_imx8mp *dwc3_imx, pm_message_t msg)
    {
    struct dwc3	*dwc = platform_get_drvdata(dwc3_imx.dwc3_pdev);
    let mut ret: c_int = 0;
    if (!dwc3_imx.pm_suspended)
    return 0;
// Wakeup disable
    dwc3_imx8mp_wakeup_disable(dwc3_imx);
    dwc3_imx.pm_suspended = false;
// Upon power loss any previous configuration is lost, restore it
    imx8mp_configure_glue(dwc3_imx);
    if (dwc3_imx.wakeup_pending) {
    dwc3_imx.wakeup_pending = false;
    if (dwc.current_dr_role == DWC3_GCTL_PRTCAP_DEVICE) {
    pm_runtime_put_autosuspend(dwc.dev);
    } else {
//
// Add wait for xhci switch from suspend
// clock to normal clock to detect connection.
//
    usleep_range(9000, 10000);
    }
    enable_irq(dwc3_imx.irq);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx8mp_pm_suspend(dev: *mut device) -> c_int {
    static int dwc3_imx8mp_pm_suspend(struct device *dev)
    {
    struct dwc3_imx8mp *dwc3_imx = dev_get_drvdata(dev);
    int ret;
    ret = dwc3_imx8mp_suspend(dwc3_imx, PMSG_SUSPEND);
    if (device_may_wakeup(dwc3_imx.dev)) {
    enable_irq_wake(dwc3_imx.irq);
    if (device_is_compatible(dev, "fsl,imx95-dwc3"))
    device_set_out_band_wakeup(dev);
    } else {
    clk_disable_unprepare(dwc3_imx.suspend_clk);
    }
    clk_disable_unprepare(dwc3_imx.hsio_clk);
    dev_dbg(dev, "dwc3 imx8mp pm suspend.\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx8mp_pm_resume(dev: *mut device) -> c_int {
    static int dwc3_imx8mp_pm_resume(struct device *dev)
    {
    struct dwc3_imx8mp *dwc3_imx = dev_get_drvdata(dev);
    int ret;
    if (device_may_wakeup(dwc3_imx.dev)) {
    disable_irq_wake(dwc3_imx.irq);
    } else {
    ret = clk_prepare_enable(dwc3_imx.suspend_clk);
    if (ret)
    return ret;
    }
    ret = clk_prepare_enable(dwc3_imx.hsio_clk);
    if (ret) {
    clk_disable_unprepare(dwc3_imx.suspend_clk);
    return ret;
    }
    ret = dwc3_imx8mp_resume(dwc3_imx, PMSG_RESUME);
    pm_runtime_disable(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    dev_dbg(dev, "dwc3 imx8mp pm resume.\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx8mp_runtime_suspend(dev: *mut device) -> c_int {
    static int dwc3_imx8mp_runtime_suspend(struct device *dev)
    {
    struct dwc3_imx8mp *dwc3_imx = dev_get_drvdata(dev);
    dev_dbg(dev, "dwc3 imx8mp runtime suspend.\n");
    return dwc3_imx8mp_suspend(dwc3_imx, PMSG_AUTO_SUSPEND);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx8mp_runtime_resume(dev: *mut device) -> c_int {
    static int dwc3_imx8mp_runtime_resume(struct device *dev)
    {
    struct dwc3_imx8mp *dwc3_imx = dev_get_drvdata(dev);
    dev_dbg(dev, "dwc3 imx8mp runtime resume.\n");
    return dwc3_imx8mp_resume(dwc3_imx, PMSG_AUTO_RESUME);
    }
    static const struct dev_pm_ops dwc3_imx8mp_dev_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(dwc3_imx8mp_pm_suspend, dwc3_imx8mp_pm_resume)
    RUNTIME_PM_OPS(dwc3_imx8mp_runtime_suspend, dwc3_imx8mp_runtime_resume,
    core::ptr::null_mut())
    };
    static const struct of_device_id dwc3_imx8mp_of_match[] = {
    { .compatible = "fsl,imx8mp-dwc3", },
    {},
    };
    MODULE_DEVICE_TABLE(of, dwc3_imx8mp_of_match);
    static struct platform_driver dwc3_imx8mp_driver = {
    .probe		= dwc3_imx8mp_probe,
    .remove		= dwc3_imx8mp_remove,
    .driver		= {
    .name	= "imx8mp-dwc3",
    .pm	= pm_ptr(&dwc3_imx8mp_dev_pm_ops),
    .of_match_table	= dwc3_imx8mp_of_match,
    },
    };
    module_platform_driver(dwc3_imx8mp_driver);
    MODULE_ALIAS("platform:imx8mp-dwc3");
    MODULE_AUTHOR("jun.li@nxp.com");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("DesignWare USB3 imx8mp Glue Layer");
