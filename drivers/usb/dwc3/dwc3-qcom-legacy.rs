//! Automatically rewritten from C to Rust
//! Source: drivers/usb/dwc3/dwc3-qcom-legacy.c
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
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
//
// Inspired by dwc3-of-simple.c
//

// USB QSCRATCH Hardware registers
pub const QSCRATCH_HS_PHY_CTRL: c_uint = 0x10;

pub const QSCRATCH_SS_PHY_CTRL: c_uint = 0x30;

pub const QSCRATCH_GENERAL_CFG: c_uint = 0x08;

pub const SDM845_QSCRATCH_BASE_OFFSET: c_uint = 0xf8800;
pub const SDM845_QSCRATCH_SIZE: c_uint = 0x400;
pub const SDM845_DWC3_CORE_SIZE: c_uint = 0xcd00;
// Interconnect path bandwidths in MBps

pub const APPS_USB_AVG_BW: c_int = 0;

// Qualcomm SoCs with multiport support has up to 4 ports
pub const DWC3_QCOM_MAX_PORTS: c_int = 4;
    static const u32 pwr_evnt_irq_stat_reg[DWC3_QCOM_MAX_PORTS] = {
    0x58,
    0x1dc,
    0x228,
    0x238,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_qcom_port {
    pub qusb2_phy_irq: c_int,
    pub dp_hs_phy_irq: c_int,
    pub dm_hs_phy_irq: c_int,
    pub ss_phy_irq: c_int,
    pub usb2_speed: enum usb_device_speed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_qcom {
    pub dev: *mut device,
    pub qscratch_base: *mut void __iomem,
    pub dwc3: *mut platform_device,
    pub clks: *mut clk,
    pub num_clocks: c_int,
    pub resets: *mut reset_control,
    pub ports: [dwc3_qcom_port; DWC3_QCOM_MAX_PORTS],
    pub num_ports: u8,
    pub edev: *mut extcon_dev,
    pub host_edev: *mut extcon_dev,
    pub vbus_nb: notifier_block,
    pub host_nb: notifier_block,
    pub mode: enum usb_dr_mode,
    pub is_suspended: bool,
    pub pm_suspended: bool,
    pub icc_path_ddr: *mut icc_path,
    pub icc_path_apps: *mut icc_path,
}

#[no_mangle]
pub unsafe extern "C" fn dwc3_qcom_setbits(base: *mut void __iomem, offset: u32, val: u32) {
    static inline void dwc3_qcom_setbits(void __iomem *base, u32 offset, u32 val)
    {
    u32 reg;
    reg = readl(base + offset);
    reg |= val;
    writel(reg, base + offset);
// ensure that above write is through
    readl(base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_qcom_clrbits(base: *mut void __iomem, offset: u32, val: u32) {
    static inline void dwc3_qcom_clrbits(void __iomem *base, u32 offset, u32 val)
    {
    u32 reg;
    reg = readl(base + offset);
    reg &= ~val;
    writel(reg, base + offset);
// ensure that above write is through
    readl(base + offset);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_vbus_override_enable(qcom: *mut dwc3_qcom, enable: bool) {
    static void dwc3_qcom_vbus_override_enable(struct dwc3_qcom *qcom, bool enable)
    {
    if (enable) {
    dwc3_qcom_setbits(qcom.qscratch_base, QSCRATCH_SS_PHY_CTRL,
    LANE0_PWR_PRESENT);
    dwc3_qcom_setbits(qcom.qscratch_base, QSCRATCH_HS_PHY_CTRL,
    UTMI_OTG_VBUS_VALID | SW_SESSVLD_SEL);
    } else {
    dwc3_qcom_clrbits(qcom.qscratch_base, QSCRATCH_SS_PHY_CTRL,
    LANE0_PWR_PRESENT);
    dwc3_qcom_clrbits(qcom.qscratch_base, QSCRATCH_HS_PHY_CTRL,
    UTMI_OTG_VBUS_VALID | SW_SESSVLD_SEL);
    }
    }
    static int dwc3_qcom_vbus_notifier(struct notifier_block *nb,
    unsigned long event, void *ptr)
    {
    struct dwc3_qcom *qcom = container_of(nb, struct dwc3_qcom, vbus_nb);
// enable vbus override for device mode
    dwc3_qcom_vbus_override_enable(qcom, event);
    qcom.mode = event ? USB_DR_MODE_PERIPHERAL : USB_DR_MODE_HOST;
    return NOTIFY_DONE;
    }
    static int dwc3_qcom_host_notifier(struct notifier_block *nb,
    unsigned long event, void *ptr)
    {
    struct dwc3_qcom *qcom = container_of(nb, struct dwc3_qcom, host_nb);
// disable vbus override in host mode
    dwc3_qcom_vbus_override_enable(qcom, !event);
    qcom.mode = event ? USB_DR_MODE_HOST : USB_DR_MODE_PERIPHERAL;
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_register_extcon(qcom: *mut dwc3_qcom) -> c_int {
    static int dwc3_qcom_register_extcon(struct dwc3_qcom *qcom)
    {
    struct device		*dev = qcom.dev;
    struct extcon_dev	*host_edev;
    int			ret;
    if (!of_property_present(dev.of_node, "extcon"))
    return 0;
    qcom.edev = extcon_get_edev_by_phandle(dev, 0);
    if (IS_ERR(qcom.edev))
    return dev_err_probe(dev, PTR_ERR(qcom.edev),
    "Failed to get extcon\n");
    qcom.vbus_nb.notifier_call = dwc3_qcom_vbus_notifier;
    qcom.host_edev = extcon_get_edev_by_phandle(dev, 1);
    if (IS_ERR(qcom.host_edev))
    qcom.host_edev = core::ptr::null_mut();
    ret = devm_extcon_register_notifier(dev, qcom.edev, EXTCON_USB,
    &qcom.vbus_nb);
    if (ret < 0) {
    dev_err(dev, "VBUS notifier register failed\n");
    return ret;
    }
    if (qcom.host_edev)
    host_edev = qcom.host_edev;
    else
    host_edev = qcom.edev;
    qcom.host_nb.notifier_call = dwc3_qcom_host_notifier;
    ret = devm_extcon_register_notifier(dev, host_edev, EXTCON_USB_HOST,
    &qcom.host_nb);
    if (ret < 0) {
    dev_err(dev, "Host notifier register failed\n");
    return ret;
    }
// Update initial VBUS override based on extcon state
    if (extcon_get_state(qcom.edev, EXTCON_USB) ||
    !extcon_get_state(host_edev, EXTCON_USB_HOST))
    dwc3_qcom_vbus_notifier(&qcom.vbus_nb, true, qcom.edev);
    else
    dwc3_qcom_vbus_notifier(&qcom.vbus_nb, false, qcom.edev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_interconnect_enable(qcom: *mut dwc3_qcom) -> c_int {
    static int dwc3_qcom_interconnect_enable(struct dwc3_qcom *qcom)
    {
    int ret;
    ret = icc_enable(qcom.icc_path_ddr);
    if (ret)
    return ret;
    ret = icc_enable(qcom.icc_path_apps);
    if (ret)
    icc_disable(qcom.icc_path_ddr);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_interconnect_disable(qcom: *mut dwc3_qcom) -> c_int {
    static int dwc3_qcom_interconnect_disable(struct dwc3_qcom *qcom)
    {
    int ret;
    ret = icc_disable(qcom.icc_path_ddr);
    if (ret)
    return ret;
    ret = icc_disable(qcom.icc_path_apps);
    if (ret)
    icc_enable(qcom.icc_path_ddr);
    return ret;
    }
//
// dwc3_qcom_interconnect_init() - Get interconnect path handles
// and set bandwidth.
// @qcom:			Pointer to the concerned usb core.
//
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_interconnect_init(qcom: *mut dwc3_qcom) -> c_int {
    static int dwc3_qcom_interconnect_init(struct dwc3_qcom *qcom)
    {
    enum usb_device_speed max_speed;
    struct device *dev = qcom.dev;
    int ret;
    qcom.icc_path_ddr = of_icc_get(dev, "usb-ddr");
    if (IS_ERR(qcom.icc_path_ddr)) {
    return dev_err_probe(dev, PTR_ERR(qcom.icc_path_ddr),
    "failed to get usb-ddr path\n");
    }
    qcom.icc_path_apps = of_icc_get(dev, "apps-usb");
    if (IS_ERR(qcom.icc_path_apps)) {
    ret = dev_err_probe(dev, PTR_ERR(qcom.icc_path_apps),
    "failed to get apps-usb path\n");
    goto put_path_ddr;
    }
    max_speed = usb_get_maximum_speed(&qcom.dwc3.dev);
    if (max_speed >= USB_SPEED_SUPER || max_speed == USB_SPEED_UNKNOWN) {
    ret = icc_set_bw(qcom.icc_path_ddr,
    USB_MEMORY_AVG_SS_BW, USB_MEMORY_PEAK_SS_BW);
    } else {
    ret = icc_set_bw(qcom.icc_path_ddr,
    USB_MEMORY_AVG_HS_BW, USB_MEMORY_PEAK_HS_BW);
    }
    if (ret) {
    dev_err(dev, "failed to set bandwidth for usb-ddr path: %d\n", ret);
    goto put_path_apps;
    }
    ret = icc_set_bw(qcom.icc_path_apps, APPS_USB_AVG_BW, APPS_USB_PEAK_BW);
    if (ret) {
    dev_err(dev, "failed to set bandwidth for apps-usb path: %d\n", ret);
    goto put_path_apps;
    }
    return 0;
    put_path_apps:
    icc_put(qcom.icc_path_apps);
    put_path_ddr:
    icc_put(qcom.icc_path_ddr);
    return ret;
    }
//
// dwc3_qcom_interconnect_exit() - Release interconnect path handles
// @qcom:			Pointer to the concerned usb core.
//
// This function is used to release interconnect path handle.
//
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_interconnect_exit(qcom: *mut dwc3_qcom) {
    static void dwc3_qcom_interconnect_exit(struct dwc3_qcom *qcom)
    {
    icc_put(qcom.icc_path_ddr);
    icc_put(qcom.icc_path_apps);
    }
// Only usable in contexts where the role can not change.
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_is_host(qcom: *mut dwc3_qcom) -> bool {
    static bool dwc3_qcom_is_host(struct dwc3_qcom *qcom)
    {
    struct dwc3 *dwc;
//
// FIXME: Fix this layering violation.
//
    dwc = platform_get_drvdata(qcom.dwc3);
// Core driver may not have probed yet.
    if (!dwc)
    return false;
    return dwc.xhci;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_read_usb2_speed(qcom: *mut dwc3_qcom, port_index: c_int) -> enum usb_device_speed {
    static enum usb_device_speed dwc3_qcom_read_usb2_speed(struct dwc3_qcom *qcom, int port_index)
    {
    struct dwc3 *dwc = platform_get_drvdata(qcom.dwc3);
    struct usb_device *udev;
    struct usb_hcd __maybe_unused *hcd;
//
// FIXME: Fix this layering violation.
//
    hcd = platform_get_drvdata(dwc.xhci);

    udev = usb_hub_find_child(hcd.self.root_hub, port_index + 1);

    udev = core::ptr::null_mut();

    if (!udev)
    return USB_SPEED_UNKNOWN;
    return udev.speed;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_enable_wakeup_irq(irq: c_int, polarity: c_uint) {
    static void dwc3_qcom_enable_wakeup_irq(int irq, unsigned int polarity)
    {
    if (!irq)
    return;
    if (polarity)
    irq_set_irq_type(irq, polarity);
    enable_irq(irq);
    enable_irq_wake(irq);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_disable_wakeup_irq(irq: c_int) {
    static void dwc3_qcom_disable_wakeup_irq(int irq)
    {
    if (!irq)
    return;
    disable_irq_wake(irq);
    disable_irq_nosync(irq);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_disable_port_interrupts(port: *mut dwc3_qcom_port) {
    static void dwc3_qcom_disable_port_interrupts(struct dwc3_qcom_port *port)
    {
    dwc3_qcom_disable_wakeup_irq(port.qusb2_phy_irq);
    if (port.usb2_speed == USB_SPEED_LOW) {
    dwc3_qcom_disable_wakeup_irq(port.dm_hs_phy_irq);
    } else if ((port.usb2_speed == USB_SPEED_HIGH) ||
    (port.usb2_speed == USB_SPEED_FULL)) {
    dwc3_qcom_disable_wakeup_irq(port.dp_hs_phy_irq);
    } else {
    dwc3_qcom_disable_wakeup_irq(port.dp_hs_phy_irq);
    dwc3_qcom_disable_wakeup_irq(port.dm_hs_phy_irq);
    }
    dwc3_qcom_disable_wakeup_irq(port.ss_phy_irq);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_enable_port_interrupts(port: *mut dwc3_qcom_port) {
    static void dwc3_qcom_enable_port_interrupts(struct dwc3_qcom_port *port)
    {
    dwc3_qcom_enable_wakeup_irq(port.qusb2_phy_irq, 0);
//
// Configure DP/DM line interrupts based on the USB2 device attached to
// the root hub port. When HS/FS device is connected, configure the DP line
// as falling edge to detect both disconnect and remote wakeup scenarios. When
// LS device is connected, configure DM line as falling edge to detect both
// disconnect and remote wakeup. When no device is connected, configure both
// DP and DM lines as rising edge to detect HS/HS/LS device connect scenario.
//
    if (port.usb2_speed == USB_SPEED_LOW) {
    dwc3_qcom_enable_wakeup_irq(port.dm_hs_phy_irq,
    IRQ_TYPE_EDGE_FALLING);
    } else if ((port.usb2_speed == USB_SPEED_HIGH) ||
    (port.usb2_speed == USB_SPEED_FULL)) {
    dwc3_qcom_enable_wakeup_irq(port.dp_hs_phy_irq,
    IRQ_TYPE_EDGE_FALLING);
    } else {
    dwc3_qcom_enable_wakeup_irq(port.dp_hs_phy_irq,
    IRQ_TYPE_EDGE_RISING);
    dwc3_qcom_enable_wakeup_irq(port.dm_hs_phy_irq,
    IRQ_TYPE_EDGE_RISING);
    }
    dwc3_qcom_enable_wakeup_irq(port.ss_phy_irq, 0);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_disable_interrupts(qcom: *mut dwc3_qcom) {
    static void dwc3_qcom_disable_interrupts(struct dwc3_qcom *qcom)
    {
    int i;
    for (i = 0; i < qcom.num_ports; i++)
    dwc3_qcom_disable_port_interrupts(&qcom.ports[i]);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_enable_interrupts(qcom: *mut dwc3_qcom) {
    static void dwc3_qcom_enable_interrupts(struct dwc3_qcom *qcom)
    {
    int i;
    for (i = 0; i < qcom.num_ports; i++)
    dwc3_qcom_enable_port_interrupts(&qcom.ports[i]);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_suspend(qcom: *mut dwc3_qcom, wakeup: bool) -> c_int {
    static int dwc3_qcom_suspend(struct dwc3_qcom *qcom, bool wakeup)
    {
    u32 val;
    int i, ret;
    if (qcom.is_suspended)
    return 0;
    for (i = 0; i < qcom.num_ports; i++) {
    val = readl(qcom.qscratch_base + pwr_evnt_irq_stat_reg[i]);
    if (!(val & PWR_EVNT_LPM_IN_L2_MASK))
    dev_err(qcom.dev, "port-%d HS-PHY not in L2\n", i + 1);
    }
    for (i = qcom.num_clocks - 1; i >= 0; i--)
    clk_disable_unprepare(qcom.clks[i]);
    ret = dwc3_qcom_interconnect_disable(qcom);
    if (ret)
    dev_warn(qcom.dev, "failed to disable interconnect: %d\n", ret);
//
// The role is stable during suspend as role switching is done from a
// freezable workqueue.
//
    if (dwc3_qcom_is_host(qcom) && wakeup) {
    for (i = 0; i < qcom.num_ports; i++)
    qcom.ports[i].usb2_speed = dwc3_qcom_read_usb2_speed(qcom, i);
    dwc3_qcom_enable_interrupts(qcom);
    }
    qcom.is_suspended = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_resume(qcom: *mut dwc3_qcom, wakeup: bool) -> c_int {
    static int dwc3_qcom_resume(struct dwc3_qcom *qcom, bool wakeup)
    {
    int ret;
    int i;
    if (!qcom.is_suspended)
    return 0;
    if (dwc3_qcom_is_host(qcom) && wakeup)
    dwc3_qcom_disable_interrupts(qcom);
    for (i = 0; i < qcom.num_clocks; i++) {
    ret = clk_prepare_enable(qcom.clks[i]);
    if (ret < 0) {
    while (--i >= 0)
    clk_disable_unprepare(qcom.clks[i]);
    return ret;
    }
    }
    ret = dwc3_qcom_interconnect_enable(qcom);
    if (ret)
    dev_warn(qcom.dev, "failed to enable interconnect: %d\n", ret);
// Clear existing events from PHY related to L2 in/out
    for (i = 0; i < qcom.num_ports; i++) {
    dwc3_qcom_setbits(qcom.qscratch_base,
    pwr_evnt_irq_stat_reg[i],
    PWR_EVNT_LPM_IN_L2_MASK | PWR_EVNT_LPM_OUT_L2_MASK);
    }
    qcom.is_suspended = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_dwc3_resume_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t qcom_dwc3_resume_irq(int irq, void *data)
    {
    struct dwc3_qcom *qcom = data;
    struct dwc3	*dwc = platform_get_drvdata(qcom.dwc3);
// If pm_suspended then let pm_resume take care of resuming h/w
    if (qcom.pm_suspended)
    return IRQ_HANDLED;
//
// This is safe as role switching is done from a freezable workqueue
// and the wakeup interrupts are disabled as part of resume.
//
    if (dwc3_qcom_is_host(qcom))
    pm_runtime_resume(&dwc.xhci.dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_select_utmi_clk(qcom: *mut dwc3_qcom) {
    static void dwc3_qcom_select_utmi_clk(struct dwc3_qcom *qcom)
    {
// Configure dwc3 to use UTMI clock as PIPE clock not present
    dwc3_qcom_setbits(qcom.qscratch_base, QSCRATCH_GENERAL_CFG,
    PIPE_UTMI_CLK_DIS);
    usleep_range(100, 1000);
    dwc3_qcom_setbits(qcom.qscratch_base, QSCRATCH_GENERAL_CFG,
    PIPE_UTMI_CLK_SEL | PIPE3_PHYSTATUS_SW);
    usleep_range(100, 1000);
    dwc3_qcom_clrbits(qcom.qscratch_base, QSCRATCH_GENERAL_CFG,
    PIPE_UTMI_CLK_DIS);
    }
    static int dwc3_qcom_request_irq(struct dwc3_qcom *qcom, int irq,
    const char *name)
    {
    int ret;
// Keep wakeup interrupts disabled until suspend
    ret = devm_request_threaded_irq(qcom.dev, irq, core::ptr::null_mut(),
    qcom_dwc3_resume_irq,
    IRQF_ONESHOT | IRQF_NO_AUTOEN,
    name, qcom);
    if (ret)
    dev_err(qcom.dev, "failed to request irq %s: %d\n", name, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_setup_port_irq(pdev: *mut platform_device, port_index: c_int, is_multiport: bool) -> c_int {
    static int dwc3_qcom_setup_port_irq(struct platform_device *pdev, int port_index, bool is_multiport)
    {
    struct dwc3_qcom *qcom = platform_get_drvdata(pdev);
    const char *irq_name;
    int irq;
    int ret;
    if (is_multiport)
    irq_name = devm_kasprintf(&pdev.dev, GFP_KERNEL, "dp_hs_phy_%d", port_index + 1);
    else
    irq_name = devm_kasprintf(&pdev.dev, GFP_KERNEL, "dp_hs_phy_irq");
    if (!irq_name)
    return -ENOMEM;
    irq = platform_get_irq_byname_optional(pdev, irq_name);
    if (irq > 0) {
    ret = dwc3_qcom_request_irq(qcom, irq, irq_name);
    if (ret)
    return ret;
    qcom.ports[port_index].dp_hs_phy_irq = irq;
    }
    if (is_multiport)
    irq_name = devm_kasprintf(&pdev.dev, GFP_KERNEL, "dm_hs_phy_%d", port_index + 1);
    else
    irq_name = devm_kasprintf(&pdev.dev, GFP_KERNEL, "dm_hs_phy_irq");
    if (!irq_name)
    return -ENOMEM;
    irq = platform_get_irq_byname_optional(pdev, irq_name);
    if (irq > 0) {
    ret = dwc3_qcom_request_irq(qcom, irq, irq_name);
    if (ret)
    return ret;
    qcom.ports[port_index].dm_hs_phy_irq = irq;
    }
    if (is_multiport)
    irq_name = devm_kasprintf(&pdev.dev, GFP_KERNEL, "ss_phy_%d", port_index + 1);
    else
    irq_name = devm_kasprintf(&pdev.dev, GFP_KERNEL, "ss_phy_irq");
    if (!irq_name)
    return -ENOMEM;
    irq = platform_get_irq_byname_optional(pdev, irq_name);
    if (irq > 0) {
    ret = dwc3_qcom_request_irq(qcom, irq, irq_name);
    if (ret)
    return ret;
    qcom.ports[port_index].ss_phy_irq = irq;
    }
    if (is_multiport)
    return 0;
    irq = platform_get_irq_byname_optional(pdev, "qusb2_phy");
    if (irq > 0) {
    ret = dwc3_qcom_request_irq(qcom, irq, "qusb2_phy");
    if (ret)
    return ret;
    qcom.ports[port_index].qusb2_phy_irq = irq;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_find_num_ports(pdev: *mut platform_device) -> c_int {
    static int dwc3_qcom_find_num_ports(struct platform_device *pdev)
    {
    char irq_name[14];
    int port_num;
    int irq;
    irq = platform_get_irq_byname_optional(pdev, "dp_hs_phy_1");
    if (irq <= 0)
    return 1;
    for (port_num = 2; port_num <= DWC3_QCOM_MAX_PORTS; port_num++) {
    sprintf(irq_name, "dp_hs_phy_%d", port_num);
    irq = platform_get_irq_byname_optional(pdev, irq_name);
    if (irq <= 0)
    return port_num - 1;
    }
    return DWC3_QCOM_MAX_PORTS;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_setup_irq(pdev: *mut platform_device) -> c_int {
    static int dwc3_qcom_setup_irq(struct platform_device *pdev)
    {
    struct dwc3_qcom *qcom = platform_get_drvdata(pdev);
    bool is_multiport;
    int ret;
    int i;
    qcom.num_ports = dwc3_qcom_find_num_ports(pdev);
    is_multiport = (qcom.num_ports > 1);
    for (i = 0; i < qcom.num_ports; i++) {
    ret = dwc3_qcom_setup_port_irq(pdev, i, is_multiport);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_clk_init(qcom: *mut dwc3_qcom, count: c_int) -> c_int {
    static int dwc3_qcom_clk_init(struct dwc3_qcom *qcom, int count)
    {
    struct device		*dev = qcom.dev;
    struct device_node	*np = dev.of_node;
    int			i;
    if (!np || !count)
    return 0;
    if (count < 0)
    return count;
    qcom.num_clocks = count;
    qcom.clks = devm_kcalloc(dev, qcom.num_clocks,
    sizeof(struct clk *), GFP_KERNEL);
    if (!qcom.clks)
    return -ENOMEM;
    for (i = 0; i < qcom.num_clocks; i++) {
    struct clk	*clk;
    int		ret;
    clk = of_clk_get(np, i);
    if (IS_ERR(clk)) {
    while (--i >= 0)
    clk_put(qcom.clks[i]);
    return PTR_ERR(clk);
    }
    ret = clk_prepare_enable(clk);
    if (ret < 0) {
    while (--i >= 0) {
    clk_disable_unprepare(qcom.clks[i]);
    clk_put(qcom.clks[i]);
    }
    clk_put(clk);
    return ret;
    }
    qcom.clks[i] = clk;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_of_register_core(pdev: *mut platform_device) -> c_int {
    static int dwc3_qcom_of_register_core(struct platform_device *pdev)
    {
    struct dwc3_qcom	*qcom = platform_get_drvdata(pdev);
    struct device_node	*np = pdev.dev.of_node;
    struct device		*dev = &pdev.dev;
    int			ret;
    struct device_node *dwc3_np __free(device_node) = of_get_compatible_child(np,
    "snps,dwc3");
    if (!dwc3_np) {
    dev_err(dev, "failed to find dwc3 core child\n");
    return -ENODEV;
    }
    ret = of_platform_populate(np, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    if (ret) {
    dev_err(dev, "failed to register dwc3 core - %d\n", ret);
    return ret;
    }
    qcom.dwc3 = of_find_device_by_node(dwc3_np);
    if (!qcom.dwc3) {
    ret = -ENODEV;
    dev_err(dev, "failed to get dwc3 platform device\n");
    of_platform_depopulate(dev);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_probe(pdev: *mut platform_device) -> c_int {
    static int dwc3_qcom_probe(struct platform_device *pdev)
    {
    struct device_node	*np = pdev.dev.of_node;
    struct device		*dev = &pdev.dev;
    struct dwc3_qcom	*qcom;
    int			ret, i;
    bool			ignore_pipe_clk;
    bool			wakeup_source;
    qcom = devm_kzalloc(&pdev.dev, sizeof(*qcom), GFP_KERNEL);
    if (!qcom)
    return -ENOMEM;
    platform_set_drvdata(pdev, qcom);
    qcom.dev = &pdev.dev;
    qcom.resets = devm_reset_control_array_get_optional_exclusive(dev);
    if (IS_ERR(qcom.resets)) {
    return dev_err_probe(&pdev.dev, PTR_ERR(qcom.resets),
    "failed to get resets\n");
    }
    ret = reset_control_assert(qcom.resets);
    if (ret) {
    dev_err(&pdev.dev, "failed to assert resets, err=%d\n", ret);
    return ret;
    }
    usleep_range(10, 1000);
    ret = reset_control_deassert(qcom.resets);
    if (ret) {
    dev_err(&pdev.dev, "failed to deassert resets, err=%d\n", ret);
    goto reset_assert;
    }
    ret = dwc3_qcom_clk_init(qcom, of_clk_get_parent_count(np));
    if (ret) {
    dev_err_probe(dev, ret, "failed to get clocks\n");
    goto reset_assert;
    }
    qcom.qscratch_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(qcom.qscratch_base)) {
    ret = PTR_ERR(qcom.qscratch_base);
    goto clk_disable;
    }
    ret = dwc3_qcom_setup_irq(pdev);
    if (ret) {
    dev_err(dev, "failed to setup IRQs, err=%d\n", ret);
    goto clk_disable;
    }
//
// Disable pipe_clk requirement if specified. Used when dwc3
// operates without SSPHY and only HS/FS/LS modes are supported.
//
    ignore_pipe_clk = device_property_read_bool(dev,
    "qcom,select-utmi-as-pipe-clk");
    if (ignore_pipe_clk)
    dwc3_qcom_select_utmi_clk(qcom);
    ret = dwc3_qcom_of_register_core(pdev);
    if (ret) {
    dev_err(dev, "failed to register DWC3 Core, err=%d\n", ret);
    goto clk_disable;
    }
    ret = dwc3_qcom_interconnect_init(qcom);
    if (ret)
    goto depopulate;
    qcom.mode = usb_get_dr_mode(&qcom.dwc3.dev);
// enable vbus override for device mode
    if (qcom.mode != USB_DR_MODE_HOST)
    dwc3_qcom_vbus_override_enable(qcom, true);
// register extcon to override sw_vbus on Vbus change later
    ret = dwc3_qcom_register_extcon(qcom);
    if (ret)
    goto interconnect_exit;
    wakeup_source = of_property_read_bool(dev.of_node, "wakeup-source");
    device_init_wakeup(&pdev.dev, wakeup_source);
    device_init_wakeup(&qcom.dwc3.dev, wakeup_source);
    qcom.is_suspended = false;
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    pm_runtime_forbid(dev);
    return 0;
    interconnect_exit:
    dwc3_qcom_interconnect_exit(qcom);
    depopulate:
    of_platform_depopulate(&pdev.dev);
    platform_device_put(qcom.dwc3);
    clk_disable:
    for (i = qcom.num_clocks - 1; i >= 0; i--) {
    clk_disable_unprepare(qcom.clks[i]);
    clk_put(qcom.clks[i]);
    }
    reset_assert:
    reset_control_assert(qcom.resets);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_remove(pdev: *mut platform_device) {
    static void dwc3_qcom_remove(struct platform_device *pdev)
    {
    struct dwc3_qcom *qcom = platform_get_drvdata(pdev);
    struct device *dev = &pdev.dev;
    int i;
    of_platform_depopulate(&pdev.dev);
    platform_device_put(qcom.dwc3);
    for (i = qcom.num_clocks - 1; i >= 0; i--) {
    clk_disable_unprepare(qcom.clks[i]);
    clk_put(qcom.clks[i]);
    }
    qcom.num_clocks = 0;
    dwc3_qcom_interconnect_exit(qcom);
    reset_control_assert(qcom.resets);
    pm_runtime_allow(dev);
    pm_runtime_disable(dev);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_pm_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused dwc3_qcom_pm_suspend(struct device *dev)
    {
    struct dwc3_qcom *qcom = dev_get_drvdata(dev);
    let mut wakeup: bool = device_may_wakeup(dev);
    int ret;
    ret = dwc3_qcom_suspend(qcom, wakeup);
    if (ret)
    return ret;
    qcom.pm_suspended = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_pm_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused dwc3_qcom_pm_resume(struct device *dev)
    {
    struct dwc3_qcom *qcom = dev_get_drvdata(dev);
    let mut wakeup: bool = device_may_wakeup(dev);
    int ret;
    ret = dwc3_qcom_resume(qcom, wakeup);
    if (ret)
    return ret;
    qcom.pm_suspended = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused dwc3_qcom_runtime_suspend(struct device *dev)
    {
    struct dwc3_qcom *qcom = dev_get_drvdata(dev);
    return dwc3_qcom_suspend(qcom, true);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_qcom_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused dwc3_qcom_runtime_resume(struct device *dev)
    {
    struct dwc3_qcom *qcom = dev_get_drvdata(dev);
    return dwc3_qcom_resume(qcom, true);
    }
    static const struct dev_pm_ops dwc3_qcom_dev_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(dwc3_qcom_pm_suspend, dwc3_qcom_pm_resume)
    SET_RUNTIME_PM_OPS(dwc3_qcom_runtime_suspend, dwc3_qcom_runtime_resume,
    core::ptr::null_mut())
    };
    static const struct of_device_id dwc3_qcom_of_match[] = {
    { .compatible = "qcom,dwc3" },
    { }
    };
    MODULE_DEVICE_TABLE(of, dwc3_qcom_of_match);
    static struct platform_driver dwc3_qcom_driver = {
    .probe		= dwc3_qcom_probe,
    .remove		= dwc3_qcom_remove,
    .driver		= {
    .name	= "dwc3-qcom-legacy",
    .pm	= &dwc3_qcom_dev_pm_ops,
    .of_match_table	= dwc3_qcom_of_match,
    },
    };
    module_platform_driver(dwc3_qcom_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("DesignWare DWC3 QCOM legacy glue Driver");
