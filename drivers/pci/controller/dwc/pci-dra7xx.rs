//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pci-dra7xx.c
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
// pcie-dra7xx - PCIe controller driver for TI DRA7xx SoCs
//
// Copyright (C) 2013-2014 Texas Instruments Incorporated - https://www.ti.com
//
// Authors: Kishon Vijay Abraham I <kishon@ti.com>
//

// PCIe controller wrapper DRA7XX configuration registers
pub const PCIECTRL_DRA7XX_CONF_IRQSTATUS_MAIN: c_uint = 0x0024;
pub const PCIECTRL_DRA7XX_CONF_IRQENABLE_SET_MAIN: c_uint = 0x0028;

    ERR_ECRC | PME_TURN_OFF | PME_TO_ACK | PM_PME | \
    LINK_REQ_RST | LINK_UP_EVT | CFG_BME_EVT | CFG_MSE_EVT)
pub const PCIECTRL_DRA7XX_CONF_IRQSTATUS_MSI: c_uint = 0x0034;
pub const PCIECTRL_DRA7XX_CONF_IRQENABLE_SET_MSI: c_uint = 0x0038;

pub const PCIECTRL_TI_CONF_DEVICE_TYPE: c_uint = 0x0100;
pub const DEVICE_TYPE_EP: c_uint = 0x0;
pub const DEVICE_TYPE_LEG_EP: c_uint = 0x1;
pub const DEVICE_TYPE_RC: c_uint = 0x4;
pub const PCIECTRL_DRA7XX_CONF_DEVICE_CMD: c_uint = 0x0104;
pub const LTSSM_EN: c_uint = 0x1;
pub const PCIECTRL_DRA7XX_CONF_PHY_CS: c_uint = 0x010C;

pub const DRA7XX_CPU_TO_BUS_ADDR: c_uint = 0x0FFFFFFF;
pub const PCIECTRL_TI_CONF_INTX_ASSERT: c_uint = 0x0124;
pub const PCIECTRL_TI_CONF_INTX_DEASSERT: c_uint = 0x0128;
pub const PCIECTRL_TI_CONF_MSI_XMT: c_uint = 0x012c;

