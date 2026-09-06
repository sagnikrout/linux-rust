//! Automatically rewritten from C to Rust
//! Source: drivers/phy/renesas/phy-rzg3e-usb3.c
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
// Renesas RZ/G3E USB3.0 PHY driver
//
// Copyright (C) 2025 Renesas Electronics Corporation
//

pub const USB3_TEST_RESET: c_uint = 0x0000;
pub const USB3_TEST_UTMICTRL2: c_uint = 0x0b04;
pub const USB3_TEST_PRMCTRL5_R: c_uint = 0x0c10;
pub const USB3_TEST_PRMCTRL6_R: c_uint = 0x0c14;
pub const USB3_TEST_RSTCTRL: c_uint = 0x1000;
pub const USB3_TEST_CLKCTRL: c_uint = 0x1004;
pub const USB3_TEST_RAMCTRL: c_uint = 0x100c;
pub const USB3_TEST_CREGCTRL: c_uint = 0x1010;
pub const USB3_TEST_LANECONFIG0: c_uint = 0x1030;

    (USB3_TEST_RSTCTRL_HARDRESET_ODEN | USB3_TEST_RSTCTRL_PIPERESET_ODEN | \
    USB3_TEST_RSTCTRL_HARDRESET | USB3_TEST_RSTCTRL_PIPERESET)

    (USB3_TEST_RSTCTRL_HARDRESET_ODEN | USB3_TEST_RSTCTRL_PIPERESET_ODEN | \
    USB3_TEST_RSTCTRL_PIPERESET)

    (USB3_TEST_RSTCTRL_HARDRESET_ODEN | USB3_TEST_RSTCTRL_PIPERESET_ODEN)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rz_usb3 {
    pub base: *mut void __iomem,
    pub rstc: *mut reset_control,
    pub skip_reinit: bool,
}

