//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/pci-ftpci100.c
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
// Support for Faraday Technology FTPC100 PCI Controller
//
// Copyright (C) 2017 Linus Walleij <linus.walleij@linaro.org>
//
// Based on the out-of-tree OpenWRT patch for Cortina Gemini:
// Copyright (C) 2009 Janos Laube <janos.dev@gmail.com>
// Copyright (C) 2009 Paulius Zaleckas <paulius.zaleckas@teltonika.lt>
// Based on SL2312 PCI controller code
// Storlink (C) 2003
//

//
// Special configuration registers directly in the first few words
// in I/O space.
//
pub const FTPCI_IOSIZE: c_uint = 0x00;
pub const FTPCI_PROT: c_uint = 0x04 /* AHB protection */;
pub const FTPCI_CTRL: c_uint = 0x08 /* PCI control signal */;
pub const FTPCI_SOFTRST: c_uint = 0x10 /* Soft reset counter and response error enable */;
pub const FTPCI_CONFIG: c_uint = 0x28 /* PCI configuration command register */;
pub const FTPCI_DATA: c_uint = 0x2C;
pub const FARADAY_PCI_STATUS_CMD: c_uint = 0x04 /* Status and command */;
pub const FARADAY_PCI_PMC: c_uint = 0x40 /* Power management control */;
pub const FARADAY_PCI_PMCSR: c_uint = 0x44 /* Power management status */;
pub const FARADAY_PCI_CTRL1: c_uint = 0x48 /* Control register 1 */;
pub const FARADAY_PCI_CTRL2: c_uint = 0x4C /* Control register 2 */;
pub const FARADAY_PCI_MEM1_BASE_SIZE: c_uint = 0x50 /* Memory base and size #1 */;
pub const FARADAY_PCI_MEM2_BASE_SIZE: c_uint = 0x54 /* Memory base and size #2 */;
pub const FARADAY_PCI_MEM3_BASE_SIZE: c_uint = 0x58 /* Memory base and size #3 */;

// Bits 31..28 gives INTD..INTA status
pub const PCI_CTRL2_INTSTS_SHIFT: c_int = 28;

// Bits 25..22 masks INTD..INTA
pub const PCI_CTRL2_INTMASK_SHIFT: c_int = 22;

// Bit 15 reserved

