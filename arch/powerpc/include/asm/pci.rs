//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/pci.h
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

// Return values for pci_controller_ops.probe_mode function

pub const PCIBIOS_MIN_IO: c_uint = 0x1000;
pub const PCIBIOS_MIN_MEM: c_uint = 0x10000000;
// Values for the `which' argument to sys_pciconfig_iobase syscall.
pub const IOBASE_BRIDGE_NUMBER: c_int = 0;
pub const IOBASE_MEMORY: c_int = 1;
pub const IOBASE_IO: c_int = 2;
pub const IOBASE_ISA_IO: c_int = 3;
pub const IOBASE_ISA_MEM: c_int = 4;
//
// Set this to 1 if you want the kernel to re-assign all PCI
// bus numbers (don't do that on ppc64 yet !)
//

extern "C" {
    pub fn set_pci_dma_ops(dma_ops: *const dma_map_ops) -> void __init;
}

// Macro flag: #define set_pci_dma_ops(d)

//
// We want to avoid touching the cacheline size or MWI bit.
// pSeries firmware sets the cacheline size (which is not the cpu cacheline
// size in all cases) and hardware treats MWI the same as memory write.
//
// Macro flag: #define PCI_DISABLE_MWI

extern "C" {
    pub fn pci_domain_nr(bus: *mut pci_bus) -> c_int;
}
// Decide whether to display the domain number in /proc
extern "C" {
    pub fn pci_proc_domain(bus: *mut pci_bus) -> c_int;
}
// Tell PCI code what kind of PCI resource mappings we support
pub const HAVE_PCI_MMAP: c_int = 1;
pub const ARCH_GENERIC_PCI_MMAP_RESOURCE: c_int = 1;
pub const arch_can_pci_mmap_io(): c_int = 1;
pub const arch_can_pci_mmap_wc(): c_int = 1;
pub const HAVE_PCI_LEGACY: c_int = 1;
extern "C" {
    pub fn pcibios_claim_one_bus(b: *mut pci_bus);
}
extern "C" {
    pub fn pcibios_finish_adding_to_bus(bus: *mut pci_bus);
}
extern "C" {
    pub fn pcibios_resource_survey();
}
extern "C" {
    pub fn remove_phb_dynamic(phb: *mut pci_controller) -> c_int;
}
extern "C" {
    pub fn pci_parse_of_flags(addr0: u32, bridge: c_int) -> c_uint;
}
extern "C" {
    pub fn of_scan_pci_bridge(dev: *mut pci_dev);
}
extern "C" {
    pub fn of_scan_bus(node: *mut device_node, bus: *mut pci_bus);
}
extern "C" {
    pub fn of_rescan_bus(node: *mut device_node, bus: *mut pci_bus);
}
extern "C" {
    pub fn pcibios_io_space_offset(hose: *mut pci_controller) -> resource_size_t;
}
extern "C" {
    pub fn pcibios_setup_bus_self(bus: *mut pci_bus);
}
extern "C" {
    pub fn pcibios_setup_phb_io_space(hose: *mut pci_controller);
}
extern "C" {
    pub fn pcibios_scan_phb(hose: *mut pci_controller);
}

