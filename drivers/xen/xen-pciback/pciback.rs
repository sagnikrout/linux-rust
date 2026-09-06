//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/xen/xen-pciback/pciback.h
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
// PCI Backend Common Data Structures & Function Declarations
//
// Author: Ryan Wilson <hap9@epoch.ncsc.mil>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_dev_entry {
    pub list: list_head,
    pub dev: *mut pci_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pcibk_device {
    pub pci_dev_data: *mut c_void,
    pub dev_lock: mutex,
    pub xdev: *mut xenbus_device,
    pub be_watch: xenbus_watch,
    pub be_watching: u8,
    pub evtchn_irq: c_int,
    pub sh_info: *mut xen_pci_sharedinfo,
    pub flags: c_ulong,
    pub op_work: work_struct,
    pub op: xen_pci_op,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pcibk_dev_data {
    pub config_fields: list_head,
    pub pci_saved_state: *mut pci_saved_state,
    pub permissive:1: c_uint,
    pub allow_interrupt_control:1: c_uint,
    pub warned_on_write:1: c_uint,
    pub enable_intx:1: c_uint,
    pub /: *mut *mut unsigned int isr_on:1; / Whether the IRQ handler is installed.,
    pub /: *mut *mut unsigned int ack_intr:1; / .. and ACK-ing,
    pub handled: c_ulong,
    pub /: *mut *mut unsigned int irq; / Saved in case device transitions to MSI/MSI-X,
    pub /: *mut *mut char irq_name[]; / xen-pcibk[000:04:00.0],
}

// Used by XenBus and xen_pcibk_ops.c
// Used by pcistub.c and conf_space_quirks.c
// Get/Put PCI Devices that are hidden from the PCI Backend Domain
extern "C" {
    pub fn pcistub_put_pci_dev(dev: *mut pci_dev);
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_XEN_PCIDEV_BACKEND) -> return;
}
// Ensure a device is turned off or reset
extern "C" {
    pub fn xen_pcibk_reset_device(pdev: *mut pci_dev);
}
// Access a virtual configuration space for a PCI device
extern "C" {
    pub fn xen_pcibk_config_init() -> c_int;
}
extern "C" {
    pub fn xen_pcibk_config_init_dev(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn xen_pcibk_config_free_dyn_fields(dev: *mut pci_dev);
}
extern "C" {
    pub fn xen_pcibk_config_reset_dev(dev: *mut pci_dev);
}
extern "C" {
    pub fn xen_pcibk_config_free_dev(dev: *mut pci_dev);
}
// Handle requests for specific devices from the frontend
// Backend registration for the two types of BDF representation:
// vpci - BDFs start at 00
// passthrough - BDFs are exactly like in the host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pcibk_backend {
    pub name: *const c_char,
    pub pdev): *mut *mut int (init)(struct xen_pcibk_device,
    pub pdev): *mut *mut void (free)(struct xen_pcibk_device,
    pub devfn): *mut c_uint,
    pub cb): *mut *mut *mut int (publish)(struct xen_pcibk_device pdev, publish_pci_root_cb,
    pub lock): bool,
    pub publish_cb): int devid, publish_pci_dev_cb,
    pub devfn): c_uint,
}

//
// Add for domain0 PCIE-AER handling. Get guest domain/bus/devfn in xen_pcibk
// before sending aer request to pcifront, so that guest could identify
// device, coopearte with xen_pcibk to finish aer recovery job if device driver
// has the capability
//
// Handles events from front-end
extern "C" {
    pub fn xen_pcibk_handle_event(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn xen_pcibk_do_op(data: *mut work_struct);
}
extern "C" {
    pub fn xen_pcibk_xenbus_register() -> c_int;
}
extern "C" {
    pub fn xen_pcibk_xenbus_unregister();
}
