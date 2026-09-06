//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-intel-gw.c
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
// PCIe host controller driver for Intel Gateway SoCs
//
// Copyright (c) 2019 Intel Corporation.
//

pub const PORT_AFR_N_FTS_GEN3: c_int = 180;
pub const PORT_AFR_N_FTS_GEN4: c_int = 196;
// PCIe Application logic Registers
pub const PCIE_APP_CCR: c_uint = 0x10;

pub const PCIE_APP_MSG_CR: c_uint = 0x30;

pub const PCIE_APP_PMC: c_uint = 0x44;

pub const PCIE_APP_IRNEN: c_uint = 0xF4;
pub const PCIE_APP_IRNCR: c_uint = 0xF8;

    (PCIE_APP_IRN_AER_REPORT | PCIE_APP_IRN_PME | \
    PCIE_APP_IRN_RX_VDM_MSG | PCIE_APP_IRN_SYS_ERR_RC | \
    PCIE_APP_IRN_PM_TO_ACK | PCIE_APP_IRN_MSG_LTR | \
    PCIE_APP_IRN_BW_MGT | PCIE_APP_IRN_LINK_AUTO_BW_STAT | \
    PCIE_APP_IRN_INTA | PCIE_APP_IRN_INTB | \
    PCIE_APP_IRN_INTC | PCIE_APP_IRN_INTD)
pub const RESET_INTERVAL_MS: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pcie {
    pub pci: dw_pcie,
    pub app_base: *mut void __iomem,
    pub reset_gpio: *mut gpio_desc,
    pub rst_intrvl: u32,
    pub core_clk: *mut clk,
    pub core_rst: *mut reset_control,
    pub phy: *mut phy,
}

