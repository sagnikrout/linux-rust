//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/pci-rcar-gen2.c
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
// pci-rcar-gen2: internal PCI bus support
//
// Copyright (C) 2013 Renesas Solutions Corp.
// Copyright (C) 2013 Cogent Embedded, Inc.
//
// Author: Valentine Barshak <valentine.barshak@cogentembedded.com>
//

// AHB-PCI Bridge PCI communication registers
pub const RCAR_AHBPCI_PCICOM_OFFSET: c_uint = 0x800;

pub const RCAR_PCIAHB_PREFETCH0: c_uint = 0x0;
pub const RCAR_PCIAHB_PREFETCH4: c_uint = 0x1;
pub const RCAR_PCIAHB_PREFETCH8: c_uint = 0x2;
pub const RCAR_PCIAHB_PREFETCH16: c_uint = 0x3;

    RCAR_PCI_INT_SIGRETABORT	| \
    RCAR_PCI_INT_REMABORT		| \
    RCAR_PCI_INT_PERR		| \
    RCAR_PCI_INT_SIGSERR		| \
    RCAR_PCI_INT_RESERR		| \
    RCAR_PCI_INT_WIN1ERR		| \
    RCAR_PCI_INT_WIN2ERR)

    RCAR_AHB_BUS_MMODE_BYTE_BURST |	\
    RCAR_AHB_BUS_MMODE_WR_INCR |	\
    RCAR_AHB_BUS_MMODE_HBUS_REQ |	\
    RCAR_AHB_BUS_SMODE_READYCTR)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_pci {
    pub dev: *mut device,
    pub reg: *mut void __iomem,
    pub mem_res: resource,
    pub cfg_res: *mut resource,
    pub irq: c_int,
}

// PCI configuration space operations
    static void __iomem *rcar_pci_cfg_base(struct pci_bus *bus, unsigned int devfn,
    int where)
    {
    struct rcar_pci *priv = bus.sysdata;
    int slot, val;
    if (!pci_is_root_bus(bus) || PCI_FUNC(devfn))
    return core::ptr::null_mut();
// Only one EHCI/OHCI device built-in
    slot = PCI_SLOT(devfn);
    if (slot > 2)
    return core::ptr::null_mut();
// bridge logic only has registers to 0x40
    if (slot == 0x0 && where >= 0x40)
    return core::ptr::null_mut();
    val = slot ? RCAR_AHBPCI_WIN1_DEVICE | RCAR_AHBPCI_WIN_CTR_CFG :
    RCAR_AHBPCI_WIN1_HOST | RCAR_AHBPCI_WIN_CTR_CFG;
    iowrite32(val, priv.reg + RCAR_AHBPCI_WIN1_CTR_REG);
    return priv.reg + (slot >> 1) * 0x100 + where;
    }

