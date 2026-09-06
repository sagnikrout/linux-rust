//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ssb/ssb_driver_gige.h
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

pub const SSB_GIGE_PCIIO: c_uint = 0x0000 /* PCI I/O Registers (1024 bytes) */;
pub const SSB_GIGE_RESERVED: c_uint = 0x0400 /* Reserved (1024 bytes) */;
pub const SSB_GIGE_PCICFG: c_uint = 0x0800 /* PCI config space (256 bytes) */;
pub const SSB_GIGE_SHIM_FLUSHSTAT: c_uint = 0x0C00 /* PCI to OCP: Flush status control (32bit) */;
pub const SSB_GIGE_SHIM_FLUSHRDA: c_uint = 0x0C04 /* PCI to OCP: Flush read address (32bit) */;
pub const SSB_GIGE_SHIM_FLUSHTO: c_uint = 0x0C08 /* PCI to OCP: Flush timeout counter (32bit) */;
pub const SSB_GIGE_SHIM_BARRIER: c_uint = 0x0C0C /* PCI to OCP: Barrier register (32bit) */;
pub const SSB_GIGE_SHIM_MAOCPSI: c_uint = 0x0C10 /* PCI to OCP: MaocpSI Control (32bit) */;
pub const SSB_GIGE_SHIM_SIOCPMA: c_uint = 0x0C14 /* PCI to OCP: SiocpMa Control (32bit) */;
// TM Status High flags
pub const SSB_GIGE_TMSHIGH_RGMII: c_uint = 0x00010000 /* Have an RGMII PHY-bus */;
// TM Status Low flags
pub const SSB_GIGE_TMSLOW_TXBYPASS: c_uint = 0x00080000 /* TX bypass (no delay) */;
pub const SSB_GIGE_TMSLOW_RXBYPASS: c_uint = 0x00100000 /* RX bypass (no delay) */;
pub const SSB_GIGE_TMSLOW_DLLEN: c_uint = 0x01000000 /* Enable DLL controls */;
// Boardflags (low)
pub const SSB_GIGE_BFL_ROBOSWITCH: c_uint = 0x0010;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_gige {
    pub dev: *mut ssb_device,
    pub lock: spinlock_t,
// True, if the device has an RGMII bus.
// False, if the device has a GMII bus.
    pub has_rgmii: bool,
// The PCI controller device.
    pub pci_controller: pci_controller,
    pub pci_ops: pci_ops,
    pub mem_resource: resource,
    pub io_resource: resource,
}

// Check whether a PCI device is a SSB Gigabit Ethernet core.
extern "C" {
    pub fn pdev_is_ssb_gige_core(pdev: *mut pci_dev) -> bool;
}
// Convert a pci_dev pointer to a ssb_gige pointer.
extern "C" {
    pub fn container_of(_arg: pdev->bus->ops, ssb_gige: struct, _arg: pci_ops) -> return;
}
// Returns whether the PHY is connected by an RGMII bus.
// Returns whether we have a Roboswitch.
// Returns whether we can only do one DMA at once.
// Returns whether we must flush posted writes.
// Get the device MAC address
// Get the device phy address
// The GigE driver is not a standalone module, because we don't have support
// for unregistering the driver. So we could not unload the module anyway.
extern "C" {
    pub fn ssb_gige_init() -> c_int;
}
// Currently we can not unregister the GigE driver,
// because we can not unregister the PCI bridge.

// Gigabit Ethernet driver disabled

