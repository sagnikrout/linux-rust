//! Automatically rewritten from C to Rust
//! Source: arch/x86/pci/mmconfig_64.c
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
// mmconfig.c - Low-level direct PCI config space access via MMCONFIG
//
// This is an 64bit optimized version that always keeps the full mmconfig
// space mapped. This allows lockless config space operation.
//

    static char __iomem *pci_dev_base(unsigned int seg, unsigned int bus, unsigned int devfn)
    {
    struct pci_mmcfg_region *cfg = pci_mmconfig_lookup(seg, bus);
    if (cfg && cfg.virt)
    return cfg.virt + (PCI_MMCFG_BUS_OFFSET(bus) | (devfn << 12));
    return core::ptr::null_mut();
    }
    static int pci_mmcfg_read(unsigned int seg, unsigned int bus,
    unsigned int devfn, int reg, int len, u32 *value)
    {
    char __iomem *addr;
// Why do we have this when nobody checks it. How about a BUG()!? -AK
    if (unlikely((bus > 255) || (devfn > 255) || (reg > 4095))) {
    err:		*value = -1;
    return -EINVAL;
    }
    rcu_read_lock();
    addr = pci_dev_base(seg, bus, devfn);
    if (!addr) {
    rcu_read_unlock();
    goto err;
    }
    switch (len) {
    case 1:
// value = mmio_config_readb(addr + reg);
    break;
    case 2:
// value = mmio_config_readw(addr + reg);
    break;
    case 4:
// value = mmio_config_readl(addr + reg);
    break;
    }
    rcu_read_unlock();
    return 0;
    }
    static int pci_mmcfg_write(unsigned int seg, unsigned int bus,
    unsigned int devfn, int reg, int len, u32 value)
    {
    char __iomem *addr;
// Why do we have this when nobody checks it. How about a BUG()!? -AK
    if (unlikely((bus > 255) || (devfn > 255) || (reg > 4095)))
    return -EINVAL;
    rcu_read_lock();
    addr = pci_dev_base(seg, bus, devfn);
    if (!addr) {
    rcu_read_unlock();
    return -EINVAL;
    }
    switch (len) {
    case 1:
    mmio_config_writeb(addr + reg, value);
    break;
    case 2:
    mmio_config_writew(addr + reg, value);
    break;
    case 4:
    mmio_config_writel(addr + reg, value);
    break;
    }
    rcu_read_unlock();
    return 0;
    }
    const struct pci_raw_ops pci_mmcfg = {
    .read =		pci_mmcfg_read,
    .write =	pci_mmcfg_write,
    };
    static void __iomem *mcfg_ioremap(struct pci_mmcfg_region *cfg)
    {
    void __iomem *addr;
    u64 start, size;
    int num_buses;
    start = cfg.address + PCI_MMCFG_BUS_OFFSET(cfg.start_bus);
    num_buses = cfg.end_bus - cfg.start_bus + 1;
    size = PCI_MMCFG_BUS_OFFSET(num_buses);
    addr = ioremap(start, size);
    if (addr)
    addr -= PCI_MMCFG_BUS_OFFSET(cfg.start_bus);
    return addr;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_arch_map(cfg: *mut pci_mmcfg_region) -> c_int {
    int pci_mmcfg_arch_map(struct pci_mmcfg_region *cfg)
    {
    cfg.virt = mcfg_ioremap(cfg);
    if (!cfg.virt) {
    pr_err("can't map ECAM at %pR\n", &cfg.res);
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_arch_unmap(cfg: *mut pci_mmcfg_region) {
    void pci_mmcfg_arch_unmap(struct pci_mmcfg_region *cfg)
    {
    if (cfg && cfg.virt) {
    iounmap(cfg.virt + PCI_MMCFG_BUS_OFFSET(cfg.start_bus));
    cfg.virt = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_arch_init() -> int __init {
    int __init pci_mmcfg_arch_init(void)
    {
    struct pci_mmcfg_region *cfg;
    list_for_each_entry(cfg, &pci_mmcfg_list, list)
    if (pci_mmcfg_arch_map(cfg)) {
    pci_mmcfg_arch_free();
    return 0;
    }
    raw_pci_ext_ops = &pci_mmcfg;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_arch_free() -> void __init {
    void __init pci_mmcfg_arch_free(void)
    {
    struct pci_mmcfg_region *cfg;
    list_for_each_entry(cfg, &pci_mmcfg_list, list)
    pci_mmcfg_arch_unmap(cfg);
    }
