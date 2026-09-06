//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/vsmp_64.c
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
// vSMPowered(tm) systems specific initialization
// Copyright (C) 2005 ScaleMP Inc.
//
// Ravikiran Thirumalai <kiran@scalemp.com>,
// Shai Fultheim <shai@scalemp.com>
// Paravirt ops integration: Glauber de Oliveira Costa <gcosta@redhat.com>,
// Ravikiran Thirumalai <kiran@scalemp.com>
//

pub const TOPOLOGY_REGISTER_OFFSET: c_uint = 0x10;

#[no_mangle]
unsafe extern "C" fn set_vsmp_ctl() -> void __init {
    static void __init set_vsmp_ctl(void)
    {
    void __iomem *address;
    unsigned int cap, ctl, cfg;
// set vSMP magic bits to indicate vSMP capable kernel
    cfg = read_pci_config(0, 0x1f, 0, PCI_BASE_ADDRESS_0);
    address = early_ioremap(cfg, 8);
    cap = readl(address);
    ctl = readl(address + 4);
    printk(KERN_INFO "vSMP CTL: capabilities:0x%08x  control:0x%08x\n",
    cap, ctl);
// If possible, let the vSMP foundation route the interrupt optimally

    if (cap & ctl & BIT(8)) {
    ctl &= ~BIT(8);

// Don't let users change irq affinity via procfs
    no_irq_affinity = 1;

    }

    writel(ctl, address + 4);
    ctl = readl(address + 4);
    pr_info("vSMP CTL: control set to:0x%08x\n", ctl);
    early_iounmap(address, 8);
    }
    let mut is_vsmp: static int = -1;
#[no_mangle]
unsafe extern "C" fn detect_vsmp_box() -> void __init {
    static void __init detect_vsmp_box(void)
    {
    is_vsmp = 0;
    if (!early_pci_allowed())
    return;
// Check if we are running on a ScaleMP vSMPowered box
    if (read_pci_config(0, 0x1f, 0, PCI_VENDOR_ID) ==
    (PCI_VENDOR_ID_SCALEMP | (PCI_DEVICE_ID_SCALEMP_VSMP_CTL << 16)))
    is_vsmp = 1;
    }
#[no_mangle]
unsafe extern "C" fn is_vsmp_box() -> c_int {
    static int is_vsmp_box(void)
    {
    if (is_vsmp != -1)
    return is_vsmp;
    else {
    WARN_ON_ONCE(1);
    return 0;
    }
    }

#[no_mangle]
unsafe extern "C" fn detect_vsmp_box() -> void __init {
    static void __init detect_vsmp_box(void)
    {
    }
#[no_mangle]
unsafe extern "C" fn is_vsmp_box() -> c_int {
    static int is_vsmp_box(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_vsmp_ctl() -> void __init {
    static void __init set_vsmp_ctl(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn vsmp_cap_cpus() -> void __init {
    static void __init vsmp_cap_cpus(void)
    {

    void __iomem *address;
    unsigned int cfg, topology, node_shift, maxcpus;
//
// CONFIG_X86_VSMP is not configured, so limit the number CPUs to the
// ones present in the first board, unless explicitly overridden by
// setup_max_cpus
//
    if (setup_max_cpus != NR_CPUS)
    return;
// Read the vSMP Foundation topology register
    cfg = read_pci_config(0, 0x1f, 0, PCI_BASE_ADDRESS_0);
    address = early_ioremap(cfg + TOPOLOGY_REGISTER_OFFSET, 4);
    if (WARN_ON(!address))
    return;
    topology = readl(address);
    node_shift = (topology >> 16) & 0x7;
    if (!node_shift)
// The value 0 should be decoded as 8
    node_shift = 8;
    maxcpus = (topology & ((1 << node_shift) - 1)) + 1;
    pr_info("vSMP CTL: Capping CPUs to %d (CONFIG_X86_VSMP is unset)\n",
    maxcpus);
    setup_max_cpus = maxcpus;
    early_iounmap(address, 4);

    }
#[no_mangle]
pub unsafe extern "C" fn vsmp_init() -> void __init {
    void __init vsmp_init(void)
    {
    detect_vsmp_box();
    if (!is_vsmp_box())
    return;
    vsmp_cap_cpus();
    set_vsmp_ctl();
    return;
    }
