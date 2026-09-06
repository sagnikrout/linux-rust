//! Automatically rewritten from C to Rust
//! Source: arch/x86/pci/mmconfig_32.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2004 Matthew Wilcox <matthew@wil.cx>
// Copyright (C) 2004 Intel Corp.
//
// mmconfig.c - Low-level direct PCI config space access via MMCONFIG
//

// Assume systems with more busses have correct MCFG

// The base address of the last MMCONFIG device accessed
    static u32 mmcfg_last_accessed_device;
    static int mmcfg_last_accessed_cpu;
//
// Functions for accessing PCI configuration space with MMCONFIG accesses
//
#[no_mangle]
unsafe extern "C" fn get_base_addr(seg: c_uint, bus: c_int, devfn: unsigned) -> u32 {
    static u32 get_base_addr(unsigned int seg, int bus, unsigned devfn)
    {
    struct pci_mmcfg_region *cfg = pci_mmconfig_lookup(seg, bus);
    if (cfg)
    return cfg.address;
    return 0;
    }
//
// This is always called under pci_config_lock
//
#[no_mangle]
unsafe extern "C" fn pci_exp_set_dev_base(base: c_uint, bus: c_int, devfn: c_int) {
    static void pci_exp_set_dev_base(unsigned int base, int bus, int devfn)
    {
    let mut dev_base: u32 = base | PCI_MMCFG_BUS_OFFSET(bus) | (devfn << 12);
    let mut cpu: c_int = smp_processor_id();
    if (dev_base != mmcfg_last_accessed_device ||
    cpu != mmcfg_last_accessed_cpu) {
    mmcfg_last_accessed_device = dev_base;
    mmcfg_last_accessed_cpu = cpu;
    set_fixmap_nocache(FIX_PCIE_MCFG, dev_base);
    }
    }
    static int pci_mmcfg_read(unsigned int seg, unsigned int bus,
    unsigned int devfn, int reg, int len, u32 *value)
    {
    unsigned long flags;
    u32 base;
    if ((bus > 255) || (devfn > 255) || (reg > 4095)) {
    err:		*value = -1;
    return -EINVAL;
    }
    rcu_read_lock();
    base = get_base_addr(seg, bus, devfn);
    if (!base) {
    rcu_read_unlock();
    goto err;
    }
    raw_spin_lock_irqsave(&pci_config_lock, flags);
    pci_exp_set_dev_base(base, bus, devfn);
    switch (len) {
    case 1:
// value = mmio_config_readb(mmcfg_virt_addr + reg);
    break;
    case 2:
// value = mmio_config_readw(mmcfg_virt_addr + reg);
    break;
    case 4:
// value = mmio_config_readl(mmcfg_virt_addr + reg);
    break;
    }
    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    rcu_read_unlock();
    return 0;
    }
    static int pci_mmcfg_write(unsigned int seg, unsigned int bus,
    unsigned int devfn, int reg, int len, u32 value)
    {
    unsigned long flags;
    u32 base;
    if ((bus > 255) || (devfn > 255) || (reg > 4095))
    return -EINVAL;
    rcu_read_lock();
    base = get_base_addr(seg, bus, devfn);
    if (!base) {
    rcu_read_unlock();
    return -EINVAL;
    }
    raw_spin_lock_irqsave(&pci_config_lock, flags);
    pci_exp_set_dev_base(base, bus, devfn);
    switch (len) {
    case 1:
    mmio_config_writeb(mmcfg_virt_addr + reg, value);
    break;
    case 2:
    mmio_config_writew(mmcfg_virt_addr + reg, value);
    break;
    case 4:
    mmio_config_writel(mmcfg_virt_addr + reg, value);
    break;
    }
    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    rcu_read_unlock();
    return 0;
    }
    const struct pci_raw_ops pci_mmcfg = {
    .read =		pci_mmcfg_read,
    .write =	pci_mmcfg_write,
    };
#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_arch_init() -> int __init {
    int __init pci_mmcfg_arch_init(void)
    {
    printk(KERN_INFO "PCI: Using ECAM for extended config space\n");
    raw_pci_ext_ops = &pci_mmcfg;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_arch_free() -> void __init {
    void __init pci_mmcfg_arch_free(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_arch_map(cfg: *mut pci_mmcfg_region) -> c_int {
    int pci_mmcfg_arch_map(struct pci_mmcfg_region *cfg)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_arch_unmap(cfg: *mut pci_mmcfg_region) {
    void pci_mmcfg_arch_unmap(struct pci_mmcfg_region *cfg)
    {
    unsigned long flags;
// Invalidate the cached mmcfg map entry.
    raw_spin_lock_irqsave(&pci_config_lock, flags);
    mmcfg_last_accessed_device = 0;
    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    }