// Bits 7..4 reserved
// Bits 3..0 TRDYW
//
// Memory configs:
// Bit 31..20 defines the PCI side memory base
// Bit 19..16 (4 bits) defines the size per below
//
pub const FARADAY_PCI_MEMBASE_MASK: c_uint = 0xfff00000;
pub const FARADAY_PCI_MEMSIZE_1MB: c_uint = 0x0;
pub const FARADAY_PCI_MEMSIZE_2MB: c_uint = 0x1;
pub const FARADAY_PCI_MEMSIZE_4MB: c_uint = 0x2;
pub const FARADAY_PCI_MEMSIZE_8MB: c_uint = 0x3;
pub const FARADAY_PCI_MEMSIZE_16MB: c_uint = 0x4;
pub const FARADAY_PCI_MEMSIZE_32MB: c_uint = 0x5;
pub const FARADAY_PCI_MEMSIZE_64MB: c_uint = 0x6;
pub const FARADAY_PCI_MEMSIZE_128MB: c_uint = 0x7;
pub const FARADAY_PCI_MEMSIZE_256MB: c_uint = 0x8;
pub const FARADAY_PCI_MEMSIZE_512MB: c_uint = 0x9;
pub const FARADAY_PCI_MEMSIZE_1GB: c_uint = 0xa;
pub const FARADAY_PCI_MEMSIZE_2GB: c_uint = 0xb;
pub const FARADAY_PCI_MEMSIZE_SHIFT: c_int = 16;
//
// The DMA base is set to 0x0 for all memory segments, it reflects the
// fact that the memory of the host system starts at 0x0.
//
pub const FARADAY_PCI_DMA_MEM1_BASE: c_uint = 0x00000000;
pub const FARADAY_PCI_DMA_MEM2_BASE: c_uint = 0x00000000;
pub const FARADAY_PCI_DMA_MEM3_BASE: c_uint = 0x00000000;
//
// struct faraday_pci_variant - encodes IP block differences
// @cascaded_irq: this host has cascaded IRQs from an interrupt controller
// embedded in the host bridge.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct faraday_pci_variant {
    pub cascaded_irq: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct faraday_pci {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub irqdomain: *mut irq_domain,
    pub bus: *mut pci_bus,
    pub bus_clk: *mut clk,
}

    static int faraday_res_to_memcfg(resource_size_t mem_base,
    resource_size_t mem_size, u32 *val)
    {
    u32 outval;
    switch (mem_size) {
    case SZ_1M:
    outval = FARADAY_PCI_MEMSIZE_1MB;
    break;
    case SZ_2M:
    outval = FARADAY_PCI_MEMSIZE_2MB;
    break;
    case SZ_4M:
    outval = FARADAY_PCI_MEMSIZE_4MB;
    break;
    case SZ_8M:
    outval = FARADAY_PCI_MEMSIZE_8MB;
    break;
    case SZ_16M:
    outval = FARADAY_PCI_MEMSIZE_16MB;
    break;
    case SZ_32M:
    outval = FARADAY_PCI_MEMSIZE_32MB;
    break;
    case SZ_64M:
    outval = FARADAY_PCI_MEMSIZE_64MB;
    break;
    case SZ_128M:
    outval = FARADAY_PCI_MEMSIZE_128MB;
    break;
    case SZ_256M:
    outval = FARADAY_PCI_MEMSIZE_256MB;
    break;
    case SZ_512M:
    outval = FARADAY_PCI_MEMSIZE_512MB;
    break;
    case SZ_1G:
    outval = FARADAY_PCI_MEMSIZE_1GB;
    break;
    case SZ_2G:
    outval = FARADAY_PCI_MEMSIZE_2GB;
    break;
    default:
    return -EINVAL;
    }
    outval <<= FARADAY_PCI_MEMSIZE_SHIFT;
// This is probably not good
    if (mem_base & ~(FARADAY_PCI_MEMBASE_MASK))
    pr_warn("truncated PCI memory base\n");
// Translate to bridge side address space
    outval |= (mem_base & FARADAY_PCI_MEMBASE_MASK);
    pr_debug("Translated pci base @%pap, size %pap to config %08x\n",
    &mem_base, &mem_size, outval);
// val = outval;
    return 0;
    }
    static int faraday_raw_pci_read_config(struct faraday_pci *p, int bus_number,
    unsigned int fn, int config, int size,
    u32 *value)
    {
    writel(PCI_CONF1_ADDRESS(bus_number, PCI_SLOT(fn),
    PCI_FUNC(fn), config),
    p.base + FTPCI_CONFIG);
// value = readl(p->base + FTPCI_DATA);
    if (size == 1)
// value = (*value >> (8 * (config & 3))) & 0xFF;
#[no_mangle]
pub unsafe extern "C" fn if(2: size ==) -> else {
    else if (size == 2)
// value = (*value >> (8 * (config & 3))) & 0xFFFF;
    return PCIBIOS_SUCCESSFUL;
    }
    static int faraday_pci_read_config(struct pci_bus *bus, unsigned int fn,
    int config, int size, u32 *value)
    {
    struct faraday_pci *p = bus.sysdata;
    dev_dbg(&bus.dev,
    "[read]  slt: %.2d, fnc: %d, cnf: 0x%.2X, val (%d bytes): 0x%.8X\n",
    PCI_SLOT(fn), PCI_FUNC(fn), config, size, *value);
    return faraday_raw_pci_read_config(p, bus.number, fn, config, size, value);
    }
    static int faraday_raw_pci_write_config(struct faraday_pci *p, int bus_number,
    unsigned int fn, int config, int size,
    u32 value)
    {
    let mut ret: c_int = PCIBIOS_SUCCESSFUL;
    writel(PCI_CONF1_ADDRESS(bus_number, PCI_SLOT(fn),
    PCI_FUNC(fn), config),
    p.base + FTPCI_CONFIG);
    switch (size) {
    case 4:
    writel(value, p.base + FTPCI_DATA);
    break;
    case 2:
    writew(value, p.base + FTPCI_DATA + (config & 3));
    break;
    case 1:
    writeb(value, p.base + FTPCI_DATA + (config & 3));
    break;
    default:
    ret = PCIBIOS_BAD_REGISTER_NUMBER;
    }
    return ret;
    }
    static int faraday_pci_write_config(struct pci_bus *bus, unsigned int fn,
    int config, int size, u32 value)
    {
    struct faraday_pci *p = bus.sysdata;
    dev_dbg(&bus.dev,
    "[write] slt: %.2d, fnc: %d, cnf: 0x%.2X, val (%d bytes): 0x%.8X\n",
    PCI_SLOT(fn), PCI_FUNC(fn), config, size, value);
    return faraday_raw_pci_write_config(p, bus.number, fn, config, size,
    value);
    }
    static struct pci_ops faraday_pci_ops = {
    .read	= faraday_pci_read_config,
    .write	= faraday_pci_write_config,
    };
#[no_mangle]
unsafe extern "C" fn faraday_pci_ack_irq(d: *mut irq_data) {
    static void faraday_pci_ack_irq(struct irq_data *d)
    {
    struct faraday_pci *p = irq_data_get_irq_chip_data(d);
    unsigned int reg;
    faraday_raw_pci_read_config(p, 0, 0, FARADAY_PCI_CTRL2, 4, &reg);
    reg &= ~(0xF << PCI_CTRL2_INTSTS_SHIFT);
    reg |= BIT(irqd_to_hwirq(d) + PCI_CTRL2_INTSTS_SHIFT);
    faraday_raw_pci_write_config(p, 0, 0, FARADAY_PCI_CTRL2, 4, reg);
    }
#[no_mangle]
unsafe extern "C" fn faraday_pci_mask_irq(d: *mut irq_data) {
    static void faraday_pci_mask_irq(struct irq_data *d)
    {
    struct faraday_pci *p = irq_data_get_irq_chip_data(d);
    unsigned int reg;
    faraday_raw_pci_read_config(p, 0, 0, FARADAY_PCI_CTRL2, 4, &reg);
    reg &= ~((0xF << PCI_CTRL2_INTSTS_SHIFT)
    | BIT(irqd_to_hwirq(d) + PCI_CTRL2_INTMASK_SHIFT));
    faraday_raw_pci_write_config(p, 0, 0, FARADAY_PCI_CTRL2, 4, reg);
    }
#[no_mangle]
unsafe extern "C" fn faraday_pci_unmask_irq(d: *mut irq_data) {
    static void faraday_pci_unmask_irq(struct irq_data *d)
    {
    struct faraday_pci *p = irq_data_get_irq_chip_data(d);
    unsigned int reg;
    faraday_raw_pci_read_config(p, 0, 0, FARADAY_PCI_CTRL2, 4, &reg);
    reg &= ~(0xF << PCI_CTRL2_INTSTS_SHIFT);
    reg |= BIT(irqd_to_hwirq(d) + PCI_CTRL2_INTMASK_SHIFT);
    faraday_raw_pci_write_config(p, 0, 0, FARADAY_PCI_CTRL2, 4, reg);
    }
#[no_mangle]
unsafe extern "C" fn faraday_pci_irq_handler(desc: *mut irq_desc) {
    static void faraday_pci_irq_handler(struct irq_desc *desc)
    {
    struct faraday_pci *p = irq_desc_get_handler_data(desc);
    struct irq_chip *irqchip = irq_desc_get_chip(desc);
    unsigned int irq_stat, reg, i;
    faraday_raw_pci_read_config(p, 0, 0, FARADAY_PCI_CTRL2, 4, &reg);
    irq_stat = reg >> PCI_CTRL2_INTSTS_SHIFT;
    chained_irq_enter(irqchip, desc);
    for (i = 0; i < 4; i++) {
    if ((irq_stat & BIT(i)) == 0)
    continue;
    generic_handle_domain_irq(p.irqdomain, i);
    }
    chained_irq_exit(irqchip, desc);
    }
    static struct irq_chip faraday_pci_irq_chip = {
    .name = "PCI",
    .irq_ack = faraday_pci_ack_irq,
    .irq_mask = faraday_pci_mask_irq,
    .irq_unmask = faraday_pci_unmask_irq,
    };
    static int faraday_pci_irq_map(struct irq_domain *domain, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    irq_set_chip_and_handler(irq, &faraday_pci_irq_chip, handle_level_irq);
    irq_set_chip_data(irq, domain.host_data);
    return 0;
    }
    static const struct irq_domain_ops faraday_pci_irqdomain_ops = {
    .map = faraday_pci_irq_map,
    };
#[no_mangle]
unsafe extern "C" fn faraday_pci_setup_cascaded_irq(p: *mut faraday_pci) -> c_int {
    static int faraday_pci_setup_cascaded_irq(struct faraday_pci *p)
    {
    struct device_node *intc = of_get_next_child(p.dev.of_node, core::ptr::null_mut());
    int irq;
    int i;
    if (!intc) {
    dev_err(p.dev, "missing child interrupt-controller node\n");
    return -EINVAL;
    }
// All PCI IRQs cascade off this one
    irq = of_irq_get(intc, 0);
    if (irq <= 0) {
    dev_err(p.dev, "failed to get parent IRQ\n");
    of_node_put(intc);
    return irq ?: -EINVAL;
    }
    p.irqdomain = irq_domain_create_linear(of_fwnode_handle(intc), PCI_NUM_INTX,
    &faraday_pci_irqdomain_ops, p);
    of_node_put(intc);
    if (!p.irqdomain) {
    dev_err(p.dev, "failed to create Gemini PCI IRQ domain\n");
    return -EINVAL;
    }
    irq_set_chained_handler_and_data(irq, faraday_pci_irq_handler, p);
    for (i = 0; i < 4; i++)
    irq_create_mapping(p.irqdomain, i);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn faraday_pci_parse_map_dma_ranges(p: *mut faraday_pci) -> c_int {
    static int faraday_pci_parse_map_dma_ranges(struct faraday_pci *p)
    {
    struct device *dev = p.dev;
    struct pci_host_bridge *bridge = pci_host_bridge_from_priv(p);
    struct resource_entry *entry;
    u32 confreg[3] = {
    FARADAY_PCI_MEM1_BASE_SIZE,
    FARADAY_PCI_MEM2_BASE_SIZE,
    FARADAY_PCI_MEM3_BASE_SIZE,
    };
    let mut i: c_int = 0;
    u32 val;
    resource_list_for_each_entry(entry, &bridge.dma_ranges) {
    let mut pci_addr: u64 = entry.res.start - entry.offset;
    let mut end: u64 = entry.res.end - entry.offset;
    int ret;
    ret = faraday_res_to_memcfg(pci_addr,
    resource_size(entry.res), &val);
    if (ret) {
    dev_err(dev,
    "DMA range %d: illegal MEM resource size\n", i);
    return -EINVAL;
    }
    dev_info(dev, "DMA MEM%d BASE: 0x%016llx . 0x%016llx config %08x\n",
    i + 1, pci_addr, end, val);
    if (i <= 2) {
    faraday_raw_pci_write_config(p, 0, 0, confreg[i],
    4, val);
    } else {
    dev_err(dev, "ignore extraneous dma-range %d\n", i);
    break;
    }
    i++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn faraday_pci_probe(pdev: *mut platform_device) -> c_int {
    static int faraday_pci_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct faraday_pci_variant *variant =
    of_device_get_match_data(dev);
    struct resource_entry *win;
    struct faraday_pci *p;
    struct resource *io;
    struct pci_host_bridge *host;
    struct clk *clk;
    let mut max_bus_speed: c_uchar = PCI_SPEED_33MHz;
    let mut cur_bus_speed: c_uchar = PCI_SPEED_33MHz;
    int ret;
    u32 val;
    host = devm_pci_alloc_host_bridge(dev, sizeof(*p));
    if (!host)
    return -ENOMEM;
    host.ops = &faraday_pci_ops;
    p = pci_host_bridge_priv(host);
    host.sysdata = p;
    p.dev = dev;
// Retrieve and enable optional clocks
    clk = devm_clk_get_enabled(dev, "PCLK");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    p.bus_clk = devm_clk_get_enabled(dev, "PCICLK");
    if (IS_ERR(p.bus_clk))
    return PTR_ERR(p.bus_clk);
    p.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(p.base))
    return PTR_ERR(p.base);
    win = resource_list_first_type(&host.windows, IORESOURCE_IO);
    if (win) {
    io = win.res;
    if (!faraday_res_to_memcfg(io.start - win.offset,
    resource_size(io), &val)) {
// setup I/O space size
    writel(val, p.base + FTPCI_IOSIZE);
    } else {
    dev_err(dev, "illegal IO mem size\n");
    return -EINVAL;
    }
    }
// Setup hostbridge
    val = readl(p.base + FTPCI_CTRL);
    val |= PCI_COMMAND_IO;
    val |= PCI_COMMAND_MEMORY;
    val |= PCI_COMMAND_MASTER;
    writel(val, p.base + FTPCI_CTRL);
// Mask and clear all interrupts
    faraday_raw_pci_write_config(p, 0, 0, FARADAY_PCI_CTRL2 + 2, 2, 0xF000);
    if (variant.cascaded_irq) {
    ret = faraday_pci_setup_cascaded_irq(p);
    if (ret) {
    dev_err(dev, "failed to setup cascaded IRQ\n");
    return ret;
    }
    }
// Check bus clock if we can gear up to 66 MHz
    if (!IS_ERR(p.bus_clk)) {
    unsigned long rate;
    u32 val;
    faraday_raw_pci_read_config(p, 0, 0,
    FARADAY_PCI_STATUS_CMD, 4, &val);
    rate = clk_get_rate(p.bus_clk);
    if ((rate == 33000000) && (val & PCI_STATUS_66MHZ_CAPABLE)) {
    dev_info(dev, "33MHz bus is 66MHz capable\n");
    max_bus_speed = PCI_SPEED_66MHz;
    ret = clk_set_rate(p.bus_clk, 66000000);
    if (ret)
    dev_err(dev, "failed to set bus clock\n");
    } else {
    dev_info(dev, "33MHz only bus\n");
    max_bus_speed = PCI_SPEED_33MHz;
    }
// Bumping the clock may fail so read back the rate
    rate = clk_get_rate(p.bus_clk);
    if (rate == 33000000)
    cur_bus_speed = PCI_SPEED_33MHz;
    if (rate == 66000000)
    cur_bus_speed = PCI_SPEED_66MHz;
    }
    ret = faraday_pci_parse_map_dma_ranges(p);
    if (ret)
    return ret;
    ret = pci_scan_root_bus_bridge(host);
    if (ret) {
    dev_err(dev, "failed to scan host: %d\n", ret);
    return ret;
    }
    p.bus = host.bus;
    p.bus.max_bus_speed = max_bus_speed;
    p.bus.cur_bus_speed = cur_bus_speed;
    pci_bus_assign_resources(p.bus);
    pci_bus_add_devices(p.bus);
    return 0;
    }
//
// We encode bridge variants here, we have at least two so it doesn't
// hurt to have infrastructure to encompass future variants as well.
//
    static const struct faraday_pci_variant faraday_regular = {
    .cascaded_irq = true,
    };
    static const struct faraday_pci_variant faraday_dual = {
    .cascaded_irq = false,
    };
    static const struct of_device_id faraday_pci_of_match[] = {
    {
    .compatible = "faraday,ftpci100",
    .data = &faraday_regular,
    },
    {
    .compatible = "faraday,ftpci100-dual",
    .data = &faraday_dual,
    },
    {},
    };
    static struct platform_driver faraday_pci_driver = {
    .driver = {
    .name = "ftpci100",
    .of_match_table = faraday_pci_of_match,
    .suppress_bind_attrs = true,
    },
    .probe  = faraday_pci_probe,
    };
    builtin_platform_driver(faraday_pci_driver);
