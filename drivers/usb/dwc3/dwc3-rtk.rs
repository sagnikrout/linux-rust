//! Automatically rewritten from C to Rust
//! Source: drivers/usb/dwc3/dwc3-rtk.c
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
// dwc3-rtk.c - Realtek DWC3 Specific Glue layer
//
// Copyright (C) 2023 Realtek Semiconductor Corporation
//

pub const WRAP_CTR_REG: c_uint = 0x0;

pub const WRAP_USB2_PHY_UTMI_REG: c_uint = 0x8;

pub const WRAP_PHY_PIPE_REG: c_uint = 0xC;

pub const WRAP_USB_HMAC_CTR0_REG: c_uint = 0x60;

pub const WRAP_USB2_PHY_REG: c_uint = 0x70;

pub const USB2_PHY_SWITCH_MASK: c_uint = 0x707;
pub const USB2_PHY_SWITCH_DEVICE: c_uint = 0x0;
pub const USB2_PHY_SWITCH_HOST: c_uint = 0x606;
pub const WRAP_APHY_REG: c_uint = 0x128;

// pm control
pub const WRAP_USB_DBUS_PWR_CTRL_REG: c_uint = 0x160;
pub const USB_DBUS_PWR_CTRL_REG: c_uint = 0x0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_rtk {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub regs_size: usize,
    pub pm_base: *mut void __iomem,
    pub dwc: *mut dwc3,
    pub cur_role: enum usb_role,
    pub role_switch: *mut usb_role_switch,
}

