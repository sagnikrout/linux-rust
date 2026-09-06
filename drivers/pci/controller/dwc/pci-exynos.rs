//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pci-exynos.c
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
// PCIe host controller driver for Samsung Exynos SoCs
//
// Copyright (C) 2013-2020 Samsung Electronics Co., Ltd.
// https://www.samsung.com
//
// Author: Jingoo Han <jg1.han@samsung.com>
// Jaehoon Chung <jh80.chung@samsung.com>
//

// PCIe ELBI registers
pub const PCIE_IRQ_PULSE: c_uint = 0x000;

pub const PCIE_IRQ_LEVEL: c_uint = 0x004;
pub const PCIE_IRQ_SPECIAL: c_uint = 0x008;
pub const PCIE_IRQ_EN_PULSE: c_uint = 0x00c;
pub const PCIE_IRQ_EN_LEVEL: c_uint = 0x010;
pub const PCIE_IRQ_EN_SPECIAL: c_uint = 0x014;
pub const PCIE_SW_WAKE: c_uint = 0x018;

pub const PCIE_CORE_RESET: c_uint = 0x01c;

pub const PCIE_STICKY_RESET: c_uint = 0x020;
pub const PCIE_NONSTICKY_RESET: c_uint = 0x024;
pub const PCIE_APP_INIT_RESET: c_uint = 0x028;
pub const PCIE_APP_LTSSM_ENABLE: c_uint = 0x02c;
pub const PCIE_ELBI_RDLH_LINKUP: c_uint = 0x074;

pub const PCIE_ELBI_LTSSM_ENABLE: c_uint = 0x1;
pub const PCIE_ELBI_SLV_AWMISC: c_uint = 0x11c;
pub const PCIE_ELBI_SLV_ARMISC: c_uint = 0x120;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_pcie {
    pub pci: dw_pcie,
    pub clks: *mut clk_bulk_data,
    pub phy: *mut phy,
    pub supplies: [regulator_bulk_data; 2],
}

