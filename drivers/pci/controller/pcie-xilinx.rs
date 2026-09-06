//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/pcie-xilinx.c
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
// PCIe host controller driver for Xilinx AXI PCIe Bridge
//
// Copyright (c) 2012 - 2014 Xilinx, Inc.
//
// Based on the Tegra PCIe driver
//
// Bits taken from Synopsys DesignWare Host controller driver and
// ARM PCI Host generic driver.
//

// Register definitions
pub const XILINX_PCIE_REG_BIR: c_uint = 0x00000130;
pub const XILINX_PCIE_REG_IDR: c_uint = 0x00000138;
pub const XILINX_PCIE_REG_IMR: c_uint = 0x0000013c;
pub const XILINX_PCIE_REG_PSCR: c_uint = 0x00000144;
pub const XILINX_PCIE_REG_RPSC: c_uint = 0x00000148;
pub const XILINX_PCIE_REG_MSIBASE1: c_uint = 0x0000014c;
pub const XILINX_PCIE_REG_MSIBASE2: c_uint = 0x00000150;
pub const XILINX_PCIE_REG_RPEFR: c_uint = 0x00000154;
pub const XILINX_PCIE_REG_RPIFR1: c_uint = 0x00000158;
pub const XILINX_PCIE_REG_RPIFR2: c_uint = 0x0000015c;
// Interrupt registers definitions

pub const XILINX_PCIE_IMR_ALL_MASK: c_uint = 0x1FF30FED;
pub const XILINX_PCIE_IMR_ENABLE_MASK: c_uint = 0x1FF30F0D;
pub const XILINX_PCIE_IDR_ALL_MASK: c_uint = 0xFFFFFFFF;
// Root Port Error FIFO Read Register definitions

pub const XILINX_PCIE_RPEFR_ALL_MASK: c_uint = 0xFFFFFFFF;
// Root Port Interrupt FIFO Read Register 1 definitions

pub const XILINX_PCIE_RPIFR1_ALL_MASK: c_uint = 0xFFFFFFFF;
pub const XILINX_PCIE_RPIFR1_INTR_SHIFT: c_int = 27;
// Bridge Info Register definitions

pub const XILINX_PCIE_BIR_ECAM_SZ_SHIFT: c_int = 16;
// Root Port Interrupt FIFO Read Register 2 definitions

// Root Port Status/control Register definitions

// Phy Status/Control Register definitions

// Number of MSI IRQs
pub const XILINX_NUM_MSI_IRQS: c_int = 128;
//
// struct xilinx_pcie - PCIe port information
// @dev: Device pointer
// @reg_base: IO Mapped Register Base
// @msi_map: Bitmap of allocated MSIs
// @map_lock: Mutex protecting the MSI allocation
// @msi_domain: MSI IRQ domain pointer
// @leg_domain: Legacy IRQ domain pointer
// @resources: Bus Resources
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xilinx_pcie {
    pub dev: *mut device,
    pub reg_base: *mut void __iomem,
    pub msi_map: [c_ulong; BITS_TO_LONGS(XILINX_NUM_MSI_IRQS)],
    pub map_lock: mutex,
    pub msi_domain: *mut irq_domain,
    pub leg_domain: *mut irq_domain,
    pub resources: list_head,
}

