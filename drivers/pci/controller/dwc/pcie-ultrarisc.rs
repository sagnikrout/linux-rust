//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-ultrarisc.c
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
// DWC PCIe RC driver for UltraRISC SoCs
//
// Copyright (C) 2026 UltraRISC Technology (Shanghai) Co., Ltd.
//

pub const PCIE_CUS_CORE: c_uint = 0x400000;

pub const ULTRARISC_PCIE_COMP_TIMEOUT_65_210MS: c_uint = 0x6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ultrarisc_pcie {
    pub pci: dw_pcie,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
}

    static struct pci_ops ultrarisc_pci_ops = {
    .map_bus = dw_pcie_own_conf_map_bus,
    .read = pci_generic_config_read32,
    .write = pci_generic_config_write32,
    };
#[no_mangle]
unsafe extern "C" fn ultrarisc_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int {
    static int ultrarisc_pcie_host_init(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct pci_host_bridge *bridge = pp.bridge;
    u8 cap_exp;
    u32 val;
    bridge.ops = &ultrarisc_pci_ops;
    if (dw_pcie_link_up(pci))
    return 0;
    val = dw_pcie_readl_dbi(pci, PCIE_CUS_CORE);
    val &= ~FAST_LINK_MODE;
    dw_pcie_writel_dbi(pci, PCIE_CUS_CORE, val);
    val = dw_pcie_readl_dbi(pci, PCIE_TIMER_CTRL_MAX_FUNC_NUM);
    FIELD_MODIFY(PORT_FLT_SF_MASK, &val, PORT_FLT_SF_VAL_64);
    dw_pcie_writel_dbi(pci, PCIE_TIMER_CTRL_MAX_FUNC_NUM, val);
    cap_exp = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    val = dw_pcie_readl_dbi(pci, cap_exp + PCI_EXP_LNKCTL2);
    FIELD_MODIFY(PCI_EXP_LNKCTL2_TLS, &val, PCI_EXP_LNKCTL2_TLS_16_0GT);
    dw_pcie_writel_dbi(pci, cap_exp + PCI_EXP_LNKCTL2, val);
    val = dw_pcie_readl_dbi(pci, PCIE_PORT_FORCE);
    FIELD_MODIFY(PORT_LINK_NUM_MASK, &val, 0);
    dw_pcie_writel_dbi(pci, PCIE_PORT_FORCE, val);
    val = dw_pcie_readl_dbi(pci, cap_exp + PCI_EXP_DEVCTL2);
    FIELD_MODIFY(PCI_EXP_DEVCTL2_COMP_TIMEOUT, &val,
    ULTRARISC_PCIE_COMP_TIMEOUT_65_210MS);
    dw_pcie_writel_dbi(pci, cap_exp + PCI_EXP_DEVCTL2, val);
    val = dw_pcie_readl_dbi(pci, PCIE_CUS_CORE);
    val &= ~(HOLD_PHY_RST | L1SUB_DISABLE);
    dw_pcie_writel_dbi(pci, PCIE_CUS_CORE, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ultrarisc_pcie_pme_turn_off(pp: *mut dw_pcie_rp) {
    static void ultrarisc_pcie_pme_turn_off(struct dw_pcie_rp *pp)
    {
//
// DP1000 does not support sending PME_Turn_Off from the RC.
// Keep this callback empty to skip the generic MSG TLP path.
//
    }
    static const struct dw_pcie_host_ops ultrarisc_pcie_host_ops = {
    .init = ultrarisc_pcie_host_init,
    .pme_turn_off = ultrarisc_pcie_pme_turn_off,
    };
#[no_mangle]
unsafe extern "C" fn ultrarisc_pcie_start_link(pci: *mut dw_pcie) -> c_int {
    static int ultrarisc_pcie_start_link(struct dw_pcie *pci)
    {
    u32 val;
    val = dw_pcie_readl_dbi(pci, PCIE_CUS_CORE);
    val |= LTSSM_ENABLE;
    dw_pcie_writel_dbi(pci, PCIE_CUS_CORE, val);
    return 0;
    }
    static const struct dw_pcie_ops dw_pcie_ops = {
    .start_link = ultrarisc_pcie_start_link,
    };
#[no_mangle]
unsafe extern "C" fn ultrarisc_pcie_enable_clks(ultra: *mut ultrarisc_pcie) -> c_int {
    static int ultrarisc_pcie_enable_clks(struct ultrarisc_pcie *ultra)
    {
    return clk_bulk_prepare_enable(ultra.num_clks, ultra.clks);
    }
#[no_mangle]
unsafe extern "C" fn ultrarisc_pcie_disable_clks(data: *mut c_void) {
    static void ultrarisc_pcie_disable_clks(void *data)
    {
    struct ultrarisc_pcie *ultra = data;
    clk_bulk_disable_unprepare(ultra.num_clks, ultra.clks);
    }
#[no_mangle]
unsafe extern "C" fn ultrarisc_pcie_init_clks(ultra: *mut ultrarisc_pcie) -> c_int {
    static int ultrarisc_pcie_init_clks(struct ultrarisc_pcie *ultra)
    {
    struct device *dev = ultra.pci.dev;
    int ret;
    ultra.num_clks = devm_clk_bulk_get_all(dev, &ultra.clks);
    if (ultra.num_clks < 0)
    return dev_err_probe(dev, ultra.num_clks, "Failed to get clocks\n");
    ret = ultrarisc_pcie_enable_clks(ultra);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable clocks\n");
    return devm_add_action_or_reset(dev, ultrarisc_pcie_disable_clks, ultra);
    }
#[no_mangle]
unsafe extern "C" fn ultrarisc_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int ultrarisc_pcie_probe(struct platform_device *pdev)
    {
    struct ultrarisc_pcie *ultra;
    struct device *dev = &pdev.dev;
    struct dw_pcie_rp *pp;
    struct dw_pcie *pci;
    int ret;
    ultra = devm_kzalloc(dev, sizeof(*ultra), GFP_KERNEL);
    if (!ultra)
    return -ENOMEM;
    pci = &ultra.pci;
    pci.dev = dev;
    pci.ops = &dw_pcie_ops;
// Set a default value suitable for at most 16 in and 16 out windows
    pci.atu_size = SZ_8K;
    pp = &pci.pp;
    platform_set_drvdata(pdev, ultra);
    ret = ultrarisc_pcie_init_clks(ultra);
    if (ret)
    return ret;
    pp.num_vectors = MAX_MSI_IRQS;
// No L2/L3 Ready indication is available on this platform
    pp.skip_l23_ready = true;
    pp.ops = &ultrarisc_pcie_host_ops;
    ret = dw_pcie_host_init(pp);
    if (ret) {
    dev_err(dev, "Failed to initialize host\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ultrarisc_pcie_suspend_noirq(dev: *mut device) -> c_int {
    static int ultrarisc_pcie_suspend_noirq(struct device *dev)
    {
    struct ultrarisc_pcie *ultra = dev_get_drvdata(dev);
    struct dw_pcie *pci = &ultra.pci;
    int ret;
//
// A failed resume leaves the DWC suspended and the clocks disabled.
// A later suspend must not access the controller or disable them again.
//
    if (pci.suspended)
    return 0;
    ret = dw_pcie_suspend_noirq(pci);
    if (ret)
    return ret;
    if (pci.suspended)
    ultrarisc_pcie_disable_clks(ultra);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ultrarisc_pcie_resume_noirq(dev: *mut device) -> c_int {
    static int ultrarisc_pcie_resume_noirq(struct device *dev)
    {
    struct ultrarisc_pcie *ultra = dev_get_drvdata(dev);
    struct dw_pcie *pci = &ultra.pci;
    int ret;
    if (pci.suspended) {
    ret = ultrarisc_pcie_enable_clks(ultra);
    if (ret)
    return ret;
    ret = dw_pcie_resume_noirq(pci);
    if (ret) {
    ultrarisc_pcie_disable_clks(ultra);
    return ret;
    }
    }
    return 0;
    }
    static const struct dev_pm_ops ultrarisc_pcie_pm_ops = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(ultrarisc_pcie_suspend_noirq,
    ultrarisc_pcie_resume_noirq)
    };
    static const struct of_device_id ultrarisc_pcie_of_match[] = {
    {
    .compatible = "ultrarisc,dp1000-pcie",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, ultrarisc_pcie_of_match);
    static struct platform_driver ultrarisc_pcie_driver = {
    .driver = {
    .name	= "ultrarisc-pcie",
    .of_match_table = ultrarisc_pcie_of_match,
    .suppress_bind_attrs = true,
    .pm = &ultrarisc_pcie_pm_ops,
    },
    .probe = ultrarisc_pcie_probe,
    };
    module_platform_driver(ultrarisc_pcie_driver);
    MODULE_DESCRIPTION("UltraRISC DP1000 DWC PCIe host controller");
    MODULE_LICENSE("GPL");