#[no_mangle]
unsafe extern "C" fn switch_usb2_role(rtk: *mut dwc3_rtk, role: enum usb_role) {
    static void switch_usb2_role(struct dwc3_rtk *rtk, enum usb_role role)
    {
    void __iomem *reg;
    int val;
    reg = rtk.regs + WRAP_USB2_PHY_REG;
    val = ~USB2_PHY_SWITCH_MASK & readl(reg);
    switch (role) {
    case USB_ROLE_DEVICE:
    writel(USB2_PHY_SWITCH_DEVICE | val, reg);
    break;
    case USB_ROLE_HOST:
    writel(USB2_PHY_SWITCH_HOST | val, reg);
    break;
    default:
    dev_dbg(rtk.dev, "%s: role=%d\n", __func__, role);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn switch_dwc3_role(rtk: *mut dwc3_rtk, role: enum usb_role) {
    static void switch_dwc3_role(struct dwc3_rtk *rtk, enum usb_role role)
    {
    if (!rtk.dwc.role_sw)
    return;
    usb_role_switch_set_role(rtk.dwc.role_sw, role);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rtk_get_role(rtk: *mut dwc3_rtk) -> enum usb_role {
    static enum usb_role dwc3_rtk_get_role(struct dwc3_rtk *rtk)
    {
    enum usb_role role;
    role = rtk.cur_role;
    if (rtk.dwc && rtk.dwc.role_sw)
    role = usb_role_switch_get_role(rtk.dwc.role_sw);
    else
    dev_dbg(rtk.dev, "%s not usb_role_switch role=%d\n", __func__, role);
    return role;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rtk_set_role(rtk: *mut dwc3_rtk, role: enum usb_role) {
    static void dwc3_rtk_set_role(struct dwc3_rtk *rtk, enum usb_role role)
    {
    rtk.cur_role = role;
    switch_dwc3_role(rtk, role);
    mdelay(10);
    switch_usb2_role(rtk, role);
    }

#[no_mangle]
unsafe extern "C" fn dwc3_usb_role_switch_set(sw: *mut usb_role_switch, role: enum usb_role) -> c_int {
    static int dwc3_usb_role_switch_set(struct usb_role_switch *sw, enum usb_role role)
    {
    struct dwc3_rtk *rtk = usb_role_switch_get_drvdata(sw);
    dwc3_rtk_set_role(rtk, role);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_usb_role_switch_get(sw: *mut usb_role_switch) -> enum usb_role {
    static enum usb_role dwc3_usb_role_switch_get(struct usb_role_switch *sw)
    {
    struct dwc3_rtk *rtk = usb_role_switch_get_drvdata(sw);
    return dwc3_rtk_get_role(rtk);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rtk_setup_role_switch(rtk: *mut dwc3_rtk) -> c_int {
    static int dwc3_rtk_setup_role_switch(struct dwc3_rtk *rtk)
    {
    let mut dwc3_role_switch: usb_role_switch_desc = {core::ptr::null_mut()};
    dwc3_role_switch.name = dev_name(rtk.dev);
    dwc3_role_switch.driver_data = rtk;
    dwc3_role_switch.allow_userspace_control = true;
    dwc3_role_switch.fwnode = dev_fwnode(rtk.dev);
    dwc3_role_switch.set = dwc3_usb_role_switch_set;
    dwc3_role_switch.get = dwc3_usb_role_switch_get;
    rtk.role_switch = usb_role_switch_register(rtk.dev, &dwc3_role_switch);
    if (IS_ERR(rtk.role_switch))
    return PTR_ERR(rtk.role_switch);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rtk_remove_role_switch(rtk: *mut dwc3_rtk) -> c_int {
    static int dwc3_rtk_remove_role_switch(struct dwc3_rtk *rtk)
    {
    if (rtk.role_switch)
    usb_role_switch_unregister(rtk.role_switch);
    rtk.role_switch = core::ptr::null_mut();
    return 0;
    }

pub const dwc3_rtk_setup_role_switch(x): c_int = 0;
pub const dwc3_rtk_remove_role_switch(x): c_int = 0;

    static const char *const speed_names[] = {
    [USB_SPEED_UNKNOWN] = "UNKNOWN",
    [USB_SPEED_LOW] = "low-speed",
    [USB_SPEED_FULL] = "full-speed",
    [USB_SPEED_HIGH] = "high-speed",
    [USB_SPEED_WIRELESS] = "wireless",
    [USB_SPEED_SUPER] = "super-speed",
    [USB_SPEED_SUPER_PLUS] = "super-speed-plus",
    };
#[no_mangle]
unsafe extern "C" fn __get_dwc3_maximum_speed(np: *mut device_node) -> enum usb_device_speed {
    static enum usb_device_speed __get_dwc3_maximum_speed(struct device_node *np)
    {
    const char *maximum_speed;
    int ret;
    struct device_node *dwc3_np __free(device_node) = of_get_compatible_child(np,
    "snps,dwc3");
    if (!dwc3_np)
    return USB_SPEED_UNKNOWN;
    ret = of_property_read_string(dwc3_np, "maximum-speed", &maximum_speed);
    if (ret < 0)
    return USB_SPEED_UNKNOWN;
    ret = match_string(speed_names, ARRAY_SIZE(speed_names), maximum_speed);
    return (ret < 0) ? USB_SPEED_UNKNOWN : ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rtk_init(rtk: *mut dwc3_rtk) -> c_int {
    static int dwc3_rtk_init(struct dwc3_rtk *rtk)
    {
    struct device *dev = rtk.dev;
    void __iomem *reg;
    int val;
    enum usb_device_speed maximum_speed;
    const struct soc_device_attribute rtk_soc_kylin_a00[] = {
    { .family = "Realtek Kylin", .revision = "A00", },
    { /* empty */ } };
    const struct soc_device_attribute rtk_soc_hercules[] = {
    { .family = "Realtek Hercules", }, { /* empty */ } };
    const struct soc_device_attribute rtk_soc_thor[] = {
    { .family = "Realtek Thor", }, { /* empty */ } };
    if (soc_device_match(rtk_soc_kylin_a00)) {
    reg = rtk.regs + WRAP_CTR_REG;
    val = readl(reg);
    writel(DISABLE_MULTI_REQ | val, reg);
    dev_info(dev, "[bug fixed] 1295/1296 A00: add workaround to disable multiple request for D-Bus");
    }
    if (soc_device_match(rtk_soc_hercules)) {
    reg = rtk.regs + WRAP_USB2_PHY_REG;
    val = readl(reg);
    writel(USB2_PHY_EN_PHY_PLL_PORT1 | val, reg);
    dev_info(dev, "[bug fixed] 1395 add workaround to disable usb2 port 2 suspend!");
    }
    reg = rtk.regs + WRAP_USB2_PHY_UTMI_REG;
    val = readl(reg);
    writel(TXHSVM_EN | val, reg);
    maximum_speed = __get_dwc3_maximum_speed(dev.of_node);
    if (maximum_speed != USB_SPEED_UNKNOWN && maximum_speed <= USB_SPEED_HIGH) {
    if (soc_device_match(rtk_soc_thor)) {
    reg = rtk.regs + WRAP_USB_HMAC_CTR0_REG;
    val = readl(reg);
    writel(U3PORT_DIS | val, reg);
    } else {
    reg = rtk.regs + WRAP_CTR_REG;
    val = readl(reg);
    writel(FORCE_PIPE3_PHY_STATUS_TO_0 | val, reg);
    reg = rtk.regs + WRAP_PHY_PIPE_REG;
    val = ~CLOCK_ENABLE_FOR_PIPE3_PCLK & readl(reg);
    writel(RESET_DISABLE_PIPE3_P0 | val, reg);
    reg =  rtk.regs + WRAP_USB_HMAC_CTR0_REG;
    val = readl(reg);
    writel(U3PORT_DIS | val, reg);
    reg = rtk.regs + WRAP_APHY_REG;
    val = readl(reg);
    writel(~USB3_MBIAS_ENABLE & val, reg);
    dev_dbg(rtk.dev, "%s: disable usb 3.0 phy\n", __func__);
    }
    }
    reg = rtk.regs + WRAP_CTR_REG;
    val = readl(reg);
    writel(DESC_R2W_MULTI_DISABLE | val, reg);
// Set phy Dp/Dm initial state to host mode to avoid the Dp glitch
    reg = rtk.regs + WRAP_USB2_PHY_REG;
    val = ~USB2_PHY_SWITCH_MASK & readl(reg);
    writel(USB2_PHY_SWITCH_HOST | val, reg);
    if (rtk.pm_base) {
    reg = rtk.pm_base + USB_DBUS_PWR_CTRL_REG;
    val = DBUS_PWR_CTRL_EN | readl(reg);
    writel(val, reg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rtk_probe_dwc3_core(rtk: *mut dwc3_rtk) -> c_int {
    static int dwc3_rtk_probe_dwc3_core(struct dwc3_rtk *rtk)
    {
    struct device *dev = rtk.dev;
    struct device_node *node = dev.of_node;
    struct platform_device *dwc3_pdev;
    struct device *dwc3_dev;
    enum usb_dr_mode dr_mode;
    let mut ret: c_int = 0;
    ret = dwc3_rtk_init(rtk);
    if (ret)
    return -EINVAL;
    ret = of_platform_populate(node, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    if (ret) {
    dev_err(dev, "failed to add dwc3 core\n");
    return ret;
    }
    struct device_node *dwc3_node __free(device_node) = of_get_compatible_child(node,
    "snps,dwc3");
    if (!dwc3_node) {
    dev_err(dev, "failed to find dwc3 core node\n");
    ret = -ENODEV;
    goto depopulate;
    }
    dwc3_pdev = of_find_device_by_node(dwc3_node);
    if (!dwc3_pdev) {
    dev_err(dev, "failed to find dwc3 core platform_device\n");
    ret = -ENODEV;
    goto depopulate;
    }
    dwc3_dev = &dwc3_pdev.dev;
    rtk.dwc = platform_get_drvdata(dwc3_pdev);
    if (!rtk.dwc) {
    dev_err(dev, "failed to find dwc3 core\n");
    ret = -ENODEV;
    goto err_pdev_put;
    }
    dr_mode = usb_get_dr_mode(dwc3_dev);
    if (dr_mode != rtk.dwc.dr_mode) {
    dev_info(dev, "dts set dr_mode=%d, but dwc3 set dr_mode=%d\n",
    dr_mode, rtk.dwc.dr_mode);
    dr_mode = rtk.dwc.dr_mode;
    }
    switch (dr_mode) {
    case USB_DR_MODE_PERIPHERAL:
    rtk.cur_role = USB_ROLE_DEVICE;
    break;
    case USB_DR_MODE_HOST:
    rtk.cur_role = USB_ROLE_HOST;
    break;
    default:
    dev_dbg(rtk.dev, "%s: dr_mode=%d\n", __func__, dr_mode);
    break;
    }
    if (device_property_read_bool(dwc3_dev, "usb-role-switch")) {
    ret = dwc3_rtk_setup_role_switch(rtk);
    if (ret) {
    dev_err(dev, "dwc3_rtk_setup_role_switch fail=%d\n", ret);
    goto err_pdev_put;
    }
    rtk.cur_role = dwc3_rtk_get_role(rtk);
    }
    switch_usb2_role(rtk, rtk.cur_role);
    platform_device_put(dwc3_pdev);
    return 0;
    err_pdev_put:
    platform_device_put(dwc3_pdev);
    depopulate:
    of_platform_depopulate(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rtk_probe(pdev: *mut platform_device) -> c_int {
    static int dwc3_rtk_probe(struct platform_device *pdev)
    {
    struct dwc3_rtk *rtk;
    struct device *dev = &pdev.dev;
    struct resource *res;
    void __iomem *regs;
    rtk = devm_kzalloc(dev, sizeof(*rtk), GFP_KERNEL);
    if (!rtk)
    return -ENOMEM;
    platform_set_drvdata(pdev, rtk);
    rtk.dev = dev;
    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    rtk.regs = regs;
    rtk.regs_size = resource_size(res);
    res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if (res) {
    rtk.pm_base = devm_ioremap_resource(dev, res);
    if (IS_ERR(rtk.pm_base))
    return PTR_ERR(rtk.pm_base);
    }
    return dwc3_rtk_probe_dwc3_core(rtk);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rtk_remove(pdev: *mut platform_device) {
    static void dwc3_rtk_remove(struct platform_device *pdev)
    {
    struct dwc3_rtk *rtk = platform_get_drvdata(pdev);
    rtk.dwc = core::ptr::null_mut();
    dwc3_rtk_remove_role_switch(rtk);
    of_platform_depopulate(rtk.dev);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rtk_shutdown(pdev: *mut platform_device) {
    static void dwc3_rtk_shutdown(struct platform_device *pdev)
    {
    struct dwc3_rtk *rtk = platform_get_drvdata(pdev);
    of_platform_depopulate(rtk.dev);
    }
    static const struct of_device_id rtk_dwc3_match[] = {
    { .compatible = "realtek,rtd-dwc3" },
    {},
    };
    MODULE_DEVICE_TABLE(of, rtk_dwc3_match);

#[no_mangle]
unsafe extern "C" fn dwc3_rtk_suspend(dev: *mut device) -> c_int {
    static int dwc3_rtk_suspend(struct device *dev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_rtk_resume(dev: *mut device) -> c_int {
    static int dwc3_rtk_resume(struct device *dev)
    {
    struct dwc3_rtk *rtk = dev_get_drvdata(dev);
    dwc3_rtk_init(rtk);
    switch_usb2_role(rtk, rtk.cur_role);
// runtime set active to reflect active state.
    pm_runtime_disable(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    return 0;
    }
    static const struct dev_pm_ops dwc3_rtk_dev_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(dwc3_rtk_suspend, dwc3_rtk_resume)
    };

    static struct platform_driver dwc3_rtk_driver = {
    .probe		= dwc3_rtk_probe,
    .remove		= dwc3_rtk_remove,
    .driver		= {
    .name	= "rtk-dwc3",
    .of_match_table = rtk_dwc3_match,
    .pm	= DEV_PM_OPS,
    },
    .shutdown	= dwc3_rtk_shutdown,
    };
    module_platform_driver(dwc3_rtk_driver);
    MODULE_AUTHOR("Stanley Chang <stanley_chang@realtek.com>");
    MODULE_DESCRIPTION("DesignWare USB3 Realtek Glue Layer");
    MODULE_ALIAS("platform:rtk-dwc3");
    MODULE_LICENSE("GPL");
    MODULE_SOFTDEP("pre: phy_rtk_usb2 phy_rtk_usb3");