#[no_mangle]
pub unsafe extern "C" fn pcie_read(pcie: *mut xilinx_pcie, reg: u32) -> u32 {
    static inline u32 pcie_read(struct xilinx_pcie *pcie, u32 reg)
    {
    return readl(pcie.reg_base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn pcie_write(pcie: *mut xilinx_pcie, val: u32, reg: u32) {
    static inline void pcie_write(struct xilinx_pcie *pcie, u32 val, u32 reg)
    {
    writel(val, pcie.reg_base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn xilinx_pcie_link_up(pcie: *mut xilinx_pcie) -> bool {
    static inline bool xilinx_pcie_link_up(struct xilinx_pcie *pcie)
    {
    return (pcie_read(pcie, XILINX_PCIE_REG_PSCR) &
    XILINX_PCIE_REG_PSCR_LNKUP) ? 1 : 0;
    }
//
// xilinx_pcie_clear_err_interrupts - Clear Error Interrupts
// @pcie: PCIe port information
//
#[no_mangle]
unsafe extern "C" fn xilinx_pcie_clear_err_interrupts(pcie: *mut xilinx_pcie) {
    static void xilinx_pcie_clear_err_interrupts(struct xilinx_pcie *pcie)
    {
    struct device *dev = pcie.dev;
    let mut val: c_ulong = pcie_read(pcie, XILINX_PCIE_REG_RPEFR);
    if (val & XILINX_PCIE_RPEFR_ERR_VALID) {
    dev_dbg(dev, "Requester ID %lu\n",
    val & XILINX_PCIE_RPEFR_REQ_ID);
    pcie_write(pcie, XILINX_PCIE_RPEFR_ALL_MASK,
    XILINX_PCIE_REG_RPEFR);
    }
    }
//
// xilinx_pcie_valid_device - Check if a valid device is present on bus
// @bus: PCI Bus structure
// @devfn: device/function
//
// Return: 'true' on success and 'false' if invalid device is found
//
#[no_mangle]
unsafe extern "C" fn xilinx_pcie_valid_device(bus: *mut pci_bus, devfn: c_uint) -> bool {
    static bool xilinx_pcie_valid_device(struct pci_bus *bus, unsigned int devfn)
    {
    struct xilinx_pcie *pcie = bus.sysdata;
// Check if link is up when trying to access downstream pcie ports
    if (!pci_is_root_bus(bus)) {
    if (!xilinx_pcie_link_up(pcie))
    return false;
    } else if (devfn > 0) {
// Only one device down on each root port
    return false;
    }
    return true;
    }
//
// xilinx_pcie_map_bus - Get configuration base
// @bus: PCI Bus structure
// @devfn: Device/function
// @where: Offset from base
//
// Return: Base address of the configuration space needed to be
// accessed.
//
    static void __iomem *xilinx_pcie_map_bus(struct pci_bus *bus,
    unsigned int devfn, int where)
    {
    struct xilinx_pcie *pcie = bus.sysdata;
    if (!xilinx_pcie_valid_device(bus, devfn))
    return core::ptr::null_mut();
    return pcie.reg_base + PCIE_ECAM_OFFSET(bus.number, devfn, where);
    }
// PCIe operations
    static struct pci_ops xilinx_pcie_ops = {
    .map_bus = xilinx_pcie_map_bus,
    .read	= pci_generic_config_read,
    .write	= pci_generic_config_write,
    };
// MSI functions
#[no_mangle]
unsafe extern "C" fn xilinx_msi_top_irq_ack(d: *mut irq_data) {
    static void xilinx_msi_top_irq_ack(struct irq_data *d)
    {
//
// xilinx_pcie_intr_handler() will have performed the Ack.
// Eventually, this should be fixed and the Ack be moved in
// the respective callbacks for INTx and MSI.
//
    }
#[no_mangle]
unsafe extern "C" fn xilinx_compose_msi_msg(data: *mut irq_data, msg: *mut msi_msg) {
    static void xilinx_compose_msi_msg(struct irq_data *data, struct msi_msg *msg)
    {
    struct xilinx_pcie *pcie = irq_data_get_irq_chip_data(data);
    let mut pa: phys_addr_t = ALIGN_DOWN(virt_to_phys(pcie), SZ_4K);
    msg.address_lo = lower_32_bits(pa);
    msg.address_hi = upper_32_bits(pa);
    msg.data = data.hwirq;
    }
    static struct irq_chip xilinx_msi_bottom_chip = {
    .name			= "Xilinx MSI",
    .irq_compose_msi_msg	= xilinx_compose_msi_msg,
    };
    static int xilinx_msi_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *args)
    {
    struct xilinx_pcie *pcie = domain.host_data;
    int hwirq, i;
    mutex_lock(&pcie.map_lock);
    hwirq = bitmap_find_free_region(pcie.msi_map, XILINX_NUM_MSI_IRQS, order_base_2(nr_irqs));
    mutex_unlock(&pcie.map_lock);
    if (hwirq < 0)
    return -ENOSPC;
    for (i = 0; i < nr_irqs; i++)
    irq_domain_set_info(domain, virq + i, hwirq + i,
    &xilinx_msi_bottom_chip, domain.host_data,
    handle_edge_irq, core::ptr::null_mut(), core::ptr::null_mut());
    return 0;
    }
    static void xilinx_msi_domain_free(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs)
    {
    struct irq_data *d = irq_domain_get_irq_data(domain, virq);
    struct xilinx_pcie *pcie = domain.host_data;
    mutex_lock(&pcie.map_lock);
    bitmap_release_region(pcie.msi_map, d.hwirq, order_base_2(nr_irqs));
    mutex_unlock(&pcie.map_lock);
    }
    static const struct irq_domain_ops xilinx_msi_domain_ops = {
    .alloc	= xilinx_msi_domain_alloc,
    .free	= xilinx_msi_domain_free,
    };
    static bool xilinx_init_dev_msi_info(struct device *dev, struct irq_domain *domain,
    struct irq_domain *real_parent, struct msi_domain_info *info)
    {
    struct irq_chip *chip = info.chip;
    if (!msi_lib_init_dev_msi_info(dev, domain, real_parent, info))
    return false;
    chip.irq_ack = xilinx_msi_top_irq_ack;
    return true;
    }

    MSI_FLAG_USE_DEF_CHIP_OPS	| \
    MSI_FLAG_NO_AFFINITY)
    static const struct msi_parent_ops xilinx_msi_parent_ops = {
    .required_flags		= XILINX_MSI_FLAGS_REQUIRED,
    .supported_flags	= MSI_GENERIC_FLAGS_MASK,
    .bus_select_token	= DOMAIN_BUS_PCI_MSI,
    .prefix			= "xilinx-",
    .init_dev_msi_info	= xilinx_init_dev_msi_info,
    };
#[no_mangle]
unsafe extern "C" fn xilinx_allocate_msi_domains(pcie: *mut xilinx_pcie) -> c_int {
    static int xilinx_allocate_msi_domains(struct xilinx_pcie *pcie)
    {
    struct irq_domain_info info = {
    .fwnode		= dev_fwnode(pcie.dev),
    .ops		= &xilinx_msi_domain_ops,
    .host_data	= pcie,
    .size		= XILINX_NUM_MSI_IRQS,
    };
    pcie.msi_domain = msi_create_parent_irq_domain(&info, &xilinx_msi_parent_ops);
    if (!pcie.msi_domain) {
    dev_err(pcie.dev, "failed to create MSI domain\n");
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_free_irq_domains(pcie: *mut xilinx_pcie) {
    static void xilinx_free_irq_domains(struct xilinx_pcie *pcie)
    {
    irq_domain_remove(pcie.msi_domain);
    irq_domain_remove(pcie.leg_domain);
    }
// INTx Functions
//
// xilinx_pcie_intx_map - Set the handler for the INTx and mark IRQ as valid
// @domain: IRQ domain
// @irq: Virtual IRQ number
// @hwirq: HW interrupt number
//
// Return: Always returns 0.
//
    static int xilinx_pcie_intx_map(struct irq_domain *domain, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(irq, &dummy_irq_chip, handle_simple_irq);
    irq_set_chip_data(irq, domain.host_data);
    return 0;
    }
// INTx IRQ Domain operations
    static const struct irq_domain_ops intx_domain_ops = {
    .map = xilinx_pcie_intx_map,
    .xlate = pci_irqd_intx_xlate,
    };
// PCIe HW Functions
//
// xilinx_pcie_intr_handler - Interrupt Service Handler
// @irq: IRQ number
// @data: PCIe port information
//
// Return: IRQ_HANDLED on success and IRQ_NONE on failure
//
#[no_mangle]
unsafe extern "C" fn xilinx_pcie_intr_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t xilinx_pcie_intr_handler(int irq, void *data)
    {
    struct xilinx_pcie *pcie = (struct xilinx_pcie *)data;
    struct device *dev = pcie.dev;
    u32 val, mask, status;
// Read interrupt decode and mask registers
    val = pcie_read(pcie, XILINX_PCIE_REG_IDR);
    mask = pcie_read(pcie, XILINX_PCIE_REG_IMR);
    status = val & mask;
    if (!status)
    return IRQ_NONE;
    if (status & XILINX_PCIE_INTR_LINK_DOWN)
    dev_warn(dev, "Link Down\n");
    if (status & XILINX_PCIE_INTR_ECRC_ERR)
    dev_warn(dev, "ECRC failed\n");
    if (status & XILINX_PCIE_INTR_STR_ERR)
    dev_warn(dev, "Streaming error\n");
    if (status & XILINX_PCIE_INTR_HOT_RESET)
    dev_info(dev, "Hot reset\n");
    if (status & XILINX_PCIE_INTR_CFG_TIMEOUT)
    dev_warn(dev, "ECAM access timeout\n");
    if (status & XILINX_PCIE_INTR_CORRECTABLE) {
    dev_warn(dev, "Correctable error message\n");
    xilinx_pcie_clear_err_interrupts(pcie);
    }
    if (status & XILINX_PCIE_INTR_NONFATAL) {
    dev_warn(dev, "Non fatal error message\n");
    xilinx_pcie_clear_err_interrupts(pcie);
    }
    if (status & XILINX_PCIE_INTR_FATAL) {
    dev_warn(dev, "Fatal error message\n");
    xilinx_pcie_clear_err_interrupts(pcie);
    }
    if (status & (XILINX_PCIE_INTR_INTX | XILINX_PCIE_INTR_MSI)) {
    struct irq_domain *domain;
    val = pcie_read(pcie, XILINX_PCIE_REG_RPIFR1);
// Check whether interrupt valid
    if (!(val & XILINX_PCIE_RPIFR1_INTR_VALID)) {
    dev_warn(dev, "RP Intr FIFO1 read error\n");
    goto error;
    }
// Decode the IRQ number
    if (val & XILINX_PCIE_RPIFR1_MSI_INTR) {
    val = pcie_read(pcie, XILINX_PCIE_REG_RPIFR2) &
    XILINX_PCIE_RPIFR2_MSG_DATA;
    domain = pcie.msi_domain;
    } else {
    val = (val & XILINX_PCIE_RPIFR1_INTR_MASK) >>
    XILINX_PCIE_RPIFR1_INTR_SHIFT;
    domain = pcie.leg_domain;
    }
// Clear interrupt FIFO register 1
    pcie_write(pcie, XILINX_PCIE_RPIFR1_ALL_MASK,
    XILINX_PCIE_REG_RPIFR1);
    generic_handle_domain_irq(domain, val);
    }
    if (status & XILINX_PCIE_INTR_SLV_UNSUPP)
    dev_warn(dev, "Slave unsupported request\n");
    if (status & XILINX_PCIE_INTR_SLV_UNEXP)
    dev_warn(dev, "Slave unexpected completion\n");
    if (status & XILINX_PCIE_INTR_SLV_COMPL)
    dev_warn(dev, "Slave completion timeout\n");
    if (status & XILINX_PCIE_INTR_SLV_ERRP)
    dev_warn(dev, "Slave Error Poison\n");
    if (status & XILINX_PCIE_INTR_SLV_CMPABT)
    dev_warn(dev, "Slave Completer Abort\n");
    if (status & XILINX_PCIE_INTR_SLV_ILLBUR)
    dev_warn(dev, "Slave Illegal Burst\n");
    if (status & XILINX_PCIE_INTR_MST_DECERR)
    dev_warn(dev, "Master decode error\n");
    if (status & XILINX_PCIE_INTR_MST_SLVERR)
    dev_warn(dev, "Master slave error\n");
    if (status & XILINX_PCIE_INTR_MST_ERRP)
    dev_warn(dev, "Master error poison\n");
    error:
// Clear the Interrupt Decode register
    pcie_write(pcie, status, XILINX_PCIE_REG_IDR);
    return IRQ_HANDLED;
    }
//
// xilinx_pcie_init_irq_domain - Initialize IRQ domain
// @pcie: PCIe port information
//
// Return: '0' on success and error value on failure
//
#[no_mangle]
unsafe extern "C" fn xilinx_pcie_init_irq_domain(pcie: *mut xilinx_pcie) -> c_int {
    static int xilinx_pcie_init_irq_domain(struct xilinx_pcie *pcie)
    {
    struct device *dev = pcie.dev;
    struct device_node *pcie_intc_node;
    int ret;
// Setup INTx
    pcie_intc_node = of_get_next_child(dev.of_node, core::ptr::null_mut());
    if (!pcie_intc_node) {
    dev_err(dev, "No PCIe Intc node found\n");
    return -ENODEV;
    }
    pcie.leg_domain = irq_domain_create_linear(of_fwnode_handle(pcie_intc_node), PCI_NUM_INTX,
    &intx_domain_ops, pcie);
    of_node_put(pcie_intc_node);
    if (!pcie.leg_domain) {
    dev_err(dev, "Failed to get a INTx IRQ domain\n");
    return -ENODEV;
    }
// Setup MSI
    if (IS_ENABLED(CONFIG_PCI_MSI)) {
    let mut pa: phys_addr_t = ALIGN_DOWN(virt_to_phys(pcie), SZ_4K);
    ret = xilinx_allocate_msi_domains(pcie);
    if (ret) {
    irq_domain_remove(pcie.leg_domain);
    return ret;
    }
    pcie_write(pcie, upper_32_bits(pa), XILINX_PCIE_REG_MSIBASE1);
    pcie_write(pcie, lower_32_bits(pa), XILINX_PCIE_REG_MSIBASE2);
    }
    return 0;
    }
//
// xilinx_pcie_init_port - Initialize hardware
// @pcie: PCIe port information
//
#[no_mangle]
unsafe extern "C" fn xilinx_pcie_init_port(pcie: *mut xilinx_pcie) {
    static void xilinx_pcie_init_port(struct xilinx_pcie *pcie)
    {
    struct device *dev = pcie.dev;
    if (xilinx_pcie_link_up(pcie))
    dev_info(dev, "PCIe Link is UP\n");
    else
    dev_info(dev, "PCIe Link is DOWN\n");
// Disable all interrupts
    pcie_write(pcie, ~XILINX_PCIE_IDR_ALL_MASK,
    XILINX_PCIE_REG_IMR);
// Clear pending interrupts
    pcie_write(pcie, pcie_read(pcie, XILINX_PCIE_REG_IDR) &
    XILINX_PCIE_IMR_ALL_MASK,
    XILINX_PCIE_REG_IDR);
// Enable all interrupts we handle
    pcie_write(pcie, XILINX_PCIE_IMR_ENABLE_MASK, XILINX_PCIE_REG_IMR);
// Enable the Bridge enable bit
    pcie_write(pcie, pcie_read(pcie, XILINX_PCIE_REG_RPSC) |
    XILINX_PCIE_REG_RPSC_BEN,
    XILINX_PCIE_REG_RPSC);
    }
//
// xilinx_pcie_parse_dt - Parse Device tree
// @pcie: PCIe port information
//
// Return: '0' on success and error value on failure
//
#[no_mangle]
unsafe extern "C" fn xilinx_pcie_parse_dt(pcie: *mut xilinx_pcie) -> c_int {
    static int xilinx_pcie_parse_dt(struct xilinx_pcie *pcie)
    {
    struct device *dev = pcie.dev;
    struct device_node *node = dev.of_node;
    struct resource regs;
    unsigned int irq;
    int err;
    err = of_address_to_resource(node, 0, &regs);
    if (err) {
    dev_err(dev, "missing \"reg\" property\n");
    return err;
    }
    pcie.reg_base = devm_pci_remap_cfg_resource(dev, &regs);
    if (IS_ERR(pcie.reg_base))
    return PTR_ERR(pcie.reg_base);
    irq = irq_of_parse_and_map(node, 0);
    err = devm_request_irq(dev, irq, xilinx_pcie_intr_handler,
    IRQF_SHARED | IRQF_NO_THREAD,
    "xilinx-pcie", pcie);
    if (err) {
    dev_err(dev, "unable to request irq %d\n", irq);
    return err;
    }
    return 0;
    }
//
// xilinx_pcie_probe - Probe function
// @pdev: Platform device pointer
//
// Return: '0' on success and error value on failure
//
#[no_mangle]
unsafe extern "C" fn xilinx_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int xilinx_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct xilinx_pcie *pcie;
    struct pci_host_bridge *bridge;
    int err;
    if (!dev.of_node)
    return -ENODEV;
    bridge = devm_pci_alloc_host_bridge(dev, sizeof(*pcie));
    if (!bridge)
    return -ENODEV;
    pcie = pci_host_bridge_priv(bridge);
    mutex_init(&pcie.map_lock);
    pcie.dev = dev;
    err = xilinx_pcie_parse_dt(pcie);
    if (err) {
    dev_err(dev, "Parsing DT failed\n");
    return err;
    }
    xilinx_pcie_init_port(pcie);
    err = xilinx_pcie_init_irq_domain(pcie);
    if (err) {
    dev_err(dev, "Failed creating IRQ Domain\n");
    return err;
    }
    bridge.sysdata = pcie;
    bridge.ops = &xilinx_pcie_ops;
    err = pci_host_probe(bridge);
    if (err)
    xilinx_free_irq_domains(pcie);
    return err;
    }
    static const struct of_device_id xilinx_pcie_of_match[] = {
    { .compatible = "xlnx,axi-pcie-host-1.00.a", },
    {}
    };
    static struct platform_driver xilinx_pcie_driver = {
    .driver = {
    .name = "xilinx-pcie",
    .of_match_table = xilinx_pcie_of_match,
    .suppress_bind_attrs = true,
    },
    .probe = xilinx_pcie_probe,
    };
    builtin_platform_driver(xilinx_pcie_driver);
