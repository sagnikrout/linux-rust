//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pci-tsm.h
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
// struct pci_tsm_ops - manage confidential links and security state
// @link_ops: Coordinate PCIe SPDM and IDE establishment via a platform TSM.
// Provide a secure session transport for TDISP state management
// (typically bare metal physical function operations).
// @devsec_ops: Lock, unlock, and interrogate the security state of the
// function via the platform TSM (typically virtual function
// operations).
//
// This operations are mutually exclusive either a tsm_dev instance
// manages physical link properties or it manages function security
// states like TDISP lock/unlock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_tsm_ops {
//
// struct pci_tsm_link_ops - Manage physical link and the TSM/DSM session
// @probe: establish context with the TSM (allocate / wrap 'struct
// pci_tsm') for follow-on link operations
// @remove: destroy link operations context
// @connect: establish / validate a secure connection (e.g. IDE)
// with the device
// @disconnect: teardown the secure link
// @bind: bind a TDI in preparation for it to be accepted by a TVM
// @unbind: remove a TDI from secure operation with a TVM
// @guest_req: marshal TVM information and state change requests
//
// Context: @probe, @remove, @connect, and @disconnect run under
// pci_tsm_rwsem held for write to sync with TSM unregistration and
// mutual exclusion of @connect and @disconnect. @connect and
// @disconnect additionally run under the DSM lock (struct
// pci_tsm_pf0::lock) as well as @probe and @remove of the subfunctions.
// @bind, @unbind, and @guest_req run under pci_tsm_rwsem held for read
// and the DSM lock.
//
    pub pdev): *mut pci_dev,
    pub tsm): *mut *mut void (remove)(struct pci_tsm,
    pub pdev): *mut *mut int (connect)(struct pci_dev,
    pub pdev): *mut *mut void (disconnect)(struct pci_dev,
    pub tdi_id): *mut *mut kvm kvm, u32,
    pub tdi): *mut *mut void (unbind)(struct pci_tdi,
    pub tsm_code): *mut u64,
//
// struct pci_tsm_devsec_ops - Manage the security state of the function
// @lock: establish context with the TSM (allocate / wrap 'struct
// pci_tsm') for follow-on security state transitions from the
// LOCKED state
// @unlock: destroy TSM context and return device to UNLOCKED state
//
// Context: @lock and @unlock run under pci_tsm_rwsem held for write to
// sync with TSM unregistration and each other
//
    pub pdev): *mut pci_dev,
    pub tsm): *mut *mut void (unlock)(struct pci_tsm,
}

//
// struct pci_tdi - Core TEE I/O Device Interface (TDI) context
// @pdev: host side representation of guest-side TDI
// @kvm: TEE VM context of bound TDI
// @tdi_id: Identifier (virtual BDF) for the TDI as referenced by the TSM and DSM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_tdi {
    pub pdev: *mut pci_dev,
    pub kvm: *mut kvm,
    pub tdi_id: u32,
}

//
// struct pci_tsm - Core TSM context for a given PCIe endpoint
// @pdev: Back ref to device function, distinguishes type of pci_tsm context
// @dsm_dev: PCI Device Security Manager for link operations on @pdev
// @tsm_dev: PCI TEE Security Manager device for Link Confidentiality or Device
// Function Security operations
// @tdi: TDI context established by the @bind link operation
//
// This structure is wrapped by low level TSM driver data and returned by
// probe()/lock(), it is freed by the corresponding remove()/unlock().
//
// For link operations it serves to cache the association between a Device
// Security Manager (DSM) and the functions that manager can assign to a TVM.
// That can be "self", for assigning function0 of a TEE I/O device, a
// sub-function (SR-IOV virtual function, or non-function0
// multifunction-device), or a downstream endpoint (PCIe upstream switch-port as
// DSM).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_tsm {
    pub pdev: *mut pci_dev,
    pub dsm_dev: *mut pci_dev,
    pub tsm_dev: *mut tsm_dev,
    pub tdi: *mut pci_tdi,
}

//
// struct pci_tsm_pf0 - Physical Function 0 TDISP link context
// @base_tsm: generic core "tsm" context
// @lock: mutual exclustion for pci_tsm_ops invocation
// @doe_mb: PCIe Data Object Exchange mailbox
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_tsm_pf0 {
    pub base_tsm: pci_tsm,
    pub lock: mutex,
    pub doe_mb: *mut pci_doe_mb,
}

// physical function0 and capable of 'connect'
//
// Allow for a Device Security Manager (DSM) associated with function0
// of an Endpoint to coordinate TDISP requests for other functions
// (physical or virtual) of the device, or allow for an Upstream Port
// DSM to accept TDISP requests for the Endpoints downstream of the
// switch.
//
// enum pci_tsm_req_scope - Scope of guest requests to be validated by TSM
//
// Guest requests are a transport for a TVM to communicate with a TSM + DSM for
// a given TDI. A TSM driver is responsible for maintaining the kernel security
// model and limit commands that may affect the host, or are otherwise outside
// the typical TDISP operational model.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_tsm_req_scope {
//
// @PCI_TSM_REQ_INFO: Read-only, without side effects, request for
// typical TDISP collateral information like Device Interface Reports.
// No device secrets are permitted, and no device state is changed.
//
    PCI_TSM_REQ_INFO = 0,
//
// @PCI_TSM_REQ_STATE_CHANGE: Request to change the TDISP state from
// UNLOCKED->LOCKED, LOCKED->RUN, or other architecture specific state
// changes to support those transitions for a TDI. No other (unrelated
// to TDISP) device / host state, configuration, or data change is
// permitted.
//
    PCI_TSM_REQ_STATE_CHANGE = 1,
//
// @PCI_TSM_REQ_DEBUG_READ: Read-only request for debug information
//
// A method to facilitate TVM information retrieval outside of typical
// TDISP operational requirements. No device secrets are permitted.
//
    PCI_TSM_REQ_DEBUG_READ = 2,
//
// @PCI_TSM_REQ_DEBUG_WRITE: Device state changes for debug purposes
//
// The request may affect the operational state of the device outside of
// the TDISP operational model. If allowed, requires CAP_SYS_RAW_IO, and
// will taint the kernel.
//
    PCI_TSM_REQ_DEBUG_WRITE = 3,
}

extern "C" {
    pub fn pci_tsm_register(tsm_dev: *mut tsm_dev) -> c_int;
}
extern "C" {
    pub fn pci_tsm_unregister(tsm_dev: *mut tsm_dev);
}
extern "C" {
    pub fn pci_tsm_pf0_destructor(tsm: *mut pci_tsm_pf0);
}
extern "C" {
    pub fn pci_tsm_bind(pdev: *mut pci_dev, kvm: *mut kvm, tdi_id: u32) -> c_int;
}
extern "C" {
    pub fn pci_tsm_unbind(pdev: *mut pci_dev);
}

