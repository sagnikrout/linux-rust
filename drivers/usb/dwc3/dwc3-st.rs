//! Automatically rewritten from C to Rust
//! Source: drivers/usb/dwc3/dwc3-st.c
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
// dwc3-st.c Support for dwc3 platform devices on ST Microelectronics platforms
//
// This is a small driver for the dwc3 to provide the glue logic
// to configure the controller. Tested on STi platforms.
//
// Copyright (C) 2014 Stmicroelectronics
//
// Author: Giuseppe Cavallaro <peppe.cavallaro@st.com>
// Contributors: Aymen Bouattay <aymen.bouattay@st.com>
// Peter Griffin <peter.griffin@linaro.org>
//
// Inspired by dwc3-omap.c and dwc3-exynos.c.
//

// glue registers
pub const CLKRST_CTRL: c_uint = 0x00;

//
// 1'b0 : The host controller complies with the xHCI revision 0.96
// 1'b1 : The host controller complies with the xHCI revision 1.0
//

pub const USB2_VBUS_MNGMNT_SEL1: c_uint = 0x2C;
//
// For all fields in USB2_VBUS_MNGMNT_SEL1
// 2’b00 : Override value from Reg 0x30 is selected
// 2’b01 : utmiotg_<signal_name> from usb3_top is selected
// 2’b10 : pipew_<signal_name> from PIPEW instance is selected
// 2’b11 : value is 1'b0
//
pub const USB2_VBUS_REG30: c_uint = 0x0;
pub const USB2_VBUS_UTMIOTG: c_uint = 0x1;
pub const USB2_VBUS_PIPEW: c_uint = 0x2;
pub const USB2_VBUS_ZERO: c_uint = 0x3;

// Static DRD configuration
pub const USB3_CONTROL_MASK: c_uint = 0xf77;

//
// struct st_dwc3 - dwc3-st driver private structure
// @dev:		device pointer
// @glue_base:		ioaddr for the glue registers
// @regmap:		regmap pointer for getting syscfg
// @syscfg_reg_off:	usb syscfg control offset
// @dr_mode:		drd static host/device config
// @rstc_pwrdn:		rest controller for powerdown signal
// @rstc_rst:		reset controller for softreset signal
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_dwc3 {
    pub dev: *mut device,
    pub glue_base: *mut void __iomem,
    pub regmap: *mut regmap,
    pub syscfg_reg_off: c_int,
    pub dr_mode: enum usb_dr_mode,
    pub rstc_pwrdn: *mut reset_control,
    pub rstc_rst: *mut reset_control,
}

