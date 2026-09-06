//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-uniphier.c
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
// PCIe host controller driver for UniPhier SoCs
// Copyright 2018 Socionext Inc.
// Author: Kunihiko Hayashi <hayashi.kunihiko@socionext.com>
//

pub const PCL_PINCTRL0: c_uint = 0x002c;

pub const PCL_PIPEMON: c_uint = 0x0044;

pub const PCL_MODE: c_uint = 0x8000;

pub const PCL_APP_READY_CTRL: c_uint = 0x8008;

pub const PCL_APP_PM0: c_uint = 0x8078;

pub const PCL_RCV_INT: c_uint = 0x8108;

pub const PCL_RCV_INTX: c_uint = 0x810c;

pub const PCL_RCV_INTX_MASK_SHIFT: c_int = 8;

pub const PCL_RCV_INTX_STATUS_SHIFT: c_int = 0;
pub const PCL_STATUS_LINK: c_uint = 0x8140;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_pcie {
    pub pci: dw_pcie,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub rst: *mut reset_control,
    pub phy: *mut phy,
    pub intx_irq_domain: *mut irq_domain,
}

    static void uniphier_pcie_ltssm_enable(struct uniphier_pcie *pcie,
    bool enable)
    {
    u32 val;
    val = readl(pcie.base + PCL_APP_READY_CTRL);
    if (enable)
    val |= PCL_APP_LTSSM_ENABLE;
    else
    val &= ~PCL_APP_LTSSM_ENABLE;
    writel(val, pcie.base + PCL_APP_READY_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_init_rc(pcie: *mut uniphier_pcie) {
    static void uniphier_pcie_init_rc(struct uniphier_pcie *pcie)
    {
    u32 val;
// set RC MODE
    val = readl(pcie.base + PCL_MODE);
    val |= PCL_MODE_REGEN;
    val &= ~PCL_MODE_REGVAL;
    writel(val, pcie.base + PCL_MODE);
// use auxiliary power detection
    val = readl(pcie.base + PCL_APP_PM0);
    val |= PCL_SYS_AUX_PWR_DET;
    writel(val, pcie.base + PCL_APP_PM0);
// assert PERST#
    val = readl(pcie.base + PCL_PINCTRL0);
    val &= ~(PCL_PERST_NOE_REGVAL | PCL_PERST_OUT_REGVAL
    | PCL_PERST_PLDN_REGVAL);
    val |= PCL_PERST_NOE_REGEN | PCL_PERST_OUT_REGEN
    | PCL_PERST_PLDN_REGEN;
    writel(val, pcie.base + PCL_PINCTRL0);
    uniphier_pcie_ltssm_enable(pcie, false);
    usleep_range(100000, 200000);
// deassert PERST#
    val = readl(pcie.base + PCL_PINCTRL0);
    val |= PCL_PERST_OUT_REGVAL | PCL_PERST_OUT_REGEN;
    writel(val, pcie.base + PCL_PINCTRL0);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_wait_rc(pcie: *mut uniphier_pcie) -> c_int {
    static int uniphier_pcie_wait_rc(struct uniphier_pcie *pcie)
    {
    u32 status;
    int ret;
// wait PIPE clock
    ret = readl_poll_timeout(pcie.base + PCL_PIPEMON, status,
    status & PCL_PCLK_ALIVE, 100000, 1000000);
    if (ret) {
    dev_err(pcie.pci.dev,
    "Failed to initialize controller in RC mode\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_link_up(pci: *mut dw_pcie) -> bool {
    static bool uniphier_pcie_link_up(struct dw_pcie *pci)
    {
    struct uniphier_pcie *pcie = to_uniphier_pcie(pci);
    u32 val, mask;
    val = readl(pcie.base + PCL_STATUS_LINK);
    mask = PCL_RDLH_LINK_UP | PCL_XMLH_LINK_UP;
    return (val & mask) == mask;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_start_link(pci: *mut dw_pcie) -> c_int {
    static int uniphier_pcie_start_link(struct dw_pcie *pci)
    {
    struct uniphier_pcie *pcie = to_uniphier_pcie(pci);
    uniphier_pcie_ltssm_enable(pcie, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_stop_link(pci: *mut dw_pcie) {
    static void uniphier_pcie_stop_link(struct dw_pcie *pci)
    {
    struct uniphier_pcie *pcie = to_uniphier_pcie(pci);
    uniphier_pcie_ltssm_enable(pcie, false);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_irq_enable(pcie: *mut uniphier_pcie) {
    static void uniphier_pcie_irq_enable(struct uniphier_pcie *pcie)
    {
    writel(PCL_RCV_INT_ALL_ENABLE, pcie.base + PCL_RCV_INT);
    writel(PCL_RCV_INTX_ALL_ENABLE, pcie.base + PCL_RCV_INTX);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_irq_mask(d: *mut irq_data) {
    static void uniphier_pcie_irq_mask(struct irq_data *d)
    {
    struct dw_pcie_rp *pp = irq_data_get_irq_chip_data(d);
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct uniphier_pcie *pcie = to_uniphier_pcie(pci);
    unsigned long flags;
    u32 val;
    raw_spin_lock_irqsave(&pp.lock, flags);
    val = readl(pcie.base + PCL_RCV_INTX);
    val |= BIT(irqd_to_hwirq(d) + PCL_RCV_INTX_MASK_SHIFT);
    writel(val, pcie.base + PCL_RCV_INTX);
    raw_spin_unlock_irqrestore(&pp.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_irq_unmask(d: *mut irq_data) {
    static void uniphier_pcie_irq_unmask(struct irq_data *d)
    {
    struct dw_pcie_rp *pp = irq_data_get_irq_chip_data(d);
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct uniphier_pcie *pcie = to_uniphier_pcie(pci);
    unsigned long flags;
    u32 val;
    raw_spin_lock_irqsave(&pp.lock, flags);
    val = readl(pcie.base + PCL_RCV_INTX);
    val &= ~BIT(irqd_to_hwirq(d) + PCL_RCV_INTX_MASK_SHIFT);
    writel(val, pcie.base + PCL_RCV_INTX);
    raw_spin_unlock_irqrestore(&pp.lock, flags);
    }
    static struct irq_chip uniphier_pcie_irq_chip = {
    .name = "PCI",
    .irq_mask = uniphier_pcie_irq_mask,
    .irq_unmask = uniphier_pcie_irq_unmask,
    };
    static int uniphier_pcie_intx_map(struct irq_domain *domain, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(irq, &uniphier_pcie_irq_chip,
    handle_level_irq);
    irq_set_chip_data(irq, domain.host_data);
    return 0;
    }
    static const struct irq_domain_ops uniphier_intx_domain_ops = {
    .map = uniphier_pcie_intx_map,
    };
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_irq_handler(desc: *mut irq_desc) {
    static void uniphier_pcie_irq_handler(struct irq_desc *desc)
    {
    struct dw_pcie_rp *pp = irq_desc_get_handler_data(desc);
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct uniphier_pcie *pcie = to_uniphier_pcie(pci);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    unsigned long reg;
    u32 val, bit;
// INT for debug
    val = readl(pcie.base + PCL_RCV_INT);
    if (val & PCL_CFG_BW_MGT_STATUS)
    dev_dbg(pci.dev, "Link Bandwidth Management Event\n");
    if (val & PCL_CFG_LINK_AUTO_BW_STATUS)
    dev_dbg(pci.dev, "Link Autonomous Bandwidth Event\n");
    if (val & PCL_CFG_AER_RC_ERR_MSI_STATUS)
    dev_dbg(pci.dev, "Root Error\n");
    if (val & PCL_CFG_PME_MSI_STATUS)
    dev_dbg(pci.dev, "PME Interrupt\n");
    writel(val, pcie.base + PCL_RCV_INT);
// INTx
    chained_irq_enter(chip, desc);
    val = readl(pcie.base + PCL_RCV_INTX);
    reg = FIELD_GET(PCL_RCV_INTX_ALL_STATUS, val);
    for_each_set_bit(bit, &reg, PCI_NUM_INTX)
    generic_handle_domain_irq(pcie.intx_irq_domain, bit);
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_config_intx_irq(pp: *mut dw_pcie_rp) -> c_int {
    static int uniphier_pcie_config_intx_irq(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct uniphier_pcie *pcie = to_uniphier_pcie(pci);
    struct device_node *np = pci.dev.of_node;
    struct device_node *np_intc;
    let mut ret: c_int = 0;
    np_intc = of_get_child_by_name(np, "legacy-interrupt-controller");
    if (!np_intc) {
    dev_err(pci.dev, "Failed to get legacy-interrupt-controller node\n");
    return -EINVAL;
    }
    pp.irq = irq_of_parse_and_map(np_intc, 0);
    if (!pp.irq) {
    dev_err(pci.dev, "Failed to get an IRQ entry in legacy-interrupt-controller\n");
    ret = -EINVAL;
    goto out_put_node;
    }
    pcie.intx_irq_domain = irq_domain_create_linear(of_fwnode_handle(np_intc), PCI_NUM_INTX,
    &uniphier_intx_domain_ops, pp);
    if (!pcie.intx_irq_domain) {
    dev_err(pci.dev, "Failed to get INTx domain\n");
    ret = -ENODEV;
    goto out_put_node;
    }
    irq_set_chained_handler_and_data(pp.irq, uniphier_pcie_irq_handler,
    pp);
    out_put_node:
    of_node_put(np_intc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int {
    static int uniphier_pcie_host_init(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct uniphier_pcie *pcie = to_uniphier_pcie(pci);
    int ret;
    ret = uniphier_pcie_config_intx_irq(pp);
    if (ret)
    return ret;
    uniphier_pcie_irq_enable(pcie);
    return 0;
    }
    static const struct dw_pcie_host_ops uniphier_pcie_host_ops = {
    .init = uniphier_pcie_host_init,
    };
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_host_enable(pcie: *mut uniphier_pcie) -> c_int {
    static int uniphier_pcie_host_enable(struct uniphier_pcie *pcie)
    {
    int ret;
    ret = clk_prepare_enable(pcie.clk);
    if (ret)
    return ret;
    ret = reset_control_deassert(pcie.rst);
    if (ret)
    goto out_clk_disable;
    uniphier_pcie_init_rc(pcie);
    ret = phy_init(pcie.phy);
    if (ret)
    goto out_rst_assert;
    ret = uniphier_pcie_wait_rc(pcie);
    if (ret)
    goto out_phy_exit;
    return 0;
    out_phy_exit:
    phy_exit(pcie.phy);
    out_rst_assert:
    reset_control_assert(pcie.rst);
    out_clk_disable:
    clk_disable_unprepare(pcie.clk);
    return ret;
    }
    static const struct dw_pcie_ops dw_pcie_ops = {
    .start_link = uniphier_pcie_start_link,
    .stop_link = uniphier_pcie_stop_link,
    .link_up = uniphier_pcie_link_up,
    };
#[no_mangle]
unsafe extern "C" fn uniphier_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct uniphier_pcie *pcie;
    int ret;
    pcie = devm_kzalloc(dev, sizeof(*pcie), GFP_KERNEL);
    if (!pcie)
    return -ENOMEM;
    pcie.pci.dev = dev;
    pcie.pci.ops = &dw_pcie_ops;
    pcie.base = devm_platform_ioremap_resource_byname(pdev, "link");
    if (IS_ERR(pcie.base))
    return PTR_ERR(pcie.base);
    pcie.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(pcie.clk))
    return PTR_ERR(pcie.clk);
    pcie.rst = devm_reset_control_get_shared(dev, core::ptr::null_mut());
    if (IS_ERR(pcie.rst))
    return PTR_ERR(pcie.rst);
    pcie.phy = devm_phy_optional_get(dev, "pcie-phy");
    if (IS_ERR(pcie.phy))
    return PTR_ERR(pcie.phy);
    platform_set_drvdata(pdev, pcie);
    ret = uniphier_pcie_host_enable(pcie);
    if (ret)
    return ret;
    pcie.pci.pp.ops = &uniphier_pcie_host_ops;
    return dw_pcie_host_init(&pcie.pci.pp);
    }
    static const struct of_device_id uniphier_pcie_match[] = {
    { .compatible = "socionext,uniphier-pcie", },
    { /* sentinel */ },
    };
    static struct platform_driver uniphier_pcie_driver = {
    .probe  = uniphier_pcie_probe,
    .driver = {
    .name = "uniphier-pcie",
    .of_match_table = uniphier_pcie_match,
    },
    };
    builtin_platform_driver(uniphier_pcie_driver);