pub const MSI_VECTOR_SHIFT: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dra7xx_pcie {
    pub pci: *mut dw_pcie,
    pub /: *mut *mut *mut void __iomem base; / DT ti_conf,
    pub /: *mut *mut int phy_count; / DT phy-names count,
    pub phy: *mut phy,
    pub irq_domain: *mut irq_domain,
    pub clk: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dra7xx_pcie_of_data {
    pub mode: enum dw_pcie_device_mode,
    pub b1co_mode_sel_mask: u32,
}

#[no_mangle]
pub unsafe extern "C" fn dra7xx_pcie_readl(pcie: *mut dra7xx_pcie, offset: u32) -> u32 {
    static inline u32 dra7xx_pcie_readl(struct dra7xx_pcie *pcie, u32 offset)
    {
    return readl(pcie.base + offset);
    }
    static inline void dra7xx_pcie_writel(struct dra7xx_pcie *pcie, u32 offset,
    u32 value)
    {
    writel(value, pcie.base + offset);
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_cpu_addr_fixup(pci: *mut dw_pcie, cpu_addr: u64) -> u64 {
    static u64 dra7xx_pcie_cpu_addr_fixup(struct dw_pcie *pci, u64 cpu_addr)
    {
    return cpu_addr & DRA7XX_CPU_TO_BUS_ADDR;
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_link_up(pci: *mut dw_pcie) -> bool {
    static bool dra7xx_pcie_link_up(struct dw_pcie *pci)
    {
    struct dra7xx_pcie *dra7xx = to_dra7xx_pcie(pci);
    let mut reg: u32 = dra7xx_pcie_readl(dra7xx, PCIECTRL_DRA7XX_CONF_PHY_CS);
    return reg & LINK_UP;
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_stop_link(pci: *mut dw_pcie) {
    static void dra7xx_pcie_stop_link(struct dw_pcie *pci)
    {
    struct dra7xx_pcie *dra7xx = to_dra7xx_pcie(pci);
    u32 reg;
    reg = dra7xx_pcie_readl(dra7xx, PCIECTRL_DRA7XX_CONF_DEVICE_CMD);
    reg &= ~LTSSM_EN;
    dra7xx_pcie_writel(dra7xx, PCIECTRL_DRA7XX_CONF_DEVICE_CMD, reg);
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_establish_link(pci: *mut dw_pcie) -> c_int {
    static int dra7xx_pcie_establish_link(struct dw_pcie *pci)
    {
    struct dra7xx_pcie *dra7xx = to_dra7xx_pcie(pci);
    struct device *dev = pci.dev;
    u32 reg;
    if (dw_pcie_link_up(pci)) {
    dev_err(dev, "link is already up\n");
    return 0;
    }
    reg = dra7xx_pcie_readl(dra7xx, PCIECTRL_DRA7XX_CONF_DEVICE_CMD);
    reg |= LTSSM_EN;
    dra7xx_pcie_writel(dra7xx, PCIECTRL_DRA7XX_CONF_DEVICE_CMD, reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_enable_msi_interrupts(dra7xx: *mut dra7xx_pcie) {
    static void dra7xx_pcie_enable_msi_interrupts(struct dra7xx_pcie *dra7xx)
    {
    dra7xx_pcie_writel(dra7xx, PCIECTRL_DRA7XX_CONF_IRQSTATUS_MSI,
    LEG_EP_INTERRUPTS | MSI);
    dra7xx_pcie_writel(dra7xx,
    PCIECTRL_DRA7XX_CONF_IRQENABLE_SET_MSI,
    MSI | LEG_EP_INTERRUPTS);
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_enable_wrapper_interrupts(dra7xx: *mut dra7xx_pcie) {
    static void dra7xx_pcie_enable_wrapper_interrupts(struct dra7xx_pcie *dra7xx)
    {
    dra7xx_pcie_writel(dra7xx, PCIECTRL_DRA7XX_CONF_IRQSTATUS_MAIN,
    INTERRUPTS);
    dra7xx_pcie_writel(dra7xx, PCIECTRL_DRA7XX_CONF_IRQENABLE_SET_MAIN,
    INTERRUPTS);
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_enable_interrupts(dra7xx: *mut dra7xx_pcie) {
    static void dra7xx_pcie_enable_interrupts(struct dra7xx_pcie *dra7xx)
    {
    dra7xx_pcie_enable_wrapper_interrupts(dra7xx);
    dra7xx_pcie_enable_msi_interrupts(dra7xx);
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int {
    static int dra7xx_pcie_host_init(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct dra7xx_pcie *dra7xx = to_dra7xx_pcie(pci);
    dra7xx_pcie_enable_interrupts(dra7xx);
    return 0;
    }
    static int dra7xx_pcie_intx_map(struct irq_domain *domain, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(irq, &dummy_irq_chip, handle_simple_irq);
    irq_set_chip_data(irq, domain.host_data);
    return 0;
    }
    static const struct irq_domain_ops intx_domain_ops = {
    .map = dra7xx_pcie_intx_map,
    .xlate = pci_irqd_intx_xlate,
    };
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_handle_msi(pp: *mut dw_pcie_rp, index: c_int) -> c_int {
    static int dra7xx_pcie_handle_msi(struct dw_pcie_rp *pp, int index)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    unsigned long val;
    int pos;
    val = dw_pcie_readl_dbi(pci, PCIE_MSI_INTR0_STATUS +
    (index * MSI_REG_CTRL_BLOCK_SIZE));
    if (!val)
    return 0;
    pos = find_first_bit(&val, MAX_MSI_IRQS_PER_CTRL);
    while (pos != MAX_MSI_IRQS_PER_CTRL) {
    generic_handle_domain_irq(pp.irq_domain,
    (index * MAX_MSI_IRQS_PER_CTRL) + pos);
    pos++;
    pos = find_next_bit(&val, MAX_MSI_IRQS_PER_CTRL, pos);
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_handle_msi_irq(pp: *mut dw_pcie_rp) {
    static void dra7xx_pcie_handle_msi_irq(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    int ret, i, count, num_ctrls;
    num_ctrls = pp.num_vectors / MAX_MSI_IRQS_PER_CTRL;
//
// Need to make sure all MSI status bits read 0 before exiting.
// Else, new MSI IRQs are not registered by the wrapper. Have an
// upperbound for the loop and exit the IRQ in case of IRQ flood
// to avoid locking up system in interrupt context.
//
    count = 0;
    do {
    ret = 0;
    for (i = 0; i < num_ctrls; i++)
    ret |= dra7xx_pcie_handle_msi(pp, i);
    count++;
    } while (ret && count <= 1000);
    if (count > 1000)
    dev_warn_ratelimited(pci.dev,
    "Too many MSI IRQs to handle\n");
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_msi_irq_handler(desc: *mut irq_desc) {
    static void dra7xx_pcie_msi_irq_handler(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    struct dra7xx_pcie *dra7xx;
    struct dw_pcie_rp *pp;
    struct dw_pcie *pci;
    unsigned long reg;
    u32 bit;
    chained_irq_enter(chip, desc);
    pp = irq_desc_get_handler_data(desc);
    pci = to_dw_pcie_from_pp(pp);
    dra7xx = to_dra7xx_pcie(pci);
    reg = dra7xx_pcie_readl(dra7xx, PCIECTRL_DRA7XX_CONF_IRQSTATUS_MSI);
    dra7xx_pcie_writel(dra7xx, PCIECTRL_DRA7XX_CONF_IRQSTATUS_MSI, reg);
    switch (reg) {
    case MSI:
    dra7xx_pcie_handle_msi_irq(pp);
    break;
    case INTA:
    case INTB:
    case INTC:
    case INTD:
    for_each_set_bit(bit, &reg, PCI_NUM_INTX)
    generic_handle_domain_irq(dra7xx.irq_domain, bit);
    break;
    }
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t dra7xx_pcie_irq_handler(int irq, void *arg)
    {
    struct dra7xx_pcie *dra7xx = arg;
    struct dw_pcie *pci = dra7xx.pci;
    struct device *dev = pci.dev;
    struct dw_pcie_ep *ep = &pci.ep;
    u32 reg;
    reg = dra7xx_pcie_readl(dra7xx, PCIECTRL_DRA7XX_CONF_IRQSTATUS_MAIN);
    if (reg & ERR_SYS)
    dev_dbg(dev, "System Error\n");
    if (reg & ERR_FATAL)
    dev_dbg(dev, "Fatal Error\n");
    if (reg & ERR_NONFATAL)
    dev_dbg(dev, "Non Fatal Error\n");
    if (reg & ERR_COR)
    dev_dbg(dev, "Correctable Error\n");
    if (reg & ERR_AXI)
    dev_dbg(dev, "AXI tag lookup fatal Error\n");
    if (reg & ERR_ECRC)
    dev_dbg(dev, "ECRC Error\n");
    if (reg & PME_TURN_OFF)
    dev_dbg(dev,
    "Power Management Event Turn-Off message received\n");
    if (reg & PME_TO_ACK)
    dev_dbg(dev,
    "Power Management Turn-Off Ack message received\n");
    if (reg & PM_PME)
    dev_dbg(dev, "PM Power Management Event message received\n");
    if (reg & LINK_REQ_RST)
    dev_dbg(dev, "Link Request Reset\n");
    if (reg & LINK_UP_EVT) {
    if (dra7xx.pci.mode == DW_PCIE_EP_TYPE)
    dw_pcie_ep_linkup(ep);
    dev_dbg(dev, "Link-up state change\n");
    }
    if (reg & CFG_BME_EVT)
    dev_dbg(dev, "CFG 'Bus Master Enable' change\n");
    if (reg & CFG_MSE_EVT)
    dev_dbg(dev, "CFG 'Memory Space Enable' change\n");
    dra7xx_pcie_writel(dra7xx, PCIECTRL_DRA7XX_CONF_IRQSTATUS_MAIN, reg);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_init_irq_domain(pp: *mut dw_pcie_rp) -> c_int {
    static int dra7xx_pcie_init_irq_domain(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct device *dev = pci.dev;
    struct dra7xx_pcie *dra7xx = to_dra7xx_pcie(pci);
    struct device_node *node = dev.of_node;
    struct device_node *pcie_intc_node =  of_get_next_child(node, core::ptr::null_mut());
    if (!pcie_intc_node) {
    dev_err(dev, "No PCIe Intc node found\n");
    return -ENODEV;
    }
    irq_set_chained_handler_and_data(pp.irq, dra7xx_pcie_msi_irq_handler,
    pp);
    dra7xx.irq_domain = irq_domain_create_linear(of_fwnode_handle(pcie_intc_node),
    PCI_NUM_INTX, &intx_domain_ops, pp);
    of_node_put(pcie_intc_node);
    if (!dra7xx.irq_domain) {
    dev_err(dev, "Failed to get a INTx IRQ domain\n");
    return -ENODEV;
    }
    return 0;
    }
    static const struct dw_pcie_host_ops dra7xx_pcie_host_ops = {
    .init = dra7xx_pcie_host_init,
    };
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_ep_init(ep: *mut dw_pcie_ep) -> c_int {
    static int dra7xx_pcie_ep_init(struct dw_pcie_ep *ep)
    {
    struct dw_pcie *pci = to_dw_pcie_from_ep(ep);
    struct dra7xx_pcie *dra7xx = to_dra7xx_pcie(pci);
    dra7xx_pcie_enable_wrapper_interrupts(dra7xx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_raise_intx_irq(dra7xx: *mut dra7xx_pcie) {
    static void dra7xx_pcie_raise_intx_irq(struct dra7xx_pcie *dra7xx)
    {
    dra7xx_pcie_writel(dra7xx, PCIECTRL_TI_CONF_INTX_ASSERT, 0x1);
    mdelay(1);
    dra7xx_pcie_writel(dra7xx, PCIECTRL_TI_CONF_INTX_DEASSERT, 0x1);
    }
    static void dra7xx_pcie_raise_msi_irq(struct dra7xx_pcie *dra7xx,
    u8 interrupt_num)
    {
    u32 reg;
    reg = (interrupt_num - 1) << MSI_VECTOR_SHIFT;
    reg |= MSI_REQ_GRANT;
    dra7xx_pcie_writel(dra7xx, PCIECTRL_TI_CONF_MSI_XMT, reg);
    }
    static int dra7xx_pcie_raise_irq(struct dw_pcie_ep *ep, u8 func_no,
    unsigned int type, u16 interrupt_num)
    {
    struct dw_pcie *pci = to_dw_pcie_from_ep(ep);
    struct dra7xx_pcie *dra7xx = to_dra7xx_pcie(pci);
    switch (type) {
    case PCI_IRQ_INTX:
    dra7xx_pcie_raise_intx_irq(dra7xx);
    break;
    case PCI_IRQ_MSI:
    dra7xx_pcie_raise_msi_irq(dra7xx, interrupt_num);
    break;
    default:
    dev_err(pci.dev, "UNKNOWN IRQ type\n");
    }
    return 0;
    }
    static const struct pci_epc_features dra7xx_pcie_epc_features = {
    DWC_EPC_COMMON_FEATURES,
    .linkup_notifier = true,
    .msi_capable = true,
    };
    static const struct pci_epc_features*
    dra7xx_pcie_get_features(struct dw_pcie_ep *ep)
    {
    return &dra7xx_pcie_epc_features;
    }
    static const struct dw_pcie_ep_ops pcie_ep_ops = {
    .init = dra7xx_pcie_ep_init,
    .raise_irq = dra7xx_pcie_raise_irq,
    .get_features = dra7xx_pcie_get_features,
    };
    static int dra7xx_add_pcie_ep(struct dra7xx_pcie *dra7xx,
    struct platform_device *pdev)
    {
    int ret;
    struct dw_pcie_ep *ep;
    struct device *dev = &pdev.dev;
    struct dw_pcie *pci = dra7xx.pci;
    ep = &pci.ep;
    ep.ops = &pcie_ep_ops;
    pci.dbi_base = devm_platform_ioremap_resource_byname(pdev, "ep_dbics");
    if (IS_ERR(pci.dbi_base))
    return PTR_ERR(pci.dbi_base);
    pci.dbi_base2 =
    devm_platform_ioremap_resource_byname(pdev, "ep_dbics2");
    if (IS_ERR(pci.dbi_base2))
    return PTR_ERR(pci.dbi_base2);
    ret = dw_pcie_ep_init(ep);
    if (ret) {
    dev_err(dev, "failed to initialize endpoint\n");
    return ret;
    }
    ret = dw_pcie_ep_init_registers(ep);
    if (ret) {
    dev_err(dev, "Failed to initialize DWC endpoint registers\n");
    dw_pcie_ep_deinit(ep);
    return ret;
    }
    pci_epc_init_notify(ep.epc);
    return 0;
    }
    static int dra7xx_add_pcie_port(struct dra7xx_pcie *dra7xx,
    struct platform_device *pdev)
    {
    int ret;
    struct dw_pcie *pci = dra7xx.pci;
    struct dw_pcie_rp *pp = &pci.pp;
    struct device *dev = pci.dev;
    pp.irq = platform_get_irq(pdev, 1);
    if (pp.irq < 0)
    return pp.irq;
// MSI IRQ is muxed
    pp.msi_irq[0] = -ENODEV;
    ret = dra7xx_pcie_init_irq_domain(pp);
    if (ret < 0)
    return ret;
    pci.dbi_base = devm_platform_ioremap_resource_byname(pdev, "rc_dbics");
    if (IS_ERR(pci.dbi_base))
    return PTR_ERR(pci.dbi_base);
    pp.ops = &dra7xx_pcie_host_ops;
    ret = dw_pcie_host_init(pp);
    if (ret) {
    dev_err(dev, "failed to initialize host\n");
    return ret;
    }
    return 0;
    }
    static const struct dw_pcie_ops dw_pcie_ops = {
    .cpu_addr_fixup = dra7xx_pcie_cpu_addr_fixup,
    .start_link = dra7xx_pcie_establish_link,
    .stop_link = dra7xx_pcie_stop_link,
    .link_up = dra7xx_pcie_link_up,
    };
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_disable_phy(dra7xx: *mut dra7xx_pcie) {
    static void dra7xx_pcie_disable_phy(struct dra7xx_pcie *dra7xx)
    {
    let mut phy_count: c_int = dra7xx.phy_count;
    while (phy_count--) {
    phy_power_off(dra7xx.phy[phy_count]);
    phy_exit(dra7xx.phy[phy_count]);
    }
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_enable_phy(dra7xx: *mut dra7xx_pcie) -> c_int {
    static int dra7xx_pcie_enable_phy(struct dra7xx_pcie *dra7xx)
    {
    let mut phy_count: c_int = dra7xx.phy_count;
    int ret;
    int i;
    for (i = 0; i < phy_count; i++) {
    ret = phy_set_mode(dra7xx.phy[i], PHY_MODE_PCIE);
    if (ret < 0)
    goto err_phy;
    ret = phy_init(dra7xx.phy[i]);
    if (ret < 0)
    goto err_phy;
    ret = phy_power_on(dra7xx.phy[i]);
    if (ret < 0) {
    phy_exit(dra7xx.phy[i]);
    goto err_phy;
    }
    }
    return 0;
    err_phy:
    while (--i >= 0) {
    phy_power_off(dra7xx.phy[i]);
    phy_exit(dra7xx.phy[i]);
    }
    return ret;
    }
    static const struct dra7xx_pcie_of_data dra7xx_pcie_rc_of_data = {
    .mode = DW_PCIE_RC_TYPE,
    };
    static const struct dra7xx_pcie_of_data dra7xx_pcie_ep_of_data = {
    .mode = DW_PCIE_EP_TYPE,
    };
    static const struct dra7xx_pcie_of_data dra746_pcie_rc_of_data = {
    .b1co_mode_sel_mask = BIT(2),
    .mode = DW_PCIE_RC_TYPE,
    };
    static const struct dra7xx_pcie_of_data dra726_pcie_rc_of_data = {
    .b1co_mode_sel_mask = GENMASK(3, 2),
    .mode = DW_PCIE_RC_TYPE,
    };
    static const struct dra7xx_pcie_of_data dra746_pcie_ep_of_data = {
    .b1co_mode_sel_mask = BIT(2),
    .mode = DW_PCIE_EP_TYPE,
    };
    static const struct dra7xx_pcie_of_data dra726_pcie_ep_of_data = {
    .b1co_mode_sel_mask = GENMASK(3, 2),
    .mode = DW_PCIE_EP_TYPE,
    };
    static const struct of_device_id of_dra7xx_pcie_match[] = {
    {
    .compatible = "ti,dra7-pcie",
    .data = &dra7xx_pcie_rc_of_data,
    },
    {
    .compatible = "ti,dra7-pcie-ep",
    .data = &dra7xx_pcie_ep_of_data,
    },
    {
    .compatible = "ti,dra746-pcie-rc",
    .data = &dra746_pcie_rc_of_data,
    },
    {
    .compatible = "ti,dra726-pcie-rc",
    .data = &dra726_pcie_rc_of_data,
    },
    {
    .compatible = "ti,dra746-pcie-ep",
    .data = &dra746_pcie_ep_of_data,
    },
    {
    .compatible = "ti,dra726-pcie-ep",
    .data = &dra726_pcie_ep_of_data,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_dra7xx_pcie_match);
//
// dra7xx_pcie_unaligned_memaccess: workaround for AM572x/AM571x Errata i870
// @dra7xx: the dra7xx device where the workaround should be applied
//
// Access to the PCIe slave port that are not 32-bit aligned will result
// in incorrect mapping to TLP Address and Byte enable fields. Therefore,
// byte and half-word accesses are not possible to byte offset 0x1, 0x2, or
// 0x3.
//
// To avoid this issue set PCIE_SS1_AXI2OCP_LEGACY_MODE_ENABLE to 1.
//
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_unaligned_memaccess(dev: *mut device) -> c_int {
    static int dra7xx_pcie_unaligned_memaccess(struct device *dev)
    {
    int ret;
    struct device_node *np = dev.of_node;
    unsigned int args[2];
    struct regmap *regmap;
    regmap = syscon_regmap_lookup_by_phandle_args(np, "ti,syscon-unaligned-access",
    2, args);
    if (IS_ERR(regmap)) {
    dev_dbg(dev, "can't get ti,syscon-unaligned-access\n");
    return -EINVAL;
    }
    ret = regmap_update_bits(regmap, args[0], args[1], args[1]);
    if (ret)
    dev_err(dev, "failed to enable unaligned access\n");
    return ret;
    }
    static int dra7xx_pcie_configure_two_lane(struct device *dev,
    u32 b1co_mode_sel_mask)
    {
    struct device_node *np = dev.of_node;
    struct regmap *pcie_syscon;
    unsigned int pcie_reg;
    u32 mask;
    u32 val;
    pcie_syscon = syscon_regmap_lookup_by_phandle_args(np, "ti,syscon-lane-sel",
    1, &pcie_reg);
    if (IS_ERR(pcie_syscon)) {
    dev_err(dev, "unable to get ti,syscon-lane-sel\n");
    return -EINVAL;
    }
    mask = b1co_mode_sel_mask | PCIE_B0_B1_TSYNCEN;
    val = PCIE_B1C0_MODE_SEL | PCIE_B0_B1_TSYNCEN;
    regmap_update_bits(pcie_syscon, pcie_reg, mask, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int dra7xx_pcie_probe(struct platform_device *pdev)
    {
    u32 reg;
    int ret;
    int irq;
    int i;
    int phy_count;
    struct phy **phy;
    struct device_link **link;
    void __iomem *base;
    struct dw_pcie *pci;
    struct dra7xx_pcie *dra7xx;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    char name[10];
    struct gpio_desc *reset;
    const struct dra7xx_pcie_of_data *data;
    enum dw_pcie_device_mode mode;
    u32 b1co_mode_sel_mask;
    data = of_device_get_match_data(dev);
    if (!data)
    return -EINVAL;
    mode = (enum dw_pcie_device_mode)data.mode;
    b1co_mode_sel_mask = data.b1co_mode_sel_mask;
    dra7xx = devm_kzalloc(dev, sizeof(*dra7xx), GFP_KERNEL);
    if (!dra7xx)
    return -ENOMEM;
    pci = devm_kzalloc(dev, sizeof(*pci), GFP_KERNEL);
    if (!pci)
    return -ENOMEM;
    pci.dev = dev;
    pci.ops = &dw_pcie_ops;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    base = devm_platform_ioremap_resource_byname(pdev, "ti_conf");
    if (IS_ERR(base))
    return PTR_ERR(base);
    phy_count = of_property_count_strings(np, "phy-names");
    if (phy_count < 0) {
    dev_err(dev, "unable to find the strings\n");
    return phy_count;
    }
    phy = devm_kcalloc(dev, phy_count, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    link = devm_kcalloc(dev, phy_count, sizeof(*link), GFP_KERNEL);
    if (!link)
    return -ENOMEM;
    dra7xx.clk = devm_clk_get_optional(dev, core::ptr::null_mut());
    if (IS_ERR(dra7xx.clk))
    return dev_err_probe(dev, PTR_ERR(dra7xx.clk),
    "clock request failed");
    ret = clk_prepare_enable(dra7xx.clk);
    if (ret)
    return ret;
    for (i = 0; i < phy_count; i++) {
    snprintf(name, sizeof(name), "pcie-phy%d", i);
    phy[i] = devm_phy_get(dev, name);
    if (IS_ERR(phy[i]))
    return PTR_ERR(phy[i]);
    link[i] = device_link_add(dev, &phy[i].dev, DL_FLAG_STATELESS);
    if (!link[i]) {
    ret = -EINVAL;
    goto err_link;
    }
    }
    dra7xx.base = base;
    dra7xx.phy = phy;
    dra7xx.pci = pci;
    dra7xx.phy_count = phy_count;
    if (phy_count == 2) {
    ret = dra7xx_pcie_configure_two_lane(dev, b1co_mode_sel_mask);
    if (ret < 0)
    dra7xx.phy_count = 1; /* Fallback to x1 lane mode */
    }
    ret = dra7xx_pcie_enable_phy(dra7xx);
    if (ret) {
    dev_err(dev, "failed to enable phy\n");
    return ret;
    }
    platform_set_drvdata(pdev, dra7xx);
    pm_runtime_enable(dev);
    ret = pm_runtime_get_sync(dev);
    if (ret < 0) {
    dev_err(dev, "pm_runtime_get_sync failed\n");
    goto err_get_sync;
    }
    reset = devm_gpiod_get_optional(dev, core::ptr::null_mut(), GPIOD_OUT_HIGH);
    if (IS_ERR(reset)) {
    ret = PTR_ERR(reset);
    dev_err(&pdev.dev, "gpio request failed, ret %d\n", ret);
    goto err_gpio;
    }
    reg = dra7xx_pcie_readl(dra7xx, PCIECTRL_DRA7XX_CONF_DEVICE_CMD);
    reg &= ~LTSSM_EN;
    dra7xx_pcie_writel(dra7xx, PCIECTRL_DRA7XX_CONF_DEVICE_CMD, reg);
    switch (mode) {
    case DW_PCIE_RC_TYPE:
    if (!IS_ENABLED(CONFIG_PCI_DRA7XX_HOST)) {
    ret = -ENODEV;
    goto err_gpio;
    }
    dra7xx_pcie_writel(dra7xx, PCIECTRL_TI_CONF_DEVICE_TYPE,
    DEVICE_TYPE_RC);
    ret = dra7xx_pcie_unaligned_memaccess(dev);
    if (ret)
    dev_err(dev, "WA for Errata i870 not applied\n");
    ret = dra7xx_add_pcie_port(dra7xx, pdev);
    if (ret < 0)
    goto err_gpio;
    break;
    case DW_PCIE_EP_TYPE:
    if (!IS_ENABLED(CONFIG_PCI_DRA7XX_EP)) {
    ret = -ENODEV;
    goto err_gpio;
    }
    dra7xx_pcie_writel(dra7xx, PCIECTRL_TI_CONF_DEVICE_TYPE,
    DEVICE_TYPE_EP);
    ret = dra7xx_pcie_unaligned_memaccess(dev);
    if (ret)
    goto err_gpio;
    ret = dra7xx_add_pcie_ep(dra7xx, pdev);
    if (ret < 0)
    goto err_gpio;
    break;
    default:
    dev_err(dev, "INVALID device type %d\n", mode);
    }
    dra7xx.pci.mode = mode;
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), dra7xx_pcie_irq_handler,
    IRQF_SHARED | IRQF_ONESHOT,
    "dra7xx-pcie-main", dra7xx);
    if (ret) {
    dev_err(dev, "failed to request irq\n");
    goto err_deinit;
    }
    return 0;
    err_deinit:
    if (dra7xx.pci.mode == DW_PCIE_RC_TYPE)
    dw_pcie_host_deinit(&dra7xx.pci.pp);
    else
    dw_pcie_ep_deinit(&dra7xx.pci.ep);
    err_gpio:
    err_get_sync:
    pm_runtime_put(dev);
    pm_runtime_disable(dev);
    dra7xx_pcie_disable_phy(dra7xx);
    err_link:
    while (--i >= 0)
    device_link_del(link[i]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_suspend(dev: *mut device) -> c_int {
    static int dra7xx_pcie_suspend(struct device *dev)
    {
    struct dra7xx_pcie *dra7xx = dev_get_drvdata(dev);
    struct dw_pcie *pci = dra7xx.pci;
    u32 val;
    if (pci.mode != DW_PCIE_RC_TYPE)
    return 0;
// clear MSE
    val = dw_pcie_readl_dbi(pci, PCI_COMMAND);
    val &= ~PCI_COMMAND_MEMORY;
    dw_pcie_writel_dbi(pci, PCI_COMMAND, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_resume(dev: *mut device) -> c_int {
    static int dra7xx_pcie_resume(struct device *dev)
    {
    struct dra7xx_pcie *dra7xx = dev_get_drvdata(dev);
    struct dw_pcie *pci = dra7xx.pci;
    u32 val;
    if (pci.mode != DW_PCIE_RC_TYPE)
    return 0;
// set MSE
    val = dw_pcie_readl_dbi(pci, PCI_COMMAND);
    val |= PCI_COMMAND_MEMORY;
    dw_pcie_writel_dbi(pci, PCI_COMMAND, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_suspend_noirq(dev: *mut device) -> c_int {
    static int dra7xx_pcie_suspend_noirq(struct device *dev)
    {
    struct dra7xx_pcie *dra7xx = dev_get_drvdata(dev);
    dra7xx_pcie_disable_phy(dra7xx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_resume_noirq(dev: *mut device) -> c_int {
    static int dra7xx_pcie_resume_noirq(struct device *dev)
    {
    struct dra7xx_pcie *dra7xx = dev_get_drvdata(dev);
    int ret;
    ret = dra7xx_pcie_enable_phy(dra7xx);
    if (ret) {
    dev_err(dev, "failed to enable phy\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dra7xx_pcie_shutdown(pdev: *mut platform_device) {
    static void dra7xx_pcie_shutdown(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct dra7xx_pcie *dra7xx = dev_get_drvdata(dev);
    int ret;
    dra7xx_pcie_stop_link(dra7xx.pci);
    ret = pm_runtime_put_sync(dev);
    if (ret < 0)
    dev_dbg(dev, "pm_runtime_put_sync failed\n");
    pm_runtime_disable(dev);
    dra7xx_pcie_disable_phy(dra7xx);
    clk_disable_unprepare(dra7xx.clk);
    }
    static const struct dev_pm_ops dra7xx_pcie_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(dra7xx_pcie_suspend, dra7xx_pcie_resume)
    NOIRQ_SYSTEM_SLEEP_PM_OPS(dra7xx_pcie_suspend_noirq,
    dra7xx_pcie_resume_noirq)
    };
    static struct platform_driver dra7xx_pcie_driver = {
    .probe = dra7xx_pcie_probe,
    .driver = {
    .name	= "dra7-pcie",
    .of_match_table = of_dra7xx_pcie_match,
    .suppress_bind_attrs = true,
    .pm	= &dra7xx_pcie_pm_ops,
    },
    .shutdown = dra7xx_pcie_shutdown,
    };
    module_platform_driver(dra7xx_pcie_driver);
    MODULE_AUTHOR("Kishon Vijay Abraham I <kishon@ti.com>");
    MODULE_DESCRIPTION("PCIe controller driver for TI DRA7xx SoCs");
    MODULE_LICENSE("GPL v2");