#[no_mangle]
pub unsafe extern "C" fn st_dwc3_readl(base: *mut void __iomem, offset: u32) -> u32 {
    static inline u32 st_dwc3_readl(void __iomem *base, u32 offset)
    {
    return readl_relaxed(base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn st_dwc3_writel(base: *mut void __iomem, offset: u32, value: u32) {
    static inline void st_dwc3_writel(void __iomem *base, u32 offset, u32 value)
    {
    writel_relaxed(value, base + offset);
    }
//
// st_dwc3_drd_init: program the port
// @dwc3_data: driver private structure
// Description: this function is to program the port as either host or device
// according to the static configuration passed from devicetree.
// OTG and dual role are not yet supported!
//
#[no_mangle]
unsafe extern "C" fn st_dwc3_drd_init(dwc3_data: *mut st_dwc3) -> c_int {
    static int st_dwc3_drd_init(struct st_dwc3 *dwc3_data)
    {
    u32 val;
    int err;
    err = regmap_read(dwc3_data.regmap, dwc3_data.syscfg_reg_off, &val);
    if (err)
    return err;
    val &= USB3_CONTROL_MASK;
    switch (dwc3_data.dr_mode) {
    case USB_DR_MODE_PERIPHERAL:
    val &= ~(USB3_DELAY_VBUSVALID
    | USB3_SEL_FORCE_OPMODE | USB3_FORCE_OPMODE(0x3)
    | USB3_SEL_FORCE_DPPULLDOWN2 | USB3_FORCE_DPPULLDOWN2
    | USB3_SEL_FORCE_DMPULLDOWN2 | USB3_FORCE_DMPULLDOWN2);
//
// USB3_PORT2_FORCE_VBUSVALID When '1' and when
// USB3_PORT2_DEVICE_NOT_HOST = 1, forces VBUSVLDEXT2 input
// of the pico PHY to 1.
//
    val |= USB3_DEVICE_NOT_HOST | USB3_FORCE_VBUSVALID;
    break;
    case USB_DR_MODE_HOST:
    val &= ~(USB3_DEVICE_NOT_HOST | USB3_FORCE_VBUSVALID
    | USB3_SEL_FORCE_OPMODE	| USB3_FORCE_OPMODE(0x3)
    | USB3_SEL_FORCE_DPPULLDOWN2 | USB3_FORCE_DPPULLDOWN2
    | USB3_SEL_FORCE_DMPULLDOWN2 | USB3_FORCE_DMPULLDOWN2);
//
// USB3_DELAY_VBUSVALID is ANDed with USB_C_VBUSVALID. Thus,
// when set to ‘0‘, it can delay the arrival of VBUSVALID
// information to VBUSVLDEXT2 input of the pico PHY.
// We don't want to do that so we set the bit to '1'.
//
    val |= USB3_DELAY_VBUSVALID;
    break;
    default:
    dev_err(dwc3_data.dev, "Unsupported mode of operation %d\n",
    dwc3_data.dr_mode);
    return -EINVAL;
    }
    return regmap_write(dwc3_data.regmap, dwc3_data.syscfg_reg_off, val);
    }
//
// st_dwc3_init: init the controller via glue logic
// @dwc3_data: driver private structure
//
#[no_mangle]
unsafe extern "C" fn st_dwc3_init(dwc3_data: *mut st_dwc3) {
    static void st_dwc3_init(struct st_dwc3 *dwc3_data)
    {
    let mut reg: u32 = st_dwc3_readl(dwc3_data.glue_base, CLKRST_CTRL);
    reg |= AUX_CLK_EN | EXT_CFG_RESET_N | XHCI_REVISION;
    reg &= ~SW_PIPEW_RESET_N;
    st_dwc3_writel(dwc3_data.glue_base, CLKRST_CTRL, reg);
// configure mux for vbus, powerpresent and bvalid signals
    reg = st_dwc3_readl(dwc3_data.glue_base, USB2_VBUS_MNGMNT_SEL1);
    reg |= SEL_OVERRIDE_VBUSVALID(USB2_VBUS_UTMIOTG) |
    SEL_OVERRIDE_POWERPRESENT(USB2_VBUS_UTMIOTG) |
    SEL_OVERRIDE_BVALID(USB2_VBUS_UTMIOTG);
    st_dwc3_writel(dwc3_data.glue_base, USB2_VBUS_MNGMNT_SEL1, reg);
    reg = st_dwc3_readl(dwc3_data.glue_base, CLKRST_CTRL);
    reg |= SW_PIPEW_RESET_N;
    st_dwc3_writel(dwc3_data.glue_base, CLKRST_CTRL, reg);
    }
#[no_mangle]
unsafe extern "C" fn st_dwc3_probe(pdev: *mut platform_device) -> c_int {
    static int st_dwc3_probe(struct platform_device *pdev)
    {
    struct st_dwc3 *dwc3_data;
    struct resource *res;
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct platform_device *child_pdev;
    struct regmap *regmap;
    int ret;
    dwc3_data = devm_kzalloc(dev, sizeof(*dwc3_data), GFP_KERNEL);
    if (!dwc3_data)
    return -ENOMEM;
    dwc3_data.glue_base =
    devm_platform_ioremap_resource_byname(pdev, "reg-glue");
    if (IS_ERR(dwc3_data.glue_base))
    return PTR_ERR(dwc3_data.glue_base);
    regmap = syscon_regmap_lookup_by_phandle(node, "st,syscfg");
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    dwc3_data.dev = dev;
    dwc3_data.regmap = regmap;
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "syscfg-reg");
    if (!res)
    return -ENXIO;
    dwc3_data.syscfg_reg_off = res.start;
    dev_vdbg(dev, "glue-logic addr 0x%p, syscfg-reg offset 0x%x\n",
    dwc3_data.glue_base, dwc3_data.syscfg_reg_off);
    struct device_node *child __free(device_node) = of_get_compatible_child(node,
    "snps,dwc3");
    if (!child) {
    dev_err(dev, "failed to find dwc3 core node\n");
    return -ENODEV;
    }
    dwc3_data.rstc_pwrdn =
    devm_reset_control_get_exclusive(dev, "powerdown");
    if (IS_ERR(dwc3_data.rstc_pwrdn))
    return dev_err_probe(dev, PTR_ERR(dwc3_data.rstc_pwrdn),
    "could not get power controller\n");
// Manage PowerDown
    reset_control_deassert(dwc3_data.rstc_pwrdn);
    dwc3_data.rstc_rst =
    devm_reset_control_get_shared(dev, "softreset");
    if (IS_ERR(dwc3_data.rstc_rst)) {
    ret = dev_err_probe(dev, PTR_ERR(dwc3_data.rstc_rst),
    "could not get reset controller\n");
    goto undo_powerdown;
    }
// Manage SoftReset
    reset_control_deassert(dwc3_data.rstc_rst);
// Allocate and initialize the core
    ret = of_platform_populate(node, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    if (ret) {
    dev_err(dev, "failed to add dwc3 core\n");
    goto undo_softreset;
    }
    child_pdev = of_find_device_by_node(child);
    if (!child_pdev) {
    dev_err(dev, "failed to find dwc3 core device\n");
    ret = -ENODEV;
    goto depopulate;
    }
    dwc3_data.dr_mode = usb_get_dr_mode(&child_pdev.dev);
    platform_device_put(child_pdev);
//
// Configure the USB port as device or host according to the static
// configuration passed from DT.
// DRD is the only mode currently supported so this will be enhanced
// as soon as OTG is available.
//
    ret = st_dwc3_drd_init(dwc3_data);
    if (ret) {
    dev_err(dev, "drd initialisation failed\n");
    goto depopulate;
    }
// ST glue logic init
    st_dwc3_init(dwc3_data);
    platform_set_drvdata(pdev, dwc3_data);
    return 0;
    depopulate:
    of_platform_depopulate(dev);
    undo_softreset:
    reset_control_assert(dwc3_data.rstc_rst);
    undo_powerdown:
    reset_control_assert(dwc3_data.rstc_pwrdn);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn st_dwc3_remove(pdev: *mut platform_device) {
    static void st_dwc3_remove(struct platform_device *pdev)
    {
    struct st_dwc3 *dwc3_data = platform_get_drvdata(pdev);
    of_platform_depopulate(&pdev.dev);
    reset_control_assert(dwc3_data.rstc_pwrdn);
    reset_control_assert(dwc3_data.rstc_rst);
    }
#[no_mangle]
unsafe extern "C" fn st_dwc3_suspend(dev: *mut device) -> c_int {
    static int st_dwc3_suspend(struct device *dev)
    {
    struct st_dwc3 *dwc3_data = dev_get_drvdata(dev);
    reset_control_assert(dwc3_data.rstc_pwrdn);
    reset_control_assert(dwc3_data.rstc_rst);
    pinctrl_pm_select_sleep_state(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_dwc3_resume(dev: *mut device) -> c_int {
    static int st_dwc3_resume(struct device *dev)
    {
    struct st_dwc3 *dwc3_data = dev_get_drvdata(dev);
    int ret;
    pinctrl_pm_select_default_state(dev);
    reset_control_deassert(dwc3_data.rstc_pwrdn);
    reset_control_deassert(dwc3_data.rstc_rst);
    ret = st_dwc3_drd_init(dwc3_data);
    if (ret) {
    dev_err(dev, "drd initialisation failed\n");
    return ret;
    }
// ST glue logic init
    st_dwc3_init(dwc3_data);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(st_dwc3_dev_pm_ops, st_dwc3_suspend, st_dwc3_resume);
    static const struct of_device_id st_dwc3_match[] = {
    { .compatible = "st,stih407-dwc3" },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, st_dwc3_match);
    static struct platform_driver st_dwc3_driver = {
    .probe = st_dwc3_probe,
    .remove = st_dwc3_remove,
    .driver = {
    .name = "usb-st-dwc3",
    .of_match_table = st_dwc3_match,
    .pm = pm_sleep_ptr(&st_dwc3_dev_pm_ops),
    },
    };
    module_platform_driver(st_dwc3_driver);
    MODULE_AUTHOR("Giuseppe Cavallaro <peppe.cavallaro@st.com>");
    MODULE_DESCRIPTION("DesignWare USB3 STi Glue Layer");
    MODULE_LICENSE("GPL v2");
