//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/embedded6xx/mvme5100.c
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
// Board setup routines for the Motorola/Emerson MVME5100.
//
// Copyright 2013 CSC Australia Pty. Ltd.
//
// Based on earlier code by:
//
// Matt Porter, MontaVista Software Inc.
// Copyright 2001 MontaVista Software Inc.
//
// Author: Stephen Chivers <schivers@csc.com>
//

pub const HAWK_MPIC_SIZE: c_uint = 0x00040000U;
pub const MVME5100_PCI_MEM_OFFSET: c_uint = 0x00000000;
// Board register addresses.
pub const BOARD_STATUS_REG: c_uint = 0xfef88080;
pub const BOARD_MODFAIL_REG: c_uint = 0xfef88090;
pub const BOARD_MODRST_REG: c_uint = 0xfef880a0;
pub const BOARD_TBEN_REG: c_uint = 0xfef880c0;
pub const BOARD_SW_READ_REG: c_uint = 0xfef880e0;
pub const BOARD_GEO_ADDR_REG: c_uint = 0xfef880e8;
pub const BOARD_EXT_FEATURE1_REG: c_uint = 0xfef880f0;
pub const BOARD_EXT_FEATURE2_REG: c_uint = 0xfef88100;
    static phys_addr_t pci_membase;
    static u_char *restart;
#[no_mangle]
unsafe extern "C" fn mvme5100_8259_cascade(desc: *mut irq_desc) {
    static void mvme5100_8259_cascade(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    let mut cascade_irq: c_uint = i8259_irq();
    if (cascade_irq)
    generic_handle_irq(cascade_irq);
    chip.irq_eoi(&desc.irq_data);
    }
#[no_mangle]
unsafe extern "C" fn mvme5100_pic_init() -> void __init {
    static void __init mvme5100_pic_init(void)
    {
    struct mpic *mpic;
    struct device_node *np;
    struct device_node *cp = core::ptr::null_mut();
    unsigned int cirq;
    let mut intack: c_ulong = 0;
    const u32 *prop = core::ptr::null_mut();
    np = of_find_node_by_type(core::ptr::null_mut(), "open-pic");
    if (!np) {
    pr_err("Could not find open-pic node\n");
    return;
    }
    mpic = mpic_alloc(np, pci_membase, 0, 16, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    of_node_put(np);
    mpic_assign_isu(mpic, 0, pci_membase + 0x10000);
    mpic_init(mpic);
    cp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "chrp,iic");
    if (cp == core::ptr::null_mut()) {
    pr_warn("mvme5100_pic_init: couldn't find i8259\n");
    return;
    }
    cirq = irq_of_parse_and_map(cp, 0);
    if (!cirq) {
    pr_warn("mvme5100_pic_init: no cascade interrupt?\n");
    return;
    }
    np = of_find_compatible_node(core::ptr::null_mut(), "pci", "mpc10x-pci");
    if (np) {
    prop = of_get_property(np, "8259-interrupt-acknowledge", core::ptr::null_mut());
    if (prop)
    intack = prop[0];
    of_node_put(np);
    }
    if (intack)
    pr_debug("mvme5100_pic_init: PCI 8259 intack at 0x%016lx\n",
    intack);
    i8259_init(cp, intack);
    of_node_put(cp);
    irq_set_chained_handler(cirq, mvme5100_8259_cascade);
    }
#[no_mangle]
unsafe extern "C" fn mvme5100_add_bridge(dev: *mut device_node) -> int __init {
    static int __init mvme5100_add_bridge(struct device_node *dev)
    {
    const int		*bus_range;
    int			len;
    struct pci_controller	*hose;
    unsigned short		devid;
    pr_info("Adding PCI host bridge %pOF\n", dev);
    bus_range = of_get_property(dev, "bus-range", &len);
    hose = pcibios_alloc_controller(dev);
    if (hose == core::ptr::null_mut())
    return -ENOMEM;
    hose.first_busno = bus_range ? bus_range[0] : 0;
    hose.last_busno = bus_range ? bus_range[1] : 0xff;
    setup_indirect_pci(hose, 0xfe000cf8, 0xfe000cfc, 0);
    pci_process_bridge_OF_ranges(hose, dev, 1);
    early_read_config_word(hose, 0, 0, PCI_DEVICE_ID, &devid);
    if (devid != PCI_DEVICE_ID_MOTOROLA_HAWK) {
    pr_err("HAWK PHB not present?\n");
    return 0;
    }
    early_read_config_dword(hose, 0, 0, PCI_BASE_ADDRESS_1, &pci_membase);
    if (pci_membase == 0) {
    pr_err("HAWK PHB mibar not correctly set?\n");
    return 0;
    }
    pr_info("mvme5100_pic_init: pci_membase: %x\n", pci_membase);
    return 0;
    }
    static const struct of_device_id mvme5100_of_bus_ids[] __initconst = {
    { .compatible = "hawk-bridge", },
    {},
    };
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn mvme5100_setup_arch() -> void __init {
    static void __init mvme5100_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("mvme5100_setup_arch()", 0);
    restart = ioremap(BOARD_MODRST_REG, 4);
    }
#[no_mangle]
unsafe extern "C" fn mvme5100_setup_pci() -> void __init {
    static void __init mvme5100_setup_pci(void)
    {
    struct device_node *np;
    for_each_compatible_node(np, "pci", "hawk-pci")
    mvme5100_add_bridge(np);
    }
#[no_mangle]
unsafe extern "C" fn mvme5100_show_cpuinfo(m: *mut seq_file) {
    static void mvme5100_show_cpuinfo(struct seq_file *m)
    {
    seq_puts(m, "Vendor\t\t: Motorola/Emerson\n");
    seq_puts(m, "Machine\t\t: MVME5100\n");
    }
#[no_mangle]
unsafe extern "C" fn mvme5100_restart(cmd: *mut c_char) -> void __noreturn {
    static void __noreturn mvme5100_restart(char *cmd)
    {
    local_irq_disable();
    mtmsr(mfmsr() | MSR_IP);
    out_8((u_char *) restart, 0x01);
    while (1)
    ;
    }
#[no_mangle]
unsafe extern "C" fn probe_of_platform_devices() -> int __init {
    static int __init probe_of_platform_devices(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), mvme5100_of_bus_ids, core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(mvme5100, probe_of_platform_devices);
    define_machine(mvme5100) {
    .name			= "MVME5100",
    .compatible		= "MVME5100",
    .setup_arch		= mvme5100_setup_arch,
    .discover_phbs		= mvme5100_setup_pci,
    .init_IRQ		= mvme5100_pic_init,
    .show_cpuinfo		= mvme5100_show_cpuinfo,
    .get_irq		= mpic_get_irq,
    .restart		= mvme5100_restart,
    .progress		= udbg_progress,
    };