#[no_mangle]
unsafe extern "C" fn rzg3e_phy_usb2test_phy_init(base: *mut void __iomem) {
    static void rzg3e_phy_usb2test_phy_init(void __iomem *base)
    {
    u32 val;
    val = readl(base + USB3_TEST_UTMICTRL2);
    val |= USB3_TEST_UTMICTRL2_CTRL_MASK | USB3_TEST_UTMICTRL2_MODE_MASK;
    writel(val, base + USB3_TEST_UTMICTRL2);
    val = readl(base + USB3_TEST_PRMCTRL5_R);
    FIELD_MODIFY(USB3_TEST_PRMCTRL5_R_TXPREEMPAMPTUNE0_MASK, &val, 2);
    writel(val, base + USB3_TEST_PRMCTRL5_R);
    val = readl(base + USB3_TEST_PRMCTRL6_R);
    FIELD_MODIFY(USB3_TEST_PRMCTRL6_R_OTGTUNE0_MASK, &val, 7);
    writel(val, base + USB3_TEST_PRMCTRL6_R);
    val = readl(base + USB3_TEST_RESET);
    val &= ~USB3_TEST_RESET_SIDDQ;
    val |= USB3_TEST_RESET_PORTRESET0_CTRL | USB3_TEST_RESET_PHY_RESET |
    USB3_TEST_RESET_PORTRESET0;
    writel(val, base + USB3_TEST_RESET);
    fsleep(10);
    val &= ~(USB3_TEST_RESET_PHY_RESET | USB3_TEST_RESET_PORTRESET0);
    writel(val, base + USB3_TEST_RESET);
    fsleep(10);
    val = readl(base + USB3_TEST_UTMICTRL2);
    val &= ~USB3_TEST_UTMICTRL2_CTRL_MASK;
    writel(val, base + USB3_TEST_UTMICTRL2);
    writel(USB3_TEST_RESET_RELEASE_OVERRIDE, base + USB3_TEST_RESET);
    }
#[no_mangle]
unsafe extern "C" fn rzg3e_phy_usb3test_phy_init(base: *mut void __iomem) -> c_int {
    static int rzg3e_phy_usb3test_phy_init(void __iomem *base)
    {
    int ret;
    u32 val;
    writel(USB3_TEST_CREGCTRL_PARA_SEL, base + USB3_TEST_CREGCTRL);
    writel(USB3_TEST_RSTCTRL_ASSERT, base + USB3_TEST_RSTCTRL);
    fsleep(20);
    writel(USB3_TEST_CLKCTRL_MPLLA_SSC_EN, base + USB3_TEST_CLKCTRL);
    writel(USB3_TEST_LANECONFIG0_DEFAULT, base + USB3_TEST_LANECONFIG0);
    writel(USB3_TEST_RSTCTRL_RELEASE_HARDRESET, base + USB3_TEST_RSTCTRL);
    ret = readl_poll_timeout_atomic(base + USB3_TEST_RAMCTRL, val,
    val & USB3_TEST_RAMCTRL_SRAM_INIT_DONE, 1, 10000);
    if (ret)
    return ret;
    writel(USB3_TEST_RSTCTRL_DEASSERT, base + USB3_TEST_RSTCTRL);
    writel(USB3_TEST_RAMCTRL_SRAM_EXT_LD_DONE, base + USB3_TEST_RAMCTRL);
    writel(USB3_TEST_RSTCTRL_RELEASE_OVERRIDE, base + USB3_TEST_RSTCTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzg3e_phy_usb3_init_helper(base: *mut void __iomem) -> c_int {
    static int rzg3e_phy_usb3_init_helper(void __iomem *base)
    {
    rzg3e_phy_usb2test_phy_init(base);
    return rzg3e_phy_usb3test_phy_init(base);
    }
#[no_mangle]
unsafe extern "C" fn rzg3e_phy_usb3_init(p: *mut phy) -> c_int {
    static int rzg3e_phy_usb3_init(struct phy *p)
    {
    struct rz_usb3 *r = phy_get_drvdata(p);
    let mut ret: c_int = 0;
    if (!r.skip_reinit)
    ret = rzg3e_phy_usb3_init_helper(r.base);
    return ret;
    }
    static const struct phy_ops rzg3e_phy_usb3_ops = {
    .init = rzg3e_phy_usb3_init,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn rzg3e_phy_usb3_probe(pdev: *mut platform_device) -> c_int {
    static int rzg3e_phy_usb3_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct phy_provider *provider;
    struct rz_usb3 *r;
    struct phy *phy;
    int ret;
    r = devm_kzalloc(dev, sizeof(*r), GFP_KERNEL);
    if (!r)
    return -ENOMEM;
    r.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(r.base))
    return PTR_ERR(r.base);
    r.rstc = devm_reset_control_get_shared_deasserted(dev, core::ptr::null_mut());
    if (IS_ERR(r.rstc))
    return dev_err_probe(dev, PTR_ERR(r.rstc), "failed to get deasserted reset\n");
//
// devm_phy_create() will call pm_runtime_enable(&phy->dev);
// And then, phy-core will manage runtime pm for this device.
//
    ret = devm_pm_runtime_enable(dev);
    if (ret < 0)
    return ret;
    phy = devm_phy_create(dev, core::ptr::null_mut(), &rzg3e_phy_usb3_ops);
    if (IS_ERR(phy))
    return dev_err_probe(dev, PTR_ERR(phy), "failed to create USB3 PHY\n");
    platform_set_drvdata(pdev, r);
    phy_set_drvdata(phy, r);
    provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(provider))
    return dev_err_probe(dev, PTR_ERR(provider), "failed to register PHY provider\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzg3e_phy_usb3_suspend(dev: *mut device) -> c_int {
    static int rzg3e_phy_usb3_suspend(struct device *dev)
    {
    struct rz_usb3 *r = dev_get_drvdata(dev);
    pm_runtime_put(dev);
    reset_control_assert(r.rstc);
    r.skip_reinit = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rzg3e_phy_usb3_resume(dev: *mut device) -> c_int {
    static int rzg3e_phy_usb3_resume(struct device *dev)
    {
    struct rz_usb3 *r = dev_get_drvdata(dev);
    int ret;
    ret = reset_control_deassert(r.rstc);
    if (ret)
    return ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    goto reset_assert;
    ret = rzg3e_phy_usb3_init_helper(r.base);
    if (ret)
    goto pm_put;
    r.skip_reinit = true;
    return 0;
    pm_put:
    pm_runtime_put(dev);
    reset_assert:
    reset_control_assert(r.rstc);
    return ret;
    }
    static const struct dev_pm_ops rzg3e_phy_usb3_pm = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(rzg3e_phy_usb3_suspend, rzg3e_phy_usb3_resume)
    };
    static const struct of_device_id rzg3e_phy_usb3_match_table[] = {
    { .compatible = "renesas,r9a09g047-usb3-phy" },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rzg3e_phy_usb3_match_table);
    static struct platform_driver rzg3e_phy_usb3_driver = {
    .driver = {
    .name = "phy_rzg3e_usb3",
    .of_match_table = rzg3e_phy_usb3_match_table,
    .pm = pm_sleep_ptr(&rzg3e_phy_usb3_pm),
    },
    .probe	= rzg3e_phy_usb3_probe,
    };
    module_platform_driver(rzg3e_phy_usb3_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Renesas RZ/G3E USB3.0 PHY Driver");
    MODULE_AUTHOR("Biju Das <biju.das.jz@bp.renesas.com>");