#[no_mangle]
unsafe extern "C" fn pcie_update_bits(base: *mut void __iomem, ofs: u32, mask: u32, val: u32) {
    static void pcie_update_bits(void __iomem *base, u32 ofs, u32 mask, u32 val)
    {
    u32 old;
    old = readl(base + ofs);
    val = (old & ~mask) | (val & mask);
    if (val != old)
    writel(val, base + ofs);
    }
#[no_mangle]
pub unsafe extern "C" fn pcie_app_wr(pcie: *mut intel_pcie, ofs: u32, val: u32) {
    static inline void pcie_app_wr(struct intel_pcie *pcie, u32 ofs, u32 val)
    {
    writel(val, pcie.app_base + ofs);
    }
    static void pcie_app_wr_mask(struct intel_pcie *pcie, u32 ofs,
    u32 mask, u32 val)
    {
    pcie_update_bits(pcie.app_base, ofs, mask, val);
    }
#[no_mangle]
pub unsafe extern "C" fn pcie_rc_cfg_rd(pcie: *mut intel_pcie, ofs: u32) -> u32 {
    static inline u32 pcie_rc_cfg_rd(struct intel_pcie *pcie, u32 ofs)
    {
    return dw_pcie_readl_dbi(&pcie.pci, ofs);
    }
#[no_mangle]
pub unsafe extern "C" fn pcie_rc_cfg_wr(pcie: *mut intel_pcie, ofs: u32, val: u32) {
    static inline void pcie_rc_cfg_wr(struct intel_pcie *pcie, u32 ofs, u32 val)
    {
    dw_pcie_writel_dbi(&pcie.pci, ofs, val);
    }
    static void pcie_rc_cfg_wr_mask(struct intel_pcie *pcie, u32 ofs,
    u32 mask, u32 val)
    {
    pcie_update_bits(pcie.pci.dbi_base, ofs, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_ltssm_enable(pcie: *mut intel_pcie) {
    static void intel_pcie_ltssm_enable(struct intel_pcie *pcie)
    {
    pcie_app_wr_mask(pcie, PCIE_APP_CCR, PCIE_APP_CCR_LTSSM_ENABLE,
    PCIE_APP_CCR_LTSSM_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_ltssm_disable(pcie: *mut intel_pcie) {
    static void intel_pcie_ltssm_disable(struct intel_pcie *pcie)
    {
    pcie_app_wr_mask(pcie, PCIE_APP_CCR, PCIE_APP_CCR_LTSSM_ENABLE, 0);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_link_setup(pcie: *mut intel_pcie) {
    static void intel_pcie_link_setup(struct intel_pcie *pcie)
    {
    u32 val;
    let mut offset: u8 = dw_pcie_find_capability(&pcie.pci, PCI_CAP_ID_EXP);
    val = pcie_rc_cfg_rd(pcie, offset + PCI_EXP_LNKCTL);
    val &= ~(PCI_EXP_LNKCTL_LD | PCI_EXP_LNKCTL_ASPMC);
    pcie_rc_cfg_wr(pcie, offset + PCI_EXP_LNKCTL, val);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_init_n_fts(pci: *mut dw_pcie) {
    static void intel_pcie_init_n_fts(struct dw_pcie *pci)
    {
    switch (pci.max_link_speed) {
    case 3:
    pci.n_fts[1] = PORT_AFR_N_FTS_GEN3;
    break;
    case 4:
    pci.n_fts[1] = PORT_AFR_N_FTS_GEN4;
    break;
    default:
    pci.n_fts[1] = PORT_AFR_N_FTS_GEN12_DFT;
    break;
    }
    pci.n_fts[0] = PORT_AFR_N_FTS_GEN12_DFT;
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_ep_rst_init(pcie: *mut intel_pcie) -> c_int {
    static int intel_pcie_ep_rst_init(struct intel_pcie *pcie)
    {
    struct device *dev = pcie.pci.dev;
    int ret;
    pcie.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(pcie.reset_gpio)) {
    ret = PTR_ERR(pcie.reset_gpio);
    if (ret != -EPROBE_DEFER)
    dev_err(dev, "Failed to request PCIe GPIO: %d\n", ret);
    return ret;
    }
// Make initial reset last for 100us
    usleep_range(100, 200);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_core_rst_assert(pcie: *mut intel_pcie) {
    static void intel_pcie_core_rst_assert(struct intel_pcie *pcie)
    {
    reset_control_assert(pcie.core_rst);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_core_rst_deassert(pcie: *mut intel_pcie) {
    static void intel_pcie_core_rst_deassert(struct intel_pcie *pcie)
    {
//
// One micro-second delay to make sure the reset pulse
// wide enough so that core reset is clean.
//
    udelay(1);
    reset_control_deassert(pcie.core_rst);
//
// Some SoC core reset also reset PHY, more delay needed
// to make sure the reset process is done.
//
    usleep_range(1000, 2000);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_device_rst_assert(pcie: *mut intel_pcie) {
    static void intel_pcie_device_rst_assert(struct intel_pcie *pcie)
    {
    gpiod_set_value_cansleep(pcie.reset_gpio, 1);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_device_rst_deassert(pcie: *mut intel_pcie) {
    static void intel_pcie_device_rst_deassert(struct intel_pcie *pcie)
    {
    msleep(pcie.rst_intrvl);
    gpiod_set_value_cansleep(pcie.reset_gpio, 0);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_core_irq_enable(pcie: *mut intel_pcie) {
    static void intel_pcie_core_irq_enable(struct intel_pcie *pcie)
    {
    pcie_app_wr(pcie, PCIE_APP_IRNEN, 0);
    pcie_app_wr(pcie, PCIE_APP_IRNCR, PCIE_APP_IRN_INT);
    pcie_app_wr(pcie, PCIE_APP_IRNEN, PCIE_APP_IRN_INT);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_core_irq_disable(pcie: *mut intel_pcie) {
    static void intel_pcie_core_irq_disable(struct intel_pcie *pcie)
    {
    pcie_app_wr(pcie, PCIE_APP_IRNEN, 0);
    pcie_app_wr(pcie, PCIE_APP_IRNCR, PCIE_APP_IRN_INT);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_get_resources(pdev: *mut platform_device) -> c_int {
    static int intel_pcie_get_resources(struct platform_device *pdev)
    {
    struct intel_pcie *pcie = platform_get_drvdata(pdev);
    struct dw_pcie *pci = &pcie.pci;
    struct device *dev = pci.dev;
    int ret;
    pcie.core_clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(pcie.core_clk)) {
    ret = PTR_ERR(pcie.core_clk);
    if (ret != -EPROBE_DEFER)
    dev_err(dev, "Failed to get clks: %d\n", ret);
    return ret;
    }
    pcie.core_rst = devm_reset_control_get(dev, core::ptr::null_mut());
    if (IS_ERR(pcie.core_rst)) {
    ret = PTR_ERR(pcie.core_rst);
    if (ret != -EPROBE_DEFER)
    dev_err(dev, "Failed to get resets: %d\n", ret);
    return ret;
    }
    ret = device_property_read_u32(dev, "reset-assert-ms",
    &pcie.rst_intrvl);
    if (ret)
    pcie.rst_intrvl = RESET_INTERVAL_MS;
    pcie.app_base = devm_platform_ioremap_resource_byname(pdev, "app");
    if (IS_ERR(pcie.app_base))
    return PTR_ERR(pcie.app_base);
    pcie.phy = devm_phy_get(dev, "pcie");
    if (IS_ERR(pcie.phy)) {
    ret = PTR_ERR(pcie.phy);
    if (ret != -EPROBE_DEFER)
    dev_err(dev, "Couldn't get pcie-phy: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_wait_l2(pcie: *mut intel_pcie) -> c_int {
    static int intel_pcie_wait_l2(struct intel_pcie *pcie)
    {
    u32 value;
    int ret;
    struct dw_pcie *pci = &pcie.pci;
    if (pci.max_link_speed < 3)
    return 0;
// Send PME_TURN_OFF message
    pcie_app_wr_mask(pcie, PCIE_APP_MSG_CR, PCIE_APP_MSG_XMT_PM_TURNOFF,
    PCIE_APP_MSG_XMT_PM_TURNOFF);
// Read PMC status and wait for falling into L2 link state
    ret = readl_poll_timeout(pcie.app_base + PCIE_APP_PMC, value,
    value & PCIE_APP_PMC_IN_L2, 20,
    jiffies_to_usecs(5 * HZ));
    if (ret)
    dev_err(pcie.pci.dev, "PCIe link enter L2 timeout!\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_turn_off(pcie: *mut intel_pcie) {
    static void intel_pcie_turn_off(struct intel_pcie *pcie)
    {
    if (dw_pcie_link_up(&pcie.pci))
    intel_pcie_wait_l2(pcie);
// Put endpoint device in reset state
    intel_pcie_device_rst_assert(pcie);
    pcie_rc_cfg_wr_mask(pcie, PCI_COMMAND, PCI_COMMAND_MEMORY, 0);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_start_link(pci: *mut dw_pcie) -> c_int {
    static int intel_pcie_start_link(struct dw_pcie *pci)
    {
    struct intel_pcie *pcie = dev_get_drvdata(pci.dev);
    intel_pcie_device_rst_deassert(pcie);
    intel_pcie_ltssm_enable(pcie);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_host_setup(pcie: *mut intel_pcie) -> c_int {
    static int intel_pcie_host_setup(struct intel_pcie *pcie)
    {
    int ret;
    struct dw_pcie *pci = &pcie.pci;
    intel_pcie_core_rst_assert(pcie);
    intel_pcie_device_rst_assert(pcie);
    intel_pcie_core_rst_deassert(pcie);
// Controller clock must be provided earlier than PHY
    ret = clk_prepare_enable(pcie.core_clk);
    if (ret) {
    dev_err(pcie.pci.dev, "Core clock enable failed: %d\n", ret);
    goto clk_err;
    }
    ret = phy_init(pcie.phy);
    if (ret)
    goto phy_err;
    intel_pcie_ltssm_disable(pcie);
    intel_pcie_link_setup(pcie);
    intel_pcie_init_n_fts(pci);
    dw_pcie_upconfig_setup(pci);
    intel_pcie_core_irq_enable(pcie);
    return 0;
    phy_err:
    clk_disable_unprepare(pcie.core_clk);
    clk_err:
    intel_pcie_core_rst_assert(pcie);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __intel_pcie_remove(pcie: *mut intel_pcie) {
    static void __intel_pcie_remove(struct intel_pcie *pcie)
    {
    intel_pcie_core_irq_disable(pcie);
    intel_pcie_turn_off(pcie);
    clk_disable_unprepare(pcie.core_clk);
    intel_pcie_core_rst_assert(pcie);
    phy_exit(pcie.phy);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_remove(pdev: *mut platform_device) {
    static void intel_pcie_remove(struct platform_device *pdev)
    {
    struct intel_pcie *pcie = platform_get_drvdata(pdev);
    struct dw_pcie_rp *pp = &pcie.pci.pp;
    dw_pcie_host_deinit(pp);
    __intel_pcie_remove(pcie);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_suspend_noirq(dev: *mut device) -> c_int {
    static int intel_pcie_suspend_noirq(struct device *dev)
    {
    struct intel_pcie *pcie = dev_get_drvdata(dev);
    int ret;
    intel_pcie_core_irq_disable(pcie);
    ret = intel_pcie_wait_l2(pcie);
    if (ret)
    return ret;
    phy_exit(pcie.phy);
    clk_disable_unprepare(pcie.core_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_resume_noirq(dev: *mut device) -> c_int {
    static int intel_pcie_resume_noirq(struct device *dev)
    {
    struct intel_pcie *pcie = dev_get_drvdata(dev);
    return intel_pcie_host_setup(pcie);
    }
#[no_mangle]
unsafe extern "C" fn intel_pcie_rc_init(pp: *mut dw_pcie_rp) -> c_int {
    static int intel_pcie_rc_init(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct intel_pcie *pcie = dev_get_drvdata(pci.dev);
    return intel_pcie_host_setup(pcie);
    }
    static const struct dw_pcie_ops intel_pcie_ops = {
    .start_link = intel_pcie_start_link,
    };
    static const struct dw_pcie_host_ops intel_pcie_dw_ops = {
    .init = intel_pcie_rc_init,
    };
#[no_mangle]
unsafe extern "C" fn intel_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int intel_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct intel_pcie *pcie;
    struct dw_pcie_rp *pp;
    struct resource *res;
    struct dw_pcie *pci;
    int ret;
    pcie = devm_kzalloc(dev, sizeof(*pcie), GFP_KERNEL);
    if (!pcie)
    return -ENOMEM;
    platform_set_drvdata(pdev, pcie);
    pci = &pcie.pci;
    pci.dev = dev;
    pci.use_parent_dt_ranges = true;
    pp = &pci.pp;
    ret = intel_pcie_get_resources(pdev);
    if (ret)
    return ret;
    ret = intel_pcie_ep_rst_init(pcie);
    if (ret)
    return ret;
    pci.ops = &intel_pcie_ops;
    pp.ops = &intel_pcie_dw_ops;
//
// If the 'atu' region is not available in the devicetree, use the
// default offset from DBI region for backwards compatibility. The
// 'atu' region should always be specified in the devicetree, as
// this is a hardware-specific address that should not be defined
// in the driver.
//
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "atu");
    if (!res) {
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "dbi");
    pci.dbi_base = devm_pci_remap_cfg_resource(pci.dev, res);
    if (IS_ERR(pci.dbi_base))
    return PTR_ERR(pci.dbi_base);
    pci.dbi_phys_addr = res.start;
    pci.atu_base = devm_ioremap(dev, res.start + 0xC0000, SZ_4K);
    if (!pci.atu_base) {
    dev_err(dev, "failed to remap ATU space\n");
    return -ENOMEM;
    }
    pci.atu_size = SZ_4K;
    pci.atu_phys_addr = res.start + 0xC0000;
    dev_warn(dev, "ATU region not specified in DT. Using default offset\n");
    }
    ret = dw_pcie_host_init(pp);
    if (ret) {
    dev_err(dev, "Cannot initialize host\n");
    return ret;
    }
    return 0;
    }
    static const struct dev_pm_ops intel_pcie_pm_ops = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(intel_pcie_suspend_noirq,
    intel_pcie_resume_noirq)
    };
    static const struct of_device_id of_intel_pcie_match[] = {
    { .compatible = "intel,lgm-pcie" },
    {}
    };
    static struct platform_driver intel_pcie_driver = {
    .probe = intel_pcie_probe,
    .remove = intel_pcie_remove,
    .driver = {
    .name = "intel-gw-pcie",
    .of_match_table = of_intel_pcie_match,
    .pm = &intel_pcie_pm_ops,
    },
    };
    builtin_platform_driver(intel_pcie_driver);
