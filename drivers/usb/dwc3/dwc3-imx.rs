//! Automatically rewritten from C to Rust
//! Source: drivers/usb/dwc3/dwc3-imx.c
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
// dwc3-imx.c - NXP i.MX Soc USB3 Specific Glue layer
//
// Copyright 2026 NXP
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
pub struct dwc3_imx {
    pub dwc: dwc3,
    pub dev: *mut device,
    pub blkctl_base: *mut void __iomem,
    pub glue_base: *mut void __iomem,
    pub hsio_clk: *mut clk,
    pub suspend_clk: *mut clk,
    pub irq: c_int,
    pub pm_suspended: bool,
    pub wakeup_pending: bool,
    pub permanent_attached:1: unsigned,
    pub disable_pwr_ctrl:1: unsigned,
    pub overcur_active_low:1: unsigned,
    pub power_active_low:1: unsigned,
}

#[no_mangle]
unsafe extern "C" fn dwc3_imx_get_property(dwc_imx: *mut dwc3_imx) {
    static void dwc3_imx_get_property(struct dwc3_imx *dwc_imx)
    {
    struct device	*dev = dwc_imx.dev;
    dwc_imx.permanent_attached =
    device_property_read_bool(dev, "fsl,permanently-attached");
    dwc_imx.disable_pwr_ctrl =
    device_property_read_bool(dev, "fsl,disable-port-power-control");
    dwc_imx.overcur_active_low =
    device_property_read_bool(dev, "fsl,over-current-active-low");
    dwc_imx.power_active_low =
    device_property_read_bool(dev, "fsl,power-active-low");
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_configure_glue(dwc_imx: *mut dwc3_imx) {
    static void dwc3_imx_configure_glue(struct dwc3_imx *dwc_imx)
    {
    u32		value;
    if (!dwc_imx.glue_base)
    return;
    value = readl(dwc_imx.glue_base + USB_CTRL0);
    if (dwc_imx.permanent_attached)
    value |= USB_CTRL0_USB2_FIXED | USB_CTRL0_USB3_FIXED;
    else
    value &= ~(USB_CTRL0_USB2_FIXED | USB_CTRL0_USB3_FIXED);
    if (dwc_imx.disable_pwr_ctrl)
    value &= ~USB_CTRL0_PORTPWR_EN;
    else
    value |= USB_CTRL0_PORTPWR_EN;
    writel(value, dwc_imx.glue_base + USB_CTRL0);
    value = readl(dwc_imx.glue_base + USB_CTRL1);
    if (dwc_imx.overcur_active_low)
    value |= USB_CTRL1_OC_POLARITY;
    else
    value &= ~USB_CTRL1_OC_POLARITY;
    if (dwc_imx.power_active_low)
    value |= USB_CTRL1_PWR_POLARITY;
    else
    value &= ~USB_CTRL1_PWR_POLARITY;
    writel(value, dwc_imx.glue_base + USB_CTRL1);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_wakeup_enable(dwc_imx: *mut dwc3_imx, msg: pm_message_t) {
    static void dwc3_imx_wakeup_enable(struct dwc3_imx *dwc_imx, pm_message_t msg)
    {
    struct dwc3	*dwc = &dwc_imx.dwc;
    u32		val;
    val = readl(dwc_imx.blkctl_base + USB_WAKEUP_CTRL);
    if (dwc.current_dr_role == DWC3_GCTL_PRTCAP_HOST && dwc.xhci) {
    val |= USB_WAKEUP_EN | USB_WAKEUP_DPDM_EN;
    if (PMSG_IS_AUTO(msg))
    val |= USB_WAKEUP_SS_CONN | USB_WAKEUP_U3_EN;
    } else {
    val |= USB_WAKEUP_EN | USB_WAKEUP_VBUS_EN |
    USB_WAKEUP_VBUS_SRC_SESS_VAL;
    }
    writel(val, dwc_imx.blkctl_base + USB_WAKEUP_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_wakeup_disable(dwc_imx: *mut dwc3_imx) {
    static void dwc3_imx_wakeup_disable(struct dwc3_imx *dwc_imx)
    {
    u32	val;
    val = readl(dwc_imx.blkctl_base + USB_WAKEUP_CTRL);
    val &= ~(USB_WAKEUP_EN | USB_WAKEUP_EN_MASK);
    writel(val, dwc_imx.blkctl_base + USB_WAKEUP_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t dwc3_imx_interrupt(int irq, void *data)
    {
    struct dwc3_imx	*dwc_imx = data;
    struct dwc3	*dwc = &dwc_imx.dwc;
    if (!dwc_imx.pm_suspended)
    return IRQ_HANDLED;
    disable_irq_nosync(dwc_imx.irq);
    dwc_imx.wakeup_pending = true;
    if (dwc.current_dr_role == DWC3_GCTL_PRTCAP_HOST && dwc.xhci)
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
    static struct dwc3_glue_ops dwc3_imx_glue_ops = {
    .pre_set_role = dwc3_imx_pre_set_role,
    };
    static const struct property_entry dwc3_imx_properties[] = {
    PROPERTY_ENTRY_BOOL("xhci-missing-cas-quirk"),
    PROPERTY_ENTRY_BOOL("xhci-skip-phy-init-quirk"),
    {},
    };
    static const struct software_node dwc3_imx_swnode = {
    .properties = dwc3_imx_properties,
    };
#[no_mangle]
unsafe extern "C" fn dwc3_imx_probe(pdev: *mut platform_device) -> c_int {
    static int dwc3_imx_probe(struct platform_device *pdev)
    {
    struct device		*dev = &pdev.dev;
    struct dwc3_imx		*dwc_imx;
    struct dwc3		*dwc;
    struct resource		*res;
    const char		*irq_name;
    let mut probe_data: dwc3_probe_data = {};
    int			ret, irq;
    dwc_imx = devm_kzalloc(dev, sizeof(*dwc_imx), GFP_KERNEL);
    if (!dwc_imx)
    return -ENOMEM;
    platform_set_drvdata(pdev, dwc_imx);
    dwc_imx.dev = dev;
    dwc3_imx_get_property(dwc_imx);
    dwc_imx.blkctl_base = devm_platform_ioremap_resource_byname(pdev, "blkctl");
    if (IS_ERR(dwc_imx.blkctl_base))
    return PTR_ERR(dwc_imx.blkctl_base);
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "glue");
    if (!res) {
    dev_warn(dev, "Base address for glue layer missing\n");
    } else {
    dwc_imx.glue_base = devm_ioremap_resource(dev, res);
    if (IS_ERR(dwc_imx.glue_base))
    return PTR_ERR(dwc_imx.glue_base);
    }
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "core");
    if (!res)
    return dev_err_probe(dev, -ENODEV, "missing core memory resource\n");
    dwc_imx.hsio_clk = devm_clk_get_enabled(dev, "hsio");
    if (IS_ERR(dwc_imx.hsio_clk))
    return dev_err_probe(dev, PTR_ERR(dwc_imx.hsio_clk),
    "Failed to get hsio clk\n");
    dwc_imx.suspend_clk = devm_clk_get_enabled(dev, "suspend");
    if (IS_ERR(dwc_imx.suspend_clk))
    return dev_err_probe(dev, PTR_ERR(dwc_imx.suspend_clk),
    "Failed to get suspend clk\n");
    irq = platform_get_irq_byname(pdev, "wakeup");
    if (irq < 0)
    return irq;
    dwc_imx.irq = irq;
    irq_name = devm_kasprintf(dev, GFP_KERNEL, "%s:wakeup", dev_name(dev));
    if (!irq_name)
    return dev_err_probe(dev, -ENOMEM, "failed to create irq_name\n");
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), dwc3_imx_interrupt,
    IRQF_ONESHOT | IRQF_NO_AUTOEN,
    irq_name, dwc_imx);
    if (ret)
    return dev_err_probe(dev, ret, "failed to request IRQ #%d\n", irq);
    ret = device_add_software_node(dev, &dwc3_imx_swnode);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add software node\n");
    dwc3_imx_configure_glue(dwc_imx);
    dwc = &dwc_imx.dwc;
    dwc.dev = dev;
    dwc.glue_ops = &dwc3_imx_glue_ops;
    probe_data.res = res;
    probe_data.dwc = dwc;
    probe_data.properties = DWC3_DEFAULT_PROPERTIES;
    probe_data.properties.needs_full_reinit = true;
    ret = dwc3_core_probe(&probe_data);
    if (ret) {
    device_remove_software_node(dev);
    return ret;
    }
    device_set_wakeup_capable(dev, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_remove(pdev: *mut platform_device) {
    static void dwc3_imx_remove(struct platform_device *pdev)
    {
    struct device	*dev = &pdev.dev;
    struct dwc3	*dwc = dev_get_drvdata(dev);
    dwc3_core_remove(dwc);
    device_remove_software_node(dev);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_suspend(dwc_imx: *mut dwc3_imx, msg: pm_message_t) {
    static void dwc3_imx_suspend(struct dwc3_imx *dwc_imx, pm_message_t msg)
    {
    if (dwc_imx.pm_suspended)
    return;
    if (PMSG_IS_AUTO(msg) || device_may_wakeup(dwc_imx.dev))
    dwc3_imx_wakeup_enable(dwc_imx, msg);
    enable_irq(dwc_imx.irq);
    dwc_imx.pm_suspended = true;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_resume(dwc_imx: *mut dwc3_imx, msg: pm_message_t) {
    static void dwc3_imx_resume(struct dwc3_imx *dwc_imx, pm_message_t msg)
    {
    struct dwc3	*dwc = &dwc_imx.dwc;
    if (!dwc_imx.pm_suspended)
    return;
    dwc_imx.pm_suspended = false;
    if (!dwc_imx.wakeup_pending)
    disable_irq_nosync(dwc_imx.irq);
    dwc3_imx_wakeup_disable(dwc_imx);
// Upon power loss any previous configuration is lost, restore it
    dwc3_imx_configure_glue(dwc_imx);
    if (dwc_imx.wakeup_pending) {
    dwc_imx.wakeup_pending = false;
    if (dwc.current_dr_role == DWC3_GCTL_PRTCAP_DEVICE)
    pm_runtime_put_autosuspend(dwc.dev);
    else
//
// Add wait for xhci switch from suspend
// clock to normal clock to detect connection.
//
    usleep_range(9000, 10000);
    }
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_runtime_suspend(dev: *mut device) -> c_int {
    static int dwc3_imx_runtime_suspend(struct device *dev)
    {
    struct dwc3	*dwc = dev_get_drvdata(dev);
    struct dwc3_imx	*dwc_imx = to_dwc3_imx(dwc);
    int		ret;
    ret = dwc3_runtime_suspend(dwc);
    if (ret)
    return ret;
    dwc3_imx_suspend(dwc_imx, PMSG_AUTO_SUSPEND);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_runtime_resume(dev: *mut device) -> c_int {
    static int dwc3_imx_runtime_resume(struct device *dev)
    {
    struct dwc3	*dwc = dev_get_drvdata(dev);
    struct dwc3_imx	*dwc_imx = to_dwc3_imx(dwc);
    dwc3_imx_resume(dwc_imx, PMSG_AUTO_RESUME);
    return dwc3_runtime_resume(dwc);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_runtime_idle(dev: *mut device) -> c_int {
    static int dwc3_imx_runtime_idle(struct device *dev)
    {
    return dwc3_runtime_idle(dev_get_drvdata(dev));
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_pm_suspend(dev: *mut device) -> c_int {
    static int dwc3_imx_pm_suspend(struct device *dev)
    {
    struct dwc3	*dwc = dev_get_drvdata(dev);
    struct dwc3_imx *dwc_imx = to_dwc3_imx(dwc);
    int		ret;
    ret = dwc3_pm_suspend(dwc);
    if (ret)
    return ret;
    dwc3_imx_suspend(dwc_imx, PMSG_SUSPEND);
    if (device_may_wakeup(dev)) {
    enable_irq_wake(dwc_imx.irq);
    device_set_out_band_wakeup(dev);
    } else {
    clk_disable_unprepare(dwc_imx.suspend_clk);
    }
    clk_disable_unprepare(dwc_imx.hsio_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_pm_resume(dev: *mut device) -> c_int {
    static int dwc3_imx_pm_resume(struct device *dev)
    {
    struct dwc3	*dwc = dev_get_drvdata(dev);
    struct dwc3_imx *dwc_imx = to_dwc3_imx(dwc);
    int		ret;
    if (device_may_wakeup(dwc_imx.dev)) {
    disable_irq_wake(dwc_imx.irq);
    } else {
    ret = clk_prepare_enable(dwc_imx.suspend_clk);
    if (ret)
    return ret;
    }
    ret = clk_prepare_enable(dwc_imx.hsio_clk);
    if (ret) {
    clk_disable_unprepare(dwc_imx.suspend_clk);
    return ret;
    }
    dwc3_imx_resume(dwc_imx, PMSG_RESUME);
    ret = dwc3_pm_resume(dwc);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_complete(dev: *mut device) {
    static void dwc3_imx_complete(struct device *dev)
    {
    dwc3_pm_complete(dev_get_drvdata(dev));
    }
#[no_mangle]
unsafe extern "C" fn dwc3_imx_prepare(dev: *mut device) -> c_int {
    static int dwc3_imx_prepare(struct device *dev)
    {
    return dwc3_pm_prepare(dev_get_drvdata(dev));
    }
    static const struct dev_pm_ops dwc3_imx_dev_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(dwc3_imx_pm_suspend, dwc3_imx_pm_resume)
    RUNTIME_PM_OPS(dwc3_imx_runtime_suspend, dwc3_imx_runtime_resume,
    dwc3_imx_runtime_idle)
    .complete = pm_sleep_ptr(dwc3_imx_complete),
    .prepare = pm_sleep_ptr(dwc3_imx_prepare),
    };
    static const struct of_device_id dwc3_imx_of_match[] = {
    { .compatible = "nxp,imx8mp-dwc3", },
    {},
    };
    MODULE_DEVICE_TABLE(of, dwc3_imx_of_match);
    static struct platform_driver dwc3_imx_driver = {
    .probe		= dwc3_imx_probe,
    .remove		= dwc3_imx_remove,
    .driver		= {
    .name	= "imx-dwc3",
    .pm	= pm_ptr(&dwc3_imx_dev_pm_ops),
    .of_match_table	= dwc3_imx_of_match,
    },
    };
    module_platform_driver(dwc3_imx_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("DesignWare USB3 i.MX Glue Layer");
