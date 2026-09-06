//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-sophgo.c
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
// Sophgo DesignWare based PCIe host controller driver
//

pub const PCIE_INT_SIGNAL: c_uint = 0xc48;
pub const PCIE_INT_EN: c_uint = 0xca0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sophgo_pcie {
    pub pci: dw_pcie,
    pub app_base: *mut void __iomem,
    pub clks: *mut clk_bulk_data,
    pub clk_cnt: c_uint,
    pub irq_domain: *mut irq_domain,
}

#[no_mangle]
unsafe extern "C" fn sophgo_pcie_readl_app(sophgo: *mut sophgo_pcie, reg: u32) -> c_int {
    static int sophgo_pcie_readl_app(struct sophgo_pcie *sophgo, u32 reg)
    {
    return readl_relaxed(sophgo.app_base + reg);
    }
#[no_mangle]
unsafe extern "C" fn sophgo_pcie_writel_app(sophgo: *mut sophgo_pcie, val: u32, reg: u32) {
    static void sophgo_pcie_writel_app(struct sophgo_pcie *sophgo, u32 val, u32 reg)
    {
    writel_relaxed(val, sophgo.app_base + reg);
    }
#[no_mangle]
unsafe extern "C" fn sophgo_pcie_intx_handler(desc: *mut irq_desc) {
    static void sophgo_pcie_intx_handler(struct irq_desc *desc)
    {
    struct dw_pcie_rp *pp = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct sophgo_pcie *sophgo = to_sophgo_pcie(pci);
    unsigned long hwirq, reg;
    chained_irq_enter(chip, desc);
    reg = sophgo_pcie_readl_app(sophgo, PCIE_INT_SIGNAL);
    reg = FIELD_GET(PCIE_INT_SIGNAL_INTX, reg);
    for_each_set_bit(hwirq, &reg, PCI_NUM_INTX)
    generic_handle_domain_irq(sophgo.irq_domain, hwirq);
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn sophgo_intx_irq_mask(d: *mut irq_data) {
    static void sophgo_intx_irq_mask(struct irq_data *d)
    {
    struct dw_pcie_rp *pp = irq_data_get_irq_chip_data(d);
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct sophgo_pcie *sophgo = to_sophgo_pcie(pci);
    unsigned long flags;
    u32 val;
    raw_spin_lock_irqsave(&pp.lock, flags);
    val = sophgo_pcie_readl_app(sophgo, PCIE_INT_EN);
    val &= ~FIELD_PREP(PCIE_INT_EN_INTX, BIT(d.hwirq));
    sophgo_pcie_writel_app(sophgo, val, PCIE_INT_EN);
    raw_spin_unlock_irqrestore(&pp.lock, flags);
    };
#[no_mangle]
unsafe extern "C" fn sophgo_intx_irq_unmask(d: *mut irq_data) {
    static void sophgo_intx_irq_unmask(struct irq_data *d)
    {
    struct dw_pcie_rp *pp = irq_data_get_irq_chip_data(d);
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct sophgo_pcie *sophgo = to_sophgo_pcie(pci);
    unsigned long flags;
    u32 val;
    raw_spin_lock_irqsave(&pp.lock, flags);
    val = sophgo_pcie_readl_app(sophgo, PCIE_INT_EN);
    val |= FIELD_PREP(PCIE_INT_EN_INTX, BIT(d.hwirq));
    sophgo_pcie_writel_app(sophgo, val, PCIE_INT_EN);
    raw_spin_unlock_irqrestore(&pp.lock, flags);
    };
    static struct irq_chip sophgo_intx_irq_chip = {
    .name			= "INTx",
    .irq_mask		= sophgo_intx_irq_mask,
    .irq_unmask		= sophgo_intx_irq_unmask,
    };
    static int sophgo_pcie_intx_map(struct irq_domain *domain, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(irq, &sophgo_intx_irq_chip, handle_level_irq);
    irq_set_chip_data(irq, domain.host_data);
    return 0;
    }
    static const struct irq_domain_ops intx_domain_ops = {
    .map = sophgo_pcie_intx_map,
    };
#[no_mangle]
unsafe extern "C" fn sophgo_pcie_init_irq_domain(pp: *mut dw_pcie_rp) -> c_int {
    static int sophgo_pcie_init_irq_domain(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct sophgo_pcie *sophgo = to_sophgo_pcie(pci);
    struct device *dev = sophgo.pci.dev;
    struct fwnode_handle *intc;
    int irq;
    intc = device_get_named_child_node(dev, "interrupt-controller");
    if (!intc) {
    dev_err(dev, "missing child interrupt-controller node\n");
    return -ENODEV;
    }
    irq = fwnode_irq_get(intc, 0);
    if (irq < 0) {
    dev_err(dev, "failed to get INTx irq number\n");
    fwnode_handle_put(intc);
    return irq;
    }
    sophgo.irq_domain = irq_domain_create_linear(intc, PCI_NUM_INTX,
    &intx_domain_ops, pp);
    fwnode_handle_put(intc);
    if (!sophgo.irq_domain) {
    dev_err(dev, "failed to get a INTx irq domain\n");
    return -EINVAL;
    }
    return irq;
    }
#[no_mangle]
unsafe extern "C" fn sophgo_pcie_msi_enable(pp: *mut dw_pcie_rp) {
    static void sophgo_pcie_msi_enable(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct sophgo_pcie *sophgo = to_sophgo_pcie(pci);
    unsigned long flags;
    u32 val;
    raw_spin_lock_irqsave(&pp.lock, flags);
    val = sophgo_pcie_readl_app(sophgo, PCIE_INT_EN);
    val |= PCIE_INT_EN_INT_MSI;
    sophgo_pcie_writel_app(sophgo, val, PCIE_INT_EN);
    raw_spin_unlock_irqrestore(&pp.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sophgo_pcie_disable_l0s_l1(pp: *mut dw_pcie_rp) {
    static void sophgo_pcie_disable_l0s_l1(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    u32 offset, val;
    offset = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    dw_pcie_dbi_ro_wr_en(pci);
    val = dw_pcie_readl_dbi(pci, PCI_EXP_LNKCAP + offset);
    val &= ~(PCI_EXP_LNKCAP_ASPM_L0S | PCI_EXP_LNKCAP_ASPM_L1);
    dw_pcie_writel_dbi(pci, PCI_EXP_LNKCAP + offset, val);
    dw_pcie_dbi_ro_wr_dis(pci);
    }
#[no_mangle]
unsafe extern "C" fn sophgo_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int {
    static int sophgo_pcie_host_init(struct dw_pcie_rp *pp)
    {
    int irq;
    irq = sophgo_pcie_init_irq_domain(pp);
    if (irq < 0)
    return irq;
    irq_set_chained_handler_and_data(irq, sophgo_pcie_intx_handler, pp);
    sophgo_pcie_disable_l0s_l1(pp);
    sophgo_pcie_msi_enable(pp);
    return 0;
    }
    static const struct dw_pcie_host_ops sophgo_pcie_host_ops = {
    .init = sophgo_pcie_host_init,
    };
#[no_mangle]
unsafe extern "C" fn sophgo_pcie_clk_init(sophgo: *mut sophgo_pcie) -> c_int {
    static int sophgo_pcie_clk_init(struct sophgo_pcie *sophgo)
    {
    struct device *dev = sophgo.pci.dev;
    int ret;
    ret = devm_clk_bulk_get_all_enabled(dev, &sophgo.clks);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to get clocks\n");
    sophgo.clk_cnt = ret;
    return 0;
    }
    static int sophgo_pcie_resource_get(struct platform_device *pdev,
    struct sophgo_pcie *sophgo)
    {
    sophgo.app_base = devm_platform_ioremap_resource_byname(pdev, "app");
    if (IS_ERR(sophgo.app_base))
    return dev_err_probe(&pdev.dev, PTR_ERR(sophgo.app_base),
    "failed to map app registers\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sophgo_pcie_configure_rc(sophgo: *mut sophgo_pcie) -> c_int {
    static int sophgo_pcie_configure_rc(struct sophgo_pcie *sophgo)
    {
    struct dw_pcie_rp *pp;
    pp = &sophgo.pci.pp;
    pp.ops = &sophgo_pcie_host_ops;
    return dw_pcie_host_init(pp);
    }
#[no_mangle]
unsafe extern "C" fn sophgo_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int sophgo_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sophgo_pcie *sophgo;
    int ret;
    sophgo = devm_kzalloc(dev, sizeof(*sophgo), GFP_KERNEL);
    if (!sophgo)
    return -ENOMEM;
    platform_set_drvdata(pdev, sophgo);
    sophgo.pci.dev = dev;
    ret = sophgo_pcie_resource_get(pdev, sophgo);
    if (ret)
    return ret;
    ret = sophgo_pcie_clk_init(sophgo);
    if (ret)
    return ret;
    return sophgo_pcie_configure_rc(sophgo);
    }
    static const struct of_device_id sophgo_pcie_of_match[] = {
    { .compatible = "sophgo,sg2044-pcie" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sophgo_pcie_of_match);
    static struct platform_driver sophgo_pcie_driver = {
    .driver = {
    .name = "sophgo-pcie",
    .of_match_table = sophgo_pcie_of_match,
    .suppress_bind_attrs = true,
    },
    .probe = sophgo_pcie_probe,
    };
    builtin_platform_driver(sophgo_pcie_driver);
