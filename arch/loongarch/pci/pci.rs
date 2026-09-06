//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/pci/pci.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

pub const PCI_DEVICE_ID_LOONGSON_HOST: c_uint = 0x7a00;
pub const PCI_DEVICE_ID_LOONGSON_DC1: c_uint = 0x7a06;
pub const PCI_DEVICE_ID_LOONGSON_DC2: c_uint = 0x7a36;
pub const PCI_DEVICE_ID_LOONGSON_DC3: c_uint = 0x7a46;
pub const PCI_DEVICE_ID_LOONGSON_GPU1: c_uint = 0x7a15;
pub const PCI_DEVICE_ID_LOONGSON_GPU2: c_uint = 0x7a25;
pub const PCI_DEVICE_ID_LOONGSON_GPU3: c_uint = 0x7a35;
    int raw_pci_read(unsigned int domain, unsigned int bus, unsigned int devfn,
    int reg, int len, u32 *val)
    {
    struct pci_bus *bus_tmp = pci_find_bus(domain, bus);
    if (bus_tmp)
    return bus_tmp.ops.read(bus_tmp, devfn, reg, len, val);
    return -EINVAL;
    }
    int raw_pci_write(unsigned int domain, unsigned int bus, unsigned int devfn,
    int reg, int len, u32 val)
    {
    struct pci_bus *bus_tmp = pci_find_bus(domain, bus);
    if (bus_tmp)
    return bus_tmp.ops.write(bus_tmp, devfn, reg, len, val);
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn mcfg_addr_init(node: c_int) -> phys_addr_t {
    phys_addr_t mcfg_addr_init(int node)
    {
    return (((u64)node << 44) | MCFG_EXT_PCICFG_BASE);
    }
#[no_mangle]
unsafe extern "C" fn pcibios_init() -> int __init {
    static int __init pcibios_init(void)
    {
    unsigned int lsize;
//
// Set PCI cacheline size to that of the last level in the
// cache hierarchy.
//
    lsize = cpu_last_level_cache_line_size();
    if (lsize) {
    pci_dfl_cache_line_size = lsize >> 2;
    pr_debug("PCI: pci_cache_line_size set to %d bytes\n", lsize);
    }
    return 0;
    }
    subsys_initcall(pcibios_init);
#[no_mangle]
pub unsafe extern "C" fn pcibios_device_add(dev: *mut pci_dev) -> c_int {
    int pcibios_device_add(struct pci_dev *dev)
    {
    int id;
    struct irq_domain *dom;
    id = pci_domain_nr(dev.bus);
    dom = irq_find_matching_fwnode(get_pch_msi_handle(id), DOMAIN_BUS_PCI_MSI);
    dev_set_msi_domain(&dev.dev, dom);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pcibios_alloc_irq(dev: *mut pci_dev) -> c_int {
    int pcibios_alloc_irq(struct pci_dev *dev)
    {
    if (acpi_disabled)
    return 0;
    if (pci_dev_msi_enabled(dev))
    return 0;
    return acpi_pci_irq_enable(dev);
    }
#[no_mangle]
unsafe extern "C" fn pci_fixup_vgadev(pdev: *mut pci_dev) {
    static void pci_fixup_vgadev(struct pci_dev *pdev)
    {
    struct pci_dev *devp = core::ptr::null_mut();
    while ((devp = pci_get_class(PCI_CLASS_DISPLAY_VGA << 8, devp))) {
    if (devp.vendor != PCI_VENDOR_ID_LOONGSON) {
    vga_set_default_device(devp);
    dev_info(&pdev.dev,
    "Overriding boot device as %X:%X\n",
    devp.vendor, devp.device);
    }
    }
    }
    DECLARE_PCI_FIXUP_FINAL(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_DC1, pci_fixup_vgadev);
    DECLARE_PCI_FIXUP_FINAL(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_DC2, pci_fixup_vgadev);
    DECLARE_PCI_FIXUP_FINAL(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_DC3, pci_fixup_vgadev);
pub const CRTC_NUM_MAX: c_int = 2;
pub const CRTC_OUTPUT_ENABLE: c_uint = 0x100;
#[no_mangle]
unsafe extern "C" fn loongson_gpu_fixup_dma_hang(pdev: *mut pci_dev, on: bool) {
    static void loongson_gpu_fixup_dma_hang(struct pci_dev *pdev, bool on)
    {
    u32 i, val, count, crtc_offset, device;
    void __iomem *crtc_reg, *base, *regbase;
    static u32 crtc_status[CRTC_NUM_MAX] = { 0 };
    base = pdev.bus.ops.map_bus(pdev.bus, pdev.devfn + 1, 0);
    device = readw(base + PCI_DEVICE_ID);
    regbase = ioremap(readq(base + PCI_BASE_ADDRESS_0) & ~0xffull, SZ_64K);
    if (!regbase) {
    pci_err(pdev, "Failed to ioremap()\n");
    return;
    }
    switch (device) {
    case PCI_DEVICE_ID_LOONGSON_DC2:
    crtc_reg = regbase + 0x1240;
    crtc_offset = 0x10;
    break;
    case PCI_DEVICE_ID_LOONGSON_DC3:
    crtc_reg = regbase;
    crtc_offset = 0x400;
    break;
    default:
    iounmap(regbase);
    return;
    }
    for (i = 0; i < CRTC_NUM_MAX; i++, crtc_reg += crtc_offset) {
    val = readl(crtc_reg);
    if (!on)
    crtc_status[i] = val;
// No need to fixup if the status is off at startup.
    if (!(crtc_status[i] & CRTC_OUTPUT_ENABLE))
    continue;
    if (on)
    val |= CRTC_OUTPUT_ENABLE;
    else
    val &= ~CRTC_OUTPUT_ENABLE;
    mb();
    writel(val, crtc_reg);
    for (count = 0; count < 40; count++) {
    val = readl(crtc_reg) & CRTC_OUTPUT_ENABLE;
    if ((on && val) || (!on && !val))
    break;
    udelay(1000);
    }
    pci_info(pdev, "DMA hang fixup at reg[0x%lx]: 0x%x\n",
    (unsigned long)crtc_reg & 0xffff, readl(crtc_reg));
    }
    iounmap(regbase);
    }
#[no_mangle]
unsafe extern "C" fn pci_fixup_dma_hang_early(pdev: *mut pci_dev) {
    static void pci_fixup_dma_hang_early(struct pci_dev *pdev)
    {
    loongson_gpu_fixup_dma_hang(pdev, false);
    }
    DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_GPU2, pci_fixup_dma_hang_early);
    DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_GPU3, pci_fixup_dma_hang_early);
#[no_mangle]
unsafe extern "C" fn pci_fixup_dma_hang_final(pdev: *mut pci_dev) {
    static void pci_fixup_dma_hang_final(struct pci_dev *pdev)
    {
    loongson_gpu_fixup_dma_hang(pdev, true);
    }
    DECLARE_PCI_FIXUP_FINAL(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_GPU2, pci_fixup_dma_hang_final);
    DECLARE_PCI_FIXUP_FINAL(PCI_VENDOR_ID_LOONGSON, PCI_DEVICE_ID_LOONGSON_GPU3, pci_fixup_dma_hang_final);
