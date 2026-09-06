//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/embedded6xx/linkstation.c
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
// Board setup routines for the Buffalo Linkstation / Kurobox Platform.
//
// Copyright (C) 2006 G. Liakhovetski (g.liakhovetski@gmx.de)
//
// Based on sandpoint.c by Mark A. Greer
//
// This file is licensed under the terms of the GNU General Public License
// version 2.  This program is licensed "as is" without any warranty of
// any kind, whether express or implied.
//

    static const struct of_device_id of_bus_ids[] __initconst = {
    { .type = "soc", },
    { .compatible = "simple-bus", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn declare_of_platform_devices() -> int __init {
    static int __init declare_of_platform_devices(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), of_bus_ids, core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(linkstation, declare_of_platform_devices);
#[no_mangle]
unsafe extern "C" fn linkstation_add_bridge(dev: *mut device_node) -> int __init {
    static int __init linkstation_add_bridge(struct device_node *dev)
    {

    int len;
    struct pci_controller *hose;
    const int *bus_range;
    printk("Adding PCI host bridge %pOF\n", dev);
    bus_range = of_get_property(dev, "bus-range", &len);
    if (bus_range == core::ptr::null_mut() || len < 2 * sizeof(int))
    printk(KERN_WARNING "Can't get bus-range for %pOF, assume"
    " bus 0\n", dev);
    hose = pcibios_alloc_controller(dev);
    if (hose == core::ptr::null_mut())
    return -ENOMEM;
    hose.first_busno = bus_range ? bus_range[0] : 0;
    hose.last_busno = bus_range ? bus_range[1] : 0xff;
    setup_indirect_pci(hose, 0xfec00000, 0xfee00000, 0);
// Interpret the "ranges" property
// This also maps the I/O region and sets isa_io/mem_base
    pci_process_bridge_OF_ranges(hose, dev, 1);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn linkstation_setup_arch() -> void __init {
    static void __init linkstation_setup_arch(void)
    {
    printk(KERN_INFO "BUFFALO Network Attached Storage Series\n");
    printk(KERN_INFO "(C) 2002-2005 BUFFALO INC.\n");
    }
#[no_mangle]
unsafe extern "C" fn linkstation_setup_pci() -> void __init {
    static void __init linkstation_setup_pci(void)
    {
    struct device_node *np;
// Lookup PCI host bridges
    for_each_compatible_node(np, "pci", "mpc10x-pci")
    linkstation_add_bridge(np);
    }
//
// Interrupt setup and service.  Interrupts on the linkstation come
// from the four PCI slots plus onboard 8241 devices: I2C, DUART.
//
#[no_mangle]
unsafe extern "C" fn linkstation_init_IRQ() -> void __init {
    static void __init linkstation_init_IRQ(void)
    {
    struct mpic *mpic;
    mpic = mpic_alloc(core::ptr::null_mut(), 0, 0, 4, 0, " EPIC     ");
    BUG_ON(mpic == core::ptr::null_mut());
// PCI IRQs
    mpic_assign_isu(mpic, 0, mpic.paddr + 0x10200);
// I2C
    mpic_assign_isu(mpic, 1, mpic.paddr + 0x11000);
// ttyS0, ttyS1
    mpic_assign_isu(mpic, 2, mpic.paddr + 0x11100);
    mpic_init(mpic);
    }
#[no_mangle]
unsafe extern "C" fn linkstation_restart(cmd: *mut c_char) -> void __noreturn {
    static void __noreturn linkstation_restart(char *cmd)
    {
    local_irq_disable();
// Reset system via AVR
    avr_uart_configure();
// Send reboot command
    avr_uart_send('C');
    for(;;)  /* Spin until reset happens */
    avr_uart_send('G');	/* "kick" */
    }
#[no_mangle]
unsafe extern "C" fn linkstation_power_off() -> void __noreturn {
    static void __noreturn linkstation_power_off(void)
    {
    local_irq_disable();
// Power down system via AVR
    avr_uart_configure();
// send shutdown command
    avr_uart_send('E');
    for(;;)  /* Spin until power-off happens */
    avr_uart_send('G');	/* "kick" */
// NOTREACHED
    }
#[no_mangle]
unsafe extern "C" fn linkstation_halt() -> void __noreturn {
    static void __noreturn linkstation_halt(void)
    {
    linkstation_power_off();
// NOTREACHED
    }
#[no_mangle]
unsafe extern "C" fn linkstation_show_cpuinfo(m: *mut seq_file) {
    static void linkstation_show_cpuinfo(struct seq_file *m)
    {
    seq_printf(m, "vendor\t\t: Buffalo Technology\n");
    seq_printf(m, "machine\t\t: Linkstation I/Kurobox(HG)\n");
    }
#[no_mangle]
unsafe extern "C" fn linkstation_probe() -> int __init {
    static int __init linkstation_probe(void)
    {
    pm_power_off = linkstation_power_off;
    return 1;
    }
    define_machine(linkstation){
    .name 			= "Buffalo Linkstation",
    .compatible		= "linkstation",
    .probe 			= linkstation_probe,
    .setup_arch 		= linkstation_setup_arch,
    .discover_phbs		= linkstation_setup_pci,
    .init_IRQ 		= linkstation_init_IRQ,
    .show_cpuinfo 		= linkstation_show_cpuinfo,
    .get_irq 		= mpic_get_irq,
    .restart 		= linkstation_restart,
    .halt	 		= linkstation_halt,
    };
