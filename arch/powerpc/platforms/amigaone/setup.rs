//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/amigaone/setup.c
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
// AmigaOne platform setup
//
// Copyright 2008 Gerhard Pircher (gerhard_pircher@gmx.net)
//
// Based on original amigaone_setup.c source code
// Copyright 2003 by Hans-Joerg Frieden and Thomas Frieden
//

    extern void __flush_disable_L1(void);
#[no_mangle]
unsafe extern "C" fn amigaone_show_cpuinfo(m: *mut seq_file) {
    static void amigaone_show_cpuinfo(struct seq_file *m)
    {
    seq_printf(m, "vendor\t\t: Eyetech Ltd.\n");
    }
#[no_mangle]
unsafe extern "C" fn amigaone_add_bridge(dev: *mut device_node) -> int __init {
    static int __init amigaone_add_bridge(struct device_node *dev)
    {
    const u32 *cfg_addr, *cfg_data;
    int len;
    const int *bus_range;
    struct pci_controller *hose;
    printk(KERN_INFO "Adding PCI host bridge %pOF\n", dev);
    cfg_addr = of_get_address(dev, 0, core::ptr::null_mut(), core::ptr::null_mut());
    cfg_data = of_get_address(dev, 1, core::ptr::null_mut(), core::ptr::null_mut());
    if ((cfg_addr == core::ptr::null_mut()) || (cfg_data == core::ptr::null_mut()))
    return -ENODEV;
    bus_range = of_get_property(dev, "bus-range", &len);
    if ((bus_range == core::ptr::null_mut()) || (len < 2 * sizeof(int)))
    printk(KERN_WARNING "Can't get bus-range for %pOF, assume"
    " bus 0\n", dev);
    hose = pcibios_alloc_controller(dev);
    if (hose == core::ptr::null_mut())
    return -ENOMEM;
    hose.first_busno = bus_range ? bus_range[0] : 0;
    hose.last_busno = bus_range ? bus_range[1] : 0xff;
    setup_indirect_pci(hose, cfg_addr[0], cfg_data[0], 0);
// Interpret the "ranges" property
// This also maps the I/O region and sets isa_io/mem_base
    pci_process_bridge_OF_ranges(hose, dev, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amigaone_setup_arch() -> void __init {
    static void __init amigaone_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("Linux/PPC "UTS_RELEASE"\n", 0);
    }
#[no_mangle]
unsafe extern "C" fn amigaone_discover_phbs() -> void __init {
    static void __init amigaone_discover_phbs(void)
    {
    struct device_node *np;
    let mut phb: c_int = -ENODEV;
// Lookup PCI host bridges.
    for_each_compatible_node(np, "pci", "mai-logic,articia-s")
    phb = amigaone_add_bridge(np);
    BUG_ON(phb != 0);
    }
#[no_mangle]
unsafe extern "C" fn amigaone_init_IRQ() -> void __init {
    static void __init amigaone_init_IRQ(void)
    {
    struct device_node *pic, *np = core::ptr::null_mut();
    const unsigned long *prop = core::ptr::null_mut();
    let mut int_ack: c_ulong = 0;
// Search for ISA interrupt controller.
    pic = of_find_compatible_node(core::ptr::null_mut(), "interrupt-controller",
    "pnpPNP,000");
    BUG_ON(pic == core::ptr::null_mut());
// Look for interrupt acknowledge address in the PCI root node.
    np = of_find_compatible_node(core::ptr::null_mut(), "pci", "mai-logic,articia-s");
    if (np) {
    prop = of_get_property(np, "8259-interrupt-acknowledge", core::ptr::null_mut());
    if (prop)
    int_ack = prop[0];
    of_node_put(np);
    }
    if (int_ack == 0)
    printk(KERN_WARNING "Cannot find PCI interrupt acknowledge"
    " address, polling\n");
    i8259_init(pic, int_ack);
    ppc_md.get_irq = i8259_irq;
    irq_set_default_domain(i8259_get_host());
    }
#[no_mangle]
unsafe extern "C" fn request_isa_regions() -> int __init {
    static int __init request_isa_regions(void)
    {
    request_region(0x00, 0x20, "dma1");
    request_region(0x40, 0x20, "timer");
    request_region(0x80, 0x10, "dma page reg");
    request_region(0xc0, 0x20, "dma2");
    return 0;
    }
    machine_device_initcall(amigaone, request_isa_regions);
#[no_mangle]
unsafe extern "C" fn amigaone_restart(cmd: *mut c_char) -> void __noreturn {
    static void __noreturn amigaone_restart(char *cmd)
    {
    local_irq_disable();
// Flush and disable caches.
    __flush_disable_L1();
// Set SRR0 to the reset vector and turn on MSR_IP.
    mtspr(SPRN_SRR0, 0xfff00100);
    mtspr(SPRN_SRR1, MSR_IP);
// Do an rfi to jump back to firmware.
    __asm__ __volatile__("rfi" : : : "memory");
// Not reached.
    while (1);
    }
#[no_mangle]
unsafe extern "C" fn amigaone_probe() -> int __init {
    static int __init amigaone_probe(void)
    {
//
// Coherent memory access cause complete system lockup! Thus
// disable this CPU feature, even if the CPU needs it.
//
    cur_cpu_spec.cpu_features &= ~CPU_FTR_NEED_COHERENT;
    DMA_MODE_READ = 0x44;
    DMA_MODE_WRITE = 0x48;
    return 1;
    }
    define_machine(amigaone) {
    .name			= "AmigaOne",
    .compatible		= "eyetech,amigaone",
    .probe			= amigaone_probe,
    .setup_arch		= amigaone_setup_arch,
    .discover_phbs		= amigaone_discover_phbs,
    .show_cpuinfo		= amigaone_show_cpuinfo,
    .init_IRQ		= amigaone_init_IRQ,
    .restart		= amigaone_restart,
    .progress		= udbg_progress,
    };
