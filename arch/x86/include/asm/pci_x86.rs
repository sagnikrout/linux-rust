//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pci_x86.h
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
// Low-Level PCI Access for i386 machines.
//
// (c) 1999 Martin Mares <mj@ucw.cz>
//

pub const PCI_PROBE_BIOS: c_uint = 0x0001;
pub const PCI_PROBE_CONF1: c_uint = 0x0002;
pub const PCI_PROBE_CONF2: c_uint = 0x0004;
pub const PCI_PROBE_MMCONF: c_uint = 0x0008;
pub const PCI_PROBE_MASK: c_uint = 0x000f;
pub const PCI_PROBE_NOEARLY: c_uint = 0x0010;
pub const PCI_NO_CHECKS: c_uint = 0x0400;
pub const PCI_USE_PIRQ_MASK: c_uint = 0x0800;
pub const PCI_ASSIGN_ROMS: c_uint = 0x1000;
pub const PCI_BIOS_IRQ_SCAN: c_uint = 0x2000;
pub const PCI_ASSIGN_ALL_BUSSES: c_uint = 0x4000;
pub const PCI_CAN_SKIP_ISA_ALIGN: c_uint = 0x8000;
pub const PCI_USE__CRS: c_uint = 0x10000;
pub const PCI_CHECK_ENABLE_AMD_MMCONF: c_uint = 0x20000;
pub const PCI_HAS_IO_ECS: c_uint = 0x40000;
pub const PCI_NOASSIGN_ROMS: c_uint = 0x80000;
pub const PCI_ROOT_NO_CRS: c_uint = 0x100000;
pub const PCI_NOASSIGN_BARS: c_uint = 0x200000;
pub const PCI_BIG_ROOT_WINDOW: c_uint = 0x400000;
pub const PCI_USE_E820: c_uint = 0x800000;
pub const PCI_NO_E820: c_uint = 0x1000000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_bf_sort_state {
    pci_bf_sort_default,
    pci_force_nobf,
    pci_force_bf,
    pci_dmi_bf,
}

// pci-i386.c
extern "C" {
    pub fn pcibios_resource_survey();
}
extern "C" {
    pub fn pcibios_set_cache_line_size();
}
// pci-pc.c
extern "C" {
    pub fn pcibios_scan_specific_bus(busn: c_int);
}
// pci-irq.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_info {
    pub /: *mut *mut u8 bus, devfn; / Bus, device and function,
    pub dependent,: *mut *mut u8 link; / IRQ line ID, chipset,
    pub /: *mut *mut u16 bitmap; / Available IRQs,
    pub irq: [} __attribute__((packed)); 4],
    pub /: *mut *mut u8 slot; / Slot number, 0=onboard,
    pub rfu: u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_routing_table {
    pub /: *mut *mut u32 signature; / PIRQ_SIGNATURE should be here,
    pub /: *mut *mut u16 version; / PIRQ_VERSION,
    pub /: *mut *mut u16 size; / Table size in bytes,
    pub /: *mut *mut u8 rtr_bus, rtr_devfn; / Where the interrupt router lies,
    pub to: *mut *mut u16 exclusive_irqs; / IRQs devoted exclusively,
    pub of: *mut *mut u16 rtr_vendor, rtr_device; / Vendor and device ID,
    pub /: *mut *mut u32 miniport_data; / Crap,
    pub rfu: [u8; 11],
    pub /: *mut *mut u8 checksum; / Modulo 256 checksum must give 0,
    pub slots: [irq_info; ],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irt_routing_table {
    pub /: *mut *mut u32 signature; / IRT_SIGNATURE should be here,
    pub /: *mut *mut u8 size; / Number of entries provided,
    pub /: *mut *mut u8 used; / Number of entries actually used,
    pub to: *mut *mut u16 exclusive_irqs; / IRQs devoted exclusively,
    pub slots: [irq_info; ],
    pub __attribute__((packed)): },
    pub pcibios_irq_mask: extern unsigned int,
    pub pci_config_lock: extern raw_spinlock_t,
    pub dev): *mut *mut extern int (pcibios_enable_irq)(struct pci_dev,
    pub dev): *mut *mut extern void (pcibios_disable_irq)(struct pci_dev,
    pub dev): *mut extern bool mp_should_keep_irq(struct device,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_raw_ops {
    pub val): *mut int reg, int len, u32,
    pub val): int reg, int len, u32,
}

// arch_initcall level

extern "C" {
    pub fn pci_direct_probe() -> c_int;
}
extern "C" {
    pub fn pci_direct_init(type: c_int);
}

extern "C" {
    pub fn pci_pcbios_init();
}

extern "C" {
    pub fn dmi_check_pciprobe() -> void __init;
}
extern "C" {
    pub fn dmi_check_skip_isa_align() -> void __init;
}
// some common used subsys_initcalls

extern "C" {
    pub fn pci_acpi_init() -> int __init;
}

extern "C" {
    pub fn pcibios_irq_init() -> void __init;
}
extern "C" {
    pub fn pcibios_init() -> int __init;
}
extern "C" {
    pub fn pci_legacy_init() -> c_int;
}
extern "C" {
    pub fn pcibios_fixup_irqs();
}
// pci-mmconfig.c
// "PCI MMCONFIG %04x [bus %02x-%02x]"

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_mmcfg_region {
    pub list: list_head,
    pub res: resource,
    pub address: u64,
    pub virt: *mut char __iomem,
    pub segment: u16,
    pub start_bus: u8,
    pub end_bus: u8,
    pub name: [c_char; PCI_MMCFG_RESOURCE_NAME_LEN],
}

extern "C" {
    pub fn pci_mmcfg_arch_init() -> int __init;
}
extern "C" {
    pub fn pci_mmcfg_arch_free() -> void __init;
}
extern "C" {
    pub fn pci_mmcfg_arch_map(cfg: *mut pci_mmcfg_region) -> c_int;
}
extern "C" {
    pub fn pci_mmcfg_arch_unmap(cfg: *mut pci_mmcfg_region);
}
extern "C" {
    pub fn pci_mmconfig_delete(seg: u16, start: u8, end: u8) -> c_int;
}

//
// On AMD Fam10h CPUs, all PCI MMIO configuration space accesses must use
// %eax.  No other source or target registers may be used.  The following
// mmio_config_* accessors enforce this.  See "BIOS and Kernel Developer's
// Guide (BKDG) For AMD Family 10h Processors", rev. 3.48, sec 2.11.1,
// "MMIO Configuration Coding Requirements".
//
extern "C" {
    pub fn volatile((%1): "movb, (pos): %%al" : "=a" (val) : "r") -> asm;
}
extern "C" {
    pub fn volatile((%1): "movw, (pos): %%ax" : "=a" (val) : "r") -> asm;
}
extern "C" {
    pub fn volatile((%1): "movl, (pos): %%eax" : "=a" (val) : "r") -> asm;
}
extern "C" {
    pub fn volatile(%%al: "movb, (val): (%1)" : : "a", "memory": "r" (pos) :) -> asm;
}
extern "C" {
    pub fn volatile(%%ax: "movw, (val): (%1)" : : "a", "memory": "r" (pos) :) -> asm;
}
extern "C" {
    pub fn volatile(%%eax: "movl, (val): (%1)" : : "a", "memory": "r" (pos) :) -> asm;
}

