//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-amd-mdb.c
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
// PCIe host controller driver for AMD MDB PCIe Bridge
//
// Copyright (C) 2024-2025, Advanced Micro Devices, Inc.
//

pub const AMD_MDB_TLP_IR_STATUS_MISC: c_uint = 0x4C0;
pub const AMD_MDB_TLP_IR_MASK_MISC: c_uint = 0x4C4;
pub const AMD_MDB_TLP_IR_ENABLE_MISC: c_uint = 0x4C8;
pub const AMD_MDB_TLP_IR_DISABLE_MISC: c_uint = 0x4CC;

// Interrupt registers definitions.
pub const AMD_MDB_PCIE_INTR_CMPL_TIMEOUT: c_int = 15;
pub const AMD_MDB_PCIE_INTR_INTX: c_int = 16;
pub const AMD_MDB_PCIE_INTR_PM_PME_RCVD: c_int = 24;
pub const AMD_MDB_PCIE_INTR_PME_TO_ACK_RCVD: c_int = 25;
pub const AMD_MDB_PCIE_INTR_MISC_CORRECTABLE: c_int = 26;
pub const AMD_MDB_PCIE_INTR_NONFATAL: c_int = 27;
pub const AMD_MDB_PCIE_INTR_FATAL: c_int = 28;

    (						\
    IMR(CMPL_TIMEOUT)	|		\
    IMR(PM_PME_RCVD)	|		\
    IMR(PME_TO_ACK_RCVD)	|		\
    IMR(MISC_CORRECTABLE)	|		\
    IMR(NONFATAL)		|		\
    IMR(FATAL)		|		\
    AMD_MDB_TLP_PCIE_INTX_MASK		\
    )
//
// struct amd_mdb_pcie - PCIe port information
// @pci: DesignWare PCIe controller structure
// @slcr: MDB System Level Control and Status Register (SLCR) base
// @intx_domain: INTx IRQ domain pointer
// @mdb_domain: MDB IRQ domain pointer
// @perst_gpio: GPIO descriptor for PERST# signal handling
// @intx_irq: INTx IRQ interrupt number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_mdb_pcie {
    pub pci: dw_pcie,
    pub slcr: *mut void __iomem,
    pub intx_domain: *mut irq_domain,
    pub mdb_domain: *mut irq_domain,
    pub perst_gpio: *mut gpio_desc,
    pub intx_irq: c_int,
}

    static const struct dw_pcie_host_ops amd_mdb_pcie_host_ops = {
    };
