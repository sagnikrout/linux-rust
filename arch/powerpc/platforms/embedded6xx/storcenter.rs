//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/embedded6xx/storcenter.c
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


//
// Board setup routines for the storcenter
//
// Copyright 2007 (C) Oyvind Repvik (nail@nslu2-linux.org)
// Copyright 2007 Andy Wilcox, Jon Loeliger
//
// Based on linkstation.c by G. Liakhovetski
//
// This file is licensed under the terms of the GNU General Public License
// version 2.  This program is licensed "as is" without any warranty of
// any kind, whether express or implied.
//

    static const struct of_device_id storcenter_of_bus[] __initconst = {
    { .name = "soc", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn storcenter_device_probe() -> int __init {
    static int __init storcenter_device_probe(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), storcenter_of_bus, core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(storcenter, storcenter_device_probe);
#[no_mangle]
unsafe extern "C" fn storcenter_add_bridge(dev: *mut device_node) -> int __init {
    static int __init storcenter_add_bridge(struct device_node *dev)
    {

    int len;
    struct pci_controller *hose;
    const int *bus_range;
    printk("Adding PCI host bridge %pOF\n", dev);
    hose = pcibios_alloc_controller(dev);
    if (hose == core::ptr::null_mut())
    return -ENOMEM;
    bus_range = of_get_property(dev, "bus-range", &len);
    hose.first_busno = bus_range ? bus_range[0] : 0;
    hose.last_busno = bus_range ? bus_range[1] : 0xff;
    setup_indirect_pci(hose, MPC10X_MAPB_CNFG_ADDR, MPC10X_MAPB_CNFG_DATA, 0);
// Interpret the "ranges" property
// This also maps the I/O region and sets isa_io/mem_base
    pci_process_bridge_OF_ranges(hose, dev, 1);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn storcenter_setup_arch() -> void __init {
    static void __init storcenter_setup_arch(void)
    {
    printk(KERN_INFO "IOMEGA StorCenter\n");
    }
#[no_mangle]
unsafe extern "C" fn storcenter_setup_pci() -> void __init {
    static void __init storcenter_setup_pci(void)
    {
    struct device_node *np;
// Lookup PCI host bridges
    for_each_compatible_node(np, "pci", "mpc10x-pci")
    storcenter_add_bridge(np);
    }
//
// Interrupt setup and service.  Interrupts on the turbostation come
// from the four PCI slots plus onboard 8241 devices: I2C, DUART.
//
#[no_mangle]
unsafe extern "C" fn storcenter_init_IRQ() -> void __init {
    static void __init storcenter_init_IRQ(void)
    {
    struct mpic *mpic;
    mpic = mpic_alloc(core::ptr::null_mut(), 0, 0, 16, 0, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
//
// 16 Serial Interrupts followed by 16 Internal Interrupts.
// I2C is the second internal, so it is at 17, 0x11020.
//
    mpic_assign_isu(mpic, 0, mpic.paddr + 0x10200);
    mpic_assign_isu(mpic, 1, mpic.paddr + 0x11000);
    mpic_init(mpic);
    }
#[no_mangle]
unsafe extern "C" fn storcenter_restart(cmd: *mut c_char) -> void __noreturn {
    static void __noreturn storcenter_restart(char *cmd)
    {
    local_irq_disable();
// Set exception prefix high - to the firmware
    mtmsr(mfmsr() | MSR_IP);
    isync();
// Wait for reset to happen
    for (;;) ;
    }
    define_machine(storcenter){
    .name 			= "IOMEGA StorCenter",
    .compatible		= "iomega,storcenter",
    .setup_arch 		= storcenter_setup_arch,
    .discover_phbs 		= storcenter_setup_pci,
    .init_IRQ 		= storcenter_init_IRQ,
    .get_irq 		= mpic_get_irq,
    .restart 		= storcenter_restart,
    };