#[no_mangle]
unsafe extern "C" fn exynos_pcie_writel(base: *mut void __iomem, val: u32, reg: u32) {
    static void exynos_pcie_writel(void __iomem *base, u32 val, u32 reg)
    {
    writel(val, base + reg);
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_readl(base: *mut void __iomem, reg: u32) -> u32 {
    static u32 exynos_pcie_readl(void __iomem *base, u32 reg)
    {
    return readl(base + reg);
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_sideband_dbi_w_mode(ep: *mut exynos_pcie, on: bool) {
    static void exynos_pcie_sideband_dbi_w_mode(struct exynos_pcie *ep, bool on)
    {
    struct dw_pcie *pci = &ep.pci;
    u32 val;
    val = exynos_pcie_readl(pci.elbi_base, PCIE_ELBI_SLV_AWMISC);
    if (on)
    val |= PCIE_ELBI_SLV_DBI_ENABLE;
    else
    val &= ~PCIE_ELBI_SLV_DBI_ENABLE;
    exynos_pcie_writel(pci.elbi_base, val, PCIE_ELBI_SLV_AWMISC);
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_sideband_dbi_r_mode(ep: *mut exynos_pcie, on: bool) {
    static void exynos_pcie_sideband_dbi_r_mode(struct exynos_pcie *ep, bool on)
    {
    struct dw_pcie *pci = &ep.pci;
    u32 val;
    val = exynos_pcie_readl(pci.elbi_base, PCIE_ELBI_SLV_ARMISC);
    if (on)
    val |= PCIE_ELBI_SLV_DBI_ENABLE;
    else
    val &= ~PCIE_ELBI_SLV_DBI_ENABLE;
    exynos_pcie_writel(pci.elbi_base, val, PCIE_ELBI_SLV_ARMISC);
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_assert_core_reset(ep: *mut exynos_pcie) {
    static void exynos_pcie_assert_core_reset(struct exynos_pcie *ep)
    {
    struct dw_pcie *pci = &ep.pci;
    u32 val;
    val = exynos_pcie_readl(pci.elbi_base, PCIE_CORE_RESET);
    val &= ~PCIE_CORE_RESET_ENABLE;
    exynos_pcie_writel(pci.elbi_base, val, PCIE_CORE_RESET);
    exynos_pcie_writel(pci.elbi_base, 0, PCIE_STICKY_RESET);
    exynos_pcie_writel(pci.elbi_base, 0, PCIE_NONSTICKY_RESET);
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_deassert_core_reset(ep: *mut exynos_pcie) {
    static void exynos_pcie_deassert_core_reset(struct exynos_pcie *ep)
    {
    struct dw_pcie *pci = &ep.pci;
    u32 val;
    val = exynos_pcie_readl(pci.elbi_base, PCIE_CORE_RESET);
    val |= PCIE_CORE_RESET_ENABLE;
    exynos_pcie_writel(pci.elbi_base, val, PCIE_CORE_RESET);
    exynos_pcie_writel(pci.elbi_base, 1, PCIE_STICKY_RESET);
    exynos_pcie_writel(pci.elbi_base, 1, PCIE_NONSTICKY_RESET);
    exynos_pcie_writel(pci.elbi_base, 1, PCIE_APP_INIT_RESET);
    exynos_pcie_writel(pci.elbi_base, 0, PCIE_APP_INIT_RESET);
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_start_link(pci: *mut dw_pcie) -> c_int {
    static int exynos_pcie_start_link(struct dw_pcie *pci)
    {
    u32 val;
    val = exynos_pcie_readl(pci.elbi_base, PCIE_SW_WAKE);
    val &= ~PCIE_BUS_EN;
    exynos_pcie_writel(pci.elbi_base, val, PCIE_SW_WAKE);
// assert LTSSM enable
    exynos_pcie_writel(pci.elbi_base, PCIE_ELBI_LTSSM_ENABLE,
    PCIE_APP_LTSSM_ENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_clear_irq_pulse(ep: *mut exynos_pcie) {
    static void exynos_pcie_clear_irq_pulse(struct exynos_pcie *ep)
    {
    struct dw_pcie *pci = &ep.pci;
    let mut val: u32 = exynos_pcie_readl(pci.elbi_base, PCIE_IRQ_PULSE);
    exynos_pcie_writel(pci.elbi_base, val, PCIE_IRQ_PULSE);
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t exynos_pcie_irq_handler(int irq, void *arg)
    {
    struct exynos_pcie *ep = arg;
    exynos_pcie_clear_irq_pulse(ep);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_enable_irq_pulse(ep: *mut exynos_pcie) {
    static void exynos_pcie_enable_irq_pulse(struct exynos_pcie *ep)
    {
    struct dw_pcie *pci = &ep.pci;
    u32 val = IRQ_INTA_ASSERT | IRQ_INTB_ASSERT |
    IRQ_INTC_ASSERT | IRQ_INTD_ASSERT;
    exynos_pcie_writel(pci.elbi_base, val, PCIE_IRQ_EN_PULSE);
    exynos_pcie_writel(pci.elbi_base, 0, PCIE_IRQ_EN_LEVEL);
    exynos_pcie_writel(pci.elbi_base, 0, PCIE_IRQ_EN_SPECIAL);
    }
    static u32 exynos_pcie_read_dbi(struct dw_pcie *pci, void __iomem *base,
    u32 reg, size_t size)
    {
    struct exynos_pcie *ep = to_exynos_pcie(pci);
    u32 val;
    exynos_pcie_sideband_dbi_r_mode(ep, true);
    dw_pcie_read(base + reg, size, &val);
    exynos_pcie_sideband_dbi_r_mode(ep, false);
    return val;
    }
    static void exynos_pcie_write_dbi(struct dw_pcie *pci, void __iomem *base,
    u32 reg, size_t size, u32 val)
    {
    struct exynos_pcie *ep = to_exynos_pcie(pci);
    exynos_pcie_sideband_dbi_w_mode(ep, true);
    dw_pcie_write(base + reg, size, val);
    exynos_pcie_sideband_dbi_w_mode(ep, false);
    }
    static int exynos_pcie_rd_own_conf(struct pci_bus *bus, unsigned int devfn,
    int where, int size, u32 *val)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(bus.sysdata);
    if (PCI_SLOT(devfn))
    return PCIBIOS_DEVICE_NOT_FOUND;
// val = dw_pcie_read_dbi(pci, where, size);
    return PCIBIOS_SUCCESSFUL;
    }
    static int exynos_pcie_wr_own_conf(struct pci_bus *bus, unsigned int devfn,
    int where, int size, u32 val)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(bus.sysdata);
    if (PCI_SLOT(devfn))
    return PCIBIOS_DEVICE_NOT_FOUND;
    dw_pcie_write_dbi(pci, where, size, val);
    return PCIBIOS_SUCCESSFUL;
    }
    static struct pci_ops exynos_pci_ops = {
    .read = exynos_pcie_rd_own_conf,
    .write = exynos_pcie_wr_own_conf,
    };
#[no_mangle]
unsafe extern "C" fn exynos_pcie_link_up(pci: *mut dw_pcie) -> bool {
    static bool exynos_pcie_link_up(struct dw_pcie *pci)
    {
    let mut val: u32 = exynos_pcie_readl(pci.elbi_base, PCIE_ELBI_RDLH_LINKUP);
    return val & PCIE_ELBI_XMLH_LINKUP;
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int {
    static int exynos_pcie_host_init(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct exynos_pcie *ep = to_exynos_pcie(pci);
    pp.bridge.ops = &exynos_pci_ops;
    exynos_pcie_assert_core_reset(ep);
    phy_init(ep.phy);
    phy_power_on(ep.phy);
    exynos_pcie_deassert_core_reset(ep);
    exynos_pcie_enable_irq_pulse(ep);
    return 0;
    }
    static const struct dw_pcie_host_ops exynos_pcie_host_ops = {
    .init = exynos_pcie_host_init,
    };
    static int exynos_add_pcie_port(struct exynos_pcie *ep,
    struct platform_device *pdev)
    {
    struct dw_pcie *pci = &ep.pci;
    struct dw_pcie_rp *pp = &pci.pp;
    struct device *dev = &pdev.dev;
    int ret;
    pp.irq = platform_get_irq(pdev, 0);
    if (pp.irq < 0)
    return pp.irq;
    ret = devm_request_irq(dev, pp.irq, exynos_pcie_irq_handler,
    IRQF_SHARED, "exynos-pcie", ep);
    if (ret) {
    dev_err(dev, "failed to request irq\n");
    return ret;
    }
    pp.ops = &exynos_pcie_host_ops;
    pp.msi_irq[0] = -ENODEV;
    ret = dw_pcie_host_init(pp);
    if (ret) {
    dev_err(dev, "failed to initialize host\n");
    return ret;
    }
    return 0;
    }
    static const struct dw_pcie_ops dw_pcie_ops = {
    .read_dbi = exynos_pcie_read_dbi,
    .write_dbi = exynos_pcie_write_dbi,
    .link_up = exynos_pcie_link_up,
    .start_link = exynos_pcie_start_link,
    };
#[no_mangle]
unsafe extern "C" fn exynos_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct exynos_pcie *ep;
    struct device_node *np = dev.of_node;
    int ret;
    ep = devm_kzalloc(dev, sizeof(*ep), GFP_KERNEL);
    if (!ep)
    return -ENOMEM;
    ep.pci.dev = dev;
    ep.pci.ops = &dw_pcie_ops;
    ep.phy = devm_of_phy_get(dev, np, core::ptr::null_mut());
    if (IS_ERR(ep.phy))
    return PTR_ERR(ep.phy);
    ret = devm_clk_bulk_get_all_enabled(dev, &ep.clks);
    if (ret < 0)
    return ret;
    ep.supplies[0].supply = "vdd18";
    ep.supplies[1].supply = "vdd10";
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(ep.supplies),
    ep.supplies);
    if (ret)
    return ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(ep.supplies), ep.supplies);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, ep);
    ret = exynos_add_pcie_port(ep, pdev);
    if (ret < 0)
    goto fail_probe;
    return 0;
    fail_probe:
    phy_exit(ep.phy);
    regulator_bulk_disable(ARRAY_SIZE(ep.supplies), ep.supplies);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_remove(pdev: *mut platform_device) {
    static void exynos_pcie_remove(struct platform_device *pdev)
    {
    struct exynos_pcie *ep = platform_get_drvdata(pdev);
    dw_pcie_host_deinit(&ep.pci.pp);
    exynos_pcie_assert_core_reset(ep);
    phy_power_off(ep.phy);
    phy_exit(ep.phy);
    regulator_bulk_disable(ARRAY_SIZE(ep.supplies), ep.supplies);
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_suspend_noirq(dev: *mut device) -> c_int {
    static int exynos_pcie_suspend_noirq(struct device *dev)
    {
    struct exynos_pcie *ep = dev_get_drvdata(dev);
    exynos_pcie_assert_core_reset(ep);
    phy_power_off(ep.phy);
    phy_exit(ep.phy);
    regulator_bulk_disable(ARRAY_SIZE(ep.supplies), ep.supplies);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_pcie_resume_noirq(dev: *mut device) -> c_int {
    static int exynos_pcie_resume_noirq(struct device *dev)
    {
    struct exynos_pcie *ep = dev_get_drvdata(dev);
    struct dw_pcie *pci = &ep.pci;
    struct dw_pcie_rp *pp = &pci.pp;
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(ep.supplies), ep.supplies);
    if (ret)
    return ret;
// exynos_pcie_host_init controls ep->phy
    exynos_pcie_host_init(pp);
    dw_pcie_setup_rc(pp);
    exynos_pcie_start_link(pci);
    return dw_pcie_wait_for_link(pci);
    }
    static const struct dev_pm_ops exynos_pcie_pm_ops = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(exynos_pcie_suspend_noirq,
    exynos_pcie_resume_noirq)
    };
    static const struct of_device_id exynos_pcie_of_match[] = {
    { .compatible = "samsung,exynos5433-pcie", },
    { },
    };
    static struct platform_driver exynos_pcie_driver = {
    .probe		= exynos_pcie_probe,
    .remove		= exynos_pcie_remove,
    .driver = {
    .name	= "exynos-pcie",
    .of_match_table = exynos_pcie_of_match,
    .pm		= &exynos_pcie_pm_ops,
    },
    };
    module_platform_driver(exynos_pcie_driver);
    MODULE_DESCRIPTION("Samsung Exynos PCIe host controller driver");
    MODULE_LICENSE("GPL v2");
    MODULE_DEVICE_TABLE(of, exynos_pcie_of_match);
