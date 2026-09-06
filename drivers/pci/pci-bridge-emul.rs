//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/pci-bridge-emul.h
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

// PCI configuration space of a PCI-to-PCI bridge.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bridge_emul_conf {
    pub vendor: __le16,
    pub device: __le16,
    pub command: __le16,
    pub status: __le16,
    pub class_revision: __le32,
    pub cache_line_size: u8,
    pub latency_timer: u8,
    pub header_type: u8,
    pub bist: u8,
    pub bar: [__le32; 2],
    pub primary_bus: u8,
    pub secondary_bus: u8,
    pub subordinate_bus: u8,
    pub secondary_latency_timer: u8,
    pub iobase: u8,
    pub iolimit: u8,
    pub secondary_status: __le16,
    pub membase: __le16,
    pub memlimit: __le16,
    pub pref_mem_base: __le16,
    pub pref_mem_limit: __le16,
    pub prefbaseupper: __le32,
    pub preflimitupper: __le32,
    pub iobaseupper: __le16,
    pub iolimitupper: __le16,
    pub capabilities_pointer: u8,
    pub reserve: [u8; 3],
    pub romaddr: __le32,
    pub intline: u8,
    pub intpin: u8,
    pub bridgectrl: __le16,
}

// PCI configuration space of the PCIe capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bridge_emul_pcie_conf {
    pub cap_id: u8,
    pub next: u8,
    pub cap: __le16,
    pub devcap: __le32,
    pub devctl: __le16,
    pub devsta: __le16,
    pub lnkcap: __le32,
    pub lnkctl: __le16,
    pub lnksta: __le16,
    pub slotcap: __le32,
    pub slotctl: __le16,
    pub slotsta: __le16,
    pub rootctl: __le16,
    pub rootcap: __le16,
    pub rootsta: __le32,
    pub devcap2: __le32,
    pub devctl2: __le16,
    pub devsta2: __le16,
    pub lnkcap2: __le32,
    pub lnkctl2: __le16,
    pub lnksta2: __le16,
    pub slotcap2: __le32,
    pub slotctl2: __le16,
    pub slotsta2: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bridge_emul_ops {
//
// Called when reading from the regular PCI bridge
// configuration space. Return PCI_BRIDGE_EMUL_HANDLED when the
// operation has handled the read operation and filled in the
// *value, or PCI_BRIDGE_EMUL_NOT_HANDLED when the read should
// be emulated by the common code by reading from the
// in-memory copy of the configuration space.
//
    pub value): *mut int reg, u32,
//
// Same as ->read_base(), except it is for reading from the
// PCIe capability configuration space.
//
    pub value): *mut int reg, u32,
//
// Same as ->read_base(), except it is for reading from the
// PCIe extended capability configuration space.
//
    pub value): *mut int reg, u32,
//
// Called when writing to the regular PCI bridge configuration
// space. old is the current value, new is the new value being
// written, and mask indicates which parts of the value are
// being changed.
//
    pub mask): u32 old, u32 new, u32,
//
// Same as ->write_base(), except it is for writing from the
// PCIe capability configuration space.
//
    pub mask): u32 old, u32 new, u32,
//
// Same as ->write_base(), except it is for writing from the
// PCIe extended capability configuration space.
//
    pub mask): u32 old, u32 new, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bridge_emul {
    pub conf: pci_bridge_emul_conf,
    pub pcie_conf: pci_bridge_emul_pcie_conf,
    pub ops: *const pci_bridge_emul_ops,
    pub pci_regs_behavior: *mut pci_bridge_reg_behavior,
    pub pcie_cap_regs_behavior: *mut pci_bridge_reg_behavior,
    pub data: *mut c_void,
    pub pcie_start: u8,
    pub ssid_start: u8,
    pub has_pcie: bool,
    pub subsystem_vendor_id: u16,
    pub subsystem_id: u16,
}

//
// PCI bridge does not support forwarding of prefetchable memory
// requests between primary and secondary buses.
//
// PCI bridge does not support forwarding of IO requests between
// primary and secondary buses.
//
extern "C" {
    pub fn pci_bridge_emul_cleanup(bridge: *mut pci_bridge_emul);
}