// if debug enabled, then attach an error handler irq to the bridge
#[no_mangle]
unsafe extern "C" fn rcar_pci_err_irq(irq: c_int, pw: *mut c_void) -> irqreturn_t {
    static irqreturn_t rcar_pci_err_irq(int irq, void *pw)
    {
    struct rcar_pci *priv = pw;
    struct device *dev = priv.dev;
    let mut status: u32 = ioread32(priv.reg + RCAR_PCI_INT_STATUS_REG);
    if (status & RCAR_PCI_INT_ALLERRORS) {
    dev_err(dev, "error irq: status %08x\n", status);
// clear the error(s)
    iowrite32(status & RCAR_PCI_INT_ALLERRORS,
    priv.reg + RCAR_PCI_INT_STATUS_REG);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn rcar_pci_setup_errirq(priv: *mut rcar_pci) {
    static void rcar_pci_setup_errirq(struct rcar_pci *priv)
    {
    struct device *dev = priv.dev;
    int ret;
    u32 val;
    ret = devm_request_irq(dev, priv.irq, rcar_pci_err_irq,
    IRQF_SHARED, "error irq", priv);
    if (ret) {
    dev_err(dev, "cannot claim IRQ for error handling\n");
    return;
    }
    val = ioread32(priv.reg + RCAR_PCI_INT_ENABLE_REG);
    val |= RCAR_PCI_INT_ALLERRORS;
    iowrite32(val, priv.reg + RCAR_PCI_INT_ENABLE_REG);
    }

    static inline void rcar_pci_setup_errirq(struct rcar_pci *priv) { }

// PCI host controller setup
#[no_mangle]
unsafe extern "C" fn rcar_pci_setup(priv: *mut rcar_pci) {
    static void rcar_pci_setup(struct rcar_pci *priv)
    {
    struct pci_host_bridge *bridge = pci_host_bridge_from_priv(priv);
    struct device *dev = priv.dev;
    void __iomem *reg = priv.reg;
    struct resource_entry *entry;
    unsigned long window_size;
    unsigned long window_addr;
    unsigned long window_pci;
    u32 val;
    entry = resource_list_first_type(&bridge.dma_ranges, IORESOURCE_MEM);
    if (!entry) {
    window_addr = 0x40000000;
    window_pci = 0x40000000;
    window_size = SZ_1G;
    } else {
    window_addr = entry.res.start;
    window_pci = entry.res.start - entry.offset;
    window_size = resource_size(entry.res);
    }
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
    val = ioread32(reg + RCAR_PCI_UNIT_REV_REG);
    dev_info(dev, "PCI: revision %x\n", val);
// Disable Direct Power Down State and assert reset
    val = ioread32(reg + RCAR_USBCTR_REG) & ~RCAR_USBCTR_DIRPD;
    val |= RCAR_USBCTR_USBH_RST | RCAR_USBCTR_PLL_RST;
    iowrite32(val, reg + RCAR_USBCTR_REG);
    udelay(4);
// De-assert reset and reset PCIAHB window1 size
    val &= ~(RCAR_USBCTR_PCIAHB_WIN1_MASK | RCAR_USBCTR_PCICLK_MASK |
    RCAR_USBCTR_USBH_RST | RCAR_USBCTR_PLL_RST);
// Setup PCIAHB window1 size
    switch (window_size) {
    case SZ_2G:
    val |= RCAR_USBCTR_PCIAHB_WIN1_2G;
    break;
    case SZ_1G:
    val |= RCAR_USBCTR_PCIAHB_WIN1_1G;
    break;
    case SZ_512M:
    val |= RCAR_USBCTR_PCIAHB_WIN1_512M;
    break;
    default:
    pr_warn("unknown window size %ld - defaulting to 256M\n",
    window_size);
    window_size = SZ_256M;
    fallthrough;
    case SZ_256M:
    val |= RCAR_USBCTR_PCIAHB_WIN1_256M;
    break;
    }
    iowrite32(val, reg + RCAR_USBCTR_REG);
// Configure AHB master and slave modes
    iowrite32(RCAR_AHB_BUS_MODE, reg + RCAR_AHB_BUS_CTR_REG);
// Configure PCI arbiter
    val = ioread32(reg + RCAR_PCI_ARBITER_CTR_REG);
    val |= RCAR_PCI_ARBITER_PCIREQ0 | RCAR_PCI_ARBITER_PCIREQ1 |
    RCAR_PCI_ARBITER_PCIBP_MODE;
    iowrite32(val, reg + RCAR_PCI_ARBITER_CTR_REG);
// PCI-AHB mapping
    iowrite32(window_addr | RCAR_PCIAHB_PREFETCH16,
    reg + RCAR_PCIAHB_WIN1_CTR_REG);
// AHB-PCI mapping: OHCI/EHCI registers
    val = priv.mem_res.start | RCAR_AHBPCI_WIN_CTR_MEM;
    iowrite32(val, reg + RCAR_AHBPCI_WIN2_CTR_REG);
// Enable AHB-PCI bridge PCI configuration access
    iowrite32(RCAR_AHBPCI_WIN1_HOST | RCAR_AHBPCI_WIN_CTR_CFG,
    reg + RCAR_AHBPCI_WIN1_CTR_REG);
// Set PCI-AHB Window1 address
    iowrite32(window_pci | PCI_BASE_ADDRESS_MEM_PREFETCH,
    reg + PCI_BASE_ADDRESS_1);
// Set AHB-PCI bridge PCI communication area address
    val = priv.cfg_res.start + RCAR_AHBPCI_PCICOM_OFFSET;
    iowrite32(val, reg + PCI_BASE_ADDRESS_0);
    val = ioread32(reg + PCI_COMMAND);
    val |= PCI_COMMAND_SERR | PCI_COMMAND_PARITY |
    PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER;
    iowrite32(val, reg + PCI_COMMAND);
// Enable PCI interrupts
    iowrite32(RCAR_PCI_INT_A | RCAR_PCI_INT_B | RCAR_PCI_INT_PME,
    reg + RCAR_PCI_INT_ENABLE_REG);
    rcar_pci_setup_errirq(priv);
    }
    static struct pci_ops rcar_pci_ops = {
    .map_bus = rcar_pci_cfg_base,
    .read	= pci_generic_config_read,
    .write	= pci_generic_config_write,
    };
#[no_mangle]
unsafe extern "C" fn rcar_pci_probe(pdev: *mut platform_device) -> c_int {
    static int rcar_pci_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct resource *cfg_res, *mem_res;
    struct rcar_pci *priv;
    struct pci_host_bridge *bridge;
    void __iomem *reg;
    bridge = devm_pci_alloc_host_bridge(dev, sizeof(*priv));
    if (!bridge)
    return -ENOMEM;
    priv = pci_host_bridge_priv(bridge);
    bridge.sysdata = priv;
    reg = devm_platform_get_and_ioremap_resource(pdev, 0, &cfg_res);
    if (IS_ERR(reg))
    return PTR_ERR(reg);
    mem_res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if (!mem_res || !mem_res.start)
    return -ENODEV;
    if (mem_res.start & 0xFFFF)
    return -EINVAL;
    priv.mem_res = *mem_res;
    priv.cfg_res = cfg_res;
    priv.irq = platform_get_irq(pdev, 0);
    priv.reg = reg;
    priv.dev = dev;
    if (priv.irq < 0) {
    dev_err(dev, "no valid irq found\n");
    return priv.irq;
    }
    bridge.ops = &rcar_pci_ops;
    pci_add_flags(PCI_REASSIGN_ALL_BUS);
    rcar_pci_setup(priv);
    return pci_host_probe(bridge);
    }
    static const struct of_device_id rcar_pci_of_match[] = {
    { .compatible = "renesas,pci-r8a7790", },
    { .compatible = "renesas,pci-r8a7791", },
    { .compatible = "renesas,pci-r8a7794", },
    { .compatible = "renesas,pci-rcar-gen2", },
    { .compatible = "renesas,pci-rzn1", },
    { },
    };
    static struct platform_driver rcar_pci_driver = {
    .driver = {
    .name = "pci-rcar-gen2",
    .suppress_bind_attrs = true,
    .of_match_table = rcar_pci_of_match,
    },
    .probe = rcar_pci_probe,
    };
    builtin_platform_driver(rcar_pci_driver);