#[no_mangle]
unsafe extern "C" fn amd_mdb_intx_irq_mask(data: *mut irq_data) {
    static void amd_mdb_intx_irq_mask(struct irq_data *data)
    {
    struct amd_mdb_pcie *pcie = irq_data_get_irq_chip_data(data);
    struct dw_pcie *pci = &pcie.pci;
    struct dw_pcie_rp *port = &pci.pp;
    unsigned long flags;
    u32 val;
    raw_spin_lock_irqsave(&port.lock, flags);
    val = FIELD_PREP(AMD_MDB_TLP_PCIE_INTX_MASK,
    AMD_MDB_PCIE_INTR_INTX_ASSERT(data.hwirq));
//
// Writing '1' to a bit in AMD_MDB_TLP_IR_DISABLE_MISC disables that
// interrupt, writing '0' has no effect.
//
    writel_relaxed(val, pcie.slcr + AMD_MDB_TLP_IR_DISABLE_MISC);
    raw_spin_unlock_irqrestore(&port.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn amd_mdb_intx_irq_unmask(data: *mut irq_data) {
    static void amd_mdb_intx_irq_unmask(struct irq_data *data)
    {
    struct amd_mdb_pcie *pcie = irq_data_get_irq_chip_data(data);
    struct dw_pcie *pci = &pcie.pci;
    struct dw_pcie_rp *port = &pci.pp;
    unsigned long flags;
    u32 val;
    raw_spin_lock_irqsave(&port.lock, flags);
    val = FIELD_PREP(AMD_MDB_TLP_PCIE_INTX_MASK,
    AMD_MDB_PCIE_INTR_INTX_ASSERT(data.hwirq));
//
// Writing '1' to a bit in AMD_MDB_TLP_IR_ENABLE_MISC enables that
// interrupt, writing '0' has no effect.
//
    writel_relaxed(val, pcie.slcr + AMD_MDB_TLP_IR_ENABLE_MISC);
    raw_spin_unlock_irqrestore(&port.lock, flags);
    }
    static struct irq_chip amd_mdb_intx_irq_chip = {
    .name		= "AMD MDB INTx",
    .irq_mask	= amd_mdb_intx_irq_mask,
    .irq_unmask	= amd_mdb_intx_irq_unmask,
    };
//
// amd_mdb_pcie_intx_map - Set the handler for the INTx and mark IRQ as valid
// @domain: IRQ domain
// @irq: Virtual IRQ number
// @hwirq: Hardware interrupt number
//
// Return: Always returns '0'.
//
    static int amd_mdb_pcie_intx_map(struct irq_domain *domain,
    unsigned int irq, irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(irq, &amd_mdb_intx_irq_chip,
    handle_level_irq);
    irq_set_chip_data(irq, domain.host_data);
    irq_set_status_flags(irq, IRQ_LEVEL);
    return 0;
    }
// INTx IRQ domain operations.
    static const struct irq_domain_ops amd_intx_domain_ops = {
    .map = amd_mdb_pcie_intx_map,
    };
#[no_mangle]
unsafe extern "C" fn dw_pcie_rp_intx(irq: c_int, args: *mut c_void) -> irqreturn_t {
    static irqreturn_t dw_pcie_rp_intx(int irq, void *args)
    {
    struct amd_mdb_pcie *pcie = args;
    unsigned long val;
    int i, int_status;
    val = readl_relaxed(pcie.slcr + AMD_MDB_TLP_IR_STATUS_MISC);
    int_status = FIELD_GET(AMD_MDB_TLP_PCIE_INTX_MASK, val);
    for (i = 0; i < PCI_NUM_INTX; i++) {
    if (int_status & AMD_MDB_PCIE_INTR_INTX_ASSERT(i))
    generic_handle_domain_irq(pcie.intx_domain, i);
    }
    return IRQ_HANDLED;
    }

    static const struct {
    const char	*sym;
    const char	*str;
    } intr_cause[32] = {
    _IC(CMPL_TIMEOUT,	"Completion timeout"),
    _IC(PM_PME_RCVD,	"PM_PME message received"),
    _IC(PME_TO_ACK_RCVD,	"PME_TO_ACK message received"),
    _IC(MISC_CORRECTABLE,	"Correctable error message"),
    _IC(NONFATAL,		"Non fatal error message"),
    _IC(FATAL,		"Fatal error message"),
    };
#[no_mangle]
unsafe extern "C" fn amd_mdb_event_irq_mask(d: *mut irq_data) {
    static void amd_mdb_event_irq_mask(struct irq_data *d)
    {
    struct amd_mdb_pcie *pcie = irq_data_get_irq_chip_data(d);
    struct dw_pcie *pci = &pcie.pci;
    struct dw_pcie_rp *port = &pci.pp;
    unsigned long flags;
    u32 val;
    raw_spin_lock_irqsave(&port.lock, flags);
    val = BIT(d.hwirq);
    writel_relaxed(val, pcie.slcr + AMD_MDB_TLP_IR_DISABLE_MISC);
    raw_spin_unlock_irqrestore(&port.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn amd_mdb_event_irq_unmask(d: *mut irq_data) {
    static void amd_mdb_event_irq_unmask(struct irq_data *d)
    {
    struct amd_mdb_pcie *pcie = irq_data_get_irq_chip_data(d);
    struct dw_pcie *pci = &pcie.pci;
    struct dw_pcie_rp *port = &pci.pp;
    unsigned long flags;
    u32 val;
    raw_spin_lock_irqsave(&port.lock, flags);
    val = BIT(d.hwirq);
    writel_relaxed(val, pcie.slcr + AMD_MDB_TLP_IR_ENABLE_MISC);
    raw_spin_unlock_irqrestore(&port.lock, flags);
    }
    static struct irq_chip amd_mdb_event_irq_chip = {
    .name		= "AMD MDB RC-Event",
    .irq_mask	= amd_mdb_event_irq_mask,
    .irq_unmask	= amd_mdb_event_irq_unmask,
    };
    static int amd_mdb_pcie_event_map(struct irq_domain *domain,
    unsigned int irq, irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(irq, &amd_mdb_event_irq_chip,
    handle_level_irq);
    irq_set_chip_data(irq, domain.host_data);
    irq_set_status_flags(irq, IRQ_LEVEL);
    return 0;
    }
    static const struct irq_domain_ops event_domain_ops = {
    .map = amd_mdb_pcie_event_map,
    };
#[no_mangle]
unsafe extern "C" fn amd_mdb_pcie_event(irq: c_int, args: *mut c_void) -> irqreturn_t {
    static irqreturn_t amd_mdb_pcie_event(int irq, void *args)
    {
    struct amd_mdb_pcie *pcie = args;
    unsigned long val;
    int i;
    val = readl_relaxed(pcie.slcr + AMD_MDB_TLP_IR_STATUS_MISC);
    val &= ~readl_relaxed(pcie.slcr + AMD_MDB_TLP_IR_MASK_MISC);
    for_each_set_bit(i, &val, 32)
    generic_handle_domain_irq(pcie.mdb_domain, i);
    writel_relaxed(val, pcie.slcr + AMD_MDB_TLP_IR_STATUS_MISC);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn amd_mdb_pcie_free_irq_domains(pcie: *mut amd_mdb_pcie) {
    static void amd_mdb_pcie_free_irq_domains(struct amd_mdb_pcie *pcie)
    {
    if (pcie.intx_domain) {
    irq_domain_remove(pcie.intx_domain);
    pcie.intx_domain = core::ptr::null_mut();
    }
    if (pcie.mdb_domain) {
    irq_domain_remove(pcie.mdb_domain);
    pcie.mdb_domain = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn amd_mdb_pcie_init_port(pcie: *mut amd_mdb_pcie) -> c_int {
    static int amd_mdb_pcie_init_port(struct amd_mdb_pcie *pcie)
    {
    unsigned long val;
// Disable all TLP interrupts.
    writel_relaxed(AMD_MDB_PCIE_IMR_ALL_MASK,
    pcie.slcr + AMD_MDB_TLP_IR_DISABLE_MISC);
// Clear pending TLP interrupts.
    val = readl_relaxed(pcie.slcr + AMD_MDB_TLP_IR_STATUS_MISC);
    val &= AMD_MDB_PCIE_IMR_ALL_MASK;
    writel_relaxed(val, pcie.slcr + AMD_MDB_TLP_IR_STATUS_MISC);
// Enable all TLP interrupts.
    writel_relaxed(AMD_MDB_PCIE_IMR_ALL_MASK,
    pcie.slcr + AMD_MDB_TLP_IR_ENABLE_MISC);
    return 0;
    }
//
// amd_mdb_pcie_init_irq_domains - Initialize IRQ domain
// @pcie: PCIe port information
// @pdev: Platform device
//
// Return: Returns '0' on success and error value on failure.
//
    static int amd_mdb_pcie_init_irq_domains(struct amd_mdb_pcie *pcie,
    struct platform_device *pdev)
    {
    struct dw_pcie *pci = &pcie.pci;
    struct dw_pcie_rp *pp = &pci.pp;
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct device_node *pcie_intc_node;
    int err;
    pcie_intc_node = of_get_child_by_name(node, "interrupt-controller");
    if (!pcie_intc_node) {
    dev_err(dev, "No PCIe Intc node found\n");
    return -ENODEV;
    }
    pcie.mdb_domain = irq_domain_create_linear(of_fwnode_handle(pcie_intc_node), 32,
    &event_domain_ops, pcie);
    if (!pcie.mdb_domain) {
    err = -ENOMEM;
    dev_err(dev, "Failed to add MDB domain\n");
    goto out;
    }
    irq_domain_update_bus_token(pcie.mdb_domain, DOMAIN_BUS_NEXUS);
    pcie.intx_domain = irq_domain_create_linear(of_fwnode_handle(pcie_intc_node),
    PCI_NUM_INTX, &amd_intx_domain_ops, pcie);
    if (!pcie.intx_domain) {
    err = -ENOMEM;
    dev_err(dev, "Failed to add INTx domain\n");
    goto mdb_out;
    }
    of_node_put(pcie_intc_node);
    irq_domain_update_bus_token(pcie.intx_domain, DOMAIN_BUS_WIRED);
    raw_spin_lock_init(&pp.lock);
    return 0;
    mdb_out:
    amd_mdb_pcie_free_irq_domains(pcie);
    out:
    of_node_put(pcie_intc_node);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn amd_mdb_pcie_intr_handler(irq: c_int, args: *mut c_void) -> irqreturn_t {
    static irqreturn_t amd_mdb_pcie_intr_handler(int irq, void *args)
    {
    struct amd_mdb_pcie *pcie = args;
    struct device *dev;
    struct irq_data *d;
    dev = pcie.pci.dev;
//
// In the future, error reporting will be hooked to the AER subsystem.
// Currently, the driver prints a warning message to the user.
//
    d = irq_domain_get_irq_data(pcie.mdb_domain, irq);
    if (intr_cause[d.hwirq].str)
    dev_warn(dev, "%s\n", intr_cause[d.hwirq].str);
    else
    dev_warn_once(dev, "Unknown IRQ %ld\n", d.hwirq);
    return IRQ_HANDLED;
    }
    static int amd_mdb_setup_irq(struct amd_mdb_pcie *pcie,
    struct platform_device *pdev)
    {
    struct dw_pcie *pci = &pcie.pci;
    struct dw_pcie_rp *pp = &pci.pp;
    struct device *dev = &pdev.dev;
    int i, irq, err;
    amd_mdb_pcie_init_port(pcie);
    pp.irq = platform_get_irq(pdev, 0);
    if (pp.irq < 0)
    return pp.irq;
    for (i = 0; i < ARRAY_SIZE(intr_cause); i++) {
    if (!intr_cause[i].str)
    continue;
    irq = irq_create_mapping(pcie.mdb_domain, i);
    if (!irq) {
    dev_err(dev, "Failed to map MDB domain interrupt\n");
    return -ENOMEM;
    }
    err = devm_request_irq(dev, irq, amd_mdb_pcie_intr_handler,
    IRQF_NO_THREAD, intr_cause[i].sym, pcie);
    if (err) {
    dev_err(dev, "Failed to request IRQ %d, err=%d\n",
    irq, err);
    return err;
    }
    }
    pcie.intx_irq = irq_create_mapping(pcie.mdb_domain,
    AMD_MDB_PCIE_INTR_INTX);
    if (!pcie.intx_irq) {
    dev_err(dev, "Failed to map INTx interrupt\n");
    return -ENXIO;
    }
    err = devm_request_irq(dev, pcie.intx_irq, dw_pcie_rp_intx,
    IRQF_NO_THREAD, core::ptr::null_mut(), pcie);
    if (err) {
    dev_err(dev, "Failed to request INTx IRQ %d, err=%d\n",
    pcie.intx_irq, err);
    return err;
    }
// Plug the main event handler.
    err = devm_request_irq(dev, pp.irq, amd_mdb_pcie_event, IRQF_NO_THREAD,
    "amd_mdb pcie_irq", pcie);
    if (err) {
    dev_err(dev, "Failed to request event IRQ %d, err=%d\n",
    pp.irq, err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_mdb_parse_pcie_port(pcie: *mut amd_mdb_pcie) -> c_int {
    static int amd_mdb_parse_pcie_port(struct amd_mdb_pcie *pcie)
    {
    struct device *dev = pcie.pci.dev;
    struct device_node *pcie_port_node __maybe_unused;
//
// This platform currently supports only one Root Port, so the loop
// will execute only once.
// TODO: Enhance the driver to handle multiple Root Ports in the future.
//
    for_each_child_of_node_with_prefix(dev.of_node, pcie_port_node, "pcie") {
    pcie.perst_gpio = devm_fwnode_gpiod_get(dev, of_fwnode_handle(pcie_port_node),
    "reset", GPIOD_OUT_HIGH, core::ptr::null_mut());
    if (IS_ERR(pcie.perst_gpio))
    return dev_err_probe(dev, PTR_ERR(pcie.perst_gpio),
    "Failed to request reset GPIO\n");
    return 0;
    }
    return -ENODEV;
    }
    static int amd_mdb_add_pcie_port(struct amd_mdb_pcie *pcie,
    struct platform_device *pdev)
    {
    struct dw_pcie *pci = &pcie.pci;
    struct dw_pcie_rp *pp = &pci.pp;
    struct device *dev = &pdev.dev;
    int err;
    pcie.slcr = devm_platform_ioremap_resource_byname(pdev, "slcr");
    if (IS_ERR(pcie.slcr))
    return PTR_ERR(pcie.slcr);
    err = amd_mdb_pcie_init_irq_domains(pcie, pdev);
    if (err)
    return err;
    err = amd_mdb_setup_irq(pcie, pdev);
    if (err) {
    dev_err(dev, "Failed to set up interrupts, err=%d\n", err);
    goto out;
    }
    pp.ops = &amd_mdb_pcie_host_ops;
    if (pcie.perst_gpio) {
    mdelay(PCIE_T_PVPERL_MS);
    gpiod_set_value_cansleep(pcie.perst_gpio, 0);
    mdelay(PCIE_RESET_CONFIG_WAIT_MS);
    }
    err = dw_pcie_host_init(pp);
    if (err) {
    dev_err(dev, "Failed to initialize host, err=%d\n", err);
    goto out;
    }
    return 0;
    out:
    amd_mdb_pcie_free_irq_domains(pcie);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn amd_mdb_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int amd_mdb_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct amd_mdb_pcie *pcie;
    struct dw_pcie *pci;
    int ret;
    pcie = devm_kzalloc(dev, sizeof(*pcie), GFP_KERNEL);
    if (!pcie)
    return -ENOMEM;
    pci = &pcie.pci;
    pci.dev = dev;
    platform_set_drvdata(pdev, pcie);
    ret = amd_mdb_parse_pcie_port(pcie);
//
// If amd_mdb_parse_pcie_port returns -ENODEV, it indicates that the
// PCIe Bridge node was not found in the device tree. This is not
// considered a fatal error and will trigger a fallback where the
// reset GPIO is acquired directly from the PCIe Host Bridge node.
//
    if (ret) {
    if (ret != -ENODEV)
    return ret;
    pcie.perst_gpio = devm_gpiod_get_optional(dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(pcie.perst_gpio))
    return dev_err_probe(dev, PTR_ERR(pcie.perst_gpio),
    "Failed to request reset GPIO\n");
    }
    return amd_mdb_add_pcie_port(pcie, pdev);
    }
#[no_mangle]
unsafe extern "C" fn amd_mdb_pcie_shutdown(pdev: *mut platform_device) {
    static void amd_mdb_pcie_shutdown(struct platform_device *pdev)
    {
    struct amd_mdb_pcie *pcie = platform_get_drvdata(pdev);
    gpiod_set_value_cansleep(pcie.perst_gpio, 1);
    }
    static const struct of_device_id amd_mdb_pcie_of_match[] = {
    {
    .compatible = "amd,versal2-mdb-host",
    },
    {},
    };
    static struct platform_driver amd_mdb_pcie_driver = {
    .driver = {
    .name	= "amd-mdb-pcie",
    .of_match_table = amd_mdb_pcie_of_match,
    .suppress_bind_attrs = true,
    },
    .probe = amd_mdb_pcie_probe,
    .shutdown = amd_mdb_pcie_shutdown,
    };
    builtin_platform_driver(amd_mdb_pcie_driver);
