//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/pcie-xilinx-cpm.c
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
// PCIe host controller driver for Xilinx Versal CPM DMA Bridge
//
// (C) Copyright 2019 - 2020, Xilinx, Inc.
//

// Register definitions
pub const XILINX_CPM_PCIE_REG_IDR: c_uint = 0x00000E10;
pub const XILINX_CPM_PCIE_REG_IMR: c_uint = 0x00000E14;
pub const XILINX_CPM_PCIE_REG_PSCR: c_uint = 0x00000E1C;
pub const XILINX_CPM_PCIE_REG_RPSC: c_uint = 0x00000E20;
pub const XILINX_CPM_PCIE_REG_RPEFR: c_uint = 0x00000E2C;
pub const XILINX_CPM_PCIE_REG_IDRN: c_uint = 0x00000E38;
pub const XILINX_CPM_PCIE_REG_IDRN_MASK: c_uint = 0x00000E3C;
pub const XILINX_CPM_PCIE_MISC_IR_STATUS: c_uint = 0x00000340;
pub const XILINX_CPM_PCIE_MISC_IR_ENABLE: c_uint = 0x00000348;

pub const XILINX_CPM_PCIE0_IR_STATUS: c_uint = 0x000002A0;
pub const XILINX_CPM_PCIE1_IR_STATUS: c_uint = 0x000002B4;
pub const XILINX_CPM_PCIE0_IR_ENABLE: c_uint = 0x000002A8;
pub const XILINX_CPM_PCIE1_IR_ENABLE: c_uint = 0x000002BC;

    (						\
    IMR(LINK_DOWN)		|		\
    IMR(HOT_RESET)		|		\
    IMR(CFG_PCIE_TIMEOUT)	|		\
    IMR(CFG_TIMEOUT)	|		\
    IMR(CORRECTABLE)	|		\
    IMR(NONFATAL)		|		\
    IMR(FATAL)		|		\
    IMR(CFG_ERR_POISON)	|		\
    IMR(PME_TO_ACK_RCVD)	|		\
    IMR(INTX)		|		\
    IMR(PM_PME_RCVD)	|		\
    IMR(SLV_UNSUPP)		|		\
    IMR(SLV_UNEXP)		|		\
    IMR(SLV_COMPL)		|		\
    IMR(SLV_ERRP)		|		\
    IMR(SLV_CMPABT)		|		\
    IMR(SLV_ILLBUR)		|		\
    IMR(MST_DECERR)		|		\
    IMR(MST_SLVERR)		|		\
    IMR(SLV_PCIE_TIMEOUT)			\
    )
pub const XILINX_CPM_PCIE_IDR_ALL_MASK: c_uint = 0xFFFFFFFF;

pub const XILINX_CPM_PCIE_IDRN_SHIFT: c_int = 16;
// Root Port Error FIFO Read Register definitions

pub const XILINX_CPM_PCIE_RPEFR_ALL_MASK: c_uint = 0xFFFFFFFF;
// Root Port Status/control Register definitions

// Phy Status/Control Register definitions

    enum xilinx_cpm_version {
    CPM,
    CPM5,
    CPM5_HOST1,
    CPM5NC_HOST,
    };
//
// struct xilinx_cpm_variant - CPM variant information
// @version: CPM version
// @ir_status: Offset for the error interrupt status register
// @ir_enable: Offset for the CPM5 local error interrupt enable register
// @ir_misc_value: A bitmask for the miscellaneous interrupt status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_cpm_variant {
    pub version: enum xilinx_cpm_version,
    pub ir_status: u32,
    pub ir_enable: u32,
    pub ir_misc_value: u32,
}

//
// struct xilinx_cpm_pcie - PCIe port information
// @dev: Device pointer
// @reg_base: Bridge Register Base
// @cpm_base: CPM System Level Control and Status Register(SLCR) Base
// @intx_domain: Legacy IRQ domain pointer
// @cpm_domain: CPM IRQ domain pointer
// @cfg: Holds mappings of config space window
// @intx_irq: legacy interrupt number
// @irq: Error interrupt number
// @lock: lock protecting shared register access
// @variant: CPM version check pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_cpm_pcie {
    pub dev: *mut device,
    pub reg_base: *mut void __iomem,
    pub cpm_base: *mut void __iomem,
    pub intx_domain: *mut irq_domain,
    pub cpm_domain: *mut irq_domain,
    pub cfg: *mut pci_config_window,
    pub intx_irq: c_int,
    pub irq: c_int,
    pub lock: raw_spinlock_t,
    pub variant: *const xilinx_cpm_variant,
}

#[no_mangle]
unsafe extern "C" fn pcie_read(port: *mut xilinx_cpm_pcie, reg: u32) -> u32 {
    static u32 pcie_read(struct xilinx_cpm_pcie *port, u32 reg)
    {
    return readl_relaxed(port.reg_base + reg);
    }
    static void pcie_write(struct xilinx_cpm_pcie *port,
    u32 val, u32 reg)
    {
    writel_relaxed(val, port.reg_base + reg);
    }
#[no_mangle]
unsafe extern "C" fn cpm_pcie_link_up(port: *mut xilinx_cpm_pcie) -> bool {
    static bool cpm_pcie_link_up(struct xilinx_cpm_pcie *port)
    {
    return (pcie_read(port, XILINX_CPM_PCIE_REG_PSCR) &
    XILINX_CPM_PCIE_REG_PSCR_LNKUP);
    }
#[no_mangle]
unsafe extern "C" fn cpm_pcie_clear_err_interrupts(port: *mut xilinx_cpm_pcie) {
    static void cpm_pcie_clear_err_interrupts(struct xilinx_cpm_pcie *port)
    {
    let mut val: c_ulong = pcie_read(port, XILINX_CPM_PCIE_REG_RPEFR);
    if (val & XILINX_CPM_PCIE_RPEFR_ERR_VALID) {
    dev_dbg(port.dev, "Requester ID %lu\n",
    val & XILINX_CPM_PCIE_RPEFR_REQ_ID);
    pcie_write(port, XILINX_CPM_PCIE_RPEFR_ALL_MASK,
    XILINX_CPM_PCIE_REG_RPEFR);
    }
    }
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_mask_leg_irq(data: *mut irq_data) {
    static void xilinx_cpm_mask_leg_irq(struct irq_data *data)
    {
    struct xilinx_cpm_pcie *port = irq_data_get_irq_chip_data(data);
    unsigned long flags;
    u32 mask;
    u32 val;
    mask = BIT(data.hwirq + XILINX_CPM_PCIE_IDRN_SHIFT);
    raw_spin_lock_irqsave(&port.lock, flags);
    val = pcie_read(port, XILINX_CPM_PCIE_REG_IDRN_MASK);
    pcie_write(port, (val & (~mask)), XILINX_CPM_PCIE_REG_IDRN_MASK);
    raw_spin_unlock_irqrestore(&port.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_unmask_leg_irq(data: *mut irq_data) {
    static void xilinx_cpm_unmask_leg_irq(struct irq_data *data)
    {
    struct xilinx_cpm_pcie *port = irq_data_get_irq_chip_data(data);
    unsigned long flags;
    u32 mask;
    u32 val;
    mask = BIT(data.hwirq + XILINX_CPM_PCIE_IDRN_SHIFT);
    raw_spin_lock_irqsave(&port.lock, flags);
    val = pcie_read(port, XILINX_CPM_PCIE_REG_IDRN_MASK);
    pcie_write(port, (val | mask), XILINX_CPM_PCIE_REG_IDRN_MASK);
    raw_spin_unlock_irqrestore(&port.lock, flags);
    }
    static struct irq_chip xilinx_cpm_leg_irq_chip = {
    .name		= "INTx",
    .irq_mask	= xilinx_cpm_mask_leg_irq,
    .irq_unmask	= xilinx_cpm_unmask_leg_irq,
    };
//
// xilinx_cpm_pcie_intx_map - Set the handler for the INTx and mark IRQ as valid
// @domain: IRQ domain
// @irq: Virtual IRQ number
// @hwirq: HW interrupt number
//
// Return: Always returns 0.
//
    static int xilinx_cpm_pcie_intx_map(struct irq_domain *domain,
    unsigned int irq, irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(irq, &xilinx_cpm_leg_irq_chip,
    handle_level_irq);
    irq_set_chip_data(irq, domain.host_data);
    irq_set_status_flags(irq, IRQ_LEVEL);
    return 0;
    }
// INTx IRQ Domain operations
    static const struct irq_domain_ops intx_domain_ops = {
    .map = xilinx_cpm_pcie_intx_map,
    };
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_pcie_intx_flow(desc: *mut irq_desc) {
    static void xilinx_cpm_pcie_intx_flow(struct irq_desc *desc)
    {
    struct xilinx_cpm_pcie *port = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    unsigned long val;
    int i;
    chained_irq_enter(chip, desc);
    val = FIELD_GET(XILINX_CPM_PCIE_IDRN_MASK,
    pcie_read(port, XILINX_CPM_PCIE_REG_IDRN));
    for_each_set_bit(i, &val, PCI_NUM_INTX)
    generic_handle_domain_irq(port.intx_domain, i);
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_mask_event_irq(d: *mut irq_data) {
    static void xilinx_cpm_mask_event_irq(struct irq_data *d)
    {
    struct xilinx_cpm_pcie *port = irq_data_get_irq_chip_data(d);
    u32 val;
    raw_spin_lock(&port.lock);
    val = pcie_read(port, XILINX_CPM_PCIE_REG_IMR);
    val &= ~BIT(d.hwirq);
    pcie_write(port, val, XILINX_CPM_PCIE_REG_IMR);
    raw_spin_unlock(&port.lock);
    }
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_unmask_event_irq(d: *mut irq_data) {
    static void xilinx_cpm_unmask_event_irq(struct irq_data *d)
    {
    struct xilinx_cpm_pcie *port = irq_data_get_irq_chip_data(d);
    u32 val;
    raw_spin_lock(&port.lock);
    val = pcie_read(port, XILINX_CPM_PCIE_REG_IMR);
    val |= BIT(d.hwirq);
    pcie_write(port, val, XILINX_CPM_PCIE_REG_IMR);
    raw_spin_unlock(&port.lock);
    }
    static struct irq_chip xilinx_cpm_event_irq_chip = {
    .name		= "RC-Event",
    .irq_mask	= xilinx_cpm_mask_event_irq,
    .irq_unmask	= xilinx_cpm_unmask_event_irq,
    };
    static int xilinx_cpm_pcie_event_map(struct irq_domain *domain,
    unsigned int irq, irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(irq, &xilinx_cpm_event_irq_chip,
    handle_level_irq);
    irq_set_chip_data(irq, domain.host_data);
    irq_set_status_flags(irq, IRQ_LEVEL);
    return 0;
    }
    static const struct irq_domain_ops event_domain_ops = {
    .map = xilinx_cpm_pcie_event_map,
    };
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_pcie_event_flow(desc: *mut irq_desc) {
    static void xilinx_cpm_pcie_event_flow(struct irq_desc *desc)
    {
    struct xilinx_cpm_pcie *port = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    const struct xilinx_cpm_variant *variant = port.variant;
    unsigned long val;
    int i;
    chained_irq_enter(chip, desc);
    val =  pcie_read(port, XILINX_CPM_PCIE_REG_IDR);
    val &= pcie_read(port, XILINX_CPM_PCIE_REG_IMR);
    for_each_set_bit(i, &val, 32)
    generic_handle_domain_irq(port.cpm_domain, i);
    pcie_write(port, val, XILINX_CPM_PCIE_REG_IDR);
    if (variant.ir_status) {
    val = readl_relaxed(port.cpm_base + variant.ir_status);
    if (val)
    writel_relaxed(val, port.cpm_base +
    variant.ir_status);
    }
//
// XILINX_CPM_PCIE_MISC_IR_STATUS register is mapped to
// CPM SLCR block.
//
    val = readl_relaxed(port.cpm_base + XILINX_CPM_PCIE_MISC_IR_STATUS);
    if (val)
    writel_relaxed(val,
    port.cpm_base + XILINX_CPM_PCIE_MISC_IR_STATUS);
    chained_irq_exit(chip, desc);
    }

    [XILINX_PCIE_INTR_ ## x] = { __stringify(x), s }
    static const struct {
    const char      *sym;
    const char      *str;
    } intr_cause[32] = {
    _IC(LINK_DOWN,		"Link Down"),
    _IC(HOT_RESET,		"Hot reset"),
    _IC(CFG_TIMEOUT,	"ECAM access timeout"),
    _IC(CORRECTABLE,	"Correctable error message"),
    _IC(NONFATAL,		"Non fatal error message"),
    _IC(FATAL,		"Fatal error message"),
    _IC(SLV_UNSUPP,		"Slave unsupported request"),
    _IC(SLV_UNEXP,		"Slave unexpected completion"),
    _IC(SLV_COMPL,		"Slave completion timeout"),
    _IC(SLV_ERRP,		"Slave Error Poison"),
    _IC(SLV_CMPABT,		"Slave Completer Abort"),
    _IC(SLV_ILLBUR,		"Slave Illegal Burst"),
    _IC(MST_DECERR,		"Master decode error"),
    _IC(MST_SLVERR,		"Master slave error"),
    _IC(CFG_PCIE_TIMEOUT,	"PCIe ECAM access timeout"),
    _IC(CFG_ERR_POISON,	"ECAM poisoned completion received"),
    _IC(PME_TO_ACK_RCVD,	"PME_TO_ACK message received"),
    _IC(PM_PME_RCVD,	"PM_PME message received"),
    _IC(SLV_PCIE_TIMEOUT,	"PCIe completion timeout received"),
    };
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_pcie_intr_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xilinx_cpm_pcie_intr_handler(int irq, void *dev_id)
    {
    struct xilinx_cpm_pcie *port = dev_id;
    struct device *dev = port.dev;
    struct irq_data *d;
    d = irq_domain_get_irq_data(port.cpm_domain, irq);
    switch (d.hwirq) {
    case XILINX_PCIE_INTR_CORRECTABLE:
    case XILINX_PCIE_INTR_NONFATAL:
    case XILINX_PCIE_INTR_FATAL:
    cpm_pcie_clear_err_interrupts(port);
    fallthrough;
    default:
    if (intr_cause[d.hwirq].str)
    dev_warn(dev, "%s\n", intr_cause[d.hwirq].str);
    else
    dev_warn(dev, "Unknown IRQ %ld\n", d.hwirq);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_free_irq_domains(port: *mut xilinx_cpm_pcie) {
    static void xilinx_cpm_free_irq_domains(struct xilinx_cpm_pcie *port)
    {
    if (port.intx_domain) {
    irq_domain_remove(port.intx_domain);
    port.intx_domain = core::ptr::null_mut();
    }
    if (port.cpm_domain) {
    irq_domain_remove(port.cpm_domain);
    port.cpm_domain = core::ptr::null_mut();
    }
    }
//
// xilinx_cpm_pcie_init_irq_domain - Initialize IRQ domain
// @port: PCIe port information
//
// Return: '0' on success and error value on failure
//
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_pcie_init_irq_domain(port: *mut xilinx_cpm_pcie) -> c_int {
    static int xilinx_cpm_pcie_init_irq_domain(struct xilinx_cpm_pcie *port)
    {
    struct device *dev = port.dev;
    struct device_node *node = dev.of_node;
    struct device_node *pcie_intc_node;
// Setup INTx
    pcie_intc_node = of_get_next_child(node, core::ptr::null_mut());
    if (!pcie_intc_node) {
    dev_err(dev, "No PCIe Intc node found\n");
    return -EINVAL;
    }
    port.cpm_domain = irq_domain_create_linear(of_fwnode_handle(pcie_intc_node), 32,
    &event_domain_ops, port);
    if (!port.cpm_domain)
    goto out;
    irq_domain_update_bus_token(port.cpm_domain, DOMAIN_BUS_NEXUS);
    port.intx_domain = irq_domain_create_linear(of_fwnode_handle(pcie_intc_node), PCI_NUM_INTX,
    &intx_domain_ops, port);
    if (!port.intx_domain)
    goto out;
    irq_domain_update_bus_token(port.intx_domain, DOMAIN_BUS_WIRED);
    of_node_put(pcie_intc_node);
    raw_spin_lock_init(&port.lock);
    return 0;
    out:
    xilinx_cpm_free_irq_domains(port);
    of_node_put(pcie_intc_node);
    dev_err(dev, "Failed to allocate IRQ domains\n");
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_setup_irq(port: *mut xilinx_cpm_pcie) -> c_int {
    static int xilinx_cpm_setup_irq(struct xilinx_cpm_pcie *port)
    {
    struct device *dev = port.dev;
    struct platform_device *pdev = to_platform_device(dev);
    int i, irq;
    port.irq = platform_get_irq(pdev, 0);
    if (port.irq < 0)
    return port.irq;
    for (i = 0; i < ARRAY_SIZE(intr_cause); i++) {
    int err;
    if (!intr_cause[i].str)
    continue;
    irq = irq_create_mapping(port.cpm_domain, i);
    if (!irq) {
    dev_err(dev, "Failed to map interrupt\n");
    return -ENXIO;
    }
    err = devm_request_irq(dev, irq, xilinx_cpm_pcie_intr_handler,
    0, intr_cause[i].sym, port);
    if (err) {
    dev_err(dev, "Failed to request IRQ %d\n", irq);
    return err;
    }
    }
    port.intx_irq = irq_create_mapping(port.cpm_domain,
    XILINX_PCIE_INTR_INTX);
    if (!port.intx_irq) {
    dev_err(dev, "Failed to map INTx interrupt\n");
    return -ENXIO;
    }
// Plug the INTx chained handler
    irq_set_chained_handler_and_data(port.intx_irq,
    xilinx_cpm_pcie_intx_flow, port);
// Plug the main event chained handler
    irq_set_chained_handler_and_data(port.irq,
    xilinx_cpm_pcie_event_flow, port);
    return 0;
    }
//
// xilinx_cpm_pcie_init_port - Initialize hardware
// @port: PCIe port information
//
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_pcie_init_port(port: *mut xilinx_cpm_pcie) {
    static void xilinx_cpm_pcie_init_port(struct xilinx_cpm_pcie *port)
    {
    const struct xilinx_cpm_variant *variant = port.variant;
    if (variant.version == CPM5NC_HOST)
    return;
    if (cpm_pcie_link_up(port))
    dev_info(port.dev, "PCIe Link is UP\n");
    else
    dev_info(port.dev, "PCIe Link is DOWN\n");
// Disable all interrupts
    pcie_write(port, ~XILINX_CPM_PCIE_IDR_ALL_MASK,
    XILINX_CPM_PCIE_REG_IMR);
// Clear pending interrupts
    pcie_write(port, pcie_read(port, XILINX_CPM_PCIE_REG_IDR) &
    XILINX_CPM_PCIE_IMR_ALL_MASK,
    XILINX_CPM_PCIE_REG_IDR);
//
// XILINX_CPM_PCIE_MISC_IR_ENABLE register is mapped to
// CPM SLCR block.
//
    writel(variant.ir_misc_value,
    port.cpm_base + XILINX_CPM_PCIE_MISC_IR_ENABLE);
    if (variant.ir_enable) {
    writel(XILINX_CPM_PCIE_IR_LOCAL,
    port.cpm_base + variant.ir_enable);
    }
// Set Bridge enable bit
    pcie_write(port, pcie_read(port, XILINX_CPM_PCIE_REG_RPSC) |
    XILINX_CPM_PCIE_REG_RPSC_BEN,
    XILINX_CPM_PCIE_REG_RPSC);
    }
//
// xilinx_cpm_pcie_parse_dt - Parse Device tree
// @port: PCIe port information
// @bus_range: Bus resource
//
// Return: '0' on success and error value on failure
//
    static int xilinx_cpm_pcie_parse_dt(struct xilinx_cpm_pcie *port,
    struct resource *bus_range)
    {
    struct device *dev = port.dev;
    struct platform_device *pdev = to_platform_device(dev);
    struct resource *res;
    port.cpm_base = devm_platform_ioremap_resource_byname(pdev,
    "cpm_slcr");
    if (IS_ERR(port.cpm_base))
    return PTR_ERR(port.cpm_base);
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "cfg");
    if (!res)
    return -ENXIO;
    port.cfg = pci_ecam_create(dev, res, bus_range,
    &pci_generic_ecam_ops);
    if (IS_ERR(port.cfg))
    return PTR_ERR(port.cfg);
    if (port.variant.version == CPM5 ||
    port.variant.version == CPM5_HOST1) {
    port.reg_base = devm_platform_ioremap_resource_byname(pdev,
    "cpm_csr");
    if (IS_ERR(port.reg_base))
    return PTR_ERR(port.reg_base);
    } else {
    port.reg_base = port.cfg.win;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_free_interrupts(port: *mut xilinx_cpm_pcie) {
    static void xilinx_cpm_free_interrupts(struct xilinx_cpm_pcie *port)
    {
    irq_set_chained_handler_and_data(port.intx_irq, core::ptr::null_mut(), core::ptr::null_mut());
    irq_set_chained_handler_and_data(port.irq, core::ptr::null_mut(), core::ptr::null_mut());
    }
//
// xilinx_cpm_pcie_probe - Probe function
// @pdev: Platform device pointer
//
// Return: '0' on success and error value on failure
//
#[no_mangle]
unsafe extern "C" fn xilinx_cpm_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int xilinx_cpm_pcie_probe(struct platform_device *pdev)
    {
    struct xilinx_cpm_pcie *port;
    struct device *dev = &pdev.dev;
    struct pci_host_bridge *bridge;
    struct resource_entry *bus;
    int err;
    bridge = devm_pci_alloc_host_bridge(dev, sizeof(*port));
    if (!bridge)
    return -ENODEV;
    port = pci_host_bridge_priv(bridge);
    port.dev = dev;
    port.variant = of_device_get_match_data(dev);
    if (port.variant.version != CPM5NC_HOST) {
    err = xilinx_cpm_pcie_init_irq_domain(port);
    if (err)
    return err;
    }
    bus = resource_list_first_type(&bridge.windows, IORESOURCE_BUS);
    if (!bus) {
    err = -ENODEV;
    goto err_free_irq_domains;
    }
    err = xilinx_cpm_pcie_parse_dt(port, bus.res);
    if (err) {
    dev_err(dev, "Parsing DT failed\n");
    goto err_free_irq_domains;
    }
    xilinx_cpm_pcie_init_port(port);
    if (port.variant.version != CPM5NC_HOST) {
    err = xilinx_cpm_setup_irq(port);
    if (err) {
    dev_err(dev, "Failed to set up interrupts\n");
    goto err_setup_irq;
    }
    }
    bridge.sysdata = port.cfg;
    bridge.ops = (struct pci_ops *)&pci_generic_ecam_ops.pci_ops;
    err = pci_host_probe(bridge);
    if (err < 0)
    goto err_host_bridge;
    return 0;
    err_host_bridge:
    if (port.variant.version != CPM5NC_HOST)
    xilinx_cpm_free_interrupts(port);
    err_setup_irq:
    pci_ecam_free(port.cfg);
    err_free_irq_domains:
    if (port.variant.version != CPM5NC_HOST)
    xilinx_cpm_free_irq_domains(port);
    return err;
    }
    static const struct xilinx_cpm_variant cpm_host = {
    .version = CPM,
    .ir_misc_value = XILINX_CPM_PCIE0_MISC_IR_LOCAL,
    };
    static const struct xilinx_cpm_variant cpm5_host = {
    .version = CPM5,
    .ir_misc_value = XILINX_CPM_PCIE0_MISC_IR_LOCAL,
    .ir_status = XILINX_CPM_PCIE0_IR_STATUS,
    .ir_enable = XILINX_CPM_PCIE0_IR_ENABLE,
    };
    static const struct xilinx_cpm_variant cpm5_host1 = {
    .version = CPM5_HOST1,
    .ir_misc_value = XILINX_CPM_PCIE1_MISC_IR_LOCAL,
    .ir_status = XILINX_CPM_PCIE1_IR_STATUS,
    .ir_enable = XILINX_CPM_PCIE1_IR_ENABLE,
    };
    static const struct xilinx_cpm_variant cpm5n_host = {
    .version = CPM5NC_HOST,
    };
    static const struct of_device_id xilinx_cpm_pcie_of_match[] = {
    {
    .compatible = "xlnx,versal-cpm-host-1.00",
    .data = &cpm_host,
    },
    {
    .compatible = "xlnx,versal-cpm5-host",
    .data = &cpm5_host,
    },
    {
    .compatible = "xlnx,versal-cpm5-host1",
    .data = &cpm5_host1,
    },
    {
    .compatible = "xlnx,versal-cpm5nc-host",
    .data = &cpm5n_host,
    },
    {}
    };
    static struct platform_driver xilinx_cpm_pcie_driver = {
    .driver = {
    .name = "xilinx-cpm-pcie",
    .of_match_table = xilinx_cpm_pcie_of_match,
    .suppress_bind_attrs = true,
    },
    .probe = xilinx_cpm_pcie_probe,
    };
    builtin_platform_driver(xilinx_cpm_pcie_driver);
